#[macro_use]
mod processes;
mod sectors;
mod planner;
mod resources;

use crate::kinds::{OutputMap, ResourceMap, ByproductMap};
pub use self::sectors::{Sector, Modifier};
pub use self::processes::{Process, ProcessFeature, Feedstock};
pub use self::planner::{ProductionOrder, calculate_required};
pub use self::resources::CellGrid;


pub fn produce(orders: &[ProductionOrder], resources: &ResourceMap<f32>) -> (OutputMap<Vec<f32>>, ResourceMap<f32>, ByproductMap<f32>) {
    // Calculate the sector's output
    let (produced, consumed, byproducts) = planner::calculate_production(&orders, &resources);

    // Calculate production per output type
    let mut produced_by_type: OutputMap<Vec<f32>> = OutputMap::default();
    for (amount, order) in produced.iter().zip(orders) {
        produced_by_type[order.output].push(*amount);
    }

    (produced_by_type, consumed, byproducts)
}
