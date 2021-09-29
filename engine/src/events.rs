use crate::game::State;
use crate::effects::Effect;
use rand::{Rng, rngs::StdRng, seq::SliceRandom};

// TODO arcs
// TODO event severity/variations? just as separate events?

const MAX_EVENTS_PER_TURN: usize = 5;

#[derive(Debug, PartialEq)]
enum Status {
    Random,
    Queued(usize),
    Triggered
}

#[derive(Debug, Default)]
pub struct EventPool {
    events: Vec<(Event, Status)>,
}

impl EventPool {
    pub fn add_event(&mut self, event: Event, status: Status) {
        self.events.push((event, status));
    }

    pub fn roll(&mut self, state: &State, rng: &mut StdRng) -> Vec<&Event> {
        let mut happening: Vec<&Event> = Vec::with_capacity(MAX_EVENTS_PER_TURN);

        // Clean up expired/stale events
        self.events.retain(|(ev, status)| !(*status == Status::Queued(0) || (*status == Status::Triggered && !ev.repeats)));

        self.events.shuffle(rng);
        for (ev, status) in &mut self.events {
            match status {
                Status::Queued(i) => {
                    *i -= 1;
                    if *i == 0 {
                        happening.push(ev);
                    }
                },
                Status::Random|Status::Triggered => {
                    // Reset repeating events
                    if ev.repeats {
                        *status = Status::Random;
                    }
                    if happening.len() < MAX_EVENTS_PER_TURN {
                        let prob = (ev.prob)(state);
                        if rng.gen::<f32>() <= prob {
                            *status = Status::Triggered;
                            happening.push(ev);
                        }
                    }
                },
            }
        }

        happening
    }
}


#[derive(Debug, Clone)]
pub struct Event {
    /// An id linking this event
    /// to user-facing details
    /// (e.g. event text, etc).
    id: usize,

    /// If this event can repeat or
    /// if it can only happens once.
    repeats: bool,

    /// The id of the story arc this
    /// event is a part of, if any.
    arc: Option<usize>,

    /// A function that takes the current
    /// game state and returns the probability
    /// of this event's occurrence.
    prob: &'static dyn Probability,

    /// Choices the player chooses from.
    pub choices: Vec<Choice>,

    /// Effects applied when this event occurs.
    pub effects: Vec<Effect>
}

#[derive(Debug, Clone)]
pub struct Choice {
    id: usize,
    effects: Vec<Effect>,

    /// A function that takes the current
    /// game state and returns whether or not
    /// this choice is available.
    condition: &'static dyn Condition
}


// Small hack so we can derive Debug
pub trait Condition: Fn(&State) -> bool {}
impl<F> Condition for F where F: Fn(&State) -> bool {}
impl std::fmt::Debug for dyn Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Condition")
    }
}

pub trait Probability: Fn(&State) -> f32 {}
impl<F> Probability for F where F: Fn(&State) -> f32 {}
impl std::fmt::Debug for dyn Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Probability")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_event_pool() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(0);
        let mut pool = EventPool::default();

        pool.add_event(Event {
            id: 0,
            arc: None,
            repeats: false,
            choices: vec![],
            effects: vec![],
            prob: &|_state: &State| { 1.0 }
        }, Status::Random);

        pool.add_event(Event {
            id: 1,
            arc: None,
            repeats: true,
            choices: vec![],
            effects: vec![],
            prob: &|_state: &State| { 1.0 }
        }, Status::Random);

        pool.add_event(Event {
            id: 2,
            arc: None,
            repeats: false,
            choices: vec![],
            effects: vec![],
            prob: &|_state: &State| { 0.0 }
        }, Status::Random);

        pool.add_event(Event {
            id: 3,
            arc: None,
            repeats: false,
            choices: vec![],
            effects: vec![],
            prob: &|_state: &State| { 1.0 }
        }, Status::Queued(2));

        pool.add_event(Event {
            id: 4,
            arc: None,
            repeats: false,
            choices: vec![],
            effects: vec![],
            prob: &|_state: &State| { 1.0 }
        }, Status::Queued(1));


        let state = State::default();
        let events = pool.roll(&state, &mut rng);

        // Queued event should have triggered
        assert!(events.iter().any(|ev| ev.id == 4));

        // Random events should have triggered
        assert!(events.iter().any(|ev| ev.id == 0));
        assert!(events.iter().any(|ev| ev.id == 1));

        let events = pool.roll(&state, &mut rng);

        // Should not have triggered
        assert!(events.iter().all(|ev| ev.id != 4));
        assert!(events.iter().all(|ev| ev.id != 0));

        // Should have triggered
        assert!(events.iter().any(|ev| ev.id == 3));
        assert!(events.iter().any(|ev| ev.id == 1));
    }
}
