use super::Condition;
use crate::game::State;

#[derive(Debug, Copy, Clone)]
pub enum Likelihood {
  Impossible,
  Improbable,
  Rare,
  Unlikely,
  Random,
  Likely,
  Guaranteed
}

impl Likelihood {
    pub fn p(&self) -> f32 {
        match self {
            Likelihood::Impossible => 0.,
            Likelihood::Improbable => 0.00005,
            Likelihood::Rare       => 0.0005,
            Likelihood::Unlikely   => 0.005,
            Likelihood::Random     => 0.05,
            Likelihood::Likely     => 0.15,
            Likelihood::Guaranteed => 1.,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Probability {
    pub likelihood: Likelihood,
    pub conditions: Vec<Condition>
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
