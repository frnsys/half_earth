use crate::{climate::EmissionsData, display::DisplayEvent};
use enum_iterator::Sequence;
use enum_map::EnumMap;
use hes_engine::{
    Change,
    IconEvent,
    Id,
    Income,
    Output,
    State,
};
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

    /// Research points can either be directly given
    /// or they can be purchased with PC.
    /// This tracks the research points purchased with
    /// PC so we know they can be refunded for PC,
    /// and the cost they were paid at.
    ///
    /// We need to track their paid cost because there are
    /// different flags that affect the cost of research points
    /// (see [`StateExt::buy_point`]) so the amount to refund for
    /// an individual point may differ.
    ///
    /// Also note that `self.research - self.refundable_research`
    /// gives us the research points that can't be refunded.
    pub refundable_research: RefundablePoints,
}

// Note: this is necessary for deserializing
// the previous representation of points, which was just a `usize`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefundablePoints {
    Amount(usize),
    Values(Vec<usize>),
}
impl Default for RefundablePoints {
    fn default() -> Self {
        Self::Values(Vec::default())
    }
}
impl RefundablePoints {
    pub fn len(&self) -> usize {
        match self {
            Self::Amount(n) => *n,
            Self::Values(items) => items.len(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Amount(_) => *self = Self::Values(vec![]),
            Self::Values(items) => items.clear(),
        }
    }

    pub fn push(&mut self, cost: usize) {
        match self {
            Self::Amount(n) => *n += 1,
            Self::Values(items) => items.push(cost),
        }
    }

    pub fn refund(&mut self, n_points: usize) -> usize {
        match self {
            Self::Amount(n) => {
                // NOTE: This is just a fallback implementation,
                // which will sometimes be wrong since there may be discounts
                // that mean the refunded point cost should be lower than what's
                // given here. However this should only affect old save files;
                // new games will use the correct `Self::Values` implementation.
                let n_refunded = n_points.min(*n);
                *n = n.saturating_sub(n_refunded);
                *n * (consts::POINT_COST as usize)
            }
            Self::Values(items) => {
                let n_refundable_points = items.len();
                let split_at = n_refundable_points.saturating_sub(n_points);
                let refunded = items.split_off(split_at);
                refunded.iter().sum()
            }
        }
    }
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

    pub fn finish(&mut self) {
        *self = Tutorial::Ready;
    }
}

/// Transient UI-state that is not preserved b/w sessions.
#[derive(Default, Serialize, Deserialize)]
pub struct UIState {
    pub start_year: usize,
    pub tutorial: Tutorial,

    pub annual_region_events: BTreeMap<Id, Vec<IconEvent>>,

    #[serde(skip)]
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

    pub emissions: EmissionsData,
}
impl UIState {
    pub fn new(start_year: usize) -> Self {
        Self {
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
}
