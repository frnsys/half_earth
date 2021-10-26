use crate::npcs::NPC;
use crate::world::World;
use crate::industries::Industry;
use crate::projects::{Project, Status, Type as ProjectType};
use crate::production::{ProductionOrder, Process, produce, calculate_required, update_mixes};
use crate::kinds::{OutputMap, ResourceMap, ByproductMap, FeedstockMap};
use crate::events::{EventPool, Effect, Type as EventType};
use crate::{content, consts};
use rand::{SeedableRng, rngs::SmallRng};
use serde::Serialize;
use crate::utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Difficulty {
    Easy,
    Normal,
    Hard
}

#[derive(Clone, Serialize)]
pub enum Request {
    Project,
    Process
}

#[wasm_bindgen]
pub struct GameInterface {
    rng: SmallRng,
    game: Game,
}

#[wasm_bindgen]
impl GameInterface {
    pub fn new(difficulty: Difficulty) -> GameInterface {
        // So we get tracebacks in the console
        utils::set_panic_hook();

        GameInterface {
            rng: SmallRng::from_entropy(),
            game: Game::new(difficulty),
        }
    }

    pub fn step(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.step())?)
    }

    pub fn state(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.state)?)
    }

    pub fn change_political_capital(&mut self, amount: isize) {
        self.game.state.political_capital += amount;
    }

    pub fn set_event_choice(&mut self, event_id: usize, region_id: Option<usize>, choice_id: usize) {
        let effects = self.game.set_event_choice(event_id, choice_id);
        for effect in effects {
            effect.apply(&mut self.game, region_id);
        }
    }

    pub fn set_project_points(&mut self, project_id: usize, points: usize) {
        self.game.state.projects[project_id].set_points(points);
    }

    pub fn start_project(&mut self, project_id: usize) {
        let project = &mut self.game.state.projects[project_id];
        if project.kind == ProjectType::Policy {
            project.status = Status::Active;
        } else {
            project.status = Status::Building;
        }
    }

    pub fn stop_project(&mut self, project_id: usize) {
        let project = &mut self.game.state.projects[project_id];
        if project.progress > 0. {
            project.status = Status::Halted;
        } else {
            project.status = Status::Inactive;
        }
    }

    pub fn ban_process(&mut self, process_id: usize) {
        let process = &mut self.game.state.processes[process_id];
        process.banned = true;
    }

    pub fn unban_process(&mut self, process_id: usize) {
        let process = &mut self.game.state.processes[process_id];
        process.banned = false;
    }

    pub fn roll_icon_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::Icon, None, &mut self.rng))?)
    }

    pub fn roll_world_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::World, Some(1), &mut self.rng))?)
    }

    pub fn roll_planning_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::Planning, None, &mut self.rng))?)
    }

    pub fn roll_breaks_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::Breaks, Some(2), &mut self.rng))?)
    }

    pub fn apply_event(&mut self, event_id: usize, region_id: Option<usize>) {
        self.game.apply_event(event_id, region_id);
    }

    pub fn check_requests(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.state.check_requests())?)
    }

    pub fn collect_recently_completed(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.collect_recently_completed(&mut self.rng))?)
    }

    pub fn set_tgav(&mut self, tgav: f32) {
        self.game.state.world.temperature = tgav;
    }

    pub fn active_autoclickers(&self) -> Result<JsValue, JsValue> {
        let projects = self.game.state.projects.iter().filter(|p| p.status == Status::Active || p.status == Status::Finished);
        let autoclicks: Vec<&Effect> = projects.flat_map(|p| p.effects.iter().filter(|e| match e {
            Effect::AutoClick(_, _) => true,
            _ => false
        })).collect();
        Ok(serde_wasm_bindgen::to_value(&autoclicks)?)
    }
}

pub struct Game {
    pub state: State,
    pub event_pool: EventPool,
    // choice_history: [usize; 3],
}

