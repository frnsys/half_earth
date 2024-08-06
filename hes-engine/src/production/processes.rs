use super::ProductionOrder;
use crate::{
    flavor::ProcessFlavor,
    kinds::{
        ByproductMap,
        Feedstock,
        FeedstockMap,
        Output,
        OutputMap,
        ResourceMap,
    },
    npcs::RELATIONSHIP_CHANGE_AMOUNT,
    Collection,
    HasId,
    Id,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    IntoStaticStr,
    EnumIter,
    EnumString,
    Display,
)]
pub enum ProcessFeature {
    UsesPesticides,
    UsesSynFertilizer,
    UsesLivestock,
    UsesOil,
    IsIntermittent,
    CanMeltdown,
    MakesNuclearWaste,
    IsSolar,
    IsCCS,
    IsCombustion,
    IsFossil,
    IsLaborIntensive,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default,
)]
pub struct Process {
    pub id: Id,
    pub name: String,
    pub mix_share: usize,
    pub limit: Option<f32>,
    pub output: Output,

    pub output_modifier: f32,
    pub byproduct_modifiers: ByproductMap,

    pub resources: ResourceMap,
    pub byproducts: ByproductMap,
    pub feedstock: (Feedstock, f32),

    pub features: Vec<ProcessFeature>,

    // If the player has unlocked this process.
    pub locked: bool,

    pub supporters: Vec<Id>,
    pub opposers: Vec<Id>,
    pub flavor: ProcessFlavor,
    pub notes: String,
}

impl Display for Process {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl HasId for Process {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl Process {
    pub fn new() -> Process {
        Process {
            id: Id::new_v4(),
            name: "New Process".into(),
            ..Default::default()
        }
    }

    /// Generates production orders based on the provided demand
    /// and this sector's process mix.
    pub fn production_order(
        &self,
        demand: &OutputMap,
    ) -> ProductionOrder {
        // Production order amount can't be more than the process's limit,
        // if there is one.
        let mut amount =
            demand[self.output] * self.mix_percent() as f32;
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

    pub fn adj_resources(&self) -> ResourceMap {
        self.resources / (1. + self.output_modifier)
    }

    pub fn adj_byproducts(&self) -> ByproductMap {
        (self.byproducts * (self.byproduct_modifiers + 1.))
            / (1. + self.output_modifier)
    }

    pub fn adj_feedstock_amount(&self) -> f32 {
        self.feedstock.1 / (1. + self.output_modifier)
    }

    pub fn extinction_rate(&self, starting_land: f32) -> f32 {
        let pressure = self.adj_byproducts().biodiversity;
        let land = self.adj_resources().land;
        (pressure / 3e16 + land / starting_land) * 100.
    }

    pub fn max_share(
        &self,
        output_demand: &OutputMap,
        feedstocks: &FeedstockMap,
    ) -> usize {
        let mut max_share = 1.;
        let demand = output_demand[self.output];

        // Hard-coded limit
        if let Some(limit) = self.limit {
            max_share = (limit / demand).min(1.);
        }

        // Limit based on feedstock supply
        let (kind, per_output) = self.feedstock;
        match kind {
            Feedstock::Soil | Feedstock::Other => {}
            _ => {
                let limit = feedstocks[kind] / per_output;
                let supply_max_share = (limit / demand).min(1.);
                max_share = max_share.min(supply_max_share);
            }
        }

        ((max_share * 100.) / 5.).floor() as usize
    }

    /// Changes this process's mix share by the specified amount.
    pub fn change_mix_share(
        &mut self,
        change: isize,
    ) -> ProcessChanges {
        let was_banned = self.is_banned();
        let was_promoted = self.is_promoted();
        if change < 0 {
            self.mix_share = self
                .mix_share
                .saturating_sub(change.abs() as usize);
        } else {
            self.mix_share += change as usize;
        }

        let (support_change, oppose_change) =
            if !was_banned && self.is_banned() {
                // Ban
                (-1., 1.)
            } else if was_banned && !self.is_banned() {
                // Unban
                (1., -1.)
            } else if was_promoted && !self.is_promoted() {
                // Unpromote
                (-1., 1.)
            } else if !was_promoted && self.is_promoted() {
                // Promote
                (1., -1.)
            } else {
                (0., 0.)
            };

        let mut changes = ProcessChanges::default();
        for npc_id in &self.supporters {
            changes.relationships.push((
                *npc_id,
                support_change * RELATIONSHIP_CHANGE_AMOUNT,
            ));
        }
        for npc_id in &self.opposers {
            changes.relationships.push((
                *npc_id,
                oppose_change * RELATIONSHIP_CHANGE_AMOUNT,
            ));
        }
        changes
    }
}

#[derive(Default)]
pub struct ProcessChanges {
    pub relationships: Vec<(Id, f32)>,
}

impl Collection<Process> {
    pub fn unlocked(&self) -> impl Iterator<Item = &Process> {
        self.iter().filter(|p| !p.locked)
    }

    pub fn orders(
        &self,
        demand: &OutputMap,
    ) -> Vec<ProductionOrder> {
        self.iter()
            .map(|p| p.production_order(&demand))
            .collect()
    }

    pub fn max_shares(
        &self,
        output_demand: &OutputMap,
        feedstocks: &FeedstockMap,
    ) -> Vec<usize> {
        self.iter()
            .map(|p| p.max_share(output_demand, feedstocks))
            .collect::<Vec<_>>()
    }

    pub fn over_limit(
        &self,
        output_demand: OutputMap,
        feedstocks: FeedstockMap,
    ) -> impl Iterator<Item = &Process> {
        self.iter().filter(move |p| {
            let max_share =
                p.max_share(&output_demand, &feedstocks);
            p.mix_share > 0 && p.mix_share > max_share
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::kinds::{Feedstock, Output};

    #[test]
    fn test_output_limit() {
        let mut p = Process {
            id: Id::new_v4(),
            name: "Test Process A".into(),
            mix_share: 20, // Full mix share
            output: Output::Fuel,
            resources: resources!(water: 1.),
            feedstock: (Feedstock::Oil, 1.),
            ..Default::default()
        };

        let demand = outputs!(
            fuel: 1000.,
            electricity: 0.,
            animal_calories: 0.,
            plant_calories: 0.
        );
        let order = p.production_order(&demand);
        assert_eq!(order.amount, 1000.);

        // Fuel demand is more than this process's limit
        p.limit = Some(100.);

        let order = p.production_order(&demand);
        assert_eq!(order.amount, 100.);
    }
}
