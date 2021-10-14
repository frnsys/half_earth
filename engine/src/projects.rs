use crate::game::State;
use crate::events::{Effect, Probability};
use rand::{Rng, rngs::SmallRng};
use serde::Serialize;

#[derive(Serialize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Inactive,
    Building,
    Active,
    Stalled,
    Halted,
    Finished,
}

impl Default for Status {
    fn default() -> Self {
        Status::Inactive
    }
}

pub struct Outcome {
    pub effects: Vec<Effect>,
    pub probability: Probability,
}

#[derive(Serialize)]
#[derive(Default)]
pub struct Project {
    pub id: usize,
    pub name: &'static str,
    pub years: usize,
    pub ongoing: bool,
    pub locked: bool,
    pub progress: f32,
    pub status: Status,

    #[serde(skip_serializing)]
    pub effects: Vec<Effect>,

    #[serde(skip_serializing)]
    pub outcomes: Vec<Outcome>,
}

impl Project {
    /// Advance this project's implementation by one month.
    pub fn build(&mut self) -> bool {
        match &mut self.status {
            Status::Building => {
                self.progress += 1./12.;
                if (self.progress - self.years as f32).abs() <= 1e-4 {
                    if self.ongoing {
                        self.status = Status::Active;
                    } else {
                        self.status = Status::Finished;
                    }
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }

    /// Roll to see the outcome of this project
    pub fn roll_outcome(&self, state: &State, rng: &mut SmallRng) -> Option<(&Outcome, usize)> {
        let mut outcome = None;
        for (i, o) in self.outcomes.iter().enumerate() {
            match o.probability.eval(state, None) {
                Some(likelihood) => {
                    let prob = likelihood.p();
                    if rng.gen::<f32>() <= prob {
                        outcome = Some((o, i));
                        break;
                    }
                },
                None => ()
            }
        }
        outcome
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use crate::events::{Likelihood, Condition, Comparator, WorldVariable};

    #[test]
    fn test_build_project() {
        let mut p = Project {
            id: 0,
            name: "Test Project",
            years: 1,
            ongoing: false,
            locked: false,
            status: Status::Building,
            progress: 0.,
            effects: vec![],
            outcomes: vec![Outcome {
                effects: vec![],
                probability: Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![]
                }
            }],
        };

        for _ in 0..12 {
            p.build();
        }
        assert_eq!(p.status, Status::Finished);

        p.ongoing = true;
        p.status = Status::Building;
        p.progress = 0.;
        for _ in 0..12 {
            p.build();
        }
        assert_eq!(p.status, Status::Active);
    }

    #[test]
    fn test_project_outcomes() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let p = Project {
            id: 0,
            name: "Test Project",
            years: 1,
            ongoing: false,
            locked: false,
            status: Status::Building,
            progress: 0.,
            effects: vec![],
            outcomes: vec![Outcome {
                effects: vec![],
                probability: Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![
                        Condition::WorldVariable(
                            WorldVariable::Year,
                            Comparator::Equal, 10.)]
                }
            }, Outcome {
                effects: vec![],
                probability: Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![]
                }
            }],
        };

        let mut state = State::default();

        // Should be the second outcome
        // since the first condition isn't met
        let outcome = p.roll_outcome(&state, &mut rng);
        let (_outcome, i) = outcome.unwrap();
        assert_eq!(i, 1);

        // Now should be the first,
        state.world.year = 10;
        let outcome = p.roll_outcome(&state, &mut rng);
        let (_outcome, i) = outcome.unwrap();
        assert_eq!(i, 0);
    }
}
