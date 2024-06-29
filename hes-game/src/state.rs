use std::collections::HashMap;

use enum_iterator::Sequence;
use enum_map::EnumMap;
use extend::ext;
use hes_engine::{
    events::{Event, Flag},
    game::Update,
    kinds::{Feedstock, Output},
    production::Process,
    regions::Income,
    world::World,
    Game, ProjectType,
};
use leptos::*;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};
use serde::{Deserialize, Serialize};

use crate::{
    consts::{POINT_COST, WIN_EMISSIONS, WIN_EXTINCTION, WIN_TEMPERATURE},
    display::{factors, format, Var},
};

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct GameState {
    pub game: Game,
    pub ui: UIState,
}
impl GameState {
    pub fn load() -> GameState {
        let (read, _, _) = use_local_storage::<GameState, JsonCodec>("hes.state");
        read.get()
    }

    pub fn resume() {
        // TODO
        // todo!();
    }

    pub fn restart() {
        // TODO
        // todo!();
    }

    pub fn update(&mut self) {
        self.ui.factors = factors::rank(&self.game.state);
    }

    pub fn has_save() -> bool {
        let (state, _, _) = use_local_storage::<Option<GameState>, JsonCodec>("hes.save");
        state.get().is_some()
    }

    /// If we won the game.
    pub fn won(&self) -> bool {
        self.game.state.world.emissions_gt() <= WIN_EMISSIONS
            && self.game.state.world.extinction_rate <= WIN_EXTINCTION
            && self.game.state.world.temperature <= WIN_TEMPERATURE
    }

    /// Maximum production share for a process.
    pub fn process_max_share(&self, process: &Process) -> usize {
        let mut max_share = 1.;
        let demand = self.game.state.output_demand[process.output];

        // Hard-coded limit
        if let Some(limit) = process.limit {
            max_share = (limit / demand).min(1.);
        }

        // Limit based on feedstock supply
        let (feedstock, per_output) = process.feedstock;
        match feedstock {
            Feedstock::Other | Feedstock::Soil => {}
            _ => {
                let feedstock_limit = self.game.state.feedstocks[feedstock] / per_output;
                let feedstock_max_share = (feedstock_limit / demand).min(1.);
                max_share = max_share.min(feedstock_max_share);
            }
        }

        (max_share * 100. / 5.).floor() as usize
    }

    fn new(world: World) -> Self {
        let mut ui_state = UIState::default();

        let (meta, _, _) = use_local_storage::<Meta, JsonCodec>("hes.meta");

        let runs = meta.with_untracked(|meta| meta.runs_played);
        ui_state.start_year = world.year;
        ui_state.tutorial = meta.with_untracked(|meta| meta.tutorial);

        let mut game = Game::new(world);
        game.set_runs_played(runs);

        // Set all starting projects/processes as "viewed"
        ui_state.viewed = game
            .state
            .world
            .projects
            .iter()
            .filter(|p| !p.locked)
            .map(|p| p.ref_id.clone())
            .chain(
                game.state
                    .world
                    .processes
                    .iter()
                    .filter(|p| !p.locked)
                    .map(|p| p.ref_id.clone()),
            )
            .collect();

        Self { game, ui: ui_state }
    }
}

// New game just clears the saved game data
// fn new_game(world: World) {
//     let (_, _, clear_state) = use_local_storage::<Option<GameState>, JsonCodec>("hes.save");
//     clear_state();
// }

/// Settings that persist across sessions.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub read_help: Vec<String>,
    pub hide_help: bool,
    pub sound: bool,
}
impl Settings {
    pub fn get() -> (Signal<Settings>, WriteSignal<Settings>) {
        let (read, write, _) = use_local_storage::<Settings, JsonCodec>("hes.settings");
        (read, write)
    }
}

pub fn read_state<F, R>(func: F) -> impl Fn() -> R + Copy
where
    F: Fn(&Game, &UIState) -> R + Copy,
{
    move || {
        let state = expect_context::<RwSignal<crate::state::GameState>>();
        state.with(|state| func(&state.game, &state.ui))
    }
}

#[macro_export]
macro_rules! state {
    ($func:expr) => {
        crate::state::read_state($func)
    };
}

#[macro_export]
macro_rules! state_with {
    (|$state:ident, $ui:ident $(, $args:ident)*| $body:block) => {
        move || {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            let $state = (move || state.with(|state| state.game.clone())).into_signal();
            let $ui = (move || state.with(|state| state.ui.clone())).into_signal();
            with!(|$state, $ui $(, $args)*| $body)
        }
    };
}