impl Game {
    /// Create a new instance of game with
    /// all the content loaded in
    pub fn new(difficulty: Difficulty) -> Game {
        let mut state = State {
            political_capital: 10,
            malthusian_points: 0,
            hes_points: 0,
            falc_points: 0,

            world: content::world(difficulty),
            projects: content::projects(),
            processes: content::processes(),
            industries: content::industries(),
            npcs: content::npcs(),
            recently_completed: Vec::new(),

            runs: 0,
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
            resources_demand: resources!(),
            resources: consts::STARTING_RESOURCES,
            feedstocks: consts::FEEDSTOCK_RESERVES,
            byproducts: byproducts!(),
            produced: outputs!(),
            consumed_resources: resources!(),
            consumed_feedstocks: feedstocks!(),
        };

        let (output_demand, _) = state.calculate_demand();
        let orders: Vec<ProductionOrder> = state.processes.iter()
            .map(|p| p.production_order(&output_demand)).collect();
        let (required_resources, _) = calculate_required(&orders);
        state.resources.electricity = required_resources.electricity;
        state.resources.fuel = required_resources.fuel;

        state.init();

        Game {
            state,
            event_pool: EventPool::new(content::events()),
        }
    }

    pub fn step(&mut self) -> Vec<usize> {
        self.state.step()
    }

    pub fn roll_events_of_kind(&mut self, kind: EventType, limit: Option<usize>, rng: &mut SmallRng) -> Vec<(usize, Option<usize>)> {
        // Roll for events and collect effects
        let events = self.event_pool.roll_for_kind(kind, &self.state, limit, rng);
        events.iter().map(|(ev, region_id)| (ev.id, *region_id)).collect()
    }

    pub fn apply_event(&mut self, event_id: usize, region_id: Option<usize>) {
        let mut effects = vec![];
        let event = &self.event_pool.events[event_id];
        for effect in &event.effects {
            effects.push((effect.clone(), region_id));
        }

        for (effect, region_id) in effects {
            effect.apply(self, region_id);
        }
    }

    pub fn set_event_choice(&mut self, event_id: usize, choice_id: usize) -> Vec<Effect> {
        let effects = self.event_pool.events[event_id].set_choice(choice_id);
        effects.clone()
    }

    pub fn collect_recently_completed(&mut self, rng: &mut SmallRng) -> Vec<(usize, Option<usize>)> {
        let results = self.state.collect_recently_completed(rng);

        // New effects to apply are gathered here.
        // (Mostly to avoid borrowing conflicts)
        // (Effect, Option<RegionId>)
        let mut effects: Vec<(Effect, Option<usize>)> = Vec::new();
        for (id, outcome_id) in &results {
            let project = &self.state.projects[*id];
            for effect in &project.effects {
                effects.push((effect.clone(), None));
            }
            if let Some(i) = outcome_id {
                let outcome = &project.outcomes[*i];
                for effect in &outcome.effects {
                    effects.push((effect.clone(), None));
                }
            }
        }

        for (effect, region_id) in effects {
            // TODO should be applied immediately instead, when we show
            // the project completion notification
            effect.apply(self, region_id);
        }

        results
    }
}

#[derive(Default, Serialize)]
pub struct State {
    pub world: World,
    pub runs: usize,
    pub industries: Vec<Industry>,
    pub projects: Vec<Project>,
    pub processes: Vec<Process>,

    pub political_capital: isize,
    pub malthusian_points: usize,
    pub hes_points: usize,
    pub falc_points: usize,
    pub npcs: Vec<NPC>,

    // Recently completed projects
    pub recently_completed: Vec<usize>,

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
    pub byproducts: ByproductMap<f32>,
    pub resources_demand: ResourceMap<f32>,
    pub resources: ResourceMap<f32>,
    pub feedstocks: FeedstockMap<f32>,
    pub produced: OutputMap<f32>,
    pub consumed_resources: ResourceMap<f32>,
    pub consumed_feedstocks: FeedstockMap<f32>,
}

