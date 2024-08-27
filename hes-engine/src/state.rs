use std::collections::BTreeMap;

use crate::{
    events::{
        Condition,
        Effect,
        Event,
        EventPool,
        Flag,
        Phase,
        Request,
    },
    kinds::*,
    npcs::NPC,
    outputs,
    production::{calculate_required, produce, ProcessChanges},
    projects::{
        Group,
        Outcome,
        Project,
        ProjectChanges,
        Status,
        Type as ProjectType,
    },
    resources,
    world::World,
    Collection,
    Id,
};
use serde::{Deserialize, Serialize};

const LIFESPAN: usize = 60;
const PRODUCTION_SHORTAGE_PENALTY: f32 = 60.;

/// Have to all be below these values to win
const WIN_EMISSIONS: f32 = 0.0;
const WIN_EXTINCTION: f32 = 20.0;
const WIN_TEMPERATURE: f32 = 1.0;

/// Represents the game state.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct State {
    pub world: World,
    pub runs: usize,

    pub game_over: bool,
    pub death_year: usize,

    pub political_capital: isize,
    pub research_points: isize,
    pub npcs: Collection<NPC>,

    // Requests: (
    //  request type,
    //  entity id,
    //  state (active: true/false),
    //  political capital bounty
    // )
    pub requests: Vec<(Request, Id, bool, usize)>,
    pub flags: Vec<Flag>,

    // Keep track of what policies
    // need to have rolled outcomes
    pub policy_queue: Vec<Id>,

    pub produced: Production,
    pub resources: Resources,
    pub feedstocks: Feedstocks,
    pub output_demand: OutputDemand,
    pub resource_demand: ResourceDemand,

    /// Note that the biodiversity values here
    /// should be ignored, as we do the proper
    /// scaling in `Process::extinction_rate`.
    pub byproducts: Byproducts,

    pub protected_land: f32,

    pub shortages_outlook: f32,
    pub emissions: Emissions,
    pub last_outlook: f32,

    pub events: Vec<Event>,

    pub event_pool: EventPool,
}

impl Default for State {
    fn default() -> Self {
        Self::new(World::default())
    }
}

impl State {
    pub fn new(mut world: World) -> State {
        let mut npcs = NPC::load();
        let n_npcs =
            npcs.iter().filter(|npc| !npc.locked).count()
                as f32;
        for npc in npcs.iter_mut() {
            if !npc.locked {
                npc.seats = 1. / n_npcs;
            }
        }

        // Ensure that all projects have at least one outcome.
        for project in world.projects.iter_mut() {
            if project.outcomes.is_empty() {
                project.outcomes.push(Outcome::default());
            }
        }

        let events = world.events.clone();
        let death_year = world.year + LIFESPAN;

        let resources = Reserve::from(world.starting_resources);
        let feedstocks =
            Reserve::from(world.feedstock_reserves);

        let mut state = State {
            npcs,
            world,
            political_capital: 100,
            research_points: 0,
            death_year,
            resources,
            feedstocks,

            protected_land: 0.1, // Starts at 10%

            events: vec![],
            event_pool: EventPool::new(events),

            runs: 0,
            game_over: false,

            last_outlook: 0.,
            shortages_outlook: 0.,
            emissions: Emissions::default(),
            produced: Production::default(),
            output_demand: OutputDemand::default(),
            resource_demand: ResourceDemand::default(),
            byproducts: Byproducts::default(),

            flags: vec![],
            requests: vec![],
            policy_queue: vec![],
        };
        state.initialize();
        state
    }

    fn initialize(&mut self) {
        self.last_outlook = self.outlook();
        self.update_demand();
        self.step_production();
        self.update_project_costs();
        self.world.update_climate(self.world.temperature);
    }

    /// If we won the game.
    pub fn won(&self) -> bool {
        self.emissions.as_gtco2eq() <= WIN_EMISSIONS
            && self.world.extinction_rate <= WIN_EXTINCTION
            && self.world.temperature <= WIN_TEMPERATURE
    }

    pub fn things_are_good(&self) -> bool {
        self.world.temperature <= 1.
            || self.world.extinction_rate <= 20.
            || self.emissions.as_gtco2eq() <= 0.
    }

    pub fn apply_disaster(
        &mut self,
        intensity: isize,
        region_id: &Id,
    ) {
        self.world.regions[region_id].base_habitability -=
            intensity as f32;
    }

