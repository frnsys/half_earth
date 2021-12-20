use crate::surface;
use crate::npcs::{NPC, NPCRelation};
use crate::world::World;
use crate::game::Difficulty;
use crate::industries::Industry;
use crate::events::{Effect, Request, Flag};
use crate::projects::{Project, Status, Type as ProjectType};
use crate::production::{
    ProductionOrder, Process,
    produce, calculate_required};
use crate::kinds::{OutputMap, ResourceMap, ByproductMap, FeedstockMap};
use crate::{content, consts};
use rand::rngs::SmallRng;
use serde::Serialize;

#[derive(Default, Serialize, Clone)]
pub struct State {
    pub world: World,
    pub flags: Vec<Flag>,
    pub runs: usize,
    pub industries: Vec<Industry>,
    pub projects: Vec<Project>,
    pub processes: Vec<Process>,

    pub game_over: bool,

    pub political_capital: isize,
    pub research_points: isize,
    pub malthusian_points: usize,
    pub hes_points: usize,
    pub falc_points: usize,
    pub npcs: Vec<NPC>,

    // Requests: (
    //  request type,
    //  entity id,
    //  state (active: true/false),
    //  political capital bounty
    // )
    pub requests: Vec<(Request, usize, bool, usize)>,

    // Modifiers should start as all 1.
    pub output_modifier: OutputMap<f32>,
    pub output_demand: OutputMap<f32>,
    pub output_demand_modifier: OutputMap<f32>,
    pub output_demand_extras: OutputMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub resources_demand: ResourceMap<f32>,
    pub resources: ResourceMap<f32>,
    pub feedstocks: FeedstockMap<f32>,
    pub produced: OutputMap<f32>,
    pub produced_by_process: Vec<f32>,
    pub consumed_resources: ResourceMap<f32>,
    pub consumed_feedstocks: FeedstockMap<f32>,
    pub protected_land: f32,
}

impl State {
    pub fn new(difficulty: Difficulty) -> State {
        let mut state = State {
            // political_capital: 10,
            political_capital: 100,
            research_points: 0,
            malthusian_points: 0,
            hes_points: 0,
            falc_points: 0,
            flags: Vec::new(),
            game_over: false,

            world: content::world(difficulty),
            projects: content::projects(),
            processes: content::processes(),
            industries: content::industries(),
            npcs: content::npcs(),

            runs: 1, // TODO TEMP TESTING
            // runs: 0,
            requests: Vec::new(),

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
            resources: consts::STARTING_RESOURCES,
            feedstocks: consts::FEEDSTOCK_RESERVES,
            byproducts: byproducts!(),
            produced: outputs!(),
            produced_by_process: Vec::new(),
            consumed_resources: resources!(),
            consumed_feedstocks: feedstocks!(),
            protected_land: 0.,
        };

        let (output_demand, _) = state.calculate_demand();
        let orders: Vec<ProductionOrder> = state.processes.iter()
            .map(|p| p.production_order(&output_demand)).collect();
        let (required_resources, _) = calculate_required(&orders);
        state.resources.electricity = required_resources.electricity;
        state.resources.fuel = required_resources.fuel;

        // Bit of a hack to generate initial state values
        state.step_production();

        for project in &mut state.projects {
            project.update_cost(state.world.year, &state.output_demand);
        }

        state.update_region_temps();

        state
    }

    pub fn calculate_demand(&self) -> (OutputMap<f32>, ResourceMap<f32>) {
        // Aggregate demand across regions
        let mut output_demand = outputs!();

        // Ignore electric/fuel, captured by everything else
        let world_demand = self.world.demand();
        output_demand.animal_calories += world_demand.animal_calories;
        output_demand.plant_calories += world_demand.plant_calories;

        // Demand and impacts from non-modeled industries
        let lic_pop = self.world.lic_population();
        let industry_demand = self.industries.iter()
            .fold(resources!(), |acc, ind| acc + ind.adj_resources() * ind.demand_modifier) * lic_pop;
        output_demand.fuel += industry_demand.fuel;
        output_demand.electricity += industry_demand.electricity;

        // Water and land demand
        let mut resources_demand = resources!();
        resources_demand.water += industry_demand.water;
        resources_demand.land += industry_demand.land;

        // Apply modifiers
        ((output_demand + self.output_demand_extras) * self.output_demand_modifier, resources_demand)
    }

