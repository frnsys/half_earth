use crate::kinds::{ResourceMap, ByproductMap, Output};

#[derive(Debug, Copy, Clone)]
pub enum ProcessFeature {
    BuildsSoil,
    DegradesSoil,
    UsesPesticides,
    UsesSynFertilizer,
    UsesLivestock,
    IsIntermittent,
    IsNuclear,
    IsSolar,
    IsCCS,
}

#[derive(Debug, Copy, Clone)]
pub enum Feedstock {
    Soil,
    Oil,
    Coal,
    Uranium,
    Lithium,
    NaturalGas,
    Other
}

pub struct Process {
    pub id: usize,
    pub name: &'static str,
    pub mix_share: f32,
    pub output: Output,
    pub resources: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub feedstock: (Feedstock, f32),

    // If the player has unlocked and/or banned
    // this process.
    pub locked: bool,
    pub banned: bool,
}
