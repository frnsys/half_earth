use std::collections::HashMap;

use hes_engine::{
    events::Flag, kinds::Feedstock, production::Process, world::World, Game, ProjectType,
};
use leptos::*;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};
use serde::{Deserialize, Serialize};

use crate::consts::{POINT_COST, WIN_EMISSIONS, WIN_EXTINCTION, WIN_TEMPERATURE};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct GameState {
    game: Game,
    ui: UIState,
}
impl GameState {
    pub fn resume() {
        // TODO
        // todo!();
    }

    pub fn restart() {
        // TODO
        // todo!();
    }

    pub fn has_save() -> bool {
        let (state, _, _) = use_local_storage::<Option<GameState>, JsonCodec>("hes.save");
        state.get().is_some()
    }

    /// Cost for the next point for a project, taking into
    /// account discounts.
    pub fn next_point_cost(&self, kind: ProjectType) -> u8 {
        let mut discount = 0;
        if kind == ProjectType::Research {
            if self.game.state.flags.contains(&Flag::HyperResearch) {
                discount += 1;
            }
            if self.game.state.is_ally("The Accelerationist") {
                discount += 1;
            }
        }
        0.max(POINT_COST - discount)
    }

    /// If we won the game.
    pub fn won(&self) -> bool {
        self.game.state.world.total_emissions_gt() <= WIN_EMISSIONS
            && self.game.state.world.extinction_rate <= WIN_EXTINCTION
            && self.game.state.world.temperature <= WIN_TEMPERATURE
    }

    /// Maximum production share for a process.
    pub fn process_max_share(&self, process: &Process) -> f32 {
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

        (max_share * 100. / 5.).floor()
    }

    fn new(world: World) -> Self {
        let mut game = Game::new(world);
        let mut ui_state = UIState::default();

        let (meta, _, _) = use_local_storage::<Meta, JsonCodec>("hes.meta");

        let runs = meta.with_untracked(|meta| meta.runs_played);
        game.set_runs_played(runs);
        ui_state.tutorial = meta.with_untracked(|meta| meta.tutorial);

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

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
struct Meta {
    runs_played: usize,
    tutorial: u8,
}

/// Transient UI-state that is not preserved b/w sessions.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
struct UIState {
    phase: Phase,
    new_run_count: u32,
    tutorial_restarted: bool,
    tutorial: u8,

    // Track which events have occurred
    // events: [],
    //
    // annualRegionEvents: {},
    //
    // // Track planned process mix changes
    // processMixChanges: {
    //   Electricity: {},
    //   Fuel: {},
    //   PlantCalories: {},
    //   AnimalCalories: {},
    // },
    //
    // // Track changes made to the plan
    // // in a given session, so they can
    // // be reversed/refunded
    // planChanges: {},
    // queuedUpgrades: {},
    //
    // // Compare beginning and end
    // cycleStartState: {
    //   emissions: 0,
    //   extinctionRate: 0,
    //   contentedness: 0
    // },
    // history: {
    //   emissions: [],
    //   land_use: [],
    // },
    //
    // points: {
    //   research: 0,
    //   initiative: 0,
    // },
    // refundableResearchPoints: 0,
    /// Viewed project and process ids,
    /// so we can keep track of which ones are new
    viewed: Vec<String>,
    // // Kind of hacky
    // extraSeats: {}
}

// TODO load settings

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
enum Phase {
    #[default]
    Intro,
    Interstitial,
    Planning,
    Events,
    Report,
    GameOver,
    GameWin,
}
