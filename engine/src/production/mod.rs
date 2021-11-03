#[macro_use]
mod planner;
mod processes;

use crate::kinds::{OutputMap, ResourceMap, ByproductMap, FeedstockMap};
pub use self::processes::{Process, ProcessFeature, ProcessStatus, update_mixes};
pub use self::planner::{ProductionOrder, calculate_required};


pub fn produce(orders: &[ProductionOrder], resources: &ResourceMap<f32>, feedstocks: &FeedstockMap<f32>) -> (Vec<f32>, OutputMap<f32>, ResourceMap<f32>, FeedstockMap<f32>, ByproductMap<f32>) {
    // Calculate the output
    let (produced, consumed_r, consumed_f, byproducts) = planner::calculate_production(&orders, &resources, &feedstocks);

    // Calculate production per output type
    let mut produced_by_type: OutputMap<f32> = OutputMap::default();
    for (amount, order) in produced.iter().zip(orders) {
        produced_by_type[order.process.output] += amount * order.process.output_modifier;
    }

    (produced, produced_by_type, consumed_r, consumed_f, byproducts)
}