    pub fn outlook(&self) -> f32 {
        self.world.outlook() - self.shortages_outlook
    }

    pub fn change_political_capital(&mut self, amount: isize) {
        self.political_capital += amount;
    }

    pub fn collect_research_points(&mut self) -> isize {
        let points = self.research_points;
        self.research_points = 0;
        points
    }

    pub fn step_year(&mut self, tgav: f32) -> Vec<Update> {
        let mut updates = vec![];
        let changes = self.step_projects();
        for (id, changes) in changes {
            if changes.completed {
                updates.push(Update::Project { id });
            }
            self.apply_changes(changes);
        }
        self.update_demand();
        self.step_production();
        updates.extend(self.step_world(tgav));
        self.world.year += 1;

        if self.is_planning_year() {
            let mut outcomes = self.roll_new_policy_outcomes();
            updates.append(&mut outcomes);
        }

        updates
    }

    pub fn is_planning_year(&self) -> bool {
        self.world.year % 5 == 0
    }

    pub fn apply_effects(
        &mut self,
        effects: &[Effect],
        region_id: Option<Id>,
    ) {
        for effect in effects {
            effect.apply(self, region_id);
        }
    }

    pub fn apply_event(
        &mut self,
        event_id: Id,
        region_id: Option<Id>,
    ) {
        let mut effects = vec![];
        let event = &self.event_pool.events[&event_id];
        self.events.push(event.clone());

        for effect in &event.effects {
            effects.push((effect.clone(), region_id));
        }

        for (effect, region_id) in effects {
            effect.apply(self, region_id);
        }
    }

    pub fn eval_conditions(
        &self,
        conditions: &[Condition],
        region_id: Option<Id>,
    ) -> bool {
        if conditions.is_empty() {
            true
        } else {
            conditions.iter().all(|c| c.eval(self, region_id))
        }
    }

    fn apply_changes<C: Changes>(&mut self, changes: C) {
        changes.apply(self);
        self.update_demand();
    }

    /// Recompute base demands from scratch.
    /// NOTE: This is the *only* place the base
    /// demands should be updated. If base demand
    /// needs to be influenced elsewhere then use
    /// `modifiable.modifier` or `modifiable.factor`
    /// instead.
    fn update_demand(&mut self) {
        let (
            output_demand,
            mut resource_demand,
            industry_byproducts,
        ) = {
            let world = &self.world;
            let mut output_demand = outputs!();
            let mut resource_demand = resources!();

            // Ignore electric/fuel, captured by everything else
            let world_demand = world.region_demand();
            output_demand.animal_calories +=
                world_demand.animal_calories;
            output_demand.plant_calories +=
                world_demand.plant_calories;

            // Demand and impacts from non-modeled industries
            let lic_pop = world.lic_population();
            let industry_demand =
                world.industries.resource_demand(lic_pop);
            let industry_byproducts =
                world.industries.byproducts(lic_pop);

            output_demand.fuel = industry_demand.fuel;
            output_demand.electricity =
                industry_demand.electricity;

            // Water and land demand from industries.
            // Process resource demand will be added later.
            resource_demand.water = industry_demand.water;
            resource_demand.land = industry_demand.land;

            // Electrification is only relevant for non-modeled industry;
            // for processes we always rely on their actual values.
            if self.flags.contains(&Flag::Electrified) {
                let electrified = output_demand.fuel * 0.8;
                output_demand.electricity += electrified;
                output_demand.fuel -= electrified;
            }

            // For vegan-ish diets, move some animal calorie
            // demand to plants.
            let cal_change =
                if self.flags.contains(&Flag::Vegan) {
                    output_demand.animal_calories * 0.9
                } else if self.flags.contains(&Flag::Vegetarian)
                {
                    output_demand.animal_calories * 0.75
                } else {
                    0.
                };
            output_demand.animal_calories -= cal_change;
            output_demand.plant_calories += cal_change;

            (
                output_demand,
                resource_demand,
                industry_byproducts,
            )
        };
        self.output_demand.base = output_demand;

        // Generate production orders based on current process mixes and demand
        let total_demand = self.output_demand.total();
        let orders = self.world.processes.orders(&total_demand);

        // Calculate required resources so we can add in food energy requirements
        let (required_resources, required_feedstocks) =
            calculate_required(&orders);
        self.output_demand.base.electricity +=
            required_resources.electricity;
        self.output_demand.base.fuel += required_resources.fuel;

        // Now re-calculate orders
        let total_demand = self.output_demand.total();
        let orders = self.world.processes.orders(&total_demand);

        // Apply land protection
        self.resources.available.land =
            self.world.starting_resources.land
                * (1. - self.protected_land);

        // Run production function
        let (
            produced_by_process,
            produced_by_type,
            consumed_resources,
            consumed_feedstocks,
            production_byproducts,
        ) = produce(
            &orders,
            &self.resources.available,
            &self.feedstocks.available,
        );

        self.produced.by_process = produced_by_process;
        self.produced.amount = produced_by_type;

        resource_demand.water += consumed_resources.water;
        resource_demand.land += consumed_resources.land;
        resource_demand.fuel = output_demand.fuel;
        resource_demand.electricity = output_demand.electricity;
        self.resource_demand.base = resource_demand;
        self.resources.consumed = consumed_resources;
        self.resources.required = required_resources;

        self.feedstocks.consumed = consumed_feedstocks;
        self.feedstocks.required = required_feedstocks;

        self.byproducts.base =
            production_byproducts + industry_byproducts;
        self.emissions.update(self.byproducts.total());
    }

