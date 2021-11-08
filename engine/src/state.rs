use crate::npcs::NPC;
use crate::world::World;
use crate::industries::Industry;
use crate::events::{Effect, Request, Flag};
use crate::projects::{Project, Status, Type as ProjectType};
use crate::production::{
    ProductionOrder, Priority, Process, ProcessStatus,
    produce, calculate_required, update_mixes};
use crate::kinds::{OutputMap, ResourceMap, ByproductMap, FeedstockMap};
use crate::consts;
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
    pub priority: Priority,

    pub political_capital: isize,
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
    pub fn init(&mut self) {
        let (output_demand, _) = self.calculate_demand();
        let orders: Vec<ProductionOrder> = self.processes.iter()
            .map(|p| p.production_order(&output_demand)).collect();
        let (required_resources, _) = calculate_required(&orders);
        self.resources.electricity = required_resources.electricity;
        self.resources.fuel = required_resources.fuel;

        // Bit of a hack to generate initial state values
        self.step_production();

        for project in &mut self.projects {
            project.update_cost(&self.output_demand);
        }
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
        let industry_demand = self.industries.iter().fold(resources!(), |acc, ind| acc + ind.resources * ind.demand_modifier) * lic_pop;
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
            if prev_progress > 0. {
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

        for id in &completed_projects {
            let project = &self.projects[*id];
            match project.roll_outcome(self, rng) {
                Some((outcome, _i)) => {
                    for effect in &outcome.effects {
                        add_effects.push((effect.clone(), None));
                    }
                },
                None => ()
            }
        }

        for project in &mut self.projects {
            project.update_cost(&self.output_demand);
        }

        (completed_projects, remove_effects, add_effects)
    }

    pub fn step_production(&mut self) {
        let (output_demand, resources_demand) = self.calculate_demand();
        self.output_demand = output_demand;
        self.resources_demand = resources_demand;
        self.byproducts = byproducts!();

        let lic_pop = self.world.lic_population();
        self.byproducts += self.industries.iter().fold(byproducts!(), |acc, ind| acc + ind.byproducts * ind.demand_modifier) * lic_pop;

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

        // Apply land protection
        self.resources.land = consts::STARTING_RESOURCES.land * (1. - self.protected_land);

        // Run production function
        let (produced_by_process,
             produced_by_type,
             consumed_resources,
             consumed_feedstocks,
             byproducts) = produce(&orders, &self.resources, &self.feedstocks);
        self.produced_by_process = produced_by_process; // TODO apply modifiers
        self.produced = produced_by_type * self.output_modifier;
        self.byproducts += byproducts;

        self.consumed_resources = consumed_resources;
        self.consumed_feedstocks = consumed_feedstocks;
        self.resources_demand.water += consumed_resources.water;
        self.resources_demand.land += consumed_resources.land;

        self.world.co2_emissions = byproducts.co2 + self.world.byproduct_mods.co2;
        self.world.ch4_emissions = byproducts.ch4 + self.world.byproduct_mods.ch4;
        self.world.n2o_emissions = byproducts.n2o + self.world.byproduct_mods.n2o;
        self.world.extinction_rate = (self.resources_demand.land/consts::STARTING_RESOURCES.land * 100.)
            + self.world.temperature.powf(2.)
            + self.world.sea_level_rise.powf(2.)
            - self.world.byproduct_mods.biodiversity;

        // Float imprecision sometimes causes these values
        // to be slightly negative, so ensure they aren't
        self.feedstocks -= consumed_feedstocks;
        self.resources.fuel -= consumed_resources.fuel - self.produced.fuel;
        self.resources.fuel = self.resources.fuel.max(0.);

        // Electricity from past turn disappears unless storage network is built
        // TODO this kind of breaks everything because then it means we have 0
        // electricity for the next production step
        self.resources.electricity -= consumed_resources.electricity - self.produced.electricity;
        self.resources.electricity = self.resources.electricity.max(0.);
        // if self.flags.contains(&Flag::EnergyStorage3) {
        //     self.resources.electricity *= 0.95;
        // } else if self.flags.contains(&Flag::EnergyStorage2) {
        //     self.resources.electricity *= 0.65;
        // } else if self.flags.contains(&Flag::EnergyStorage1) {
        //     self.resources.electricity *= 0.35;
        // } else {
        //     self.resources.electricity = 0.;
        // }

        // Get resource deficit/surplus
        let (required_resources, required_feedstocks) = calculate_required(&orders);

        // Weigh resources by scarcity
        let resource_weights = required_resources / self.resources;
        let feedstock_weights = required_feedstocks / self.feedstocks;

        // Update mixes according to resource scarcity
        update_mixes(&mut self.processes, &self.output_demand, &resource_weights, &feedstock_weights, &self.priority);
    }

    pub fn step_world(&mut self) {
        self.world.year += 1;
        self.world.update_pop();
        self.world.develop_regions();
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

    pub fn start_project(&mut self, project_id: usize) -> Vec<Effect> {
        let mut effects: Vec<Effect> = Vec::new();
        let project = &mut self.projects[project_id];

        if project.kind == ProjectType::Policy {
            project.status = Status::Active;
            for effect in &project.effects {
                effects.push(effect.clone());
            }
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
        }

        (remove_effects, add_effects)
    }

    pub fn promote_process(&mut self, process_id: usize) {
        let process = &mut self.processes[process_id];
        process.status = ProcessStatus::Promoted;

        for npc_id in &process.supporters {
            self.npcs[*npc_id].relationship += 1;
        }
        for npc_id in &process.opposers {
            self.npcs[*npc_id].relationship -= 1;
        }
    }

    pub fn unpromote_process(&mut self, process_id: usize) {
        let process = &mut self.processes[process_id];
        process.status = ProcessStatus::Neutral;

        for npc_id in &process.supporters {
            self.npcs[*npc_id].relationship -= 1;
        }
        for npc_id in &process.opposers {
            self.npcs[*npc_id].relationship += 1;
        }
    }

    pub fn ban_process(&mut self, process_id: usize) {
        let process = &mut self.processes[process_id];
        process.status = ProcessStatus::Banned;

        for npc_id in &process.supporters {
            self.npcs[*npc_id].relationship -= 1;
        }
        for npc_id in &process.opposers {
            self.npcs[*npc_id].relationship += 1;
        }
    }

    pub fn unban_process(&mut self, process_id: usize) {
        let process = &mut self.processes[process_id];
        process.status = ProcessStatus::Neutral;

        for npc_id in &process.supporters {
            self.npcs[*npc_id].relationship += 1;
        }
        for npc_id in &process.opposers {
            self.npcs[*npc_id].relationship -= 1;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_foo() {
        // TODO
    }
}
