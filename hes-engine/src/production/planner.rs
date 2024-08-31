use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

use super::processes::Process;
use crate::{byproducts, feedstocks, kinds::*, resources};

#[derive(Debug)]
pub struct ProductionOrder<'a> {
    pub process: &'a Process,
    pub amount: f32,
}

/// Returns a value in `[0.0, 1.0]` to score the efficiency of resource
/// usage for a process. `1.0` means the most efficient.
///
/// * Pressure is the overall demand for the resource over
///     the total available amount of the resource.
/// * Relative intensity is the relative intensity of this process
///     for that resource compared to the most intensive process.
///
/// The idea here is that higher pressure on a resource means
/// that lower-intensity usages will be valued more.
fn efficiency_score(
    pressure: f32,
    relative_intensity: f32,
) -> f32 {
    let val = (1. / (relative_intensity + 1.)).powf(pressure);
    (val - 0.5) / 0.5
}

fn resource_score(
    required: &ResourceMap,
    demand: &ResourceMap,
    available: &ResourceMap,
    max_intensity: &ResourceMap,
) -> f32 {
    let pressure = *demand / *available;
    let rel_intensity = *required / *max_intensity;
    required
        .items()
        .into_iter()
        .map(|(k, v)| {
            // If this doesn't use this resource that's
            // an +infinite score.
            if v == 0. || max_intensity[k] == 0. {
                f32::INFINITY
            } else {
                efficiency_score(pressure[k], rel_intensity[k])
            }
        })
        // Take the worst score as the overall score.
        .fold(f32::INFINITY, |a, b| a.min(b))
}

fn feedstock_score(
    required: f32,
    demand: f32,
    available: f32,
    max_intensity: f32,
) -> f32 {
    if required == 0. {
        f32::INFINITY
    } else {
        efficiency_score(
            demand / available,
            required / max_intensity,
        )
    }
}

/// Rank production orders according to efficiency.
/// That is we prioritize by lowest-intensity in terms
/// of feedstock and resource usage.
fn rank_orders(
    orders: &[ProductionOrder],
    indices: &mut [usize],
    demand: (&ResourceMap, &FeedstockMap),
    resources: &ResourceMap,
    feedstocks: &FeedstockMap,
) {
    let mut resource_scores = vec![];
    let mut feedstock_scores = vec![];
    let mut scores: BTreeMap<usize, (f32, f32)> =
        BTreeMap::default();

    let mut max_intensity = resources!();
    for i in indices.iter() {
        let process = orders[*i].process;
        let resources = process.adj_resources();
        for (k, val) in max_intensity.items_mut() {
            *val = val.max(resources[k]);
        }
    }
    let mut max_intensity_fs = feedstocks!();
    for i in indices.iter() {
        let process = orders[*i].process;
        let feedstock = process.adj_feedstock_amount();
        max_intensity_fs[process.feedstock.0] =
            max_intensity_fs[process.feedstock.0]
                .max(feedstock);
    }

    for i in indices.iter() {
        let process = orders[*i].process;

        let r = process.adj_resources();
        let resource_score = resource_score(
            &r,
            resources,
            demand.0,
            &max_intensity,
        );
        resource_scores.push(resource_score);

        let f = process.adj_feedstock_amount();
        let fs = process.feedstock.0;
        let feedstock_score = feedstock_score(
            f,
            demand.1[fs],
            feedstocks[fs],
            max_intensity_fs[fs],
        );
        feedstock_scores.push(feedstock_score);

        scores.insert(*i, (resource_score, feedstock_score));
    }

    let max_resource_score = resource_scores
        .iter()
        .fold(-f32::INFINITY, |a, &b| a.max(b));
    let max_feedstock_score = feedstock_scores
        .iter()
        .fold(-f32::INFINITY, |a, &b| a.max(b));

    // Sort so that the best (lowest) scoring are at the end
    indices.sort_by_cached_key(|i| {
        let (r, f) = scores.get(i).unwrap();
        let resource_score = r / max_resource_score;
        let feedstock_score = f / max_feedstock_score;
        let score = feedstock_score.min(resource_score);

        // Hacky
        (score * 1e6).round() as isize
    });
}

