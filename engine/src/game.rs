use rand::rngs::StdRng;
use crate::world::World;
use crate::player::Player;
use crate::regions::Region;
use crate::projects::{Project, Status};
use crate::production::{ProductionOrder, Process, ExtractionManager, produce, calculate_required, update_mixes};
use crate::kinds::{OutputMap, ResourceMap, ByproductMap, FeedstockMap};
use crate::events::{Flag, EventPool, Effect};
use crate::content;

pub enum Difficulty {
    Easy,
    Normal,
    Hard
}

pub struct Game {
    pub state: State,
    pub event_pool: EventPool,
}

impl Game {
    /// Create a new instance of game with
    /// all the content loaded in
    pub fn new(difficulty: Difficulty) -> Game {
        let world = match difficulty {
            Difficulty::Easy => content::WORLDS[0],
            Difficulty::Normal => content::WORLDS[1],
            Difficulty::Hard => content::WORLDS[2],
        };

        Game {
            state: State {
                world,
                player: Player {
                    political_capital: 100,
                },
                regions: content::regions(),
                projects: content::projects(),
                processes: content::processes(),
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
                feedstocks: feedstocks!(),
                byproducts: byproducts!(),
                extraction: ExtractionManager {
                    // TODO
                }
            },
            event_pool: EventPool::new(content::events()),
        }
    }

    pub fn step(&mut self, rng: &mut StdRng) {
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

#[derive(Default)]
pub struct State {
    pub world: World,
    pub player: Player,
    pub regions: Vec<Region>,
    pub projects: Vec<Project>,
    pub processes: Vec<Process>,
    pub flags: Vec<Flag>,
    pub runs: usize,

    // Modifiers should start as all 1.
    pub output: OutputMap<f32>,
    pub output_modifier: OutputMap<f32>,
    pub output_demand: OutputMap<f32>,
    pub output_demand_modifier: OutputMap<f32>,
    pub resources_demand: ResourceMap<f32>,
    pub resources: ResourceMap<f32>,
    pub feedstocks: FeedstockMap<f32>,
    pub byproducts: ByproductMap<f32>,
    extraction: ExtractionManager,
}


impl State {
    pub fn step(&mut self, rng: &mut StdRng) -> Vec<(Effect, Option<usize>)> {
        // Extract feedstocks
        self.feedstocks += self.extraction.extract();

        // Aggregate demand across regions
        // TODO use self.output_demand
        let mut demand = self.regions.iter().fold(outputs!(), |mut acc, region| {
            acc += region.demand();
            acc
        });
        demand *= self.output_demand_modifier;

        // TODO industry demand

        // Generate production orders based on current process mixes and demand
        let orders: Vec<ProductionOrder> = self.processes.iter()
            .map(|p| p.production_order(&demand)).collect();

        // Run production function
        let (mut produced_by_type, consumed_resources, consumed_feedstocks, byproducts) = produce(&orders, &self.resources, &self.feedstocks);
        produced_by_type *= self.output_modifier;

        self.byproducts += byproducts;
        self.feedstocks -= consumed_feedstocks;
        self.resources.fuel -= consumed_resources.fuel + produced_by_type.fuel;
        self.resources.electricity -= consumed_resources.electricity + produced_by_type.electricity;

        // Calculate production shorfalls
        let shortfalls = demand - produced_by_type;

        // Get resource deficit/surplus
        let (required_resources, required_feedstocks) = calculate_required(&orders);

        // Weigh resources by scarcity
        let resource_weights = required_resources / self.resources;
        let feedstock_weights = required_feedstocks / self.feedstocks;

        // Update mixes according to resource scarcity
        update_mixes(&mut self.processes, &demand, &resource_weights, &feedstock_weights);

        // Expand/contract extraction
        self.extraction.adjust(&required_feedstocks);

        // New effects to apply are gathered here.
        // (Mostly to avoid borrowing conflicts)
        let mut effects: Vec<(Effect, Option<usize>)> = Vec::new();

        // Advance projects
        for project in self.projects.iter_mut().filter(|p| match p.status {
            Status::Building(_) => true,
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
                Some((outcome, i)) => {
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
