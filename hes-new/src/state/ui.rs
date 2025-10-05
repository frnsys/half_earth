use crate::{
    AUDIO,
    audio,
    display::DisplayEvent,
    views::{
        HudAction,
        MenuAction,
        game::{Interstitial, Intro, Session},
        render_hud,
        render_menu,
    },
};
use enum_iterator::Sequence;
use enum_map::EnumMap;
use hes_engine::{
    Change,
    IconEvent,
    Id,
    Income,
    Output,
    Process,
    State,
};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The state at the start of a 5-year cycle,
/// for generating comparisons for the report.
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
    pub completed_projects: Vec<Id>,
}

/// Currently staged plan changes.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanChange {
    pub points: usize,
    pub upgrades: usize,
    pub downgrades: usize,
    pub withdrawn: bool,
    pub passed: bool,
}

/// Available/unused points.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Points {
    pub research: isize,
    pub initiative: isize,
    pub refundable_research: usize,
}

/// Phase of the game.
#[derive(Default)]
pub enum Phase {
    Intro(Intro),
    Interstitial(Interstitial),
    Planning(Session),
    Events,
    Report,
    GameOver,
    GameWin,

    #[default] // TODO
    Foo,
}

#[derive(
    Debug,
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

/// Transient UI-state that is not preserved b/w sessions.
#[derive(Default, Serialize, Deserialize)]
pub struct UIState {
    #[serde(skip)]
    pub phase: Phase,

    #[serde(skip)]
    pub(crate) show_menu: bool,

    pub start_year: usize,
    pub tutorial: Tutorial,

    pub annual_region_events: BTreeMap<Id, Vec<IconEvent>>,
    pub world_events: Vec<DisplayEvent>,

    // Track state changes between planning cycles.
    #[serde(default)]
    pub change_history: Vec<(usize, Vec<Change>)>,

    #[serde(default)]
    pub process_mix_history:
        Vec<(usize, EnumMap<Output, BTreeMap<String, usize>>)>,

    #[serde(default)]
    pub session_start_state: State,

    // Track planned process mix changes
    pub process_mix_changes:
        EnumMap<Output, BTreeMap<Id, isize>>,

    // Track changes made to the plan
    // in a given session, so they can
    // be reversed/refunded
    pub plan_changes: BTreeMap<Id, PlanChange>,
    pub queued_upgrades: BTreeMap<Id, bool>,

    // Compare beginning and end
    pub cycle_start_state: CycleStart,

    pub points: Points,
    pub process_points: isize,

    /// Viewed project and process ids,
    /// so we can keep track of which ones are new
    pub viewed: Vec<Id>,
}
impl UIState {
    pub fn intro(start_year: usize, state: &mut State) -> Self {
        Self {
            phase: Phase::Intro(Intro::new(state)),
            start_year,
            ..Default::default()
        }
    }

    pub fn cycle_start_snapshot(&mut self, state: &State) {
        self.annual_region_events.clear();
        self.world_events.clear();

        self.cycle_start_state.year = state.world.year;
        self.cycle_start_state.extinction_rate =
            state.world.extinction_rate;
        self.cycle_start_state.contentedness = state.outlook();
        self.cycle_start_state.temperature =
            state.world.temperature;
        self.cycle_start_state.emissions =
            state.emissions.as_gtco2eq();
        self.cycle_start_state.region_incomes = state
            .world
            .regions
            .iter()
            .map(|r| r.income)
            .collect();
        self.cycle_start_state.parliament =
            state.npcs.iter().map(|npc| npc.seats).collect();
        self.cycle_start_state.completed_projects.clear();
    }

    pub fn has_process_mix_changes(
        &self,
        output: Output,
    ) -> bool {
        self.process_mix_changes[output]
            .iter()
            .any(|(_, change)| *change != 0)
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
    ) {
        match &mut self.phase {
            Phase::Intro(view) => {
                let next = view.render(ui, state);
                if next {
                    let view = Interstitial::new(state);
                    self.phase = Phase::Interstitial(view);
                }
            }
            Phase::Interstitial(view) => {
                let next = view.render(ui, state);
                if next {
                    if state.won() {
                        self.phase = Phase::GameWin;
                    } else if state.game_over {
                        self.phase = Phase::GameOver;
                    } else {
                        let session = Session::new(state, self);
                        self.phase = Phase::Planning(session);
                    }
                }
            }
            Phase::Planning(session) => {
                if let Some(action) = render_hud(ui, state) {
                    match action {
                        HudAction::OpenMenu => {
                            self.show_menu = true;
                        }
                    }
                }
                if self.show_menu {
                    if let Some(action) = render_menu(ui, state)
                    {
                        match action {
                            MenuAction::CloseMenu => {
                                self.show_menu = false
                            }
                            MenuAction::RestartGame => todo!(),
                            MenuAction::ShowCredits => todo!(),
                            MenuAction::ToggleSound => todo!(),
                            MenuAction::HideHelp => todo!(),
                        }
                    }
                } else {
                    session.render(
                        ui,
                        state,
                        &mut self.tutorial,
                        &self.annual_region_events,
                        &self.process_mix_changes,
                        &self.viewed,
                        &self.points,
                        &self.plan_changes,
                    );
                }
            }
            Phase::Events => todo!(),
            Phase::Report => todo!(),
            Phase::GameOver => todo!(),
            Phase::GameWin => todo!(),
            Phase::Foo => unreachable!(),
        }
    }
}

pub fn format_year_log(
    year: usize,
    changes: &[Change],
    mixes: &EnumMap<Output, BTreeMap<String, usize>>,
) -> String {
    [
        format!("\n[{year}]"),
        changes
            .iter()
            .map(|diff| diff.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        "Production Mix:".into(),
        mixes
            .iter()
            .map(|(output, mix)| {
                let mut parts = vec![format!("  [{output}]")];
                for (name, mix) in mix {
                    parts.push(format!("    {name}:{mix}"));
                }
                parts.join("\n")
            })
            .collect::<Vec<_>>()
            .join("\n"),
    ]
    .join("\n")
}