fn produce_amount(
    order: &ProductionOrder,
    available_resources: &mut ResourceMap,
    available_feedstocks: &mut FeedstockMap,
    produced_byproducts: &mut ByproductMap,
) -> f32 {
    let byproducts = order.process.adj_byproducts();
    let feedstock = order.process.feedstock.0;
    let feedstock_amount = order.process.adj_feedstock_amount();
    let resources = order.process.adj_resources();

    let feedstock_max = match feedstock {
        Feedstock::Other | Feedstock::Soil => order.amount,
        _ => available_feedstocks[feedstock] / feedstock_amount,
    };
    let resource_max = resources
        .items()
        .into_iter()
        .map(|(k, v)| available_resources[k] / v)
        .fold(f32::INFINITY, |a, b| a.min(b));

    let amount_produced = order
        .amount
        .min(feedstock_max.min(resource_max))
        .max(0.);

    for (k, v) in resources.items() {
        available_resources[k] = (available_resources[k]
            - v * amount_produced)
            .max(0.);
    }
    for (k, v) in byproducts.items() {
        produced_byproducts[k] += v * amount_produced;
    }
    available_feedstocks[feedstock] -=
        feedstock_amount * amount_produced;

    amount_produced
}

#[derive(Default)]
struct Outputs {
    fuel: Vec<usize>,
    electricity: Vec<usize>,
    plant_calories: Vec<usize>,
    animal_calories: Vec<usize>,
}
impl Outputs {
    pub fn values(&self) -> [&[usize]; 4] {
        [
            &self.fuel,
            &self.electricity,
            &self.plant_calories,
            &self.animal_calories,
        ]
    }

    pub fn items_mut(
        &mut self,
    ) -> [(Output, &mut Vec<usize>); 4] {
        [
            (Output::Fuel, &mut self.fuel),
            (Output::Electricity, &mut self.electricity),
            (Output::PlantCalories, &mut self.plant_calories),
            (Output::AnimalCalories, &mut self.animal_calories),
        ]
    }
}
impl Index<Output> for Outputs {
    type Output = Vec<usize>;
    fn index(&self, index: Output) -> &Self::Output {
        match index {
            Output::Fuel => &self.fuel,
            Output::Electricity => &self.electricity,
            Output::PlantCalories => &self.plant_calories,
            Output::AnimalCalories => &self.animal_calories,
        }
    }
}
impl IndexMut<Output> for Outputs {
    fn index_mut(
        &mut self,
        index: Output,
    ) -> &mut Self::Output {
        match index {
            Output::Fuel => &mut self.fuel,
            Output::Electricity => &mut self.electricity,
            Output::PlantCalories => &mut self.plant_calories,
            Output::AnimalCalories => &mut self.animal_calories,
        }
    }
}

pub fn calculate_production(
    orders: &[ProductionOrder],
    demand: (&ResourceMap, &FeedstockMap),
    starting_resources: &ResourceMap,
    starting_feedstocks: &FeedstockMap,
) -> (Vec<f32>, ResourceMap, FeedstockMap, ByproductMap) {
    let mut resources = starting_resources.clone();
    let mut feedstocks = starting_feedstocks.clone();
    let mut produced_byproducts: ByproductMap = byproducts!();
    let mut produced = vec![0.; orders.len()];

    let mut orders_by_output: Outputs = Outputs::default();
    for (i, order) in orders.iter().enumerate() {
        orders_by_output[order.process.output].push(i);
    }

    let mut continue_production = true;
    while continue_production {
        for (_, order_idxs) in orders_by_output.items_mut() {
            if order_idxs.is_empty() {
                continue;
            }

            rank_orders(
                orders,
                order_idxs,
                demand,
                &resources,
                &feedstocks,
            );

            // Ok to unwrap b/c we check if `orders` is empty
            let order_idx = order_idxs.pop().unwrap();
            let amount = produce_amount(
                &orders[order_idx],
                &mut resources,
                &mut feedstocks,
                &mut produced_byproducts,
            );
            produced[order_idx] = amount;
        }
        continue_production = !orders_by_output
            .values()
            .iter()
            .all(|idxs| idxs.is_empty());
    }

    let consumed_resources = *starting_resources - resources;
    let consumed_feedstocks = *starting_feedstocks - feedstocks;
    (
        produced,
        consumed_resources,
        consumed_feedstocks,
        produced_byproducts,
    )
}