    pub fn step_projects(&mut self, rng: &mut SmallRng) ->
        (Vec<usize>, Vec<(Effect, Option<usize>)>, Vec<(Effect, Option<usize>)>) {
        // New effects to apply are gathered here.
        // (Mostly to avoid borrowing conflicts)
        // (Effect, Option<RegionId>)
        let mut remove_effects: Vec<(Effect, Option<usize>)> = Vec::new();
        let mut add_effects: Vec<(Effect, Option<usize>)> = Vec::new();

        // Advance projects
        let mut completed_projects = Vec::new();
        for project in self.projects.iter_mut().filter(|p| match p.status {
            Status::Building => true,
            _ => false
        }) {
            let prev_progress = project.progress;
            if prev_progress > 0. && project.gradual {
                for effect in &project.effects {
                    remove_effects.push((effect.clone() * project.progress, None));
                }
            }
            let completed = project.build();
            if project.gradual {
                for effect in &project.effects {
                    add_effects.push((effect.clone() * project.progress, None));
                }
            } else if completed {
                for effect in &project.effects {
                    add_effects.push((effect.clone(), None));
                }
                completed_projects.push(project.id);
            }
        }

        // Ugh hacky
        let mut outcomes: Vec<(usize, usize)> = Vec::new();
        for id in &completed_projects {
            let project = &self.projects[*id];
            match project.roll_outcome(self, rng) {
                Some((outcome, i)) => {
                    for effect in &outcome.effects {
                        add_effects.push((effect.clone(), None));
                    }
                    outcomes.push((*id, i));
                },
                None => ()
            }
        }

        for (id, i) in outcomes {
            self.projects[id].active_outcome = Some(i);
        }

        for project in &mut self.projects {
            project.update_cost(self.world.year, &self.output_demand);
        }

        (completed_projects, remove_effects, add_effects)
    }

    pub fn step_production(&mut self) {
        let (output_demand, resources_demand) = self.calculate_demand();
        self.output_demand = output_demand;
        self.resources_demand = resources_demand;
        self.byproducts = byproducts!();

        let lic_pop = self.world.lic_population();
        self.byproducts += self.industries.iter()
            .fold(byproducts!(), |acc, ind| acc + ind.adj_byproducts() * ind.demand_modifier) * lic_pop;

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
        let orders: Vec<ProductionOrder> = self.processes.iter()
            .map(|p| p.production_order(&self.output_demand)).collect();

        // Calculate required resources so we can add in food energy requirements
        let (required_resources, required_feedstocks) = calculate_required(&orders);
        self.output_demand.electricity += required_resources.electricity;
        self.output_demand.fuel += required_resources.fuel;

        // Now re-calculate orders
        let orders: Vec<ProductionOrder> = self.processes.iter()
            .map(|p| p.production_order(&self.output_demand)).collect();

        // Apply land protection
        self.resources.land = consts::STARTING_RESOURCES.land * (1. - self.protected_land);

        // Run production function
        let (produced_by_process,
             produced_by_type,
             consumed_resources,
             consumed_feedstocks,
             byproducts) = produce(&orders, &self.resources, &self.feedstocks);
        self.produced_by_process = produced_by_process;
        self.produced = produced_by_type * self.output_modifier;
        self.byproducts += byproducts;

        self.consumed_resources = consumed_resources;
        self.consumed_feedstocks = consumed_feedstocks;
        self.resources_demand.water += consumed_resources.water;
        self.resources_demand.land += consumed_resources.land;

        self.world.co2_emissions = byproducts.co2 + self.world.byproduct_mods.co2;
        self.world.ch4_emissions = byproducts.ch4 + self.world.byproduct_mods.ch4;
        self.world.n2o_emissions = byproducts.n2o + self.world.byproduct_mods.n2o;
        self.world.extinction_rate = self.processes.iter().zip(&self.produced_by_process).fold(0., |acc, (p, amount)| {
            acc + (p.extinction_rate() * *amount)
        }) + self.world.base_extinction_rate();

        // Float imprecision sometimes causes these values
        // to be slightly negative, so ensure they aren't
        self.feedstocks -= consumed_feedstocks;
        for k in self.feedstocks.keys() {
            self.feedstocks[k] = f32::max(self.feedstocks[k], 0.);
        }
        self.resources.fuel -= consumed_resources.fuel - self.produced.fuel;
        self.resources.fuel = self.resources.fuel.max(0.);
        self.resources.electricity -= consumed_resources.electricity - self.produced.electricity;
        self.resources.electricity = self.resources.electricity.max(0.);

        // Get resource deficit/surplus
        let (required_resources, required_feedstocks) = calculate_required(&orders);

        // Weigh resources by scarcity;
        // higher weight = higher scarcity
        let mut resource_weights = resources!();
        for (k, v) in required_resources.items() {
            resource_weights[k] = f32::min(f32::max(v / self.resources[k], 0.), 1.);
        }
        resource_weights.electricity = 2.;
        let mut feedstock_weights = feedstocks!();
        for (k, v) in required_feedstocks.items() {
            feedstock_weights[k] = f32::min(f32::max(v / self.feedstocks[k], 0.), 1.);
        }
        feedstock_weights.soil = 0.; // TODO add this back in?
        feedstock_weights.other = 0.;
    }

