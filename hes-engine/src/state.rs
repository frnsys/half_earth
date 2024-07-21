use std::collections::HashMap;

use crate::{
    events::{Effect, Event, EventPool, Flag, Phase, Request},
    game::Update,
    kinds::{
        ByproductMap,
        Feedstock,
        FeedstockMap,
        OutputMap,
        ResourceMap,
    },
    npcs::{update_seats, NPC},
    production::{
        calculate_required,
        produce,
        Process,
        ProductionOrder,
    },
    projects::{
        Group,
        Outcome,
        Project,
        Status,
        Type as ProjectType,
    },
    surface,
    world::World,
    Collection,
    Id,
};
use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};

const LIFESPAN: usize = 60;
const RELATIONSHIP_CHANGE_AMOUNT: f32 = 0.5;
const PRODUCTION_SHORTAGE_PENALTY: f32 = 60.;

/// Represents the game state.
#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
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
    pub new_policies: Vec<Id>,

    // Modifiers should start as all 1.
    pub output_modifier: OutputMap,
    pub output_demand: OutputMap,
    pub output_demand_modifier: OutputMap,
    pub output_demand_extras: OutputMap,
    pub byproducts: ByproductMap,
    pub resources_demand: ResourceMap,
    pub resources: ResourceMap,
    pub feedstocks: FeedstockMap,
    pub produced: OutputMap,
    pub produced_by_process: HashMap<Id, f32>,
    pub consumed_resources: ResourceMap,
    pub consumed_feedstocks: FeedstockMap,
    pub required_resources: ResourceMap,
    pub required_feedstocks: FeedstockMap,

    pub protected_land: f32,

    pub precipitation: f32, // global precip avg
    pub temp_outlook: f32,
    pub shortages_outlook: f32,
    pub water_stress: f32, // 0-100%
    pub co2_emissions: f32,
    pub ch4_emissions: f32,
    pub n2o_emissions: f32,
    pub temperature_modifier: f32,
    pub byproduct_mods: ByproductMap,
    pub population_growth_modifier: f32,
    pub sea_level_rise_modifier: f32, // meters

    last_outlook: f32,

    // TODO track what events have occurred
    pub events: Vec<Event>,

    pub event_pool: EventPool,
}

impl State {
    pub fn new(world: World) -> State {
        let mut npcs = NPC::load();
        let n_npcs =
            npcs.iter().filter(|npc| !npc.locked).count()
                as f32;
        for npc in npcs.iter_mut() {
            if !npc.locked {
                npc.seats = 1. / n_npcs;
            }
        }

        let events = world.events.clone();
        let death_year = world.year + LIFESPAN;
        let resources = world.starting_resources.clone();
        let feedstocks = world.feedstock_reserves.clone();

        let mut state = State {
            world,
            political_capital: 100,
            research_points: 0,
            flags: Vec::new(),
            game_over: false,
            death_year,
            npcs,

            runs: 0,
            requests: Vec::new(),
            new_policies: Vec::new(),
            events: vec![],

            output_modifier: outputs!(
                fuel: 1.,
                electricity: 1.,
                animal_calories: 1.,
                plant_calories: 1.
            ),
            output_demand: outputs!(),
            output_demand_modifier: outputs!(
                fuel: 1.,
                electricity: 1.,
                animal_calories: 1.,
                plant_calories: 1.
            ),
            output_demand_extras: outputs!(),
            resources_demand: resources!(),
            resources,
            feedstocks,
            byproducts: byproducts!(),
            produced: outputs!(),
            produced_by_process: HashMap::default(),
            consumed_resources: resources!(),
            consumed_feedstocks: feedstocks!(),
            required_resources: resources!(),
            required_feedstocks: feedstocks!(),
            protected_land: 0.1, // Starts at 10%

            // These are all computed
            precipitation: 0.,
            temp_outlook: 0.,
            shortages_outlook: 0.,
            water_stress: 0.,
            co2_emissions: 0.,
            ch4_emissions: 0.,
            n2o_emissions: 0.,
            temperature_modifier: 0.,
            byproduct_mods: byproducts!(),
            population_growth_modifier: 0.,
            sea_level_rise_modifier: 0.,
            last_outlook: 0.,

            event_pool: EventPool::new(events),
        };
        state.last_outlook = state.outlook();

        let (output_demand, _) = state.calculate_demand();
        let orders: Vec<ProductionOrder> = state
            .world
            .processes
            .iter()
            .map(|p| p.production_order(&output_demand))
            .collect();
        let (required_resources, required_feedstocks) =
            calculate_required(&orders);
        state.resources.electricity =
            required_resources.electricity;
        state.resources.fuel = required_resources.fuel;
        state.required_resources = required_resources;
        state.required_feedstocks = required_feedstocks;

        // Bit of a hack to generate initial state values
        state.step_production();

        let modifier = 1.;
        let income_level = state.world.income_level();
        for project in state.world.projects.iter_mut() {
            project.update_cost(
                state.world.year,
                income_level,
                &state.output_demand,
                modifier,
            );
            project.update_required_majority(&state.npcs);
        }

        state.update_region_temps();

        state
    }

