use crate::npcs::{NPC, NPCRelation};
use crate::state::State;
use crate::kinds::{Output, OutputMap};
use crate::events::{Effect, Probability};
use rand::{Rng, rngs::SmallRng};
use serde::{Serialize, Deserialize};
use crate::save::{Saveable, coerce};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Inactive,
    Building,
    Active,
    Halted,
    Stalled,
    Finished,
}

impl Default for Status {
    fn default() -> Self {
        Status::Inactive
    }
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq)]
pub enum Group {
  Other,
  Space,
  Nuclear,
  Restoration,
  Agriculture,
  Food,
  Geoengineering,
  Population,
  Control,
  Protection,
  Electrification,
  Behavior,
  Limits,
  Energy,
  Materials,
  Buildings,
  Cities,
}

impl Default for Group {
    fn default() -> Self {
        Group::Other
    }
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Policy,
    Research,
    Initiative
}

impl Default for Type {
    fn default() -> Self {
        Type::Policy
    }
}

#[derive(Serialize, Clone)]
pub enum Cost {
    Fixed(usize),
    Dynamic(f32, Factor),
}

#[derive(Serialize, Copy, Clone)]
pub enum Factor {
    Time,
    Income,
    Output(Output),
}

impl Default for Cost {
    fn default() -> Self {
        Cost::Fixed(0)
    }
}

#[derive(Serialize, Clone)]
pub struct Outcome {
    pub effects: Vec<Effect>,

    #[serde(skip_serializing)]
    pub probability: Probability,
}

#[derive(Serialize, Default, Clone)]
pub struct Upgrade {
    pub cost: usize,
    pub effects: Vec<Effect>,
    pub active: bool,
}

#[derive(Serialize, Default, Clone)]
pub struct Project {
    pub id: usize,
    pub ref_id: &'static str,
    pub name: &'static str,
    pub kind: Type,
    pub group: Group,
    pub ongoing: bool,
    pub gradual: bool,
    pub locked: bool,

    // For policies, the cost is the political capital cost;
    // for research and initiatives, it's the base years to completion
    pub cost: usize,
    pub base_cost: Cost,
    pub cost_modifier: f32,
    pub progress: f32,
    pub points: usize,
    pub estimate: usize,
    pub status: Status,
    pub level: usize,
    pub completed_at: usize,
    pub required_majority: f32,

    #[serde(skip_serializing)]
    pub effects: Vec<Effect>,

    #[serde(skip_serializing)]
    pub outcomes: Vec<Outcome>,

    #[serde(skip_serializing)]
    pub upgrades: Vec<Upgrade>,
    pub active_outcome: Option<usize>,

    pub supporters: Vec<usize>,
    pub opposers: Vec<usize>,
}

/// How many years a project takes to complete
/// for the given amount of points.
/// Has to be at least 1
pub fn years_for_points(points: usize, cost: usize) -> f32 {
    (cost as f32/(points as f32).powf(1./2.75)).round().max(1.)
}