    fn step_production(&mut self) {
        self.feedstocks.consume(self.feedstocks.consumed);

        // Water and land aren't "consumed" as land
        // can obviously be re-purposed and we assume water
        // is more or less renewable.
        // Then we subtract out fuel and electricity usage
        // that's compensated by production.
        let mut consumed_resources = self.resources.consumed;
        consumed_resources.water = 0.;
        consumed_resources.land = 0.;
        consumed_resources.fuel -=
            self.produced.of(Output::Fuel);
        consumed_resources.electricity -=
            self.produced.of(Output::Fuel);
        self.resources.consume(consumed_resources);

        // Weigh resources by scarcity;
        // higher weight = higher scarcity
        let mut resource_weights = self.resources.scarcity();
        resource_weights.electricity = 2.;
        let mut feedstock_weights = self.feedstocks.scarcity();
        feedstock_weights.soil = 0.; // TODO add this back in?
        feedstock_weights.other = 0.;

        // Outlook impacts based on production shortages
        // If all demand met is 0 it should be an instant game over, basically.
        let demand_met =
            self.produced.total() / self.output_demand.total();
        self.shortages_outlook = (PRODUCTION_SHORTAGE_PENALTY
            - ((demand_met.fuel
                + demand_met.electricity
                + demand_met.animal_calories
                + demand_met.plant_calories)
                * PRODUCTION_SHORTAGE_PENALTY
                / 4.))
            .max(0.);

        self.world
            .update_extinction_rate(&self.produced.by_process);
    }

    fn step_world(&mut self, tgav: f32) -> Vec<Update> {
        if self.world.year >= self.death_year {
            self.game_over = true;
        }

        self.world.update_populations();
        let temp_change = self.world.update_climate(tgav);

        let stop = self.flags.contains(&Flag::StopDevelopment);
        let fast = self.flags.contains(&Flag::FastDevelopment);
        let degrow = self.flags.contains(&Flag::Degrowth);
        let (regions_up, regions_down) =
            self.world.regions.develop(stop, fast, degrow);

        let wretched_ally = self.npcs.is_ally("The Fanonist");
        let consumerist_ally =
            self.npcs.is_ally("The Consumerist");
        self.world.update_outlook(
            temp_change,
            wretched_ally,
            consumerist_ally,
        );
        regions_up
            .into_iter()
            .map(|id| Update::Region { id, up: true })
            .chain(
                regions_down
                    .into_iter()
                    .map(|id| Update::Region { id, up: false }),
            )
            .collect()
    }

    // Every planning cycle
    pub fn finish_cycle(&mut self) {
        let outlook_change = self.outlook() - self.last_outlook;
        let recent_projects: Vec<&Project> = self
            .world
            .projects
            .recent(self.world.year)
            .collect();
        self.npcs
            .update_seats(outlook_change, &recent_projects);
        self.last_outlook = self.outlook();
    }

