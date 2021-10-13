use crate::game::State;
use crate::kinds::{Resource,Output};
use crate::production::{ProcessFeature, Feedstock};
use crate::variables::{WorldVariable, LocalVariable, PlayerVariable};

#[derive(Debug, Clone)]
pub enum Effect {
    LocalVariable(LocalVariable, f32),
    WorldVariable(WorldVariable, f32),
    PlayerVariable(PlayerVariable, f32),

    Resource(Resource, f32),
    Demand(Output, f32),
    Output(Output, f32),
    OutputForFeature(ProcessFeature, f32),
    Feedstock(Feedstock, f32),

    AddEvent(u32),
    TriggerEvent(u32, u32),
    UnlocksProject(u32),
    UnlocksProcess(u32),

    Migration,
    RegionLeave,
}

impl Effect {
    pub fn apply(&self, state: &mut State) {
        // match self {
        // }
        // TODO apply the effect
    }
}
