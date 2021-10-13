use crate::condition::Condition;

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

#[derive(Debug, Clone)]
pub struct Probability {
    likelihood: Likelihood,
    conditions: Vec<Condition>
}
