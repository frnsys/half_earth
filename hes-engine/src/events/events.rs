use super::{Effect, Likelihood, Probability};
use crate::{
    flavor::EventFlavor,
    state::State,
    Collection,
    HasId,
    Id,
};
use rand::{rngs::SmallRng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Display};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq,
)]
pub struct EventPool {
    pub events: Collection<Event>,

    // (phase, event id, region id, countdown)
    pub queue: Vec<(Phase, Id, Option<Id>, usize)>,
    pub triggered: Vec<(Phase, Id, Option<Id>)>,
}

impl EventPool {
    pub fn new(events: Collection<Event>) -> EventPool {
        EventPool {
            events,
            queue: Vec::new(),
            triggered: Vec::new(),
        }
    }

    pub fn queue_event(
        &mut self,
        id: Id,
        region_id: Option<Id>,
        years: usize,
    ) {
        let phase = self.events[&id].phase;
        self.queue.push((phase, id, region_id, years));
    }

    pub fn roll_for_phase(
        &mut self,
        phase: Phase,
        state: &State,
        limit: Option<usize>,
        rng: &mut SmallRng,
    ) -> Vec<(Event, Option<Id>)> {
        // Prevent duplicate events
        let mut existing: HashSet<&Id> = HashSet::new();
        for (_, ev_id, _, _) in &self.queue {
            existing.insert(ev_id);
        }
        for (_, ev_id, _) in &self.triggered {
            existing.insert(ev_id);
        }

        // Candidate event pool
        let mut valid_ids: Vec<Id> = self
            .events
            .iter()
            .filter(|ev| {
                ev.phase == phase
                    && !ev.occurred
                    && !ev.locked
                    && !existing.contains(&ev.id)
            })
            .map(|ev| ev.id)
            .collect();
        valid_ids.shuffle(rng);

        // Tick queued countdowns
        let mut i = 0;
        while i < self.queue.len() {
            let try_trigger = {
                let (_, ev_id, _, countdown) =
                    &mut self.queue[i];
                if self.events[&*ev_id].phase == phase {
                    *countdown -= 1;
                    *countdown <= 0
                } else {
                    false
                }
            };
            if try_trigger {
                let (_, ev_id, region_id, _) = self.queue[i];
                let ev = &mut self.events[&ev_id];
                if ev.roll(state, region_id, rng) {
                    self.triggered
                        .push((ev.phase, ev_id, region_id));
                }
                self.queue.remove(i);
            } else {
                i += 1;
            }
        }

        // Roll for additional events
        // These events start with countdown 0;
        // i.e. we immediately trigger them if possible.
        for ev_id in valid_ids {
            let ev = &mut self.events[&ev_id];
            // Icon-type events are always local
            if ev.phase == Phase::Icon {
                for region in state.world.regions.iter() {
                    if ev.roll(state, Some(region.id), rng) {
                        self.triggered.push((
                            ev.phase,
                            ev_id,
                            Some(region.id),
                        ));
                    }
                }
            } else {
                if ev.is_regional() {
                    for region in state.world.regions.iter() {
                        if ev.roll(state, Some(region.id), rng)
                        {
                            self.triggered.push((
                                ev.phase,
                                ev_id,
                                Some(region.id),
                            ));
                        }
                    }
                } else if ev.roll(state, None, rng) {
                    self.triggered
                        .push((ev.phase, ev_id, None));
                }
            }
        }

        // Get the first MAX_EVENTS_PER_TURN triggered events
        let mut happening = Vec::new();
        self.triggered.shuffle(rng);

        let mut i = 0;
        while i < self.triggered.len() {
            let (p, ev_id, region_id) = self.triggered[i];
            if p == phase {
                let ev = &mut self.events[&ev_id];
                if !ev.occurred {
                    happening.push((ev_id, region_id));
                    // All events except
                    // for Icon events don't repeat
                    if ev.phase != Phase::Icon {
                        ev.occurred = true;
                    }
                }
                self.triggered.remove(i);
                if let Some(n) = limit {
                    if happening.len() >= n {
                        break;
                    }
                }
            } else {
                i += 1;
            }
        }

        let mut results = vec![];
        for (ev_id, region_id) in happening {
            results
                .push((self.events[&ev_id].clone(), region_id));
        }
        results
    }
}