    pub fn check_requests(
        &mut self,
    ) -> Vec<(Request, Id, bool, usize)> {
        let mut i = 0;
        let mut completed = Vec::new();
        while i < self.requests.len() {
            let (kind, id, active, bounty) =
                self.requests[i].clone();
            let complete = match kind {
                Request::Project => {
                    let project = &self.world.projects[&id];
                    (active
                        && (project.status == Status::Active
                            || project.status
                                == Status::Finished))
                        || (!active
                            && (project.status
                                == Status::Inactive
                                || project.status
                                    == Status::Halted))
                }
                Request::Process => {
                    let process = &self.world.processes[&id];
                    (active && process.is_promoted())
                        || (!active && process.is_banned())
                }
            };
            if complete {
                self.requests.remove(i);
                completed.push((kind, id, active, bounty));
            } else {
                i += 1;
            }
        }
        completed
    }

    pub fn change_process_mix_share(
        &mut self,
        process_id: &Id,
        change: isize,
    ) {
        let changes = self.world.processes[process_id]
            .change_mix_share(change);
        self.apply_changes(changes);
    }

    pub fn process_max_share(&self, process_id: &Id) -> usize {
        let output_demand = self.output_demand.total();
        let feedstocks = self.feedstocks.available;
        self.world.processes[process_id]
            .max_share(&output_demand, &feedstocks)
    }

    pub fn roll_events(
        &mut self,
        phase: Phase,
    ) -> Vec<ResolvedEvent> {
        let mut pool = self.event_pool.clone();
        let events = pool.roll_for_phase(phase, &self);
        self.event_pool = pool;

        let events: Vec<ResolvedEvent> = events
            .into_iter()
            .map(|(ev, region_id)| ResolvedEvent {
                event: ev,
                region: region_id.map(|id| {
                    (
                        id,
                        self.world.regions[&id]
                            .name
                            .to_string(),
                    )
                }),
            })
            .collect();

        // Icon events, aka disasters,
        // are handled differently, so we don't
        // apply their effects immediately here.
        if phase != Phase::Icon {
            for ev in &events {
                self.apply_event(
                    ev.id,
                    ev.region.as_ref().map(|(id, _)| *id),
                );
            }
        }

        events
    }
}

// Project related functionality.
impl State {
    fn step_projects(&mut self) -> Vec<(Id, ProjectChanges)> {
        let mut changes =
            self.world.projects.step(self.world.year);

        let mut outcomes: Vec<(Id, usize)> = Vec::new();
        for (id, changes) in &mut changes {
            if changes.completed {
                let project = &self.world.projects[&id];
                match self.roll_project_outcome(project) {
                    Some((outcome, i)) => {
                        for effect in &outcome.effects {
                            changes
                                .add_effects
                                .push(effect.clone());
                        }
                        outcomes.push((*id, i));
                    }
                    None => (),
                }
            }
        }

        for (id, i) in outcomes {
            self.world.projects[&id].active_outcome = Some(i);
        }

        self.update_project_costs();

        changes
    }

    fn update_project_costs(&mut self) {
        let base_modifier = self.base_project_cost_modifier();
        let total_demand = self.output_demand.total();
        let income_level = self.world.regions.income_level();

        let posadist_ally = self.npcs.is_ally("The Posadist");
        let utopian_ally = self.npcs.is_ally("The Utopian");
        let animal_ally =
            self.npcs.is_ally("The Animal Liberationist");
        let environ_ally =
            self.npcs.is_ally("The Environmentalist");
        let ecofem_ally = self.npcs.is_ally("The Ecofeminist");
        let malthus_ally = self.npcs.is_ally("The Malthusian");

        for project in self.world.projects.iter_mut() {
            let mut group_modifier = 1.0;
            if posadist_ally && project.group == Group::Nuclear
            {
                group_modifier *= 0.5;
            }
            if ecofem_ally
                && matches!(
                    project.group,
                    Group::Food
                        | Group::Agriculture
                        | Group::Protection
                )
            {
                group_modifier *= 0.75;
            }
            if utopian_ally
                && matches!(
                    project.group,
                    Group::Limits
                        | Group::Protection
                        | Group::Restoration
                )
            {
                group_modifier *= 0.75;
            }
            if environ_ally
                && project.group == Group::Protection
            {
                group_modifier *= 0.5;
            }
            if animal_ally && project.group == Group::Food {
                group_modifier *= 0.5;
            }
            if malthus_ally
                && project.group == Group::Population
            {
                group_modifier *= 0.5;
            }
            if self.flags.contains(&Flag::EcosystemModeling)
                && project.group == Group::Restoration
            {
                group_modifier *= 1.1;
            }
            project.update_cost(
                self.world.year,
                income_level,
                &total_demand,
                // Modifier only relevant for built projects,
                // not policies.
                if project.kind == ProjectType::Policy {
                    1.0
                } else {
                    base_modifier * group_modifier
                },
            );
            project.update_required_majority(&self.npcs);
        }
    }