    pub fn calculate_demand(&self) -> (OutputMap, ResourceMap) {
        // Aggregate demand across regions
        let mut output_demand = outputs!();

        // Ignore electric/fuel, captured by everything else
        let world_demand = self.world.demand();
        output_demand.animal_calories +=
            world_demand.animal_calories;
        output_demand.plant_calories +=
            world_demand.plant_calories;

        // Demand and impacts from non-modeled industries
        let lic_pop = self.world.lic_population();
        let industry_demand = self
            .world
            .industries
            .iter()
            .fold(resources!(), |acc, ind| {
                acc + ind.adj_resources() * ind.demand_modifier
            })
            * lic_pop;
        output_demand.fuel += industry_demand.fuel;
        output_demand.electricity +=
            industry_demand.electricity;

        // Water and land demand
        let mut resources_demand = resources!();
        resources_demand.water += industry_demand.water;
        resources_demand.land += industry_demand.land;

        // Apply modifiers
        (
            (output_demand + self.output_demand_extras)
                * self.output_demand_modifier,
            resources_demand,
        )
    }

    pub fn outlook(&self) -> f32 {
        let region_outlook = self
            .world
            .regions
            .iter()
            .map(|r| r.outlook)
            .sum::<f32>()
            / self.world.regions.len() as f32;
        self.world.base_outlook - self.shortages_outlook
            + region_outlook
    }

    pub fn update_pop(&mut self) {
        for region in self.world.regions.iter_mut() {
            region.update_pop(
                self.world.year as f32,
                1. + self.population_growth_modifier,
                &self.world.income_pop_coefs,
            );
        }
    }

    pub fn step_projects(
        &mut self,
        rng: &mut SmallRng,
    ) -> (
        Vec<Id>,
        Vec<(Effect, Option<Id>)>,
        Vec<(Effect, Option<Id>)>,
    ) {
        // New effects to apply are gathered here.
        // (Mostly to avoid borrowing conflicts)
        // (Effect, Option<RegionId>)
        let mut remove_effects: Vec<(Effect, Option<Id>)> =
            Vec::new();
        let mut add_effects: Vec<(Effect, Option<Id>)> =
            Vec::new();

        // Advance projects
        let mut completed_projects = Vec::new();
        for project in
            self.world.projects.iter_mut().filter(|p| {
                match p.status {
                    Status::Building => true,
                    _ => false,
                }
            })
        {
            let prev_progress = project.progress;
            if prev_progress > 0. && project.gradual {
                for effect in &project.effects {
                    remove_effects.push((
                        effect.clone() * project.progress,
                        None,
                    ));
                }
            }
            let completed = project.build();
            if completed {
                project.completed_at = self.world.year;
                for effect in &project.effects {
                    add_effects.push((effect.clone(), None));
                }

                for npc_id in &project.supporters {
                    self.npcs[npc_id].relationship +=
                        RELATIONSHIP_CHANGE_AMOUNT;
                }
                for npc_id in &project.opposers {
                    self.npcs[npc_id].relationship -=
                        RELATIONSHIP_CHANGE_AMOUNT;
                }

                completed_projects.push(project.id);
            } else if project.gradual {
                for effect in &project.effects {
                    add_effects.push((
                        effect.clone() * project.progress,
                        None,
                    ));
                }
            }
        }

        let mut outcomes: Vec<(&Id, usize)> = Vec::new();
        for id in &completed_projects {
            let project = &self.world.projects[id];
            match self.roll_project_outcome(project, rng) {
                Some((outcome, i)) => {
                    for effect in &outcome.effects {
                        add_effects
                            .push((effect.clone(), None));
                    }
                    outcomes.push((id, i));
                }
                None => (),
            }
        }

        for (id, i) in outcomes {
            self.world.projects[id].active_outcome = Some(i);
        }

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

        let posadist_ally = self.is_ally("The Posadist");
        let utopian_ally = self.is_ally("The Utopian");
        let animal_ally =
            self.is_ally("The Animal Liberationist");
        let environ_ally = self.is_ally("The Environmentalist");
        let ecofem_ally = self.is_ally("The Ecofeminist");
        let malthus_ally = self.is_ally("The Malthusian");

        let income_level = self.world.income_level();
        for project in self.world.projects.iter_mut() {
            let mut group_modifier = 1.0;
            if posadist_ally && project.group == Group::Nuclear
            {
                group_modifier *= 0.5;
            }
            if ecofem_ally
                && (project.group == Group::Food
                    || project.group == Group::Agriculture
                    || project.group == Group::Protection)
            {
                group_modifier *= 0.75;
            }
            if utopian_ally
                && (project.group == Group::Limits
                    || project.group == Group::Protection
                    || project.group == Group::Restoration)
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
                modifier *= 1.1;
            }
            project.update_cost(
                self.world.year,
                income_level,
                &self.output_demand,
                if project.kind == ProjectType::Policy {
                    1.0
                } else {
                    // Modifier only relevant for built projects
                    modifier * group_modifier
                },
            );
            project.update_required_majority(&self.npcs);
        }

