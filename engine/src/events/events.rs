use crate::state::State;
use std::collections::HashSet;
use rand::{Rng, rngs::SmallRng, seq::SliceRandom};
use super::{Effect, Condition, Probability, Likelihood};
use serde_json::{json, Value};
use crate::save::{Saveable, coerce};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Default)]
pub struct EventPool {
    pub events: Vec<Event>,

    // (phase, event id, region id, countdown)
    pub queue: Vec<(Phase, usize, Option<usize>, usize)>,
    pub triggered: Vec<(Phase, usize, Option<usize>)>,
}

impl EventPool {
    pub fn new(events: Vec<Event>) -> EventPool {
        EventPool {
            events,
            queue: Vec::new(),
            triggered: Vec::new(),
        }
    }

    pub fn queue_event(&mut self, id: usize, region_id: Option<usize>, years: usize) {
        let phase = self.events[id].phase;
        self.queue.push((phase, id, region_id, years));
    }

    pub fn roll_for_phase<'a>(&'a mut self, phase: Phase, state: &State, limit: Option<usize>, rng: &mut SmallRng) -> Vec<(&'a Event, Option<usize>)> {
        // Prevent duplicate events
        let mut existing: HashSet<usize> = HashSet::new();
        for (_, ev_id, _, _) in &self.queue {
            existing.insert(*ev_id);
        }
        for (_, ev_id, _, ) in &self.triggered {
            existing.insert(*ev_id);
        }

        // Candidate event pool
        let mut valid_ids: Vec<usize> = self.events.iter().filter(|ev| ev.phase == phase && !ev.occurred && !ev.locked && !existing.contains(&ev.id)).map(|ev| ev.id).collect();
        valid_ids.shuffle(rng);

        // Tick queued countdowns
        let mut i = 0;
        while i < self.queue.len() {
            let try_trigger = {
                let (_, ev_id, _, countdown) = &mut self.queue[i];
                if self.events[*ev_id].phase == phase {
                    *countdown -= 1;
                    *countdown <= 0
                } else {
                    false
                }
            };
            if try_trigger {
                let (_, ev_id, region_id, _) = self.queue[i];
                let ev = &mut self.events[ev_id];
                if ev.roll(state, region_id, rng) {
                    self.triggered.push((ev.phase, ev_id, region_id));
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
            let ev = &mut self.events[ev_id];
            // Icon-type events are always local
            if ev.phase == Phase::Icon {
                for region in &state.world.regions {
                    if ev.roll(state, Some(region.id), rng) {
                        self.triggered.push((ev.phase, ev_id, Some(region.id)));
                    }
                }
            } else {
                if ev.regional {
                    for region in &state.world.regions {
                        if ev.roll(state, Some(region.id), rng) {
                            self.triggered.push((ev.phase, ev_id, Some(region.id)));
                        }
                    }
                } else if ev.roll(state, None, rng) {
                    self.triggered.push((ev.phase, ev_id, None));
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
                let mut ev = &mut self.events[ev_id];
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
            results.push((&self.events[ev_id], region_id));
        }
        results
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    WorldMain,
    WorldStart,
    WorldEnd,
    ReportStart,
    BreakStart,
    EndStart,
    Icon,
    PlanningStart,
    PlanningEnd,
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

#[derive(Debug, Clone)]
pub struct Event {
    pub name: &'static str,

    /// If this event requires
    /// something else to enable it.
    pub locked: bool,

    /// If this event has occurred already
    pub occurred: bool,

    /// An id linking this event
    /// to user-facing details
    /// (e.g. event text, etc).
    pub id: usize,
    pub ref_id: &'static str,

    /// This phase this event can occur in
    pub phase: Phase,

    /// If this event has any regional conditions
    pub regional: bool,

    /// The probabilities that
    /// can trigger this event.
    pub probabilities: Vec<Probability>,

    /// Effects applied when this event occurs.
    pub effects: Vec<Effect>,

    /// Associated effects/conditions
    /// for dialogue responses/branches;
    /// position in vec should correspond to
    /// the branch id.
    pub branches: Vec<(Vec<Effect>, Vec<Condition>)>,

    pub prob_modifier: f32,

    /// Icon event intensity
    pub intensity: usize,
}

impl Event {
    /// Gets the likelihood of this event occurring.
    /// If there are multiple probabilities, it returns
    /// the likelihood of the first probability that has
    /// all its conditions satisfied.
    fn eval(&self, state: &State, region_id: Option<usize>) -> Option<&Likelihood> {
        let res = self.probabilities.iter().find_map(|p| p.eval(state, region_id));
        res
    }

    /// Roll to see if the event occurs.
    fn roll(&self, state: &State, region_id: Option<usize>, rng: &mut SmallRng) -> bool {
        match self.eval(state, region_id) {
            Some(likelihood) => {
                let prob = likelihood.p();
                rng.gen::<f32>() <= (prob * self.prob_modifier)
            },
            None => false
        }
    }
}

#[derive(Serialize, Deserialize)]
struct EventSpec {
    ref_id: String,
    locked: bool,
    occurred: bool,
    prob_modifier: f32,
}

impl Saveable for EventPool {
    fn save(&self) -> Value {
        let ev_specs: Vec<EventSpec> = self.events.iter().map(|ev| {
            EventSpec {
                ref_id: ev.ref_id.to_string(),
                locked: ev.locked,
                occurred: ev.occurred,
                prob_modifier: ev.prob_modifier,
            }
        }).collect();
        json!({
            "events": ev_specs,
            "queue": self.queue,
            "triggered": self.triggered,
        })
    }

    fn load(&mut self, state: Value) {
        let ev_specs: Vec<EventSpec> = coerce(&state["events"]);
        for spec in ev_specs {
            if let Some(ev) = self.events.iter_mut().find(|ev| ev.ref_id == spec.ref_id) {
                ev.locked = spec.locked;
                ev.occurred = spec.occurred;
                ev.prob_modifier = spec.prob_modifier;
            }
        }
        self.queue = coerce(&state["queue"]);
        self.triggered = coerce(&state["triggered"]);
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use crate::regions::{Region, Income, Latitude};
    use super::super::{WorldVariable, LocalVariable, Comparator};

    fn gen_events() -> Vec<Event> {
        vec![Event {
            id: 0,
            ref_id: "test_event_a",
            name: "Test Event A",
            phase: Phase::WorldMain,
            locked: false,
            occurred: false,
            regional: false,
            prob_modifier: 1.,
            intensity: 0,
            effects: vec![],
            branches: vec![],
            probabilities: vec![Probability {
                likelihood: Likelihood::Guaranteed,
                conditions: vec![
                    Condition::WorldVariable(
                        WorldVariable::Year,
                        Comparator::Equal, 10.)
                ]
            }, Probability {
                likelihood: Likelihood::Impossible,
                conditions: vec![
                ]
            }]
        }, Event {
            id: 1,
            ref_id: "test_event_b",
            name: "Test Event B",
            phase: Phase::WorldMain,
            locked: false,
            occurred: false,
            regional: false,
            prob_modifier: 1.,
            intensity: 0,
            effects: vec![],
            branches: vec![],
            probabilities: vec![Probability {
                likelihood: Likelihood:: Guaranteed,
                conditions: vec![]
            }]
        }]
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
        let events = pool.roll_for_phase(Phase::WorldMain, &state, None, &mut rng);

        // Only event B should happen
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event B");

        // But if we set it so that event A's first condition
        // is met, it should happen
        state.world.year = 10;
        let events = pool.roll_for_phase(Phase::WorldMain, &state, None, &mut rng);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event A");
    }

    #[test]
    fn test_event_pool_local() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let events = vec![Event {
            id: 0,
            name: "Test Event A",
            ref_id: "test_event_a",
            phase: Phase::Icon,
            locked: false,
            occurred: false,
            regional: false,
            prob_modifier: 1.,
            intensity: 0,

            effects: vec![],
            branches: vec![],
            probabilities: vec![Probability {
                likelihood: Likelihood::Guaranteed,
                conditions: vec![
                    Condition::LocalVariable(
                        LocalVariable::Population,
                        Comparator::Equal, 10.)
                ]
            }, Probability {
                likelihood: Likelihood::Impossible,
                conditions: vec![
                ]
            }]
        }];
        let mut pool = EventPool {
            events,
            queue: vec![],
            triggered: vec![],
        };

        let mut state = State::default();
        state.world.regions = vec![Region {
            id: 0,
            name: "Test Region A",
            population: 0.,
            development: 0.,
            seceded: false,
            income: Income::Low,
            outlook: 0.,
            base_habitability: 0.,
            latitude: Latitude::Tropic,
            flags: vec![],
            temp_lo: 0.,
            temp_hi: 0.,
            precip_lo: 0.,
            precip_hi: 0.,
            pattern_idxs: vec![],
        }, Region {
            id: 1,
            name: "Test Region B",
            population: 0.,
            development: 0.,
            seceded: false,
            income: Income::Low,
            outlook: 0.,
            base_habitability: 0.,
            latitude: Latitude::Tropic,
            flags: vec![],
            temp_lo: 0.,
            temp_hi: 0.,
            precip_lo: 0.,
            precip_hi: 0.,
            pattern_idxs: vec![],
        }];
        let events = pool.roll_for_phase(Phase::Icon, &state, None, &mut rng);

        // No events should happen
        assert_eq!(events.len(), 0);

        // Set one region to satisfy conditions
        state.world.regions[1].population = 10.;
        let events = pool.roll_for_phase(Phase::Icon, &state, None, &mut rng);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event A");
        assert_eq!(events[0].1, Some(1));
    }

    #[test]
    fn test_event_pool_countdown() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let events = vec![Event {
            id: 0,
            ref_id: "test_event_a",
            name: "Test Event A",
            phase: Phase::WorldMain,
            prob_modifier: 1.,
            intensity: 0,

            // Note: locked so it doesn't trigger on its own
            locked: true,

            occurred: false,
            regional: false,
            effects: vec![],
            branches: vec![],
            probabilities: vec![Probability {
                likelihood: Likelihood::Guaranteed,
                conditions: vec![]
            }]
        }];
        let mut pool = EventPool {
            events,
            queue: vec![(Phase::WorldMain, 0, None, 2)],
            triggered: vec![],
        };

        let state = State::default();

        // No events should happen
        let events = pool.roll_for_phase(Phase::WorldMain, &state, None, &mut rng);
        assert_eq!(events.len(), 0);

        // Countdown finished
        let events = pool.roll_for_phase(Phase::WorldMain, &state, None, &mut rng);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_event_pool_no_dupes() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let mut pool = EventPool {
            events: vec![Event {
                id: 0,
                ref_id: "test_event_a",
                name: "Test Event A",
                phase: Phase::WorldMain,
                prob_modifier: 1.,
                intensity: 0,
                locked: false,
                occurred: false,
                regional: false,
                effects: vec![],
                branches: vec![],
                probabilities: vec![Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![]
                }]
            }],
            queue: vec![],
            triggered: vec![
                (Phase::WorldMain, 0, None),
                (Phase::WorldMain, 0, None),
                (Phase::WorldMain, 0, None),
                (Phase::WorldMain, 0, None),
                (Phase::WorldMain, 0, None),
            ],
        };

        let state = State::default();
        let events = pool.roll_for_phase(Phase::WorldMain, &state, None, &mut rng);

        // Only 1 event should happen
        assert_eq!(events.len(), 1);

        // Shouldn't happen again, even though they're pre-triggered
        for _ in 0..4 {
            let events = pool.roll_for_phase(Phase::WorldMain, &state, None, &mut rng);
            assert_eq!(events.len(), 0);
        }
    }
}

