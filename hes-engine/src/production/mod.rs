#[macro_use]
mod planner;
mod processes;

use std::collections::BTreeMap;

pub use self::{
    planner::{calculate_required, ProductionOrder},
    processes::{Process, ProcessChanges, ProcessFeature},
};
use crate::{
    kinds::{
        ByproductMap,
        FeedstockMap,
        OutputMap,
        ResourceMap,
    },
    Id,
};

pub fn produce(
    orders: &[ProductionOrder],
    resources: &ResourceMap,
    feedstocks: &FeedstockMap,
) -> (
    BTreeMap<Id, f32>,
    OutputMap,
    ResourceMap,
    FeedstockMap,
    ByproductMap,
) {
    // Calculate the output
    let (produced, consumed_r, consumed_f, byproducts) =
        planner::calculate_production(
            &orders,
            &resources,
            &feedstocks,
        );

    // Calculate production per output type
    let mut produced_by_type: OutputMap = OutputMap::default();
    for (amount, order) in produced.iter().zip(orders) {
        produced_by_type[order.process.output] += amount;
    }

    let produced = produced
        .into_iter()
        .zip(orders)
        .map(|(amount, order)| (order.process.id, amount))
        .collect();

    (
        produced,
        produced_by_type,
        consumed_r,
        consumed_f,
        byproducts,
    )
}
