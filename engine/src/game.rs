use crate::state::State;
use crate::projects::Status;
use crate::production::Priority;
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
        Ok(serde_wasm_bindgen::to_value(&self.game.step(&mut self.rng))?)
    }

    pub fn state(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.state)?)
    }

    pub fn change_political_capital(&mut self, amount: isize) {
        self.game.state.political_capital += amount;
    }

    pub fn change_local_outlook(&mut self, amount: isize, region_id: usize) {
        self.game.state.world.regions[region_id].outlook += amount as f32;
    }

    pub fn set_event_choice(&mut self, event_id: usize, region_id: Option<usize>, choice_id: usize) {
        self.game.set_event_choice(event_id, region_id, choice_id);
    }

    pub fn set_project_points(&mut self, project_id: usize, points: usize) {
        self.game.state.projects[project_id].set_points(points);
    }

    pub fn start_project(&mut self, project_id: usize) {
        self.game.start_project(project_id);
    }

    pub fn stop_project(&mut self, project_id: usize) {
        self.game.stop_project(project_id);
    }

    pub fn upgrade_project(&mut self, project_id: usize) {
        self.game.upgrade_project(project_id);
    }

    pub fn ban_process(&mut self, process_id: usize) {
        self.game.state.ban_process(process_id);
    }

    pub fn unban_process(&mut self, process_id: usize) {
        self.game.state.unban_process(process_id);
    }

    pub fn promote_process(&mut self, process_id: usize) {
        self.game.state.promote_process(process_id);
    }

    pub fn unpromote_process(&mut self, process_id: usize) {
        self.game.state.unpromote_process(process_id);
    }

    pub fn roll_icon_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::Icon, None, &mut self.rng))?)
    }

    pub fn roll_world_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::World, Some(5), &mut self.rng))?)
    }

    pub fn roll_planning_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::Planning, None, &mut self.rng))?)
    }

    pub fn roll_report_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::Report, None, &mut self.rng))?)
    }

    pub fn roll_world_start_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::WorldStart, None, &mut self.rng))?)
    }

    pub fn roll_breaks_events(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_of_kind(EventType::Breaks, None, &mut self.rng))?)
    }

    pub fn apply_event(&mut self, event_id: usize, region_id: Option<usize>) {
        self.game.apply_event(event_id, region_id);
    }

    pub fn check_requests(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.state.check_requests())?)
    }

    pub fn set_tgav(&mut self, tgav: f32) {
        self.game.state.world.temperature = tgav + self.game.state.world.temperature_modifier;
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.game.state.priority = priority;
    }

    pub fn active_autoclickers(&self) -> Result<JsValue, JsValue> {
        let projects = self.game.state.projects.iter().filter(|p| p.status == Status::Active || p.status == Status::Finished);
        let autoclicks: Vec<&Effect> = projects.flat_map(|p| p.active_effects().iter().filter(|e| match e {
            Effect::AutoClick(_, _) => true,
            _ => false
        })).collect();
        Ok(serde_wasm_bindgen::to_value(&autoclicks)?)
    }

    pub fn total_income_level(&self) -> f32 {
        self.game.state.world.regions.iter().map(|r| r.adjusted_income()).sum()
    }

    pub fn simulate(&mut self, years: usize) -> Result<JsValue, JsValue>  {
        Ok(serde_wasm_bindgen::to_value(&self.game.simulate(&mut self.rng, years))?)
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
        let mut state = State {
            // political_capital: 10,
            political_capital: 100,
            malthusian_points: 0,
            hes_points: 0,
            falc_points: 0,
            flags: Vec::new(),
            priority: Priority::Scarcity,

            world: content::world(difficulty),
            projects: content::projects(),
            processes: content::processes(),
            industries: content::industries(),
            npcs: content::npcs(),

            runs: 1, // TODO TEMP TESTING
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

        state.init();

        Game {
            state,
            event_pool: EventPool::new(content::events()),
        }
    }

    pub fn step(&mut self, rng: &mut SmallRng) -> Vec<usize> {
        let (completed_projects, remove_effects, add_effects) = self.state.step_projects(rng);
        for (effect, region_id) in remove_effects {
            effect.unapply(&mut self.state, &mut self.event_pool, region_id);
        }
        for (effect, region_id) in add_effects {
            effect.apply(&mut self.state, &mut self.event_pool, region_id);
        }
        self.state.step_production();
        self.state.step_world();

        completed_projects
    }

    /// Generate a projection
    pub fn simulate(&self, rng: &mut SmallRng, years: usize) -> Vec<Snapshot> {
        let mut snapshots: Vec<Snapshot> = Vec::new();

        // TODO can probably re structure all of this so
        // that there is only a struct that deals with all things production
        // rather than having to clone the entire state
        let mut state = self.state.clone();

        // Dummy event pool
        let mut event_pool = EventPool::new(content::events());
        for _ in 0..years {
            let (_completed_projects, remove_effects, add_effects) = state.step_projects(rng);
            for (effect, region_id) in remove_effects {
                effect.unapply(&mut state, &mut event_pool, region_id);
            }
            for (effect, region_id) in add_effects {
                effect.apply(&mut state, &mut event_pool, region_id);
            }
            state.step_production();
            state.step_world();
            snapshots.push(Snapshot {
                land_use: state.resources_demand.land,
                emissions: state.world.emissions(),
            });
        }

        snapshots
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
            effect.apply(&mut self.state, &mut self.event_pool, region_id);
        }
    }

    pub fn set_event_choice(&mut self, event_id: usize, region_id: Option<usize>, choice_id: usize) {
        let effects = self.event_pool.events[event_id].set_choice(choice_id);
        for effect in effects.clone() {
            effect.apply(&mut self.state, &mut self.event_pool, region_id);
        }
    }

    pub fn start_project(&mut self, project_id: usize) {
        let effects = self.state.start_project(project_id);
        for effect in effects {
            effect.apply(&mut self.state, &mut self.event_pool, None);
        }
    }

    pub fn stop_project(&mut self, project_id: usize) {
        let effects = self.state.stop_project(project_id);
        for effect in effects {
            effect.unapply(&mut self.state, &mut self.event_pool, None);
        }
    }

    pub fn upgrade_project(&mut self, project_id: usize) {
        let (remove_effects, add_effects) = self.state.upgrade_project(project_id);
        for effect in remove_effects {
            effect.unapply(&mut self.state, &mut self.event_pool, None);
        }
        for effect in add_effects {
            effect.apply(&mut self.state, &mut self.event_pool, None);
        }
    }
}

#[derive(Serialize)]
pub struct Snapshot {
    land_use: f32,
    emissions: f32,
}