    fn base_project_cost_modifier(&self) -> f32 {
        let mut modifier =
            if self.flags.contains(&Flag::MetalsShortage)
                && !self.flags.contains(&Flag::DeepSeaMining)
            {
                0.8
            } else {
                1.
            };
        if self.flags.contains(&Flag::MoreLabor) {
            modifier *= 0.9;
        }
        if self.flags.contains(&Flag::MoreAutomation) {
            modifier *= 0.9;
        }
        if self.flags.contains(&Flag::MoreLeisure) {
            modifier *= 1.1;
        }
        if self.flags.contains(&Flag::LaborResistance) {
            modifier *= 1.05;
        }
        if self.flags.contains(&Flag::LaborSabotage) {
            modifier *= 1.05;
        }
        modifier
    }

    pub fn start_project(&mut self, project_id: &Id) {
        let is_policy = self.world.projects[project_id].start();
        if is_policy {
            self.policy_queue.push(*project_id);
        }
    }

    pub fn stop_project(&mut self, project_id: &Id) {
        let (changes, is_policy) =
            self.world.projects[project_id].stop();
        if is_policy {
            self.policy_queue.retain(|&id| id != *project_id);
        }
        self.apply_changes(changes);
    }

    pub fn upgrade_project(&mut self, project_id: &Id) {
        let changes = self.world.projects[project_id].upgrade();
        self.apply_changes(changes);
    }

    pub fn downgrade_project(&mut self, project_id: &Id) {
        let changes =
            self.world.projects[project_id].downgrade();
        self.apply_changes(changes);
    }

    pub fn set_project_points(
        &mut self,
        project_id: &Id,
        points: usize,
    ) {
        self.world.projects[project_id].set_points(points);
    }