    pub fn step_world(&mut self) {
        self.world.year += 1;
        self.world.update_pop();

        if !self.flags.contains(&Flag::StopDevelopment) {
            self.world.develop_regions();
        }
        self.world.update_outlook();
    }

    pub fn check_requests(&mut self) -> Vec<(Request, usize, bool, usize)> {
        let mut i = 0;
        let mut completed = Vec::new();
        while i < self.requests.len() {
            let (kind, id, active, bounty) = self.requests[i].clone();
            let complete = match kind {
                Request::Project => {
                    let project = &self.projects[id];
                    (active && (project.status == Status::Active || project.status == Status::Finished))
                    || (!active && (project.status == Status::Inactive || project.status == Status::Halted))
                },
                Request::Process => {
                    let process = &self.processes[id];
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

    pub fn start_project(&mut self, project_id: usize, rng: &mut SmallRng) -> Vec<Effect> {
        let mut effects: Vec<Effect> = Vec::new();

        // Ugh hacky
        let project = &self.projects[project_id];
        let mut active_outcome = None;
        if project.kind == ProjectType::Policy {
            match project.roll_outcome(self, rng) {
                Some((outcome, i)) => {
                    for effect in &outcome.effects {
                        effects.push(effect.clone());
                    }
                    active_outcome = Some(i);
                },
                None => ()
            }
        }

        let project = &mut self.projects[project_id];

        if project.kind == ProjectType::Policy {
            project.status = Status::Active;
            for effect in &project.effects {
                effects.push(effect.clone());
            }
            project.active_outcome = active_outcome;
        } else {
            project.status = Status::Building;
        }

        for npc_id in &project.supporters {
            self.npcs[*npc_id].relationship += 1;
        }
        for npc_id in &project.opposers {
            self.npcs[*npc_id].relationship -= 1;
        }

        effects
    }

    pub fn stop_project(&mut self, project_id: usize) -> Vec<Effect> {
        let mut effects: Vec<Effect> = Vec::new();
        let project = &mut self.projects[project_id];

        if project.progress > 0. {
            project.status = Status::Halted;
        } else {
            project.status = Status::Inactive;
        }

        if project.kind == ProjectType::Policy {
            for effect in &project.effects {
                effects.push(effect.clone());
            }
        }

        for npc_id in &project.supporters {
            self.npcs[*npc_id].relationship -= 1;
        }
        for npc_id in &project.opposers {
            self.npcs[*npc_id].relationship += 1;
        }

        effects
    }

    pub fn upgrade_project(&mut self, project_id: usize) -> (Vec<Effect>, Vec<Effect>) {
        let mut remove_effects = Vec::new();
        let mut add_effects = Vec::new();

        let project = &mut self.projects[project_id];
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

    pub fn change_mix_share(&mut self, process_id: usize, change: isize) {
        let process = &mut self.processes[process_id];
        let was_banned = process.is_banned();
        let was_promoted = process.is_promoted();
        if change < 0 {
            process.mix_share = process.mix_share.saturating_sub(change.abs() as usize);
        } else {
            process.mix_share += change as usize;
        }

        let (support_change, oppose_change) = if !was_banned && process.is_banned() {
            // Ban
            (-1, 1)
        } else if was_banned && !process.is_banned() {
            // Unban
            (1, -1)
        } else if was_promoted && !process.is_promoted() {
            // Unpromote
            (-1, 1)
        } else if !was_promoted && process.is_promoted() {
            // Promote
            (1, -1)
        } else {
            (0, 0)
        };
        for npc_id in &process.supporters {
            self.npcs[*npc_id].relationship += support_change;
        }
        for npc_id in &process.opposers {
            self.npcs[*npc_id].relationship += oppose_change;
        }
    }

    pub fn is_ally(&mut self, name: &'static str) -> bool {
        let npc = self.npcs.iter().find(|n| n.name == name);
        if let Some(npc) = npc {
            npc.relation() == NPCRelation::Ally
        } else {
            false
        }
    }

    pub fn set_tgav(&mut self, tgav: f32) {
        let prev_temp = self.world.temperature;
        self.world.update_tgav(tgav);
        let temp_diff = prev_temp - self.world.temperature;
        self.update_region_temps();
        self.world.update_sea_level_rise();
        self.world.update_temp_outlook(temp_diff);
    }

    pub fn update_region_temps(&mut self) {
        let temps: Vec<f32> = surface::apply_pscl(&surface::TEMP_PATTERN_W, &surface::TEMP_PATTERN_B, surface::BASE_TEMP + self.world.temperature).collect();
        let precips: Vec<f32> = surface::apply_pscl(&surface::PRECIP_PATTERN_W, &surface::PRECIP_PATTERN_B, surface::BASE_TEMP + self.world.temperature).collect();
        for region in &mut self.world.regions {
            // We assert when generating the pattern idxs that they are not empty
            let local_temps: Vec<f32> = region.pattern_idxs.iter().map(|idx| &temps[*idx]).cloned().collect();
            let local_precips: Vec<f32> = region.pattern_idxs.iter().map(|idx| &precips[*idx]).cloned().collect();
            region.temp_lo = local_temps.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            region.temp_hi = local_temps.iter().fold(-f32::INFINITY, |a, &b| a.max(b));

            // In kg/m2/s, convert to cm/year
            // 1 kg/m2/s = 1 mm/s
            // 31536000 seconds per year, which yields mm/year
            region.precip_lo = local_precips.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            region.precip_hi = local_precips.iter().fold(-f32::INFINITY, |a, &b| a.max(b));
            region.precip_lo *= 31536000. / 10.;
            region.precip_hi *= 31536000. / 10.;
            // region.temp = region.pattern_idxs.iter().map(|idx| &temps[*idx]).sum::<f32>()/region.pattern_idxs.len() as f32;
        }
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
            assert_eq!(state.npcs[*npc_id].relationship, 4);
        }
        for npc_id in &state.processes[0].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 2);
        }

        state.change_mix_share(0, -8);
        assert_eq!(state.processes[0].is_promoted(), false);
        for npc_id in &state.processes[0].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 3);
        }
        for npc_id in &state.processes[0].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 3);
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
            assert_eq!(state.npcs[*npc_id].relationship, 2);
        }
        for npc_id in &state.processes[0].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 4);
        }

        state.change_mix_share(0, 2);
        assert_eq!(state.processes[0].is_banned(), false);
        for npc_id in &state.processes[0].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 3);
        }
        for npc_id in &state.processes[0].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 3);
        }
    }

    #[test]
    fn test_project_policy() {
        let mut rng = SmallRng::from_entropy();
        let mut state = State::new(Difficulty::Normal);

        let p = state.projects.iter().find(|p| p.kind == ProjectType::Policy && p.effects.len() > 0).unwrap();
        let id = p.id;

        assert_eq!(state.projects[id].status, Status::Inactive);

        // Start
        let effects = state.start_project(id, &mut rng);
        assert_eq!(state.projects[id].status, Status::Active);
        assert!(effects.len() > 0);

        for npc_id in &state.projects[id].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 4);
        }
        for npc_id in &state.projects[id].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 2);
        }

        // Stop
        let uneffects = state.stop_project(id);
        assert_eq!(state.projects[id].status, Status::Inactive);
        assert_eq!(effects.len(), uneffects.len());
        for npc_id in &state.projects[id].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 3);
        }
        for npc_id in &state.projects[id].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 3);
        }
    }

    #[test]
    fn test_project_other() {
        let mut rng = SmallRng::from_entropy();
        let mut state = State::new(Difficulty::Normal);

        let p = state.projects.iter().find(|p| p.kind == ProjectType::Initiative).unwrap();
        let id = p.id;

        assert_eq!(state.projects[id].status, Status::Inactive);

        // Start
        let effects = state.start_project(id, &mut rng);
        assert_eq!(state.projects[id].status, Status::Building);
        assert_eq!(effects.len(), 0); // No immediate effects

        for npc_id in &state.projects[id].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 4);
        }
        for npc_id in &state.projects[id].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 2);
        }

        // Stop
        let uneffects = state.stop_project(id);
        assert_eq!(state.projects[id].status, Status::Inactive);
        assert_eq!(effects.len(), uneffects.len());
        for npc_id in &state.projects[id].supporters {
            assert_eq!(state.npcs[*npc_id].relationship, 3);
        }
        for npc_id in &state.projects[id].opposers {
            assert_eq!(state.npcs[*npc_id].relationship, 3);
        }

        // Start again
        let effects = state.start_project(id, &mut rng);
        assert_eq!(state.projects[id].status, Status::Building);
        assert_eq!(effects.len(), 0); // No immediate effects

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

        let p = state.projects.iter().find(|p| p.upgrades.len() > 0 && p.kind == ProjectType::Policy).unwrap();
        let id = p.id;

        let effects = state.start_project(id, &mut rng);
        let (uneffects, new_effects) = state.upgrade_project(id);

        assert_eq!(state.projects[id].level, 1);
        assert!(effects.iter().eq(uneffects.iter()));
        assert!(new_effects.len() > 0);
    }

    #[test]
    fn test_project_no_upgrades() {
        let mut rng = SmallRng::from_entropy();
        let mut state = State::new(Difficulty::Normal);

        let p = state.projects.iter().find(|p| p.upgrades.len() == 0 && p.kind == ProjectType::Policy).unwrap();
        let id = p.id;

        let _effects = state.start_project(id, &mut rng);
        let (uneffects, new_effects) = state.upgrade_project(id);

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

        state.start_project(0, &mut rng);
        let completed = state.check_requests();
        assert_eq!(completed.len(), 0); // Project not yet finished

        state.projects[0].set_points(10);
        for _ in 0..100 { // Should be plenty of time to finish any project
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
        state.change_mix_share(0, -(state.processes[0].mix_share as isize));
        let completed = state.check_requests();
        assert_eq!(completed.len(), 2);
        assert_eq!(state.requests.len(), 0);
    }

    #[test]
    fn test_calculate_demand() {
        // This is a pretty simple test,
        // real testing involves calibration
        let mut state = State::new(Difficulty::Normal);

        let (output_demand, resource_demand) = state.calculate_demand();

        state.output_demand_extras = outputs!(fuel: 100.);
        let (other_output_demand, _) = state.calculate_demand();
        assert_eq!(output_demand.fuel + 100., other_output_demand.fuel);

        state.output_demand_modifier = outputs!(fuel: 2.);
        let (other_output_demand, _) = state.calculate_demand();
        assert_eq!(2. * (output_demand.fuel + 100.), other_output_demand.fuel);
    }

    #[test]
    fn test_step_projects() {
        // TODO
    }

    #[test]
    fn test_step_production() {
        // TODO
    }
}
