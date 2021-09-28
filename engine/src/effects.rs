use crate::kinds::Output;

#[derive(Debug, PartialEq)]
pub enum Flag {
    SRM,
}

#[derive(Debug, PartialEq)]
pub enum Effect {
    /// A flag that something is active
    Flag(Flag),

    /// Influence demand for output
    Demand(Output, f32),
}

impl Effect {
    // pub fn apply(&self, &mut world: World) {
    pub fn apply(&self) {
        // TODO apply the effect
    }
}
