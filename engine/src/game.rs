use crate::world::World;
use crate::industries::Industry;
use crate::projects::{Project, Status};
use crate::production::{ProductionOrder, Process, produce, calculate_required, update_mixes};
use crate::kinds::{OutputMap, ResourceMap, ByproductMap, FeedstockMap};
use crate::events::{Flag, EventPool, Effect};
use crate::{content, consts};
use rand::{SeedableRng, rngs::SmallRng};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Difficulty {
    Easy,
    Normal,
    Hard
}

#[wasm_bindgen]
pub struct GameInterface {
    rng: SmallRng,
    game: Game,
}

#[wasm_bindgen]
impl GameInterface {
    pub fn new(difficulty: Difficulty) -> GameInterface {
        GameInterface {
            rng: SmallRng::from_entropy(),
            game: Game::new(difficulty),
        }
    }

    pub fn step(&mut self) {
        self.game.step(&mut self.rng);
    }

    pub fn state(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.state)?)
    }
}

pub struct Game {
    pub state: State,
    pub event_pool: EventPool,
}

impl Game {
    /// Create a new instance of game with
    /// all the content loaded in
    pub fn new(difficulty: Difficulty) -> Game {
        Game {
            state: State {
                political_capital: 100,
                world: content::world(difficulty),
                projects: content::projects(),
                processes: content::processes(),
                industries: content::industries(),
                flags: Vec::new(),
                runs: 0,

                output: outputs!(),
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
                resources: resources!(),
                feedstocks: consts::FEEDSTOCK_RESERVES,
            },
            event_pool: EventPool::new(content::events()),
        }
    }

    pub fn step(&mut self, rng: &mut SmallRng) {
        let mut effects = self.state.step(rng);

        // Roll for events and collect effects
        let events = self.event_pool.roll(&self.state, rng);
        for (event, region_id) in events {
            for effect in &event.effects {
                effects.push((effect.clone(), region_id));
            }
        }

        for (effect, region_id) in effects {
            effect.apply(self, region_id);
        }
    }
}

#[derive(Default, Serialize)]
pub struct State {
    pub world: World,
    pub runs: usize,
    pub flags: Vec<Flag>,
    pub industries: Vec<Industry>,
    pub projects: Vec<Project>,
    pub processes: Vec<Process>,
    pub political_capital: usize,

    // Modifiers should start as all 1.
    pub output: OutputMap<f32>,
    pub output_modifier: OutputMap<f32>,
    pub output_demand: OutputMap<f32>,
    pub output_demand_modifier: OutputMap<f32>,
    pub resources_demand: ResourceMap<f32>,
    pub resources: ResourceMap<f32>,
    pub feedstocks: FeedstockMap<f32>,
}

impl State {
    pub fn step(&mut self, rng: &mut SmallRng) -> Vec<(Effect, Option<usize>)> {
        // Aggregate demand across regions
        self.output_demand = self.world.demand() * self.output_demand_modifier;

        // Demand and impacts from non-modeled industries
        let lic_pop = self.world.lic_population();
        let industry_demand = self.industries.iter().fold(resources!(), |acc, ind| acc + ind.resources) * lic_pop;
        let industry_byproducts = self.industries.iter().fold(byproducts!(), |acc, ind| acc + ind.byproducts) * lic_pop;
        self.output_demand.fuel += industry_demand.fuel;
        self.output_demand.electricity += industry_demand.electricity;

        // TODO water stress
        // industry_demand.water
        // consumed_resources.water

        // Generate production orders based on current process mixes and demand
        let orders: Vec<ProductionOrder> = self.processes.iter()
            .map(|p| p.production_order(&self.output_demand)).collect();

        // Run production function
        let (mut produced_by_type,
             consumed_resources,
             consumed_feedstocks,
             mut byproducts) = produce(&orders, &self.resources, &self.feedstocks);
        produced_by_type *= self.output_modifier;

        byproducts += industry_byproducts;
        self.world.co2_emissions = byproducts.co2;
        self.world.ch4_emissions = byproducts.ch4;
        self.world.n2o_emissions = byproducts.n2o;
        // TODO biodiversity pressure/extinction rate...how to do that?

        self.feedstocks -= consumed_feedstocks;
        self.resources.fuel -= consumed_resources.fuel + produced_by_type.fuel;
        self.resources.electricity -= consumed_resources.electricity + produced_by_type.electricity;

        // Get resource deficit/surplus
        let (required_resources, required_feedstocks) = calculate_required(&orders);

        // Weigh resources by scarcity
        let resource_weights = required_resources / self.resources;
        let feedstock_weights = required_feedstocks / self.feedstocks;

        // Update mixes according to resource scarcity
        update_mixes(&mut self.processes, &self.output_demand, &resource_weights, &feedstock_weights);

        // New effects to apply are gathered here.
        // (Mostly to avoid borrowing conflicts)
        // (Effect, Option<RegionId>)
        let mut effects: Vec<(Effect, Option<usize>)> = Vec::new();

        // Advance projects
        for project in self.projects.iter_mut().filter(|p| match p.status {
            Status::Building => true,
            _ => false
        }) {
            let completed = project.build();
            if completed {
                for effect in &project.effects {
                    effects.push((effect.clone(), None));
                }
            }
        }

        for project in &self.projects {
            match project.roll_outcome(self, rng) {
                Some((outcome, _i)) => {
                    for effect in &outcome.effects {
                        effects.push((effect.clone(), None));
                    }
                },
                None => ()
            }
        }

        effects
    }
}
