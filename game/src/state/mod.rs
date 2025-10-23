mod game;
mod prefs;
mod ui;

use std::sync::LazyLock;

use egui::mutex::RwLock;
use enum_map::EnumMap;
pub use game::StateExt;
use hes_engine::{Output, OutputMap, State, World};
pub use prefs::Settings;
use serde::{Deserialize, Serialize};
pub use ui::{PlanChange, Points, Tutorial, UIState};

use crate::{
    debug::DEBUG,
    display::factors::{self, Factor},
    vars::Var,
};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub core: State,
    pub ui: UIState,
}
impl Default for GameState {
    fn default() -> Self {
        let state = State::default();
        let ui = UIState::new(state.world.year);
        Self { core: state, ui }
    }
}
impl GameState {
    pub fn from_world(world: World) -> Self {
        let state = State::new(world);
        let ui = UIState::new(state.world.year);
        Self { core: state, ui }
    }
}

impl std::ops::Deref for GameState {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        &self.core
    }
}
impl std::ops::DerefMut for GameState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}

pub fn prepare_game(state: &mut GameState, prefs: &Settings) {
    DEBUG.apply(state);
    init_vars(&state.core);
    state.core.runs = prefs.runs_played;
    state.ui.tutorial = prefs.tutorial;

    state.ui.viewed = state
        .core
        .world
        .projects
        .unlocked()
        .map(|p| p.id)
        .chain(state.core.world.processes.unlocked().map(|p| p.id))
        .collect();
}

static BASE_OUTPUT_DEMAND: LazyLock<RwLock<[OutputMap; 4]>> =
    LazyLock::new(|| RwLock::new([OutputMap::default(); 4]));

fn init_vars(state: &State) {
    *BASE_OUTPUT_DEMAND.write() = state.world.per_capita_demand.clone().map(|d| d.base);

    update_factors(state);
}

pub fn base_demand_by_income_levels(output: Output) -> [f32; 4] {
    BASE_OUTPUT_DEMAND
        .read()
        .iter()
        .map(|demand| demand[output])
        .collect::<Vec<_>>()
        .try_into()
        .expect("Mapping from same size arrays")
}

pub static FACTORS: LazyLock<RwLock<EnumMap<Var, Vec<Factor>>>> =
    LazyLock::new(|| RwLock::new(EnumMap::default()));

pub fn update_factors(state: &hes_engine::State) {
    let mut factors = FACTORS.write();
    *factors = factors::rank(state);
}
