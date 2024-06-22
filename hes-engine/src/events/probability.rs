use serde::{Deserialize, Serialize};

use super::Condition;
use crate::state::State;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Likelihood {
    Impossible,
    Improbable,
    Rare,
    Unlikely,
    Random,
    Likely,
    Guaranteed,
}

impl Likelihood {
    pub fn p(&self) -> f32 {
        match self {
            Likelihood::Impossible => 0.,
            Likelihood::Improbable => 0.0005,
            Likelihood::Rare => 0.005,
            Likelihood::Unlikely => 0.05,
            Likelihood::Random => 0.25,
            Likelihood::Likely => 0.5,
            Likelihood::Guaranteed => 1.,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Probability {
    pub likelihood: Likelihood,
    pub conditions: Vec<Condition>,
}

impl Probability {
    pub fn eval(&self, state: &State, region_id: Option<usize>) -> Option<&Likelihood> {
        if self.conditions.iter().all(|c| c.eval(state, region_id)) {
            Some(&self.likelihood)
        } else {
            None
        }
    }
}
