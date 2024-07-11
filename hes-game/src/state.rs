use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use enum_iterator::Sequence;
use enum_map::EnumMap;
use extend::ext;
use hes_engine::{
    events::Flag,
    game::Update,
    kinds::{Feedstock, Output, OutputMap},
    production::Process,
    projects::{Project, Status},
    regions::Income,
    Game,
    ProjectType,
};
use leptos::*;
use leptos_use::{
    storage::use_local_storage,
    utils::JsonCodec,
};
use serde::{Deserialize, Serialize};

use crate::{
    consts,
    display,
    vars::Var,
    views::{rank_factors, Factor},
};

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
    pub fn load() -> GameState {
        // TODO
        // let (read, _, _) = use_local_storage::<GameState, JsonCodec>("hes.state");
        // read.get()

        let game = Game::default();
        *STARTING_WATER
            .write()
            .expect("Can write to shared value") =
            game.world.starting_resources.water;
        *STARTING_LAND
            .write()
            .expect("Can write to shared value") =
            game.world.starting_resources.land;
        *BASE_OUTPUT_DEMAND
            .write()
            .expect("Can write to shared value") =
            game.world.output_demand;

        let mut gs = GameState::new(Game::default());
        gs.ui.tutorial = Tutorial::Ready; // TODO testing

        gs
    }

    pub fn resume() {
        // TODO
        // todo!();
    }

    pub fn restart() {
        // TODO
        // todo!();
    }

    pub fn initialize_year(&mut self) {
        logging::log!("INITIALIZING YEAR");
        self.ui.factors = rank_factors(&self.game.state);
    }

    pub fn has_save() -> bool {
        let (state, _, _) = use_local_storage::<
            Option<GameState>,
            JsonCodec,
        >("hes.save");
        state.get().is_some()
    }

    /// If we won the game.
    pub fn won(&self) -> bool {
        self.game.state.emissions_gt() <= consts::WIN_EMISSIONS
            && self.game.state.world.extinction_rate
                <= consts::WIN_EXTINCTION
            && self.game.state.world.temperature
                <= consts::WIN_TEMPERATURE
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

    fn new(mut game: Game) -> Self {
        let mut ui_state = UIState::default();

        let (meta, _, _) =
            use_local_storage::<Meta, JsonCodec>("hes.meta");

        let runs = meta.with_untracked(|meta| meta.runs_played);
        ui_state.start_year = game.world.year;
        ui_state.tutorial =
            meta.with_untracked(|meta| meta.tutorial);

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
        let (read, write, _) = use_local_storage::<
            Settings,
            JsonCodec,
        >("hes.settings");
        (read, write)
    }
}

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

#[macro_export]
macro_rules! state_rw {
    ($($path:ident).+) => {
        {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            slice!(state.game.$($path).+)
        }
    };
}

#[macro_export]
macro_rules! ui_rw {
    ($($path:ident).+) => {
        {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            slice!(state.ui.$($path).+)
        }
    };
}