/// The game phase in which an event can be rolled.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Default,
)]
pub enum Phase {
    #[default]
    WorldMain,
    WorldStart,
    ReportStart,
    BreakStart,
    EndStart,
    Icon,
    PlanningStart,
    PlanningPlan,
    PlanningAdd,
    PlanningResearch,
    PlanningInitiatives,
    PlanningPolicies,
    PlanningProcesses,
    PlanningParliament,
    PlanningRegions,
    PlanningDashboard,
    PlanningPlanChange,
    InterstitialStart,
    InterstitialWin,
    CutsceneIntro,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: Id,
    pub name: String,

    /// If this event requires
    /// something else to enable it.
    pub locked: bool,

    /// If this event has occurred already
    pub occurred: bool,

    /// This phase this event can occur in
    pub phase: Phase,

    /// The probabilities that
    /// can trigger this event.
    pub probabilities: Vec<Probability>,

    /// Effects applied when this event occurs.
    pub effects: Vec<Effect>,

    pub prob_modifier: f32,

    /// Icon event intensity
    pub intensity: usize,

    pub flavor: EventFlavor,
    pub notes: String,
}
impl Default for Event {
    fn default() -> Self {
        Self {
            id: Id::new_v4(),
            name: "Default Event".into(),
            locked: false,
            occurred: false,
            phase: Phase::WorldMain,
            prob_modifier: 1.,
            intensity: 0,
            effects: vec![],
            probabilities: vec![Probability {
                likelihood: Likelihood::Guaranteed,
                conditions: vec![],
            }],
            flavor: EventFlavor::default(),
            notes: "".into(),
        }
    }
}

impl Display for Event {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl HasId for Event {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl Event {
    pub fn new() -> Event {
        Event {
            id: Id::new_v4(),
            name: "New Event".into(),
            ..Default::default()
        }
    }

    /// If this event has any regional conditions.
    pub fn is_regional(&self) -> bool {
        self.probabilities.iter().any(|prob| prob.is_regional())
    }

    /// Gets the likelihood of this event occurring.
    /// If there are multiple probabilities, it returns
    /// the likelihood of the first probability that has
    /// all its conditions satisfied.
    fn eval(
        &self,
        state: &State,
        region_id: Option<Id>,
    ) -> Option<&Likelihood> {
        let res = self
            .probabilities
            .iter()
            .find_map(|p| p.eval(state, region_id));
        res
    }

    /// Roll to see if the event occurs.
    fn roll(
        &self,
        state: &State,
        region_id: Option<Id>,
        rng: &mut SmallRng,
    ) -> bool {
        match self.eval(state, region_id) {
            Some(likelihood) => {
                let prob = likelihood.p();
                rng.gen::<f32>() <= (prob * self.prob_modifier)
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        super::{Comparator, LocalVariable, WorldVariable},
        *,
    };
    use crate::{events::Condition, regions::Region};
    use rand::SeedableRng;

    fn gen_events() -> Collection<Event> {
        vec![
            Event {
                id: Id::new_v4(),
                name: "Test Event A".into(),
                phase: Phase::WorldMain,
                probabilities: vec![
                    Probability {
                        likelihood: Likelihood::Guaranteed,
                        conditions: vec![
                            Condition::WorldVariable(
                                WorldVariable::Year,
                                Comparator::Equal,
                                10.,
                            ),
                        ],
                    },
                    Probability {
                        likelihood: Likelihood::Impossible,
                        conditions: vec![],
                    },
                ],
                ..Default::default()
            },
            Event {
                id: Id::new_v4(),
                name: "Test Event B".into(),
                phase: Phase::WorldMain,
                probabilities: vec![Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![],
                }],
                ..Default::default()
            },
        ]
        .into()
    }

    #[test]
    fn test_event_pool() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let events = gen_events();
        let mut pool = EventPool {
            events,
            queue: vec![],
            triggered: vec![],
        };

        let mut state = State::default();
        let events = pool.roll_for_phase(
            Phase::WorldMain,
            &state,
            None,
            &mut rng,
        );

        // Only event B should happen
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event B");

        // But if we set it so that event A's first condition
        // is met, it should happen
        state.world.year = 10;
        let events = pool.roll_for_phase(
            Phase::WorldMain,
            &state,
            None,
            &mut rng,
        );
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event A");
    }