pub fn write_state<F>(func: F) -> impl Fn() + Copy
where
    F: Fn(&mut Game, &mut UIState) + Copy,
{
    move || {
        let state = expect_context::<RwSignal<crate::state::GameState>>();
        state.update(|state| func(&mut state.game, &mut state.ui));
    }
}

#[macro_export]
macro_rules! write_state {
    ($func:expr) => {
        crate::state::write_state($func)
    };
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
struct Meta {
    runs_played: usize,
    tutorial: Tutorial,
}

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Sequence)]
pub enum Tutorial {
    #[default]
    Projects,
    ProjectsBack,
    Processes,
    ProcessesBack,
    Parliament,
    Dashboard,
    Regions,
    Plan,
    Ready,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CycleStart {
    pub year: usize,
    pub emissions: f32,
    pub extinction_rate: f32,
    pub contentedness: f32,
    pub temperature: f32,
    pub region_incomes: Vec<Income>,

    // Seats in parliament for each NPC faction
    pub parliament: Vec<f32>,
    pub completed_projects: Vec<Update>,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanChange {
    points: usize,
    upgrades: usize,
    downgrades: usize,
    withdrawn: bool,
    passed: bool,
}

// TODO
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconEvent {
    pub name: String,
    pub icon: String,
    pub intensity: usize,
}

/// Transient UI-state that is not preserved b/w sessions.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIState {
    pub phase: Phase,
    pub start_year: usize,
    new_run_count: u32,
    tutorial_restarted: bool,
    pub tutorial: Tutorial,
    pub factors: EnumMap<Var, Vec<factors::Factor>>,

    pub annual_region_events: HashMap<usize, Vec<IconEvent>>,
    pub world_events: Vec<usize>,

    // Track which events have occurred
    // events: [],
    //
    //
    // // Track planned process mix changes
    pub process_mix_changes: EnumMap<Output, HashMap<usize, isize>>,

    // // Track changes made to the plan
    // // in a given session, so they can
    // // be reversed/refunded
    pub plan_changes: HashMap<usize, PlanChange>,
    pub queued_upgrades: HashMap<usize, bool>,
    //
    // Compare beginning and end
    pub cycle_start_state: CycleStart,

    pub research_points : isize,
    pub initiative_points: isize,
    pub refundable_research_points: usize,

    // history: {
    //   emissions: [],
    //   land_use: [],
    // },
    //
    // points: {
    //   research: 0,
    //   initiative: 0,
    // },
    /// Viewed project and process ids,
    /// so we can keep track of which ones are new
    pub viewed: Vec<String>,
    // // Kind of hacky
    // extraSeats: {}
}

// TODO load settings

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    #[default]
    Intro,
    Interstitial,
    Planning,
    Events,
    Report,
    GameOver,
    GameWin,
}

#[ext]
pub impl Game {
    fn emissions_gt(&self) -> String {
        format::emissions(self.state.world.emissions_gt())
    }

    fn land_use_percent(&self) -> String {
        let usage = self.state.resources_demand.land;
        let total_land = self.state.world.starting_resources.land;
        let percent = usage / total_land * 100.;
        format::percent(percent, true)
    }

    fn temp_anomaly(&self) -> String {
        format!("{:+.1}C", self.state.world.temperature)
    }

    fn energy_pwh(&self) -> String {
        let energy = self.state.output_demand.energy();
        format!("{}PWh", (format::twh(energy) / 1e3).round())
    }

    fn avg_income_level(&self) -> usize {
        let mut total = 0.;
        for region in &self.state.world.regions {
            let income = region.income_level() as f32 + 1. + region.development;
            total += income;
        }
        let n_regions = self.state.world.regions.len();
        let avg = (total / n_regions as f32).round() as usize;
        avg
    }

    fn avg_habitability(&self) -> f32 {
        let mut total = 0.;
        for region in &self.state.world.regions {
            total += region.habitability();
        }
        let n_regions = self.state.world.regions.len();
        (total / n_regions as f32).round()
    }

    /// Cost for the next point for a project, taking into
    /// account discounts.
    fn next_point_cost(&self, kind: &ProjectType) -> u8 {
        let mut discount = 0;
        if *kind == ProjectType::Research {
            if self.state.flags.contains(&Flag::HyperResearch) {
                discount += 1;
            }
            if self.state.is_ally("The Accelerationist") {
                discount += 1;
            }
        }
        0.max(POINT_COST - discount)
    }
}

impl UIState {
    pub fn has_process_mix_changes(&self) -> bool {
        self.process_mix_changes
            .iter()
            .any(|(_, changes)| changes.iter().any(|(_, change)| *change != 0))
    }
}