impl Project {
    /// Advance this project's implementation
    pub fn build(&mut self) -> bool {
        match &mut self.status {
            Status::Building => {
                self.progress += 1./years_for_points(self.points, self.cost);
                if self.progress >= 1. {
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

    pub fn set_points(&mut self, points: usize) {
        self.points = points;
        self.estimate = years_for_points(self.points, self.cost) as usize;
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
        if outcome.is_none() {
            outcome = Some((&self.outcomes[0], 0));
        }
        outcome
    }

    pub fn update_cost(&mut self, year: usize, income_level: f32, demand: &OutputMap<f32>, modifier: f32) {
        let cost = match self.base_cost {
            Cost::Fixed(c) => c,
            Cost::Dynamic(m, factor) => {
                let c = match factor {
                    // Kind of arbitrarily choose 1980 as the starting point
                    Factor::Time => m * (year - 1980) as f32,
                    Factor::Income => m * (1. + income_level),
                    Factor::Output(output) => m * demand[output]
                };
                c.round() as usize
            }
        };
        self.cost = (cost as f32 * self.cost_modifier * modifier).round() as usize;
    }

    pub fn upgrade(&mut self) -> bool {
        if self.level < self.upgrades.len() {
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn downgrade(&mut self) -> bool {
        if self.level > 0 {
            self.level -= 1;
            true
        } else {
            false
        }
    }

    pub fn active_effects(&self) -> &Vec<Effect> {
        if self.level == 0 {
            &self.effects
        } else {
            &self.upgrades[self.level - 1].effects
        }
    }

    pub fn update_required_majority(&mut self, npcs: &Vec<NPC>) {
        let opposers = self.opposers.iter().filter(|id| !npcs[**id].locked && npcs[**id].relation() != NPCRelation::Ally).count();
        let supporters = self.supporters.iter().filter(|id| !npcs[**id].locked).count();
        self.required_majority = if opposers > supporters {
            0.5
        } else {
            0.
        };
    }
}

impl Saveable for Project {
    fn save(&self) -> Value {
        json!({
            "locked": self.locked,
            "cost": self.cost,
            "cost_modifier": self.cost_modifier,
            "progress": self.progress,
            "points": self.points,
            "estimate": self.estimate,
            "status": self.status,
            "level": self.level,
            "completed_at": self.completed_at,
            "required_majority": self.required_majority,
            "active_outcome": self.active_outcome,
        })
    }

    fn load(&mut self, state: Value) {
        self.locked = coerce(&state["locked"]);
        self.cost = coerce(&state["cost"]);
        self.cost_modifier = coerce(&state["cost_modifier"]);
        self.progress = coerce(&state["progress"]);
        self.points = coerce(&state["points"]);
        self.estimate = coerce(&state["estimate"]);
        self.status = coerce(&state["status"]);
        self.level = coerce(&state["level"]);
        self.completed_at = coerce(&state["completed_at"]);
        self.required_majority = coerce(&state["required_majority"]);
        self.active_outcome = coerce(&state["active_outcome"]);
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
            ref_id: "test_project",
            name: "Test Project",
            cost: 1,
            base_cost: Cost::Fixed(1),
            cost_modifier: 1.,
            required_majority: 0.,
            level: 0,
            ongoing: false,
            gradual: false,
            locked: false,
            kind: Type::Policy,
            group: Group::Other,
            status: Status::Building,
            progress: 0.,
            estimate: 0,
            points: 1,
            completed_at: 0,
            effects: vec![],
            upgrades: vec![],
            outcomes: vec![Outcome {
                effects: vec![],
                probability: Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![]
                }
            }],
            active_outcome: None,
            opposers: vec![],
            supporters: vec![],
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
    fn test_project_estimate() {
        let mut p = Project {
            id: 0,
            ref_id: "test_project",
            name: "Test Project",
            cost: 10,
            base_cost: Cost::Fixed(10),
            cost_modifier: 1.,
            required_majority: 0.,
            level: 0,
            ongoing: false,
            gradual: false,
            locked: false,
            kind: Type::Policy,
            group: Group::Other,
            status: Status::Building,
            progress: 0.,
            estimate: 0,
            points: 0,
            completed_at: 0,
            effects: vec![],
            upgrades: vec![],
            outcomes: vec![Outcome {
                effects: vec![],
                probability: Probability {
                    likelihood: Likelihood::Guaranteed,
                    conditions: vec![]
                }
            }],
            active_outcome: None,
            opposers: vec![],
            supporters: vec![],
        };

        p.set_points(1);
        assert_eq!(p.estimate, 10);
        let prev_estimate = p.estimate;

        p.set_points(10);
        assert!(prev_estimate > p.estimate);
    }

    #[test]
    fn test_project_outcomes() {
        let mut rng: SmallRng = SeedableRng::seed_from_u64(0);
        let p = Project {
            id: 0,
            ref_id: "test_project",
            name: "Test Project",
            cost: 1,
            base_cost: Cost::Fixed(1),
            cost_modifier: 1.,
            required_majority: 0.,
            level: 0,
            ongoing: false,
            gradual: false,
            locked: false,
            kind: Type::Policy,
            group: Group::Other,
            status: Status::Building,
            progress: 0.,
            estimate: 0,
            points: 0,
            completed_at: 0,
            effects: vec![],
            upgrades: vec![],
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
            active_outcome: None,
            opposers: vec![],
            supporters: vec![],
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
