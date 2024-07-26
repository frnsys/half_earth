mod game;
mod settings;
mod ui;

pub use game::GameExt;
pub use settings::Settings;
use ui::Points;
pub use ui::{Phase, PlanChange, Tutorial, UIState};

use std::sync::{LazyLock, RwLock};

use codee::string::JsonSerdeCodec;
use hes_engine::{
    events::IconEvent,
    game::Update,
    kinds::{Feedstock, Output, OutputMap},
    production::Process,
    world::World,
    Game,
    Id,
};
use leptos::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::{
    consts,
    debug::get_debug_opts,
    views::rank_factors,
};

const SAVE_KEY: &str = "hes.save";
pub static STARTING_WATER: LazyLock<RwLock<f32>> =
    LazyLock::new(|| RwLock::new(0.));
pub static STARTING_LAND: LazyLock<RwLock<f32>> =
    LazyLock::new(|| RwLock::new(0.));
pub static BASE_OUTPUT_DEMAND: LazyLock<
    RwLock<[OutputMap; 4]>,
> = LazyLock::new(|| RwLock::new([OutputMap::default(); 4]));

pub fn demand_by_income_levels(output: Output) -> [f32; 4] {
    BASE_OUTPUT_DEMAND
        .read()
        .expect("Can read base output demand")
        .iter()
        .map(|demand| demand[output])
        .collect::<Vec<_>>()
        .try_into()
        .expect("Mapping from same size arrays")
}

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct GameState {
    pub game: Game,
    pub ui: UIState,
}
impl GameState {
    pub fn new(world: World) -> GameState {
        let mut game = Game::from_world(world);
        let mut ui_state = UIState::default();

        let (settings, _) = Settings::rw();

        let runs = settings.with_untracked(|s| s.runs_played);
        ui_state.start_year = game.world.year;
        ui_state.tutorial =
            settings.with_untracked(|s| s.tutorial);

        game.set_runs_played(runs);

        // Set all starting projects/processes as "viewed"
        ui_state.viewed = game
            .state
            .world
            .projects
            .iter()
            .filter(|p| !p.locked)
            .map(|p| p.id)
            .chain(
                game.state
                    .world
                    .processes
                    .iter()
                    .filter(|p| !p.locked)
                    .map(|p| p.id),
            )
            .collect();

        let mut state = Self { game, ui: ui_state };
        state.init();
        state
    }

    pub fn load() -> GameState {
        let (read, _, _) = use_local_storage::<
            Option<GameState>,
            JsonSerdeCodec,
        >(SAVE_KEY);
        if let Some(mut state) = read.get_untracked() {
            state.init();
            state
        } else {
            Self::new(World::default())
        }
    }

    pub fn init(&mut self) {
        *STARTING_WATER
            .write()
            .expect("Can write to shared value") =
            self.game.world.starting_resources.water;
        *STARTING_LAND
            .write()
            .expect("Can write to shared value") =
            self.game.world.starting_resources.land;
        *BASE_OUTPUT_DEMAND
            .write()
            .expect("Can write to shared value") =
            self.game.world.output_demand;

        if get_debug_opts().skip_tutorial {
            self.ui.tutorial = Tutorial::Ready;
        }

        if get_debug_opts().skip_to_planning {
            self.ui.phase = Phase::Planning;
        }
    }

    pub fn save(&self) {
        let (_, write, _) = use_local_storage::<
            Option<GameState>,
            JsonSerdeCodec,
        >(SAVE_KEY);
        write.set(Some(self.clone()));
    }

    pub fn clear_save() {
        let (_, _, clear) = use_local_storage::<
            Option<GameState>,
            JsonSerdeCodec,
        >(SAVE_KEY);
        clear();
    }

    pub fn has_save() -> bool {
        let (state, _, _) = use_local_storage::<
            Option<GameState>,
            JsonSerdeCodec,
        >(SAVE_KEY);
        state.get().is_some()
    }

    pub fn start_new_run() {
        Self::clear_save();
        window().location().reload();
    }

    pub fn initialize_year(&mut self) {
        self.game.update_production();
        self.ui.factors = rank_factors(&self.game.state);
    }

    /// If we won the game.
    pub fn won(&self) -> bool {
        self.game.state.emissions_gt() <= consts::WIN_EMISSIONS
            && self.game.state.world.extinction_rate
                <= consts::WIN_EXTINCTION
            && self.game.state.world.temperature
                <= consts::WIN_TEMPERATURE
    }

    pub fn game_over(&self) -> bool {
        self.game.state.game_over
    }

    /// Maximum production share for a process.
    pub fn process_max_share(
        &self,
        process: &Process,
    ) -> usize {
        let mut max_share = 1.;
        let demand =
            self.game.state.output_demand[process.output];

        // Hard-coded limit
        if let Some(limit) = process.limit {
            max_share = (limit / demand).min(1.);
        }

        // Limit based on feedstock supply
        let (feedstock, per_output) = process.feedstock;
        match feedstock {
            Feedstock::Other | Feedstock::Soil => {}
            _ => {
                let feedstock_limit =
                    self.game.state.feedstocks[feedstock]
                        / per_output;
                let feedstock_max_share =
                    (feedstock_limit / demand).min(1.);
                max_share = max_share.min(feedstock_max_share);
            }
        }

        (max_share * 100. / 5.).floor() as usize
    }

    pub fn upgrade_projects(&mut self) {
        for (id, queued) in self.ui.queued_upgrades.iter_mut() {
            if *queued {
                *queued = false;
                self.game.upgrade_project(id);
            }
        }
    }

