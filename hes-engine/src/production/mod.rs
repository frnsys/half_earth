#[macro_use]
mod planner;
mod processes;

use crate::kinds::{OutputMap, ResourceMap, ByproductMap, FeedstockMap};
pub use self::processes::{Process, ProcessFeature};
pub use self::planner::{ProductionOrder, calculate_required};


pub fn produce(orders: &[ProductionOrder], resources: &ResourceMap, feedstocks: &FeedstockMap) -> (Vec<f32>, OutputMap, ResourceMap, FeedstockMap, ByproductMap) {
    // Calculate the output
    let (produced, consumed_r, consumed_f, byproducts) = planner::calculate_production(&orders, &resources, &feedstocks);

    // Calculate production per output type
    let mut produced_by_type: OutputMap = OutputMap::default();
    for (amount, order) in produced.iter().zip(orders) {
        produced_by_type[order.process.output] += amount;
    }

    (produced, produced_by_type, consumed_r, consumed_f, byproducts)
}
