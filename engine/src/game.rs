use crate::content;
use crate::state::State;
use crate::projects::{Status, years_remaining};
use crate::events::{EventPool, Effect, Phase};
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

    pub fn set_runs_played(&mut self, n: usize) {
        self.game.state.runs = n;
    }

    pub fn world_outlook(&self) -> f32 {
        self.game.state.world.outlook()
    }

    pub fn change_political_capital(&mut self, amount: isize) {
        self.game.state.political_capital += amount;

        if self.game.state.political_capital < 0 {
            self.game.state.game_over = true;
        } else {
            self.game.state.game_over = false;
        }
    }

    pub fn change_local_outlook(&mut self, amount: isize, region_id: usize) {
        self.game.state.world.regions[region_id].outlook += amount as f32;
    }

    pub fn change_habitability(&mut self, amount: isize, region_id: usize) {
        self.game.state.world.regions[region_id].base_habitability += amount as f32;
    }

    pub fn set_project_points(&mut self, project_id: usize, points: usize) {
        self.game.state.projects[project_id].set_points(points);
    }

    pub fn start_project(&mut self, project_id: usize) {
        self.game.start_project(project_id, &mut self.rng);
    }

    pub fn stop_project(&mut self, project_id: usize) {
        self.game.stop_project(project_id);
    }

    pub fn upgrade_project(&mut self, project_id: usize) {
        self.game.upgrade_project(project_id);
    }

    pub fn change_process_mix_share(&mut self, process_id: usize, change: isize) {
        self.game.state.change_mix_share(process_id, change);
    }

    pub fn roll_events(&mut self, phase: Phase, limit: Option<usize>) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.roll_events_for_phase(phase, limit, &mut self.rng))?)
    }

    pub fn apply_event(&mut self, event_id: usize, region_id: Option<usize>) {
        self.game.apply_event(event_id, region_id);
    }

    pub fn eval_branch_conditions(&mut self, event_id: usize, region_id: Option<usize>, branch_id: usize) -> bool {
        let (_effects, conds) = &self.game.event_pool.events[event_id].branches[branch_id];
        conds.iter().all(|c| c.eval(&self.game.state, region_id))
    }

    pub fn apply_branch_effects(&mut self, event_id: usize, region_id: Option<usize>, branch_id: usize)  {
        let mut effects = vec![];
        let (efs, _conds) = &self.game.event_pool.events[event_id].branches[branch_id];
        for ef in efs {
            effects.push(ef.clone());
        }
        for effect in effects {
            effect.apply(&mut self.game.state, &mut self.game.event_pool, region_id);
        }
    }

    pub fn check_requests(&mut self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.game.state.check_requests())?)
    }

    pub fn set_tgav(&mut self, tgav: f32) {
        self.game.state.set_tgav(tgav);
    }

    pub fn active_autoclickers(&self) -> Result<JsValue, JsValue> {
        let projects = self.game.state.projects.iter().filter(|p| p.status == Status::Active || p.status == Status::Finished);
        let autoclicks: Vec<&Effect> = projects.flat_map(|p| p.active_effects().iter().filter(|e| match e {
            Effect::AutoClick(_, _) => true,
            _ => false
        })).collect();
        Ok(serde_wasm_bindgen::to_value(&autoclicks)?)
    }

    pub fn industry_demand(&self, industry_id: usize) -> f32 {
        let ind = &self.game.state.industries[industry_id];
        ind.demand_modifier * self.game.state.world.lic_population()
    }

    pub fn industry_resources(&self, industry_id: usize) -> Result<JsValue, JsValue> {
        let ind = &self.game.state.industries[industry_id];
        Ok(serde_wasm_bindgen::to_value(&ind.adj_resources())?)
    }

    pub fn industry_byproducts(&self, industry_id: usize) -> Result<JsValue, JsValue> {
        let ind = &self.game.state.industries[industry_id];
        Ok(serde_wasm_bindgen::to_value(&ind.adj_byproducts())?)
    }

    pub fn region_demand(&self, region_id: usize) -> Result<JsValue, JsValue> {
        let reg = &self.game.state.world.regions[region_id];
        Ok(serde_wasm_bindgen::to_value(&reg.demand())?)
    }

    pub fn region_demand_levels(&self, region_id: usize) -> Result<JsValue, JsValue> {
        let reg = &self.game.state.world.regions[region_id];
        Ok(serde_wasm_bindgen::to_value(&reg.demand_levels())?)
    }

    pub fn region_income_level(&self, region_id: usize) -> usize {
        let reg = &self.game.state.world.regions[region_id];
        reg.income_level()
    }

    pub fn region_habitability(&self, region_id: usize) -> Result<JsValue, JsValue> {
        let reg = &self.game.state.world.regions[region_id];
        Ok(serde_wasm_bindgen::to_value(&reg.habitability())?)
    }

    pub fn years_remaining(&self, progress: f32, points: usize, cost: usize) -> usize {
        years_remaining(progress, points, cost)
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
        Game {
            state: State::new(difficulty),
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
                land: state.resources_demand.land,
                emissions: state.world.emissions(),
                energy: state.output_demand.electricity + state.output_demand.fuel,
                population: state.world.population()
            });
        }

        snapshots
    }

    pub fn roll_events_for_phase(&mut self, phase: Phase, limit: Option<usize>, rng: &mut SmallRng) -> Vec<(usize, Option<usize>)> {
        // Roll for events and collect effects
        let events = self.event_pool.roll_for_phase(phase, &self.state, limit, rng);
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

    pub fn start_project(&mut self, project_id: usize, rng: &mut SmallRng) {
        let effects = self.state.start_project(project_id, rng);
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
    land: f32,
    emissions: f32,
    energy: f32,
    population: f32,
}