    pub fn update_processes(&mut self) {
        let mut rem_pts = consts::PROCESS_POINTS_PER_CYCLE;
        let mut add_pts = consts::PROCESS_POINTS_PER_CYCLE;
        let changes = &mut self.ui.process_mix_changes;

        for (output, changes) in changes.iter_mut() {
            let mut total = changes
                .values()
                .map(|val| val.abs())
                .sum::<isize>();
            while rem_pts > 0 && add_pts > 0 && total > 0 {
                for (process_id, change) in changes.iter_mut() {
                    if *change < 0 && rem_pts > 0 {
                        rem_pts -= 1;
                        self.game.change_process_mix_share(
                            process_id, -1,
                        );
                        total -= 1;
                        *change += 1;
                    } else if *change > 0 && add_pts > 0 {
                        add_pts -= 1;
                        self.game.change_process_mix_share(
                            process_id, 1,
                        );
                        total -= 1;
                        *change -= 1;
                    }
                }
            }
        }
    }

    pub fn apply_disaster(
        &mut self,
        event: &IconEvent,
        event_id: &Id,
        region_id: &Id,
    ) {
        let region_events = self
            .ui
            .annual_region_events
            .entry(*region_id)
            .or_default();
        region_events.push(event.clone());

        let effect = event.intensity as f32
            * consts::EVENT_INTENSITY_TO_CONTENTEDNESS;

        self.game.change_habitability(
            -effect.round() as isize,
            region_id,
        );
        self.game.apply_event(*event_id, Some(*region_id));
    }

    pub fn step_year(&mut self) -> Vec<Update> {
        let mut updates = self.game.step();
        if self.is_planning_year() {
            let mut outcomes =
                self.game.roll_new_policy_outcomes();
            updates.append(&mut outcomes);
        }

        let completed_projects =
            updates.iter().filter_map(|update| match update {
                Update::Project { id } => Some(id),
                _ => None,
            });
        self.ui
            .cycle_start_state
            .completed_projects
            .extend(completed_projects);

        updates
    }

    fn is_planning_year(&self) -> bool {
        self.game.world.year + 1 % 5 == 0
    }
}

/// Extract a single property from the game state.
#[macro_export]
macro_rules! state {
    ($($path:ident).+) => {
        create_memo(move |_| {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            state.with(move |state| state.game.$($path).+)
        })
    };
    ($($path:ident).+ ($($arg:tt)*)) => {
        create_memo(move |_| {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            state.with(move |state| state.game.$($path).+($($arg)*))
        })
    };
}

/// Extract a single property from the UI state.
#[macro_export]
macro_rules! ui {
    ($($path:ident).+) => {
        create_memo(move |_| {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            state.with(move |state| state.ui.$($path).+)
        })
    };
    ($($path:ident).+ ($($arg:tt)*)) => {
        create_memo(move |_| {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            state.with(move |state| state.ui.$($path).+($($arg)*))
        })
    };
}

/// Write access to the game state.
#[macro_export]
macro_rules! state_rw {
    ($($path:ident).+) => {
        {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            slice!(state.game.$($path).+)
        }
    };
}

/// Write access to the UI state.
#[macro_export]
macro_rules! ui_rw {
    ($($path:ident).+) => {
        {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            slice!(state.ui.$($path).+)
        }
    };
}

/// Access the game state, optionally with other signals.
#[macro_export]
macro_rules! with_state {
    (|$state:ident, $ui:ident $(, $args:ident)*| $body:block) => {{
        let state = expect_context::<RwSignal<crate::state::GameState>>();
        move || {
            state.with(|crate::state::GameState { game: $state, ui: $ui }| {
                with_state!(@recurse [$body] $($args),*)
            })
        }
    }};
    (@recurse [$body:block]) => {
        $body
    };
    (@recurse [$body:block] $head:ident $(, $tail:ident)*) => {
        $head.with(|$head| {
            with_state!(@recurse [$body] $($tail),*)
        })
    };
}

// NOTE:
// https://github.com/leptos-rs/leptos/issues/1653
pub fn write_state<F>(func: F) -> impl Fn() + Copy
where
    F: Fn(&mut Game, &mut UIState) + Copy,
{
    move || {
        let state = expect_context::<
            RwSignal<crate::state::GameState>,
        >();
        state.update(|state| {
            func(&mut state.game, &mut state.ui)
        });
    }
}

#[macro_export]
macro_rules! write_state {
    ($func:expr) => {
        crate::state::write_state($func)
    };
}

impl UIState {
    pub fn has_any_process_mix_changes(&self) -> bool {
        self.process_mix_changes.iter().any(|(_, changes)| {
            changes.iter().any(|(_, change)| *change != 0)
        })
    }

    pub fn has_process_mix_changes(
        &self,
        output: Output,
    ) -> bool {
        self.process_mix_changes[output]
            .iter()
            .any(|(_, change)| *change != 0)
    }

    pub fn remove_point(
        &mut self,
        points: &mut isize,
        process: &Process,
    ) {
        let change = self.process_mix_changes[process.output]
            .entry(process.id)
            .or_default();
        if process.mix_share as isize + *change > 0 {
            *points += 1;
            *change -= 1;
            // this.allowBack = false;
        }
    }

    // Returns the point change.
    pub fn add_point(
        &mut self,
        points: &mut isize,
        process: &Process,
        max_share: usize,
    ) {
        if *points > 0 {
            let change = self.process_mix_changes
                [process.output]
                .entry(process.id)
                .or_default();
            if *change + 1 <= max_share as isize {
                *points -= 1;
                *change += 1;
            }
        }
    }
}
