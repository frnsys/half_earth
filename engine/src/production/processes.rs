use serde::Serialize;
use super::{ProductionOrder, planner};
use crate::kinds::{ResourceMap, ByproductMap, FeedstockMap, OutputMap, Output, Feedstock};

const MIX_CHANGE_SPEED: f32 = 0.1;

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct Process {
    pub id: usize,
    pub name: &'static str,
    pub mix_share: f32,
    pub output: Output,

    // Should start at 1.
    pub output_modifier: f32,

    pub resources: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub feedstock: (Feedstock, f32),

    #[serde(skip_serializing)]
    pub features: Vec<ProcessFeature>,

    // If the player has unlocked and/or banned
    // this process.
    pub locked: bool,
    pub banned: bool,
}

impl Process {
    /// Generates production orders based on the provided demand
    /// and this sector's process mix.
    pub fn production_order(&self, demand: &OutputMap<f32>) -> ProductionOrder {
        ProductionOrder {
            process: &self,
            amount: demand[self.output] * self.mix_share,
        }
    }
}

/// Update this process mixes to better match
/// the demand and resource weights (by scarcity).
/// This mix adjustment happens at a speed of `sector.momentum`.
pub fn update_mixes(
    processes: &mut [Process],
    demand: &OutputMap<f32>,
    resource_weights: &ResourceMap<f32>,
    feedstock_weights: &FeedstockMap<f32>) {
    let target_mix = planner::calculate_mix(&processes, &demand, &resource_weights, &feedstock_weights);
    for (process, target) in processes.iter_mut().zip(target_mix) {
        if !process.locked {
            if process.mix_share < target {
                process.mix_share += MIX_CHANGE_SPEED;
            } else if process.mix_share > target {
                process.mix_share -= MIX_CHANGE_SPEED;
            }
            process.mix_share = f32::max(process.mix_share, 0.);
        }
    }

    // Renormalize
    let mut mix_totals: OutputMap<f32> = outputs!();
    for p in processes.iter() {
        mix_totals[p.output] += p.mix_share;
    }
    for p in processes {
        p.mix_share /= mix_totals[p.output];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    fn gen_processes() -> Vec<Process> {
        vec![Process {
            id: 0,
            name: "Test Process A",
            mix_share: 0.5,
            output: Output::Fuel,
            output_modifier: 1.,
            resources: resources!(water: 10.),
            byproducts: byproducts!(),
            feedstock: (Feedstock::Oil, 1.),
            features: vec![],
            locked: false,
            banned: false,
        }, Process {
            id: 1,
            name: "Test Process B",
            mix_share: 0.5,
            output: Output::Fuel,
            output_modifier: 1.,
            resources: resources!(water: 10.),
            byproducts: byproducts!(),
            feedstock: (Feedstock::Oil, 1.),
            features: vec![],
            locked: false,
            banned: false,
        }, Process {
            id: 2,
            name: "Test Process C",
            mix_share: 1.0,
            output: Output::Electricity,
            output_modifier: 1.,
            resources: resources!(water: 2.),
            byproducts: byproducts!(),
            feedstock: (Feedstock::Oil, 1.),
            features: vec![],
            locked: false,
            banned: false,
        }]
    }

    #[test]
    fn test_update_mix_share_resources() {
        let mut processes = gen_processes();
        processes[1].resources = resources!(water: 2.);
        let demand = outputs!(fuel: 100.);
        let resource_weights = resources!(water: 100.);
        let feedstock_weights = feedstocks!();
        update_mixes(&mut processes, &demand, &resource_weights, &feedstock_weights);

        // Less water intensive process should be favored
        assert!(processes[0].mix_share < processes[1].mix_share);

        // Should be normalized
        assert_eq!(processes[0].mix_share + processes[1].mix_share, 1.0);

        // Unrelated process should be unaffected
        assert_eq!(processes[2].mix_share, 1.0);
    }

    #[test]
    fn test_update_mix_share_feedstocks() {
        let mut processes = gen_processes();
        processes[1].feedstock = (Feedstock::Coal, 1.);
        let demand = outputs!(fuel: 100.);
        let resource_weights = resources!(water: 100.);
        let feedstock_weights = feedstocks!(oil: 100.);
        update_mixes(&mut processes, &demand, &resource_weights, &feedstock_weights);

        // Process that doesn't use oil should be favored
        assert!(processes[0].mix_share < processes[1].mix_share);

        // Should be normalized
        assert_eq!(processes[0].mix_share + processes[1].mix_share, 1.0);
    }

    #[test]
    fn test_update_mix_share_banned() {
        let mut processes = gen_processes();
        processes[0].banned = true;
        let demand = outputs!(fuel: 100.);
        let resource_weights = resources!();
        let feedstock_weights = feedstocks!();
        update_mixes(&mut processes, &demand, &resource_weights, &feedstock_weights);

        // Unbanned process should be favored
        assert!(processes[0].mix_share < processes[1].mix_share);

        // Should be normalized
        approx_eq!(f32, processes[0].mix_share + processes[1].mix_share, 1.0);
    }

}
