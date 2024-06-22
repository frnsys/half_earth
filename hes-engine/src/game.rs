use crate::events::{Effect, EventPool, Phase};
use crate::projects::Status;
use crate::state::State;
use crate::utils;
use crate::world::World;
use rand::{rngs::SmallRng, SeedableRng};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Update {
    pub id: usize,

    #[wasm_bindgen(skip)]
    pub kind: UpdateType,
}

#[derive(PartialEq)]
pub enum UpdateType {
    RegionUp,
    RegionDown,
    Policy,
    Project,
}

#[wasm_bindgen]
impl Update {
    fn is_region(&self) -> bool {
        matches!(&self.kind, UpdateType::RegionUp | UpdateType::RegionDown)
    }

    fn is_region_up(&self) -> bool {
        matches!(&self.kind, UpdateType::RegionUp)
    }

    fn is_region_down(&self) -> bool {
        matches!(&self.kind, UpdateType::RegionDown)
    }

    fn is_project(&self) -> bool {
        matches!(&self.kind, UpdateType::Project | UpdateType::Policy)
    }

    fn is_policy(&self) -> bool {
        matches!(&self.kind, UpdateType::Policy)
    }
}

// #[wasm_bindgen]
// impl GameInterface {
//     pub fn step(&mut self) -> Result<JsValue, JsValue> {
//         Ok(serde_wasm_bindgen::to_value(
//             &self.game.step(&mut self.rng),
//         )?)
//     }
//
//     pub fn start_project(&mut self, project_id: usize) {
//         self.game.start_project(project_id, &mut self.rng);
//     }
//
//     pub fn stop_project(&mut self, project_id: usize) {
//         self.game.stop_project(project_id);
//     }
//
//     pub fn roll_events(&mut self, phase: Phase, limit: Option<usize>) -> Result<JsValue, JsValue> {
//         Ok(serde_wasm_bindgen::to_value(
//             &self.game.roll_events_for_phase(phase, limit, &mut self.rng),
//         )?)
//     }
//
//     pub fn eval_branch_conditions(
//         &mut self,
//         event_id: usize,
//         region_id: Option<usize>,
//         branch_id: usize,
//     ) -> bool {
//         let event = &self.game.event_pool.events[event_id];
//         if branch_id < event.branches.len() {
//             let (_effects, conds) = &event.branches[branch_id];
//             conds.iter().all(|c| c.eval(&self.game.state, region_id))
//         } else {
//             true
//         }
//     }
//
//     pub fn check_requests(&mut self) -> Result<JsValue, JsValue> {
//         Ok(serde_wasm_bindgen::to_value(
//             &self.game.state.check_requests(),
//         )?)
//     }
//
//     pub fn simulate(&mut self, years: usize) -> Result<JsValue, JsValue> {
//         Ok(serde_wasm_bindgen::to_value(
//             &self.game.simulate(&mut self.rng, years),
//         )?)
//     }
// }

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Game {
    pub state: State,
    event_pool: EventPool,

    #[serde(skip, default = "SmallRng::from_entropy")]
    rng: SmallRng,
}