    /// Roll to see the outcome of this project
    fn roll_project_outcome<'a>(
        &self,
        project: &'a Project,
    ) -> Option<(&'a Outcome, usize)> {
        let mut outcome = None;
        for (i, o) in project.outcomes.iter().enumerate() {
            match o.probability.eval(self, None) {
                Some(likelihood) => {
                    let prob = likelihood.p();
                    if fastrand::f32() <= prob {
                        outcome = Some((o, i));
                        break;
                    }
                }
                None => (),
            }
        }
        if outcome.is_none() {
            outcome = Some((&project.outcomes[0], 0));
        }
        outcome
    }

    fn roll_new_policy_outcomes(&mut self) -> Vec<Update> {
        let mut effects: Vec<Effect> = Vec::new();
        let ids: Vec<Id> =
            self.policy_queue.drain(..).collect();
        for id in &ids {
            let mut active_outcome = None;
            let proj = &self.world.projects[id];
            match self.roll_project_outcome(proj) {
                Some((outcome, i)) => {
                    for effect in &outcome.effects {
                        effects.push(effect.clone());
                    }
                    active_outcome = Some(i);
                }
                None => (),
            }
            let proj = &mut self.world.projects[id];
            proj.active_outcome = active_outcome;
            proj.status = Status::Active;
            for effect in &proj.effects {
                effects.push(effect.clone());
            }
        }

        for effect in effects {
            effect.apply(self, None);
        }
        self.update_demand();

        ids.into_iter()
            .map(|id| Update::Policy { id })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Production {
    pub factor: OutputMap,
    pub amount: OutputMap,
    pub by_process: BTreeMap<Id, f32>,
}
impl Default for Production {
    fn default() -> Self {
        Self {
            factor: OutputMap::splat(1.),
            amount: OutputMap::default(),
            by_process: BTreeMap::default(),
        }
    }
}
impl Production {
    pub fn of(&self, output: Output) -> f32 {
        self.amount[output] * self.factor[output]
    }

    pub fn total(&self) -> OutputMap {
        self.amount * self.factor
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Emissions {
    pub co2: f32,
    pub ch4: f32,
    pub n2o: f32,
}
impl Emissions {
    pub fn update(&mut self, byproducts: ByproductMap) {
        self.co2 = byproducts.co2;
        self.ch4 = byproducts.ch4;
        self.n2o = byproducts.n2o;
    }

    pub fn as_co2eq(&self) -> f32 {
        self.co2 + (self.n2o * 298.) + (self.ch4 * 36.)
    }

    pub fn as_gtco2eq(&self) -> f32 {
        self.as_co2eq() * 1e-15
    }

    /// Convert to units expected by Hector:
    /// - CO2: Pg C/y
    /// - CH4: Tg/y
    /// - N2O: Tg/y
    ///
    /// Note that Hector separates out FFI and LUC emissions
    /// but we lump them together.
    ///
    /// Units: <https://github.com/JGCRI/hector/wiki/Hector-Units>
    pub fn for_hector(&self) -> (f32, f32, f32) {
        let co2 = self.co2 * 12. / 44. * 1e-15; // Pg C/y
        let ch4 = self.ch4 * 1e-12; // Tg/y
        let n2o = self.n2o * 1e-12; // Tg/y
        (co2, ch4, n2o)
    }
}

pub trait Changes {
    fn apply(self, state: &mut State);
}
impl Changes for ProcessChanges {
    fn apply(self, state: &mut State) {
        for (id, change) in self.relationships {
            state.npcs[&id].relationship += change;
        }
    }
}
impl Changes for ProjectChanges {
    fn apply(self, state: &mut State) {
        for effect in self.remove_effects {
            effect.unapply(state, None);
        }
        for effect in self.add_effects {
            effect.apply(state, None);
        }
        for (id, change) in self.relationships {
            state.npcs[&id].relationship += change;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedEvent {
    pub event: Event,
    pub region: Option<(Id, String)>,
}
impl std::ops::Deref for ResolvedEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Update {
    Region {
        id: Id,
        up: bool, // or down
    },
    Policy {
        id: Id,
    },
    Project {
        id: Id,
    },
}

impl Update {
    pub fn is_region(&self) -> bool {
        matches!(self, Update::Region { .. })
    }

    pub fn is_region_up(&self) -> bool {
        matches!(self, Update::Region { up: true, .. })
    }

    pub fn is_region_down(&self) -> bool {
        matches!(self, Update::Region { up: false, .. })
    }

    pub fn is_project(&self) -> bool {
        matches!(
            self,
            Update::Project { .. } | Update::Policy { .. }
        )
    }

    pub fn is_policy(&self) -> bool {
        matches!(self, Update::Policy { .. })
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_promote_process() {
//         let mut state = State::new(Difficulty::Normal);
//         state.processes[0].mix_share = 0;
//         assert_eq!(state.processes[0].is_promoted(), false);
//
//         state.change_mix_share(0, 10);
//         assert_eq!(state.processes[0].is_promoted(), true);
//         for npc_id in &state.processes[0].supporters {
//             assert_eq!(
//                 state.npcs[*npc_id].relationship,
//                 3. + RELATIONSHIP_CHANGE_AMOUNT
//             );
//         }
//         for npc_id in &state.processes[0].opposers {
//             assert_eq!(
//                 state.npcs[*npc_id].relationship,
//                 3. - RELATIONSHIP_CHANGE_AMOUNT
//             );
//         }
//
//         state.change_mix_share(0, -8);
//         assert_eq!(state.processes[0].is_promoted(), false);
//         for npc_id in &state.processes[0].supporters {
//             assert_eq!(state.npcs[*npc_id].relationship, 3.);
//         }
//         for npc_id in &state.processes[0].opposers {
//             assert_eq!(state.npcs[*npc_id].relationship, 3.);
//         }
//     }
//
//     #[test]
//     fn test_ban_process() {
//         let mut state = State::new(Difficulty::Normal);
//         state.processes[0].mix_share = 5;
//         assert_eq!(state.processes[0].is_banned(), false);
//
//         state.change_mix_share(0, -5);
//         assert_eq!(state.processes[0].is_banned(), true);
//         for npc_id in &state.processes[0].supporters {
//             assert_eq!(
//                 state.npcs[*npc_id].relationship,
//                 3. - RELATIONSHIP_CHANGE_AMOUNT
//             );
//         }
//         for npc_id in &state.processes[0].opposers {
//             assert_eq!(
//                 state.npcs[*npc_id].relationship,
//                 3. + RELATIONSHIP_CHANGE_AMOUNT
//             );
//         }
//
//         state.change_mix_share(0, 2);
//         assert_eq!(state.processes[0].is_banned(), false);
//         for npc_id in &state.processes[0].supporters {
//             assert_eq!(state.npcs[*npc_id].relationship, 3.);
//         }
//         for npc_id in &state.processes[0].opposers {
//             assert_eq!(state.npcs[*npc_id].relationship, 3.);
//         }
//     }
//
//     #[test]
//     fn test_project_policy() {
//         let mut rng = SmallRng::from_entropy();
//         let mut state = State::new(Difficulty::Normal);
//
//         let id = {
//             let p = state
//                 .projects
//                 .iter_mut()
//                 .find(|p| {
//                     p.kind == ProjectType::Policy
//                         && p.effects.len() > 0
//                 })
//                 .unwrap();
//             p.set_points(10);
//             p.id
//         };
//
//         assert_eq!(state.projects[id].status, Status::Inactive);
//
//         // Start
//         state.start_project(id);
//         assert_eq!(state.projects[id].status, Status::Building);
//
//         // Build until the project is completed
//         let mut effects = vec![];
//         loop {
//             let (completed, _, effs) =
//                 state.step_projects(&mut rng);
//             if completed.contains(&id) {
//                 effects = effs
//                     .iter()
//                     .map(|(eff, _)| eff.clone())
//                     .collect();
//                 for npc_id in &state.projects[id].supporters {
//                     assert_eq!(
//                         state.npcs[*npc_id].relationship,
//                         3. + RELATIONSHIP_CHANGE_AMOUNT
//                     );
//                 }
//                 for npc_id in &state.projects[id].opposers {
//                     assert_eq!(
//                         state.npcs[*npc_id].relationship,
//                         3. - RELATIONSHIP_CHANGE_AMOUNT
//                     );
//                 }
//                 break;
//             }
//         }
//
//         // Stop
//         let uneffects = state.stop_project(id);
//         assert_eq!(state.projects[id].status, Status::Halted);
//         assert_eq!(effects.len(), uneffects.len());
//         for npc_id in &state.projects[id].supporters {
//             assert_eq!(state.npcs[*npc_id].relationship, 3.);
//         }
//         for npc_id in &state.projects[id].opposers {
//             assert_eq!(state.npcs[*npc_id].relationship, 3.);
//         }
//     }
//
//     #[test]
//     fn test_project_other() {
//         let mut rng = SmallRng::from_entropy();
//         let mut state = State::new(Difficulty::Normal);
//
//         let id = {
//             let p = state
//                 .projects
//                 .iter_mut()
//                 .find(|p| p.kind == ProjectType::Initiative)
//                 .unwrap();
//             p.set_points(10);
//             p.id
//         };
//
//         assert_eq!(state.projects[id].status, Status::Inactive);
//
//         // Start
//         state.start_project(id);
//         assert_eq!(state.projects[id].status, Status::Building);
//
//         // Build until the project is completed
//         let mut effects = vec![];
//         loop {
//             let (completed, _, effs) =
//                 state.step_projects(&mut rng);
//             if completed.contains(&id) {
//                 effects = effs
//                     .iter()
//                     .map(|(eff, _)| eff.clone())
//                     .collect();
//                 for npc_id in &state.projects[id].supporters {
//                     assert_eq!(
//                         state.npcs[*npc_id].relationship,
//                         3. + RELATIONSHIP_CHANGE_AMOUNT
//                     );
//                 }
//                 for npc_id in &state.projects[id].opposers {
//                     assert_eq!(
//                         state.npcs[*npc_id].relationship,
//                         3. - RELATIONSHIP_CHANGE_AMOUNT
//                     );
//                 }
//                 break;
//             }
//         }
//
//         // Stop
//         let uneffects = state.stop_project(id);
//         assert_eq!(state.projects[id].status, Status::Halted);
//         assert_eq!(effects.len(), uneffects.len());
//         for npc_id in &state.projects[id].supporters {
//             assert_eq!(state.npcs[*npc_id].relationship, 3.);
//         }
//         for npc_id in &state.projects[id].opposers {
//             assert_eq!(state.npcs[*npc_id].relationship, 3.);
//         }
//
//         // Start again
//         state.start_project(id);
//         assert_eq!(state.projects[id].status, Status::Building);
//
//         state.projects[id].set_points(10);
//         state.projects[id].build();
//
//         // Stop again, should be halted now
//         let uneffects = state.stop_project(id);
//         assert_eq!(state.projects[id].status, Status::Halted);
//         assert_eq!(effects.len(), uneffects.len());
//     }
//
//     #[test]
//     fn test_project_upgrades() {
//         let mut rng = SmallRng::from_entropy();
//         let mut state = State::new(Difficulty::Normal);
//
//         let id = {
//             let p = state
//                 .projects
//                 .iter_mut()
//                 .find(|p| {
//                     p.upgrades.len() > 0
//                         && p.kind == ProjectType::Policy
//                 })
//                 .unwrap();
//             p.set_points(10);
//             p.id
//         };
//
//         state.start_project(id);
//
//         let mut effects = vec![];
//         loop {
//             let (completed, _, effs) =
//                 state.step_projects(&mut rng);
//             if completed.contains(&id) {
//                 effects = effs
//                     .iter()
//                     .map(|(eff, _)| eff.clone())
//                     .collect();
//                 break;
//             }
//         }
//
//         let (uneffects, new_effects) =
//             state.upgrade_project(id);
//
//         assert_eq!(state.projects[id].level, 1);
//         assert!(effects.iter().eq(uneffects.iter()));
//         assert!(new_effects.len() > 0);
//     }
//
//     #[test]
//     fn test_project_no_upgrades() {
//         let mut rng = SmallRng::from_entropy();
//         let mut state = State::new(Difficulty::Normal);
//
//         let id = {
//             let p = state
//                 .projects
//                 .iter_mut()
//                 .find(|p| {
//                     p.upgrades.len() == 0
//                         && p.kind == ProjectType::Policy
//                 })
//                 .unwrap();
//             p.set_points(10);
//             p.id
//         };
//
//         state.start_project(id);
//
//         let mut effects = vec![];
//         loop {
//             let (completed, _, effs) =
//                 state.step_projects(&mut rng);
//             if completed.contains(&id) {
//                 effects = effs
//                     .iter()
//                     .map(|(eff, _)| eff.clone())
//                     .collect();
//                 break;
//             }
//         }
//
//         let (uneffects, new_effects) =
//             state.upgrade_project(id);
//
//         assert_eq!(state.projects[id].level, 0);
//         assert_eq!(uneffects.len(), 0);
//         assert_eq!(new_effects.len(), 0);
//     }
//
//     #[test]
//     fn test_requests() {
//         let mut rng = SmallRng::from_entropy();
//         let mut state = State::new(Difficulty::Normal);
//         state.requests.push((Request::Project, 0, true, 10));
//         state.requests.push((Request::Process, 0, true, 10));
//
//         let completed = state.check_requests();
//         assert_eq!(completed.len(), 0);
//
//         state.start_project(0);
//         let completed = state.check_requests();
//         assert_eq!(completed.len(), 0); // Project not yet finished
//
//         state.projects[0].set_points(10);
//         for _ in 0..100 {
//             // Should be plenty of time to finish any project
//             state.projects[0].build();
//         }
//
//         let completed = state.check_requests();
//         assert_eq!(completed.len(), 1);
//         assert_eq!(state.requests.len(), 1);
//
//         state.change_mix_share(0, 5);
//         let completed = state.check_requests();
//         assert_eq!(completed.len(), 1);
//         assert_eq!(state.requests.len(), 0);
//
//         state.requests.push((Request::Project, 0, false, 10));
//         state.requests.push((Request::Process, 0, false, 10));
//
//         state.stop_project(0);
//         state.change_mix_share(
//             0,
//             -(state.processes[0].mix_share as isize),
//         );
//         let completed = state.check_requests();
//         assert_eq!(completed.len(), 2);
//         assert_eq!(state.requests.len(), 0);
//     }
//
//     #[test]
//     fn test_calculate_demand() {
//         // This is a pretty simple test,
//         // real testing involves calibration
//         let mut state = State::new(Difficulty::Normal);
//
//         let (output_demand, resource_demand) =
//             state.calculate_demand();
//
//         state.output_demand_extras = outputs!(fuel: 100.);
//         let (other_output_demand, _) = state.calculate_demand();
//         assert_eq!(
//             output_demand.fuel + 100.,
//             other_output_demand.fuel
//         );
//
//         state.output_demand_modifier = outputs!(fuel: 2.);
//         let (other_output_demand, _) = state.calculate_demand();
//         assert_eq!(
//             2. * (output_demand.fuel + 100.),
//             other_output_demand.fuel
//         );
//     }
// }