#[macro_export]
macro_rules! with_state {
    (|$state:ident, $ui:ident $(, $args:ident)*| $body:block) => {{
        let state = expect_context::<RwSignal<crate::state::GameState>>();
        move || {
            state.with(|crate::state::GameState { game: $state, $ui }| {
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

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
struct Meta {
    runs_played: usize,
    tutorial: Tutorial,
}

#[derive(
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Sequence,
)]
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
impl Tutorial {
    pub fn advance(&mut self) {
        if let Some(next) = self.next() {
            *self = next;
        }
    }
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
    pub points: usize,
    pub upgrades: usize,
    pub downgrades: usize,
    pub withdrawn: bool,
    pub passed: bool,
}

// TODO
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconEvent {
    pub name: String,
    pub icon: String,
    pub intensity: usize,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Points {
    pub research: isize,
    pub initiative: isize,
    pub refundable_research: usize,
}

/// Transient UI-state that is not preserved b/w sessions.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIState {
    pub phase: Phase,
    pub start_year: usize,
    new_run_count: u32,
    tutorial_restarted: bool,
    pub tutorial: Tutorial,
    pub factors: EnumMap<Var, Vec<Factor>>,

    pub annual_region_events: HashMap<usize, Vec<IconEvent>>,
    pub world_events: Vec<usize>,

    // Track which events have occurred
    // events: [],
    //
    //
    // // Track planned process mix changes
    pub process_mix_changes:
        EnumMap<Output, HashMap<usize, isize>>,

    // // Track changes made to the plan
    // // in a given session, so they can
    // // be reversed/refunded
    pub plan_changes: HashMap<usize, PlanChange>,
    pub queued_upgrades: HashMap<usize, bool>,
    //
    // Compare beginning and end
    pub cycle_start_state: CycleStart,

    pub points: Points,

    // history: {
    //   emissions: [],
    //   land_use: [],
    // },
    //
    /// Viewed project and process ids,
    /// so we can keep track of which ones are new
    pub viewed: Vec<String>,
}

// TODO load settings

#[derive(
    Default, Clone, Copy, PartialEq, Serialize, Deserialize,
)]
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
        display::emissions(self.state.emissions_gt())
    }

    fn land_use_percent(&self) -> String {
        let usage = self.state.resources_demand.land;
        let total_land =
            self.state.world.starting_resources.land;
        let percent = usage / total_land;
        display::percent(percent, true)
    }

    fn water_use_percent(&self) -> String {
        let usage = self.state.resources_demand.water;
        let total_water = self.state.resources.water;
        let percent = usage / total_water;
        display::percent(percent, true)
    }

    fn temp_anomaly(&self) -> String {
        format!("{:+.1}C", self.state.world.temperature)
    }

    fn energy_pwh(&self) -> String {
        let energy = self.state.output_demand.energy();
        format!("{}PWh", (display::twh(energy) / 1e3).round())
    }

    fn energy_twh(&self) -> String {
        let energy = self.state.output_demand.energy();
        format!("{}TWh", display::twh(energy).round())
    }

    fn avg_income_level(&self) -> usize {
        let mut total = 0.;
        for region in &self.state.world.regions {
            let income = region.income_level() as f32
                + 1.
                + region.development;
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
    fn next_point_cost(&self, kind: &ProjectType) -> usize {
        let mut discount = 0;
        if *kind == ProjectType::Research {
            if self.state.flags.contains(&Flag::HyperResearch) {
                discount += 1;
            }
            if self.state.is_ally("The Accelerationist") {
                discount += 1;
            }
        }
        0.max(consts::POINT_COST - discount) as usize
    }

    fn player_seats(&self) -> f32 {
        self.state
            .npcs
            .iter()
            .filter(|npc| npc.is_ally())
            .map(|npc| npc.seats)
            .sum()
    }

    fn buy_point(
        &mut self,
        project: &Project,
        points: &mut Points,
    ) -> bool {
        let is_research = project.kind == ProjectType::Research;
        if project.points >= consts::MAX_POINTS {
            false
        } else if is_research && points.research > 0 {
            true
        } else {
            let cost =
                self.next_point_cost(&project.kind) as isize;
            if cost <= self.state.political_capital {
                self.change_political_capital(-cost);
                match project.kind {
                    ProjectType::Research => {
                        points.research += 1
                    }
                    ProjectType::Initiative => {
                        points.initiative += 1
                    }
                    _ => (),
                }
                if is_research {
                    points.refundable_research += 1;
                }
                true
            } else {
                false
            }
        }
    }

    fn pay_points(&mut self, project: &Project) -> bool {
        // Only policies have points paid all at once,
        // rather than assigned.
        let available = self.state.political_capital;
        if project.status == Status::Inactive
            && available >= project.cost as isize
        {
            self.change_political_capital(
                -(project.cost as isize),
            );
            true
        } else {
            false
        }
    }

    fn assign_point(
        &mut self,
        project_id: usize,
        points: &mut Points,
    ) {
        // TODO so this always has the incorrect project id,
        // i guess the focused idx is stuck at the first project's id?
        logging::log!("PROJECT ID: {}", project_id);
        let (kind, cur_points, status) = {
            let project = &self.world.projects[project_id];
            (project.kind, project.points, project.status)
        };
        let points = match kind {
            ProjectType::Research => &mut points.research,
            ProjectType::Initiative => &mut points.initiative,
            ProjectType::Policy => return,
        };
        logging::log!("POINTS: {}", points);
        if *points > 0 && cur_points < consts::MAX_POINTS {
            self.set_project_points(project_id, cur_points + 1);
            if status != Status::Building {
                self.start_project(project_id);
            }
            *points -= 1;
        }
        let project = &self.world.projects[project_id];
        logging::log!(
            "NOW PROJECT HAS POINTS: {}",
            project.points
        );
    }

    fn unassign_points(
        &mut self,
        project: &Project,
        points: usize,
    ) {
        let new_points = project.points - points;
        self.set_project_points(project.id, new_points);
        if project.status == Status::Building && new_points == 0
        {
            self.stop_project(project.id);
        }
    }

    fn pass_policy(&mut self, project: &Project) {
        if project.kind == ProjectType::Policy {
            self.start_project(project.id);
        }
    }

    fn stop_policy(&mut self, project: &Project) {
        if project.kind == ProjectType::Policy {
            self.change_political_capital(
                project.cost as isize,
            );
            self.stop_project(project.id);
        }
    }

    fn upgrade_project_x(
        &mut self,
        project: &Project,
        is_free: bool,
        queued_upgrades: &mut HashMap<usize, bool>,
    ) -> bool {
        if let Some(upgrade) = project.next_upgrade() {
            let available = self.state.political_capital;
            if is_free || available >= upgrade.cost as isize {
                if !is_free {
                    self.change_political_capital(
                        -(upgrade.cost as isize),
                    );
                }
            }

            match project.kind {
                // Policies upgraded instantly
                ProjectType::Policy => {
                    self.upgrade_project(project.id);
                }
                _ => {
                    queued_upgrades.insert(project.id, true);
                }
            }
            true
        } else {
            false
        }
    }

    fn downgrade_project_x(
        &mut self,
        project: &Project,
        queued_upgrades: &mut HashMap<usize, bool>,
    ) {
        if let Some(upgrade) = project.prev_upgrade() {
            self.change_political_capital(
                upgrade.cost as isize,
            );
            if project.kind == ProjectType::Policy {
                self.downgrade_project(project.id);
            } else {
                queued_upgrades.insert(project.id, false);
            }
        }
    }
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
