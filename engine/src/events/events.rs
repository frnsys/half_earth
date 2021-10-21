use crate::game::State;
use std::collections::HashSet;
use rand::{Rng, rngs::SmallRng, seq::SliceRandom};
use super::{Effect, Condition, Probability, Likelihood};

#[derive(Debug, Default)]
pub struct EventPool {
    pub events: Vec<Event>,

    // (event id, region id, countdown)
    pub queue: Vec<(Type, usize, Option<usize>, usize)>,
    pub triggered: Vec<(Type, usize, Option<usize>)>,
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
        let kind = self.events[id].kind;
        self.queue.push((kind, id, region_id, years));
    }

    pub fn roll_for_kind<'a>(&'a mut self, kind: Type, state: &State, limit: Option<usize>, rng: &mut SmallRng) -> Vec<(&'a Event, Option<usize>)> {
        // Prevent duplicate events
        let mut existing: HashSet<usize> = HashSet::new();
        for (_, ev_id, _, _) in &self.queue {
            existing.insert(*ev_id);
        }
        for (_, ev_id, _, ) in &self.triggered {
            existing.insert(*ev_id);
        }

        // Candidate event pool
        let mut valid_ids: Vec<usize> = self.events.iter().filter(|ev| ev.kind == kind && !ev.locked && !existing.contains(&ev.id)).map(|ev| ev.id).collect();
        valid_ids.shuffle(rng);

        // Tick queued countdowns
        let mut i = 0;
        while i < self.queue.len() {
            let try_trigger = {
                let (_, ev_id, _, countdown) = &mut self.queue[i];
                if self.events[*ev_id].kind == kind {
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
                    // All events except
                    // for Icon events don't repeat
                    if ev.kind != Type::Icon {
                        ev.locked = true;
                    }
                    self.triggered.push((ev.kind, ev_id, region_id));
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
            // Only Icon-type events are local
            if ev.kind == Type::Icon {
                for region in &state.world.regions {
                    if ev.roll(state, Some(region.id), rng) {
                        // All events except
                        // for Icon events don't repeat
                        if ev.kind != Type::Icon {
                            ev.locked = true;
                        }
                        self.triggered.push((ev.kind, ev_id, Some(region.id)));
                    }
                }
            } else {
                if ev.roll(state, None, rng) {
                    // All events except
                    // for Icon events don't repeat
                    if ev.kind != Type::Icon {
                        ev.locked = true;
                    }
                    self.triggered.push((ev.kind, ev_id, None));
                }
            }
        }

        // Get the first MAX_EVENTS_PER_TURN triggered events
        let mut happening = Vec::new();
        self.triggered.shuffle(rng);
        let n = self.triggered.len();

        let mut i = 0;
        while i < self.triggered.len() {
            let (k, ev_id, region_id) = self.triggered[i];
            if k == kind {
                happening.push((&self.events[ev_id], region_id));
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

        happening
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    World,
    Planning,
    Breaks,
    Icon
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: &'static str,

    /// If this event requires
    /// something else to enable it.
    pub locked: bool,

    /// An id linking this event
    /// to user-facing details
    /// (e.g. event text, etc).
    pub id: usize,

    /// This event's type
    pub kind: Type,

    /// The probabilities that
    /// can trigger this event.
    pub probabilities: Vec<Probability>,

    /// Choices the player chooses from.
    pub choices: Vec<Choice>,

    /// Effects applied when this event occurs.
    pub effects: Vec<Effect>
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
                rng.gen::<f32>() <= prob
            },
            None => false
        }
    }

    pub fn set_choice(&self, choice_id: usize) -> (&Vec<Effect>, &ChoiceType) {
        let choice = &self.choices[choice_id];
        (&choice.effects, &choice.kind)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ChoiceType {
    HES,
    FALC,
    Malthusian,
    None
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub kind: ChoiceType,
    pub effects: Vec<Effect>,

    /// A function that takes the current
    /// game state and returns whether or not
    /// this choice is available.
    pub conditions: Vec<Condition>
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use crate::regions::{Region, Income};
    use super::super::{WorldVariable, LocalVariable, Comparator};

    fn gen_events() -> Vec<Event> {
        vec![Event {
            id: 0,
            name: "Test Event A",
            kind: Type::World,
            locked: false,
            choices: vec![],
            effects: vec![],
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
            name: "Test Event B",
            kind: Type::World,
            locked: false,
            choices: vec![],
            effects: vec![],
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
        let events = pool.roll_for_kind(Type::World, &state, None, &mut rng);

        // Only event B should happen
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event B");

        // But if we set it so that event A's first condition
        // is met, it should happen
        state.world.year = 10;
        let events = pool.roll_for_kind(Type::World, &state, None, &mut rng);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event A");
    }

    #[test]
    fn test_event_pool_local() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let events = vec![Event {
            id: 0,
            name: "Test Event A",
            kind: Type::Icon,
            locked: false,

            choices: vec![],
            effects: vec![],
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
            seceded: false,
            income: Income::Low,
            outlook: 0.,
            base_habitability: 0.,
            base_contentedness: 0.,
        }, Region {
            id: 1,
            name: "Test Region B",
            population: 0.,
            seceded: false,
            income: Income::Low,
            outlook: 0.,
            base_habitability: 0.,
            base_contentedness: 0.,
        }];
        let events = pool.roll_for_kind(Type::Icon, &state, None, &mut rng);

        // No events should happen
        assert_eq!(events.len(), 0);

        // Set one region to satisfy conditions
        state.world.regions[1].population = 10.;
        let events = pool.roll_for_kind(Type::Icon, &state, None, &mut rng);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0.name, "Test Event A");
        assert_eq!(events[0].1, Some(1));
    }

    #[test]
    fn test_event_pool_countdown() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let events = vec![Event {
            id: 0,
            name: "Test Event A",
            kind: Type::World,

            // Note: locked so it doesn't trigger on its own
            locked: true,

            choices: vec![],
            effects: vec![],
            probabilities: vec![Probability {
                likelihood: Likelihood::Guaranteed,
                conditions: vec![]
            }]
        }];
        let mut pool = EventPool {
            events,
            queue: vec![(Type::World, 0, None, 2)],
            triggered: vec![],
        };

        let state = State::default();

        // No events should happen
        let events = pool.roll_for_kind(Type::World, &state, None, &mut rng);
        assert_eq!(events.len(), 0);

        // Countdown finished
        let events = pool.roll_for_kind(Type::World, &state, None, &mut rng);
        assert_eq!(events.len(), 1);
    }
}
