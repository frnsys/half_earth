#[macro_use]
mod processes;
mod sectors;
mod planner;
mod resources;

use crate::kinds::{OutputMap, ResourceMap, ByproductMap};
pub use self::sectors::{Sector, Modifier};
pub use self::processes::{Process, ProcessDetails};

// TODO in resources:
// - deduct consumed resources from cells
// - renewable resources need a replenish rate
// - electricity doesn't stock except based on how much battery you have
// - reqsource reuqirements of extraction, or ist hat already captured upstream?

pub fn produce(sectors: &mut [sectors::Sector], demand: &OutputMap<f32>, resources: &ResourceMap<f32>) -> (OutputMap<f32>, ResourceMap<f32>, ByproductMap<f32>) {
    // Generate production orders based on current process mixes and demand
    let orders: Vec<planner::ProductionOrder> = sectors.iter().map(|s| s.production_orders(&demand)).flatten().collect();

    // Calculate the sector's output
    let (produced, consumed, byproducts) = planner::calculate_production(&orders, &resources);

    // Calculate production per output type
    let mut produced_by_type: OutputMap<f32> = OutputMap::default();
    for (amount, order) in produced.iter().zip(&orders) {
        produced_by_type[order.output] += amount;
    }

    // Get resource deficit/surplus
    let required = planner::calculate_required(&orders);

    // Weigh resources by scarcity
    let resource_weights = required / *resources;

    // Update mix according to resource scarcity
    // TODO transition_speed is per month, can increase at the expense of decommission risk (mine leakages and what not)
    let transition_speed = 0.1; // TODO
    let orders_by_sector: Vec<Vec<planner::ProductionOrder>> = sectors.iter().map(|s| s.production_orders(&demand)).collect();
    for (sector, orders) in sectors.iter_mut().zip(orders_by_sector) {
        sector.update_mix(&orders, &demand, &resource_weights, transition_speed);
    }

    (produced_by_type, consumed, byproducts)
}