    #[test]
    fn test_event_pool_local() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let events = vec![Event {
            id: Id::new_v4(),
            name: "Test Event A".into(),
            phase: Phase::Icon,
            probabilities: vec![
                Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![Condition::LocalVariable(
                        LocalVariable::Population,
                        Comparator::Equal,
                        10.,
                    )],
                },
                Probability {
                    likelihood: Likelihood::Impossible,
                    conditions: vec![],
                },
            ],
            ..Default::default()
        }]
        .into();
        let mut pool = EventPool {
            events,
            queue: vec![],
            triggered: vec![],
        };

        let mut state = State::default();
        state.world.regions = vec![
            Region {
                id: Id::new_v4(),
                name: "Test Region A".into(),
                ..Default::default()
            },
            Region {
                id: Id::new_v4(),
                name: "Test Region B".into(),
                ..Default::default()
            },
        ]
        .into();
        let events = pool.roll_for_phase(
            Phase::Icon,
            &state,
            None,
            &mut rng,
        );

        // No events should happen
        assert_eq!(events.len(), 0);

        // Set one region to satisfy conditions
        let region = state.world.regions.by_idx_mut(1);
        region.population = 10.;
        let id = region.id;
        let events = pool.roll_for_phase(
            Phase::Icon,
            &state,
            None,
            &mut rng,
        );
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event A");
        assert_eq!(events[0].1, Some(id));
    }

    #[test]
    fn test_event_pool_countdown() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let id = Id::new_v4();
        let events = vec![Event {
            id,
            name: "Test Event A".into(),

            // Note: locked so it doesn't trigger on its own
            locked: true,
            ..Default::default()
        }]
        .into();
        let mut pool = EventPool {
            events,
            queue: vec![(Phase::WorldMain, id, None, 2)],
            triggered: vec![],
        };

        let state = State::default();

        // No events should happen
        let events = pool.roll_for_phase(
            Phase::WorldMain,
            &state,
            None,
            &mut rng,
        );
        assert_eq!(events.len(), 0);

        // Countdown finished
        let events = pool.roll_for_phase(
            Phase::WorldMain,
            &state,
            None,
            &mut rng,
        );
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_event_pool_no_dupes() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let id = Id::new_v4();
        let mut pool = EventPool {
            events: vec![Event {
                id,
                name: "Test Event A".into(),
                phase: Phase::WorldMain,
                ..Default::default()
            }]
            .into(),
            queue: vec![],
            triggered: vec![
                (Phase::WorldMain, id, None),
                (Phase::WorldMain, id, None),
                (Phase::WorldMain, id, None),
                (Phase::WorldMain, id, None),
                (Phase::WorldMain, id, None),
            ],
        };

        let state = State::default();
        let events = pool.roll_for_phase(
            Phase::WorldMain,
            &state,
            None,
            &mut rng,
        );

        // Only 1 event should happen
        assert_eq!(events.len(), 1);

        // Shouldn't happen again, even though they're pre-triggered
        for _ in 0..4 {
            let events = pool.roll_for_phase(
                Phase::WorldMain,
                &state,
                None,
                &mut rng,
            );
            assert_eq!(events.len(), 0);
        }
    }
}