        (completed_projects, remove_effects, add_effects)
    }

    pub fn update_demand(&mut self) {
        let (output_demand, resources_demand) =
            self.calculate_demand();
        self.output_demand = output_demand;
        self.resources_demand = resources_demand;
        self.byproducts = byproducts!();

        let lic_pop = self.world.lic_population();
        self.byproducts += self.world.industries.iter().fold(
            byproducts!(),
            |acc, ind| {
                acc + ind.adj_byproducts() * ind.demand_modifier
            },
        ) * lic_pop;

        if self.flags.contains(&Flag::Electrified) {
            let electrified = self.output_demand.fuel * 0.8;
            self.output_demand.electricity += electrified;
            self.output_demand.fuel -= electrified;
        }

        let cal_change = if self.flags.contains(&Flag::Vegan) {
            self.output_demand.animal_calories * 0.9
        } else if self.flags.contains(&Flag::Vegetarian) {
            self.output_demand.animal_calories * 0.75
        } else {
            0.
        };
        self.output_demand.animal_calories -= cal_change;
        self.output_demand.plant_calories += cal_change;

        // Generate production orders based on current process mixes and demand
        let orders: Vec<ProductionOrder> = self
            .world
            .processes
            .iter()
            .map(|p| p.production_order(&self.output_demand))
            .collect();

        // Calculate required resources so we can add in food energy requirements
        let (required_resources, required_feedstocks) =
            calculate_required(&orders);
        self.output_demand.electricity +=
            required_resources.electricity;
        self.output_demand.fuel += required_resources.fuel;
        self.required_resources = required_resources;
        self.required_feedstocks = required_feedstocks;

        // Now re-calculate orders
        let orders: Vec<ProductionOrder> = self
            .world
            .processes
            .iter()
            .map(|p| p.production_order(&self.output_demand))
            .collect();

        // Apply land protection
        self.resources.land =
            self.world.starting_resources.land
                * (1. - self.protected_land);

        // Run production function
        let (
            produced_by_process,
            produced_by_type,
            consumed_resources,
            consumed_feedstocks,
            byproducts,
        ) = produce(&orders, &self.resources, &self.feedstocks);
        self.produced_by_process = produced_by_process;
        self.produced = produced_by_type * self.output_modifier;
        self.byproducts += byproducts;
        self.consumed_resources = consumed_resources;
        self.consumed_feedstocks = consumed_feedstocks;
        self.resources_demand.water +=
            self.consumed_resources.water;
        self.resources_demand.land +=
            self.consumed_resources.land;
    }

    // Update production effects
    pub fn update_production(&mut self) {
        self.update_demand();

        self.co2_emissions =
            self.byproducts.co2 + self.byproduct_mods.co2;
        self.ch4_emissions =
            self.byproducts.ch4 + self.byproduct_mods.ch4;
        self.n2o_emissions =
            self.byproducts.n2o + self.byproduct_mods.n2o;
        self.world.extinction_rate = self
            .compute_extinction_rate(&self.produced_by_process);
    }

    pub fn step_production(&mut self) {
        self.update_production();

        let orders: Vec<ProductionOrder> = self
            .world
            .processes
            .iter()
            .map(|p| p.production_order(&self.output_demand))
            .collect();

        // Float imprecision sometimes causes these values
        // to be slightly negative, so ensure they aren't
        self.feedstocks -= self.consumed_feedstocks;
        for k in self.feedstocks.keys() {
            self.feedstocks[k] =
                f32::max(self.feedstocks[k], 0.);
        }
        self.resources.fuel -=
            self.consumed_resources.fuel - self.produced.fuel;
        self.resources.fuel = self.resources.fuel.max(0.);
        self.resources.electricity -=
            self.consumed_resources.electricity
                - self.produced.electricity;
        self.resources.electricity =
            self.resources.electricity.max(0.);

        // Get resource deficit/surplus
        let (required_resources, required_feedstocks) =
            calculate_required(&orders);

        // Weigh resources by scarcity;
        // higher weight = higher scarcity
        let mut resource_weights = resources!();
        for (k, v) in required_resources.items() {
            resource_weights[k] = f32::min(
                f32::max(v / self.resources[k], 0.),
                1.,
            );
        }
        resource_weights.electricity = 2.;
        let mut feedstock_weights = feedstocks!();
        for (k, v) in required_feedstocks.items() {
            feedstock_weights[k] = f32::min(
                f32::max(v / self.feedstocks[k], 0.),
                1.,
            );
        }
        feedstock_weights.soil = 0.; // TODO add this back in?
        feedstock_weights.other = 0.;

        // Outlook impacts based on production shortages
        // If all demand met is 0 it should be an instant game over, basically.
        let demand_met = self.produced / self.output_demand;
        self.shortages_outlook = PRODUCTION_SHORTAGE_PENALTY
            - ((demand_met.fuel
                + demand_met.electricity
                + demand_met.animal_calories
                + demand_met.plant_calories)
                * PRODUCTION_SHORTAGE_PENALTY
                / 4.);
    }

    pub fn sea_level_rise_rate(&self) -> f32 {
        // Meters
        // Chosen to roughly hit 1.4m-1.6m rise by 2100 in the BAU scenario
        (0.0025 * self.world.temperature.powf(1.5))
            + self.sea_level_rise_modifier
    }

    pub fn step_world(&mut self) -> Vec<Update> {
        self.world.year += 1;
        self.update_pop();

        if self.world.year >= self.death_year {
            self.game_over = true;
        }

        let stop = self.flags.contains(&Flag::StopDevelopment);
        let fast = self.flags.contains(&Flag::FastDevelopment);
        let degrow = self.flags.contains(&Flag::Degrowth);
        let (regions_up, regions_down) =
            self.world.develop_regions(stop, fast, degrow);
        let wretched_ally = self.is_ally("The Fanonist");
        let consumerist_ally = self.is_ally("The Consumerist");
        self.world
            .update_outlook(wretched_ally, consumerist_ally);
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
            .iter()
            .filter(|p| {
                if p.status == Status::Finished {
                    // Completed within the past ten years
                    p.completed_at >= self.world.year - 10
                } else {
                    p.status == Status::Active
                        || (p.status == Status::Building
                            && p.gradual)
                }
            })
            .collect();
        update_seats(
            outlook_change,
            &recent_projects,
            &mut self.npcs,
        );
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

    pub fn start_project(&mut self, project_id: &Id) {
        // Ugh hacky
        let project = &self.world.projects[project_id];
        if project.kind == ProjectType::Policy {
            self.new_policies.push(project.id);
        }

        let project = &mut self.world.projects[project_id];
        project.status = Status::Building;
    }

    pub fn stop_project(
        &mut self,
        project_id: &Id,
    ) -> Vec<Effect> {
        let mut effects: Vec<Effect> = Vec::new();
        let project = &mut self.world.projects[project_id];

        if project.status == Status::Active
            || project.status == Status::Finished
        {
            for effect in project.active_effects() {
                effects.push(effect.clone());
            }

            if let Some(outcome_id) = project.active_outcome {
                for effect in
                    &project.outcomes[outcome_id].effects
                {
                    effects.push(effect.clone());
                }
            }

            for npc_id in &project.supporters {
                self.npcs[npc_id].relationship -=
                    RELATIONSHIP_CHANGE_AMOUNT;
            }
            for npc_id in &project.opposers {
                self.npcs[npc_id].relationship +=
                    RELATIONSHIP_CHANGE_AMOUNT;
            }
        }

        if project.progress > 0. {
            project.status = Status::Halted;
        } else {
            project.status = Status::Inactive;
        }

        if project.kind == ProjectType::Policy {
            self.new_policies.retain(|&id| id != project.id);
        }

        effects
    }

    /// Roll to see the outcome of this project
    pub fn roll_project_outcome<'a>(
        &self,
        project: &'a Project,
        rng: &mut SmallRng,
    ) -> Option<(&'a Outcome, usize)> {
        let mut outcome = None;
        for (i, o) in project.outcomes.iter().enumerate() {
            match o.probability.eval(self, None) {
                Some(likelihood) => {
                    let prob = likelihood.p();
                    if rng.gen::<f32>() <= prob {
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

    pub fn roll_new_policy_outcomes(
        &mut self,
        rng: &mut SmallRng,
    ) -> (Vec<Id>, Vec<Effect>) {
        let mut effects: Vec<Effect> = Vec::new();
        let ids: Vec<Id> =
            self.new_policies.drain(..).collect();
        for id in &ids {
            let mut active_outcome = None;
            let proj = &self.world.projects[id];
            match self.roll_project_outcome(proj, rng) {
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

        (ids, effects)
    }

    pub fn upgrade_project(
        &mut self,
        project_id: &Id,
    ) -> (Vec<Effect>, Vec<Effect>) {
        let mut remove_effects = Vec::new();
        let mut add_effects = Vec::new();

        let project = &mut self.world.projects[project_id];
        for effect in project.active_effects() {
            remove_effects.push(effect.clone());
        }

        let upgraded = project.upgrade();
        if upgraded {
            for effect in project.active_effects() {
                add_effects.push(effect.clone());
            }
        } else {
            remove_effects.clear();
        }

        (remove_effects, add_effects)
    }

    pub fn downgrade_project(
        &mut self,
        project_id: &Id,
    ) -> (Vec<Effect>, Vec<Effect>) {
        let mut remove_effects = Vec::new();
        let mut add_effects = Vec::new();

        let project = &mut self.world.projects[project_id];
        for effect in project.active_effects() {
            remove_effects.push(effect.clone());
        }

        let downgraded = project.downgrade();
        if downgraded {
            for effect in project.active_effects() {
                add_effects.push(effect.clone());
            }
        } else {
            remove_effects.clear();
        }

        (remove_effects, add_effects)
    }

    pub fn change_mix_share(
        &mut self,
        process_id: &Id,
        change: isize,
    ) {
        let process = &mut self.world.processes[process_id];
        let was_banned = process.is_banned();
        let was_promoted = process.is_promoted();
        if change < 0 {
            process.mix_share = process
                .mix_share
                .saturating_sub(change.abs() as usize);
        } else {
            process.mix_share += change as usize;
        }

        let (support_change, oppose_change) =
            if !was_banned && process.is_banned() {
                // Ban
                (-1., 1.)
            } else if was_banned && !process.is_banned() {
                // Unban
                (1., -1.)
            } else if was_promoted && !process.is_promoted() {
                // Unpromote
                (-1., 1.)
            } else if !was_promoted && process.is_promoted() {
                // Promote
                (1., -1.)
            } else {
                (0., 0.)
            };
        for npc_id in &process.supporters {
            self.npcs[npc_id].relationship +=
                support_change * RELATIONSHIP_CHANGE_AMOUNT;
        }
        for npc_id in &process.opposers {
            self.npcs[npc_id].relationship +=
                oppose_change * RELATIONSHIP_CHANGE_AMOUNT;
        }
    }

    pub fn is_ally(&self, name: &'static str) -> bool {
        let npc = self.npcs.iter().find(|n| n.name == name);
        if let Some(npc) = npc {
            npc.is_ally()
        } else {
            false
        }
    }

    pub fn process_max_share(
        &self,
        process: &Process,
    ) -> usize {
        let mut max_share = 1.;
        let demand = self.output_demand[process.output];

        // Hard-coded limit
        if let Some(limit) = process.limit {
            max_share = (limit / demand).min(1.);
        }

        let (kind, per_output) = process.feedstock;
        match kind {
            Feedstock::Soil | Feedstock::Other => {}
            _ => {
                let limit = self.feedstocks[kind] / per_output;
                let supply_max_share = (limit / demand).min(1.);
                max_share = max_share.min(supply_max_share);
            }
        }

        ((max_share * 100.) / 5.).floor() as usize
    }

    pub fn emissions(&self) -> f32 {
        self.co2_emissions
            + (self.n2o_emissions * 298.)
            + (self.ch4_emissions * 36.)
    }

    pub fn emissions_gt(&self) -> f32 {
        self.emissions() * 1e-15
    }

    pub fn base_extinction_rate(&self) -> f32 {
        self.world.tgav_extinction_rate()
            + self.world.slr_extinction_rate()
            - self.byproduct_mods.biodiversity
    }

    pub fn compute_extinction_rate<'a>(
        &self,
        produced_by_process: &HashMap<Id, f32>,
    ) -> f32 {
        let lic_pop = self.world.lic_population();
        self.world.processes.iter().fold(0., |acc, p| {
            let amount =
                produced_by_process.get(&p.id).unwrap_or(&0.);
            acc + (p.extinction_rate(
                self.world.starting_resources.land,
            ) * amount)
        }) + self.world.industries.iter().fold(
            0.,
            |acc, ind| {
                acc + ind.extinction_rate(
                    self.world.starting_resources.land,
                ) * lic_pop
            },
        ) + self.base_extinction_rate()
    }

    pub fn set_tgav(&mut self, tgav: f32) {
        let prev_temp = self.world.temperature;
        self.world.temperature =
            tgav + self.temperature_modifier;
        let temp_diff = prev_temp - self.world.temperature;
        self.update_region_temps();
        self.world.sea_level_rise += self.sea_level_rise_rate();
        self.temp_outlook +=
            self.compute_temp_outlook_change(temp_diff);
    }

    pub fn compute_temp_outlook_change(
        &mut self,
        temp_change: f32,
    ) -> f32 {
        let outlook_change =
            temp_change * 6. * self.world.temperature.powf(2.);
        self.world.base_outlook += outlook_change;
        for region in self.world.regions.iter_mut() {
            region.outlook += outlook_change * 0.4;
        }
        outlook_change
    }

    pub fn update_region_temps(&mut self) {
        let temps: Vec<f32> = surface::apply_pscl(
            &surface::TEMP_PATTERN_W,
            &surface::TEMP_PATTERN_B,
            surface::BASE_TEMP + self.world.temperature,
        )
        .collect();
        let precips: Vec<f32> = surface::apply_pscl(
            &surface::PRECIP_PATTERN_W,
            &surface::PRECIP_PATTERN_B,
            surface::BASE_TEMP + self.world.temperature,
        )
        .collect();
        for region in self.world.regions.iter_mut() {
            // We assert when generating the pattern idxs that they are not empty
            let local_temps: Vec<f32> = region
                .pattern_idxs
                .iter()
                .map(|idx| &temps[*idx])
                .cloned()
                .collect();
            let local_precips: Vec<f32> = region
                .pattern_idxs
                .iter()
                .map(|idx| &precips[*idx])
                .cloned()
                .collect();
            region.temp_lo = local_temps
                .iter()
                .fold(f32::INFINITY, |a, &b| a.min(b));
            region.temp_hi = local_temps
                .iter()
                .fold(-f32::INFINITY, |a, &b| a.max(b));

            // In kg/m2/s, convert to cm/year
            // 1 kg/m2/s = 1 mm/s
            // 31536000 seconds per year, which yields mm/year
            region.precip_lo = local_precips
                .iter()
                .fold(f32::INFINITY, |a, &b| a.min(b));
            region.precip_hi = local_precips
                .iter()
                .fold(-f32::INFINITY, |a, &b| a.max(b));
            region.precip_lo *= 31536000. / 10.;
            region.precip_hi *= 31536000. / 10.;
            // region.temp = region.pattern_idxs.iter().map(|idx| &temps[*idx]).sum::<f32>()/region.pattern_idxs.len() as f32;
        }
    }

    pub fn roll_for_phase<'a>(
        &'a mut self,
        phase: Phase,
        limit: Option<usize>,
        rng: &mut SmallRng,
    ) -> Vec<(Event, Option<Id>)> {
        // TODO hacky
        let mut pool = self.event_pool.clone();
        let events =
            pool.roll_for_phase(phase, &self, limit, rng);
        self.event_pool = pool;
        events
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_promote_process() {
        let mut state = State::new(Difficulty::Normal);
        state.processes[0].mix_share = 0;
        assert_eq!(state.processes[0].is_promoted(), false);

        state.change_mix_share(0, 10);
        assert_eq!(state.processes[0].is_promoted(), true);
        for npc_id in &state.processes[0].supporters {
            assert_eq!(
                state.npcs[*npc_id].relationship,
                3. + RELATIONSHIP_CHANGE_AMOUNT
            );
        }
        for npc_id in &state.processes[0].opposers {
            assert_eq!(
                state.npcs[*npc_id].relationship,
                3. - RELATIONSHIP_CHANGE_AMOUNT
            );
        }

        state.change_mix_share(0, -8);
        assert_eq!(state.processes[0].is_promoted(), false);
        for npc_id in &state.processes[0].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 3.);
        }
        for npc_id in &state.processes[0].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 3.);
        }
    }

    #[test]
    fn test_ban_process() {
        let mut state = State::new(Difficulty::Normal);
        state.processes[0].mix_share = 5;
        assert_eq!(state.processes[0].is_banned(), false);

        state.change_mix_share(0, -5);
        assert_eq!(state.processes[0].is_banned(), true);
        for npc_id in &state.processes[0].supporters {
            assert_eq!(
                state.npcs[*npc_id].relationship,
                3. - RELATIONSHIP_CHANGE_AMOUNT
            );
        }
        for npc_id in &state.processes[0].opposers {
            assert_eq!(
                state.npcs[*npc_id].relationship,
                3. + RELATIONSHIP_CHANGE_AMOUNT
            );
        }

        state.change_mix_share(0, 2);
        assert_eq!(state.processes[0].is_banned(), false);
        for npc_id in &state.processes[0].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 3.);
        }
        for npc_id in &state.processes[0].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 3.);
        }
    }

    #[test]
    fn test_project_policy() {
        let mut rng = SmallRng::from_entropy();
        let mut state = State::new(Difficulty::Normal);

        let id = {
            let p = state
                .projects
                .iter_mut()
                .find(|p| {
                    p.kind == ProjectType::Policy
                        && p.effects.len() > 0
                })
                .unwrap();
            p.set_points(10);
            p.id
        };

        assert_eq!(state.projects[id].status, Status::Inactive);

        // Start
        state.start_project(id);
        assert_eq!(state.projects[id].status, Status::Building);

        // Build until the project is completed
        let mut effects = vec![];
        loop {
            let (completed, _, effs) =
                state.step_projects(&mut rng);
            if completed.contains(&id) {
                effects = effs
                    .iter()
                    .map(|(eff, _)| eff.clone())
                    .collect();
                for npc_id in &state.projects[id].supporters {
                    assert_eq!(
                        state.npcs[*npc_id].relationship,
                        3. + RELATIONSHIP_CHANGE_AMOUNT
                    );
                }
                for npc_id in &state.projects[id].opposers {
                    assert_eq!(
                        state.npcs[*npc_id].relationship,
                        3. - RELATIONSHIP_CHANGE_AMOUNT
                    );
                }
                break;
            }
        }

        // Stop
        let uneffects = state.stop_project(id);
        assert_eq!(state.projects[id].status, Status::Halted);
        assert_eq!(effects.len(), uneffects.len());
        for npc_id in &state.projects[id].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 3.);
        }
        for npc_id in &state.projects[id].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 3.);
        }
    }

    #[test]
    fn test_project_other() {
        let mut rng = SmallRng::from_entropy();
        let mut state = State::new(Difficulty::Normal);

        let id = {
            let p = state
                .projects
                .iter_mut()
                .find(|p| p.kind == ProjectType::Initiative)
                .unwrap();
            p.set_points(10);
            p.id
        };

        assert_eq!(state.projects[id].status, Status::Inactive);

        // Start
        state.start_project(id);
        assert_eq!(state.projects[id].status, Status::Building);

        // Build until the project is completed
        let mut effects = vec![];
        loop {
            let (completed, _, effs) =
                state.step_projects(&mut rng);
            if completed.contains(&id) {
                effects = effs
                    .iter()
                    .map(|(eff, _)| eff.clone())
                    .collect();
                for npc_id in &state.projects[id].supporters {
                    assert_eq!(
                        state.npcs[*npc_id].relationship,
                        3. + RELATIONSHIP_CHANGE_AMOUNT
                    );
                }
                for npc_id in &state.projects[id].opposers {
                    assert_eq!(
                        state.npcs[*npc_id].relationship,
                        3. - RELATIONSHIP_CHANGE_AMOUNT
                    );
                }
                break;
            }
        }

        // Stop
        let uneffects = state.stop_project(id);
        assert_eq!(state.projects[id].status, Status::Halted);
        assert_eq!(effects.len(), uneffects.len());
        for npc_id in &state.projects[id].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 3.);
        }
        for npc_id in &state.projects[id].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 3.);
        }

        // Start again
        state.start_project(id);
        assert_eq!(state.projects[id].status, Status::Building);

        state.projects[id].set_points(10);
        state.projects[id].build();

        // Stop again, should be halted now
        let uneffects = state.stop_project(id);
        assert_eq!(state.projects[id].status, Status::Halted);
        assert_eq!(effects.len(), uneffects.len());
    }

    #[test]
    fn test_project_upgrades() {
        let mut rng = SmallRng::from_entropy();
        let mut state = State::new(Difficulty::Normal);

        let id = {
            let p = state
                .projects
                .iter_mut()
                .find(|p| {
                    p.upgrades.len() > 0
                        && p.kind == ProjectType::Policy
                })
                .unwrap();
            p.set_points(10);
            p.id
        };

        state.start_project(id);

        let mut effects = vec![];
        loop {
            let (completed, _, effs) =
                state.step_projects(&mut rng);
            if completed.contains(&id) {
                effects = effs
                    .iter()
                    .map(|(eff, _)| eff.clone())
                    .collect();
                break;
            }
        }

        let (uneffects, new_effects) =
            state.upgrade_project(id);

        assert_eq!(state.projects[id].level, 1);
        assert!(effects.iter().eq(uneffects.iter()));
        assert!(new_effects.len() > 0);
    }

    #[test]
    fn test_project_no_upgrades() {
        let mut rng = SmallRng::from_entropy();
        let mut state = State::new(Difficulty::Normal);

        let id = {
            let p = state
                .projects
                .iter_mut()
                .find(|p| {
                    p.upgrades.len() == 0
                        && p.kind == ProjectType::Policy
                })
                .unwrap();
            p.set_points(10);
            p.id
        };

        state.start_project(id);

        let mut effects = vec![];
        loop {
            let (completed, _, effs) =
                state.step_projects(&mut rng);
            if completed.contains(&id) {
                effects = effs
                    .iter()
                    .map(|(eff, _)| eff.clone())
                    .collect();
                break;
            }
        }

        let (uneffects, new_effects) =
            state.upgrade_project(id);

        assert_eq!(state.projects[id].level, 0);
        assert_eq!(uneffects.len(), 0);
        assert_eq!(new_effects.len(), 0);
    }

    #[test]
    fn test_requests() {
        let mut rng = SmallRng::from_entropy();
        let mut state = State::new(Difficulty::Normal);
        state.requests.push((Request::Project, 0, true, 10));
        state.requests.push((Request::Process, 0, true, 10));

        let completed = state.check_requests();
        assert_eq!(completed.len(), 0);

        state.start_project(0);
        let completed = state.check_requests();
        assert_eq!(completed.len(), 0); // Project not yet finished

        state.projects[0].set_points(10);
        for _ in 0..100 {
            // Should be plenty of time to finish any project
            state.projects[0].build();
        }

        let completed = state.check_requests();
        assert_eq!(completed.len(), 1);
        assert_eq!(state.requests.len(), 1);

        state.change_mix_share(0, 5);
        let completed = state.check_requests();
        assert_eq!(completed.len(), 1);
        assert_eq!(state.requests.len(), 0);

        state.requests.push((Request::Project, 0, false, 10));
        state.requests.push((Request::Process, 0, false, 10));

        state.stop_project(0);
        state.change_mix_share(
            0,
            -(state.processes[0].mix_share as isize),
        );
        let completed = state.check_requests();
        assert_eq!(completed.len(), 2);
        assert_eq!(state.requests.len(), 0);
    }

    #[test]
    fn test_calculate_demand() {
        // This is a pretty simple test,
        // real testing involves calibration
        let mut state = State::new(Difficulty::Normal);

        let (output_demand, resource_demand) =
            state.calculate_demand();

        state.output_demand_extras = outputs!(fuel: 100.);
        let (other_output_demand, _) = state.calculate_demand();
        assert_eq!(
            output_demand.fuel + 100.,
            other_output_demand.fuel
        );

        state.output_demand_modifier = outputs!(fuel: 2.);
        let (other_output_demand, _) = state.calculate_demand();
        assert_eq!(
            2. * (output_demand.fuel + 100.),
            other_output_demand.fuel
        );
    }
}