/// Calculate the total required resources to completely
/// meet the demand of the provided production orders.
pub fn calculate_required(
    orders: &[ProductionOrder],
) -> (ResourceMap, FeedstockMap) {
    let mut resources = resources!();
    let mut feedstocks = feedstocks!();
    for order in orders {
        let (feedstock, amount) = order.process.feedstock;
        feedstocks[feedstock] += amount * order.amount;
        resources += order.process.resources * order.amount;
        // println!("{:?}", order.process.name);
        // println!("  Electricity: {:?}TWh", order.process.resources.electricity * order.amount * 1e-9);
        // println!("  Fuel: {:?}TWh", order.process.resources.fuel * order.amount * 1e-9);
    }
    (resources, feedstocks)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        kinds::{Feedstock, Output},
        outputs,
        Id,
    };

    fn gen_processes() -> Vec<Process> {
        vec![
            Process {
                id: Id::new_v4(),
                name: "Test Process A".into(),
                mix_share: 10,
                output: Output::Fuel,
                resources: resources!(water: 1.),
                feedstock: (Feedstock::Oil, 1.),
                ..Default::default()
            },
            Process {
                id: Id::new_v4(),
                name: "Test Process B".into(),
                mix_share: 10,
                output: Output::Fuel,
                resources: resources!(water: 1.),
                feedstock: (Feedstock::Oil, 1.),
                ..Default::default()
            },
            Process {
                id: Id::new_v4(),
                name: "Test Process C".into(),
                mix_share: 20,
                output: Output::Electricity,
                resources: resources!(water: 1.),
                feedstock: (Feedstock::Coal, 1.),
                ..Default::default()
            },
        ]
    }

    // #[test]
    // fn test_calculate_production_with_resource_limits() {
    //     let processes = gen_processes();
    //     let demand = outputs!(fuel: 100., electricity: 100.);
    //     let orders: Vec<ProductionOrder> = processes
    //         .iter()
    //         .map(|p| p.production_order(&demand))
    //         .collect();
    //
    //     let resources = resources!(water: 80.);
    //     let feedstocks = feedstocks!(oil: 100., coal: 100.);
    //     let (produced, consumed_r, consumed_f, _byproducts) =
    //         calculate_production(
    //             &orders,
    //             &resources,
    //             &feedstocks,
    //         );
    //
    //     let expected = [0., 50., 30.];
    //
    //     assert!(produced.len() == expected.len());
    //     assert!(produced.iter().zip(expected).all(
    //         |(x1, x2)| approx_eq!(f32, *x1, x2, epsilon = 1e-2)
    //     ));
    //
    //     let expected = resources!(
    //         water: 80.
    //     );
    //     assert_eq!(consumed_r, expected);
    //
    //     let expected = feedstocks!(
    //         oil: 50.,
    //         coal: 30.
    //     );
    //     assert_eq!(consumed_f, expected);
    //
    //     // Should not have created enough to meet total demand
    //     assert!(produced.iter().sum::<f32>() < 200.);
    // }

    #[test]
    fn test_calculated_required() {
        let processes = gen_processes();
        let demand = outputs!(fuel: 100., electricity: 100.);
        let orders: Vec<ProductionOrder> = processes
            .iter()
            .map(|p| p.production_order(&demand))
            .collect();

        let (required_r, required_f) =
            calculate_required(&orders);
        let expected = resources!(
            water: 200.
        );
        assert_eq!(required_r, expected);
        let expected = feedstocks!(
            oil: 100.,
            coal: 100.
        );
        assert_eq!(required_f, expected);
    }

    #[test]
    fn test_efficiency_score() {
        // Best = No pressure, no intensity
        let score = efficiency_score(0., 0.);
        assert_eq!(score, 1.);

        // Worst = High pressure, high intensity
        let score = efficiency_score(1., 1.);
        assert_eq!(score, 0.);

        // Good = High pressure, no intensity
        let score = efficiency_score(1., 0.);
        assert_eq!(score, 1.);

        // Good = No pressure, high intensity
        let score = efficiency_score(0., 1.);
        assert_eq!(score, 1.);

        // With equal intensity,
        // lower pressure is better.
        let score_a = efficiency_score(0.1, 1.);
        let score_b = efficiency_score(0.2, 1.);
        let score_c = efficiency_score(0.5, 1.);
        assert!(score_a > score_b);
        assert!(score_b > score_c);

        // With equal pressure, lower intensity
        // is better.
        let score_a = efficiency_score(0.1, 0.5);
        let score_b = efficiency_score(0.1, 1.0);
        assert!(score_a > score_b);
    }

    #[test]
    fn test_order_resource_scoring() {
        let overall_demand = resources!(
            water: 1.5,
            land: 2.1
        );
        let required_a = resources!(
            water: 1.,
            land: 1.
        );
        let required_b = resources!(
            water: 0.5,
            land: 1.1
        );
        let available = resources!(
            water: 1.,
            land: 1.
        );
        let max_intensity = resources!(
            water: 1.,
            land: 1.
        );
        let score_a = resource_score(
            &required_a,
            &available,
            &overall_demand,
            &max_intensity,
        );
        let score_b = resource_score(
            &required_b,
            &available,
            &overall_demand,
            &max_intensity,
        );

        // Process B should be better as it uses less water,
        // even though it uses a bit more land.
        assert!(score_b > score_a);

        // But if land is relatively scarce,
        // then process a should be better as it uses less land.
        let available = resources!(
            water: 1.,
            land: 0.5
        );
        let score_a = resource_score(
            &required_a,
            &overall_demand,
            &available,
            &max_intensity,
        );
        let score_b = resource_score(
            &required_b,
            &overall_demand,
            &available,
            &max_intensity,
        );
        assert!(score_a > score_b);
    }
}
