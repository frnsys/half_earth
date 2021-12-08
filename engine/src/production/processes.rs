use serde::Serialize;
use super::ProductionOrder;
use crate::kinds::{ResourceMap, ByproductMap, OutputMap, Output, Feedstock};

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum ProcessFeature {
    UsesPesticides,
    UsesSynFertilizer,
    UsesLivestock,
    IsIntermittent,
    IsNuclear,
    IsSolar,
    IsCCS,
    IsCombustion,
    IsFossil,
}

// TODO use this for labor?
#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum ProcessIntensity {
    Low,
    Medium,
    High
}

#[derive(Debug, Serialize, Clone)]
pub struct Process {
    pub id: usize,
    pub name: &'static str,
    pub mix_share: usize,
    pub limit: Option<f32>,
    pub output: Output,

    // Should start at 1.
    pub output_modifier: f32,

    pub resources: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub feedstock: (Feedstock, f32),

    pub features: Vec<ProcessFeature>,

    // If the player has unlocked this process.
    pub locked: bool,

    pub supporters: Vec<usize>,
    pub opposers: Vec<usize>,
}

impl Process {
    /// Generates production orders based on the provided demand
    /// and this sector's process mix.
    pub fn production_order(&self, demand: &OutputMap<f32>) -> ProductionOrder {
        ProductionOrder {
            process: &self,
            amount: demand[self.output] * self.mix_percent() as f32,
        }
    }

    pub fn mix_percent(&self) -> f32 {
        return self.mix_share as f32 * 0.05;
    }

    pub fn is_promoted(&self) -> bool {
        self.mix_percent() >= 0.25
    }

    pub fn is_banned(&self) -> bool {
        self.mix_share == 0
    }
}
