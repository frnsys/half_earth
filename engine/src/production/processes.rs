use crate::consts;
use super::ProductionOrder;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::kinds::{ResourceMap, ByproductMap, OutputMap, Output, Feedstock};

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize)]
pub enum ProcessFeature {
    UsesPesticides,
    UsesSynFertilizer,
    UsesLivestock,
    IsIntermittent,
    CanMeltdown,
    MakesNuclearWaste,
    IsSolar,
    IsCCS,
    IsCombustion,
    IsFossil,
    IsLaborIntensive,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: usize,
    pub name: &'static str,
    pub mix_share: usize,
    pub limit: Option<f32>,
    pub output: Output,

    pub output_modifier: f32,
    pub byproduct_modifiers: ByproductMap<f32>,

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
        // Production order amount can't be more than the process's limit,
        // if there is one.
        let mut amount = demand[self.output] * self.mix_percent() as f32;
        if let Some(limit) = self.limit {
            amount = f32::min(amount, limit);
        }
        ProductionOrder {
            process: &self,
            amount,
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

    pub fn adj_resources(&self) -> ResourceMap<f32> {
        self.resources/(1. + self.output_modifier)
    }

    pub fn adj_byproducts(&self) -> ByproductMap<f32> {
        (self.byproducts * (self.byproduct_modifiers + 1.))/(1. + self.output_modifier)
    }

    pub fn adj_feedstock_amount(&self) -> f32 {
        self.feedstock.1/(1. + self.output_modifier)
    }

    pub fn extinction_rate(&self) -> f32 {
        let pressure = self.adj_byproducts().biodiversity;
        let land = self.adj_resources().land;
        (pressure/1e4 + land/consts::STARTING_RESOURCES.land) * 100.
    }
}

impl Serialize for Process {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_struct("Process", 13)?;
        seq.serialize_field("id", &self.id)?;
        seq.serialize_field("name", &self.name)?;
        seq.serialize_field("output", &self.output)?;
        seq.serialize_field("limit", &self.limit)?;
        seq.serialize_field("mix_share", &self.mix_share)?;
        seq.serialize_field("resources", &self.adj_resources())?;
        seq.serialize_field("byproducts", &self.adj_byproducts())?;
        seq.serialize_field("feedstock", &(self.feedstock.0, self.adj_feedstock_amount()))?;
        seq.serialize_field("features", &self.features)?;
        seq.serialize_field("locked", &self.locked)?;
        seq.serialize_field("supporters", &self.supporters)?;
        seq.serialize_field("opposers", &self.opposers)?;
        seq.serialize_field("extinction_rate", &self.extinction_rate())?;
        seq.end()
    }
}
