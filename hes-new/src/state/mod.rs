mod game;
mod prefs;
mod ui;

use std::sync::{Arc, LazyLock};

use egui::mutex::RwLock;
use enum_map::EnumMap;
pub use game::StateExt;
use hes_engine::{Output, OutputMap};
pub use prefs::Settings;
pub use ui::{PlanChange, Points, Tutorial, UIState};

use serde::{Deserialize, Serialize};

use crate::{
    display::factors::{self, Factor},
    vars::Var,
};

pub static STATE: LazyLock<Arc<RwLock<State>>> =
    LazyLock::new(|| Arc::new(RwLock::new(State::default())));

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub prefs: Settings,
    pub game: hes_engine::State,
}

pub static BASE_OUTPUT_DEMAND: LazyLock<
    RwLock<[OutputMap; 4]>,
> = LazyLock::new(|| RwLock::new([OutputMap::default(); 4]));

pub fn base_demand_by_income_levels(
    output: Output,
) -> [f32; 4] {
    BASE_OUTPUT_DEMAND
        .read()
        .iter()
        .map(|demand| demand[output])
        .collect::<Vec<_>>()
        .try_into()
        .expect("Mapping from same size arrays")
}

pub static FACTORS: LazyLock<
    RwLock<EnumMap<Var, Vec<Factor>>>,
> = LazyLock::new(|| RwLock::new(EnumMap::default()));

pub fn update_factors(state: &hes_engine::State) {
    let mut factors = FACTORS.write();
    *factors = factors::rank(state);
}