impl State {
    pub fn init(&mut self) {
        // Bit of a hack to generate initial state values
        self.step();
        self.world.year -= 1;
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
        let industry_demand = self.industries.iter().fold(resources!(), |acc, ind| acc + ind.resources) * lic_pop;
        output_demand.fuel += industry_demand.fuel;
        output_demand.electricity += industry_demand.electricity;

        // Water and land demand
        let mut resources_demand = resources!();
        resources_demand.water += industry_demand.water;
        resources_demand.land += industry_demand.land;

        // Apply modifiers
        (output_demand * self.output_demand_modifier, resources_demand)
    }

    pub fn step(&mut self) -> Vec<usize> {
        let (output_demand, resources_demand) = self.calculate_demand();
        self.output_demand = output_demand;
        self.resources_demand = resources_demand;
        self.byproducts = byproducts!();

        let lic_pop = self.world.lic_population();
        self.byproducts += self.industries.iter().fold(byproducts!(), |acc, ind| acc + ind.byproducts) * lic_pop;

        // Generate production orders based on current process mixes and demand
        let orders: Vec<ProductionOrder> = self.processes.iter()
            .map(|p| p.production_order(&self.output_demand)).collect();

        // Run production function
        let (produced_by_type,
             consumed_resources,
             consumed_feedstocks,
             byproducts) = produce(&orders, &self.resources, &self.feedstocks);
        self.produced = produced_by_type * self.output_modifier;
        self.byproducts += byproducts;

        self.consumed_resources = consumed_resources;
        self.consumed_feedstocks = consumed_feedstocks;
        self.resources_demand.water += consumed_resources.water;
        self.resources_demand.land += consumed_resources.land;

        self.world.co2_emissions = byproducts.co2;
        self.world.ch4_emissions = byproducts.ch4;
        self.world.n2o_emissions = byproducts.n2o;
        self.world.extinction_rate = self.world.base_extinction_rate + self.resources_demand.land/consts::STARTING_RESOURCES.land * 100.;

        // Float imprecision sometimes causes these values
        // to be slightly negative, so ensure they aren't
        self.feedstocks -= consumed_feedstocks;
        self.resources.fuel -= consumed_resources.fuel - self.produced.fuel;
        self.resources.fuel = self.resources.fuel.max(0.);
        // TODO electricity from past turn should just disappear unless storage network is built
        self.resources.electricity -= consumed_resources.electricity - self.produced.electricity;
        self.resources.electricity = self.resources.electricity.max(0.);

        // Get resource deficit/surplus
        let (required_resources, required_feedstocks) = calculate_required(&orders);

        // Weigh resources by scarcity
        let resource_weights = required_resources / self.resources;
        let feedstock_weights = required_feedstocks / self.feedstocks;

        // Update mixes according to resource scarcity
        update_mixes(&mut self.processes, &self.output_demand, &resource_weights, &feedstock_weights);

        // Advance projects
        let mut completed_projects = Vec::new();
        for project in self.projects.iter_mut().filter(|p| match p.status {
            Status::Building => true,
            _ => false
        }) {
            let completed = project.build();
            if completed {
                self.recently_completed.push(project.id);
                completed_projects.push(project.id);
            }
        }

        self.world.year += 1;
        self.world.update_pop();
        self.world.develop_regions();

        completed_projects
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
                    (active && !process.banned)
                    || (!active && process.banned)
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

    /// Drain recently completed projects and roll for their outcomes.
    /// Returns a vec of (project id, outcome id)
    pub fn collect_recently_completed(&mut self, rng: &mut SmallRng) -> Vec<(usize, Option<usize>)> {
        let ids: Vec<usize> = self.recently_completed.drain(..).collect();

        ids.into_iter().map(|id| {
            let project: &Project = &self.projects[id];
            match project.roll_outcome(self, rng) {
                Some((_outcome, i)) => {
                    (id, Some(i))
                },
                None => (id, None)
            }
        }).collect()
    }
}