impl Game {
    pub fn from_world_string(&self, value: String) -> Self {
        let world: World = serde_json::from_str(&value).unwrap();
        Game::new(world)
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_string(&self, value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }

    pub fn step(&mut self) -> Vec<Update> {
        let (completed_projects, remove_effects, add_effects) =
            self.state.step_projects(&mut self.rng);
        for (effect, region_id) in remove_effects {
            effect.unapply(&mut self.state, &mut self.event_pool, region_id);
        }
        for (effect, region_id) in add_effects {
            effect.apply(&mut self.state, &mut self.event_pool, region_id);
        }
        self.state.step_production();
        let mut changes = self.state.step_world();

        changes.extend(completed_projects.into_iter().map(|id| Update {
            id,
            kind: UpdateType::Project,
        }));
        changes
    }

    pub fn step_cycle(&mut self) {
        self.state.step_cycle();
    }

    pub fn update_production(&mut self) {
        self.state.update_production();
    }

    pub fn change_process_mix_share(&mut self, process_id: usize, change: isize) {
        self.state.change_mix_share(process_id, change);
    }

    pub fn upgrade_project(&mut self, project_id: usize) {
        let (remove_effects, add_effects) = self.state.upgrade_project(project_id);
        for effect in remove_effects {
            effect.unapply(&mut self.state, &mut self.event_pool, None);
        }
        for effect in add_effects {
            effect.apply(&mut self.state, &mut self.event_pool, None);
        }
        self.state.update_demand();
    }

    pub fn downgrade_project(&mut self, project_id: usize) {
        let (remove_effects, add_effects) = self.state.downgrade_project(project_id);
        for effect in remove_effects {
            effect.unapply(&mut self.state, &mut self.event_pool, None);
        }
        for effect in add_effects {
            effect.apply(&mut self.state, &mut self.event_pool, None);
        }
        self.state.update_demand();
    }

    pub fn apply_event(&mut self, event_id: usize, region_id: Option<usize>) {
        let mut effects = vec![];
        let event = &self.event_pool.events[event_id];
        for effect in &event.effects {
            effects.push((effect.clone(), region_id));
        }

        for (effect, region_id) in effects {
            effect.apply(&mut self.state, &mut self.event_pool, region_id);
        }
    }

    pub fn apply_branch_effects(
        &mut self,
        event_id: usize,
        region_id: Option<usize>,
        branch_id: usize,
    ) {
        let mut effects = vec![];
        let (efs, _conds) = &self.event_pool.events[event_id].branches[branch_id];
        for ef in efs {
            effects.push(ef.clone());
        }
        for effect in effects {
            effect.apply(&mut self.state, &mut self.event_pool, region_id);
        }
    }

    pub fn roll_new_policy_outcomes(&mut self) -> Vec<Update> {
        let (ids, effects) = self.state.roll_new_policy_outcomes(&mut self.rng);
        for effect in effects {
            effect.apply(&mut self.state, &mut self.event_pool, None);
        }
        self.state.update_demand();
        ids.into_iter()
            .map(|id| Update {
                id,
                kind: UpdateType::Policy,
            })
            .collect()
    }

    /// Generate a projection
    pub fn simulate(&mut self, years: usize) -> Vec<Snapshot> {
        let mut snapshots: Vec<Snapshot> = Vec::new();
        let mut state = self.state.clone();

        // Hacky, but ignore all feedstock constraints
        // otherwise projections are weird
        // because the simulation can't react to events
        state.feedstocks.oil *= 10000.;
        state.feedstocks.coal *= 10000.;
        state.feedstocks.natural_gas *= 10000.;
        state.feedstocks.uranium *= 10000.;

        // Dummy event pool
        let mut event_pool = self.event_pool.clone();
        for _ in 0..years {
            let (_completed_projects, remove_effects, add_effects) =
                state.step_projects(&mut self.rng);
            for (effect, region_id) in remove_effects {
                effect.unapply(&mut state, &mut event_pool, region_id);
            }
            for (effect, region_id) in add_effects {
                effect.apply(&mut state, &mut event_pool, region_id);
            }
            state.step_production();
            state.step_world();
            snapshots.push(Snapshot {
                land: state.resources_demand.land,
                emissions: state.world.emissions(),
                energy: state.output_demand.electricity + state.output_demand.fuel,
                population: state.world.population(),
            });
        }

        snapshots
    }

    pub fn set_runs_played(&mut self, n: usize) {
        self.state.runs = n;
    }

    pub fn set_tgav(&mut self, tgav: f32) {
        self.state.set_tgav(tgav);
    }

    pub fn change_political_capital(&mut self, amount: isize) {
        self.state.political_capital += amount;
    }

    pub fn collect_research_points(&mut self) -> isize {
        let points = self.state.research_points;
        self.state.research_points = 0;
        points
    }

    pub fn change_local_outlook(&mut self, amount: isize, region_id: usize) {
        self.state.world.regions[region_id].outlook += amount as f32;
    }

    pub fn change_habitability(&mut self, amount: isize, region_id: usize) {
        self.state.world.regions[region_id].base_habitability += amount as f32;
    }

    pub fn set_project_points(&mut self, project_id: usize, points: usize) {
        self.state.world.projects[project_id].set_points(points);
    }

    pub fn industry_demand(&self, industry_id: usize) -> f32 {
        let ind = &self.state.world.industries[industry_id];
        ind.demand_modifier * self.state.world.lic_population()
    }
}

impl Game {
    /// Create a new instance of game with
    /// all the content loaded in
    pub fn new(world: World) -> Game {
        // So we get tracebacks in the console
        utils::set_panic_hook();

        let pool = EventPool::new(world.events.clone());
        Game {
            state: State::new(world),
            event_pool: pool,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn roll_events_for_phase(
        &mut self,
        phase: Phase,
        limit: Option<usize>,
        rng: &mut SmallRng,
    ) -> Vec<(usize, Option<usize>)> {
        // Roll for events and collect effects
        let events = self
            .event_pool
            .roll_for_phase(phase, &self.state, limit, rng);
        events
            .iter()
            .map(|(ev, region_id)| (ev.id, *region_id))
            .collect()
    }

    pub fn start_project(&mut self, project_id: usize, rng: &mut SmallRng) {
        self.state.start_project(project_id, rng);
        self.state.update_demand();
    }

    pub fn stop_project(&mut self, project_id: usize) {
        let effects = self.state.stop_project(project_id);
        for effect in effects {
            effect.unapply(&mut self.state, &mut self.event_pool, None);
        }
        self.state.update_demand();
    }
}

#[wasm_bindgen]
pub struct Snapshot {
    land: f32,
    emissions: f32,
    energy: f32,
    population: f32,
}
