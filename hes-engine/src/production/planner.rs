use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use super::processes::Process;
use crate::kinds::{ByproductMap, Feedstock, FeedstockMap, Output, ResourceMap};

#[derive(Debug)]
pub struct ProductionOrder<'a> {
    pub process: &'a Process,
    pub amount: f32,
}

fn rank_orders(
    orders: &[ProductionOrder],
    indices: &mut [usize],
    resources: &ResourceMap,
    feedstocks: &FeedstockMap,
) {
    let mut byproduct_maxs: ByproductMap = byproducts!();
    let mut resource_scores = vec![];
    let mut feedstock_scores = vec![];
    let mut scores: HashMap<usize, (f32, f32)> = HashMap::default();

    for i in indices.iter() {
        let process = orders[*i].process;
        let byproducts = process.adj_byproducts();
        for (k, v) in byproducts.items() {
            byproduct_maxs[k] = v.max(byproduct_maxs[k]);
        }

        let r = process.adj_resources();
        let resource_score: f32 = r
            .items()
            .iter()
            .map(|(k, v)| *v / (resources[*k] + 1.))
            .sum();
        resource_scores.push(resource_score);

        let f = process.adj_feedstock_amount();
        let feedstock_score = f / (feedstocks[process.feedstock.0] + 1.);
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

        let byproducts = orders[*i].process.adj_byproducts();
        let byproduct_score: f32 = byproducts
            .items()
            .iter()
            .map(|(k, v)| *v / (byproduct_maxs[*k] + 1.))
            .sum();
        let score = feedstock_score + resource_score + byproduct_score;

        // Hacky
        (score * 100000.).round() as isize
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
        .map(|(k, v)| available_resources[k] / v)
        .iter()
        .fold(f32::INFINITY, |a, &b| a.min(b));

    let amount_produced = order.amount.min(feedstock_max.min(resource_max)).max(0.);

    for (k, v) in resources.items() {
        available_resources[k] = (available_resources[k] - v * amount_produced).max(0.);
    }
    for (k, v) in byproducts.items() {
        produced_byproducts[k] += v * amount_produced;
    }
    available_feedstocks[feedstock] -= feedstock_amount * amount_produced;

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

    pub fn items_mut(&mut self) -> [(Output, &mut Vec<usize>); 4] {
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
    fn index_mut(&mut self, index: Output) -> &mut Self::Output {
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

            rank_orders(orders, order_idxs, &resources, &feedstocks);

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
        continue_production = !orders_by_output.values().iter().all(|idxs| idxs.is_empty());
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
pub fn calculate_required(orders: &[ProductionOrder]) -> (ResourceMap, FeedstockMap) {
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
    use float_cmp::approx_eq;

    use super::*;
    use crate::kinds::{Feedstock, Output};

    fn gen_processes() -> Vec<Process> {
        vec![
            Process {
                id: 0,
                ref_id: "test_process_a",
                name: "Test Process A",
                limit: None,
                mix_share: 10,
                output: Output::Fuel,
                output_modifier: 0.,
                byproduct_modifiers: byproducts!(),
                resources: resources!(water: 1.),
                byproducts: byproducts!(),
                feedstock: (Feedstock::Oil, 1.),
                features: vec![],
                locked: false,
                opposers: vec![],
                supporters: vec![],
            },
            Process {
                id: 1,
                ref_id: "test_process_b",
                name: "Test Process B",
                limit: None,
                mix_share: 10,
                output: Output::Fuel,
                output_modifier: 0.,
                byproduct_modifiers: byproducts!(),
                resources: resources!(water: 1.),
                byproducts: byproducts!(),
                feedstock: (Feedstock::Oil, 1.),
                features: vec![],
                locked: false,
                opposers: vec![],
                supporters: vec![],
            },
            Process {
                id: 2,
                ref_id: "test_process_c",
                name: "Test Process C",
                limit: None,
                mix_share: 20,
                output: Output::Electricity,
                output_modifier: 0.,
                byproduct_modifiers: byproducts!(),
                resources: resources!(water: 1.),
                byproducts: byproducts!(),
                feedstock: (Feedstock::Coal, 1.),
                features: vec![],
                locked: false,
                opposers: vec![],
                supporters: vec![],
            },
        ]
    }

    #[test]
    fn test_calculate_production_with_resource_limits() {
        let processes = gen_processes();
        let demand = outputs!(fuel: 100., electricity: 100.);
        let orders: Vec<ProductionOrder> = processes
            .iter()
            .map(|p| p.production_order(&demand))
            .collect();

        let resources = resources!(water: 80.);
        let feedstocks = feedstocks!(oil: 100., coal: 100.);
        let (produced, consumed_r, consumed_f, _byproducts) =
            calculate_production(&orders, &resources, &feedstocks);

        let expected = [0., 50., 30.];

        let (required_r, required_f) = calculate_required(&orders);
        // println!("Required Resources: {:?}", required_r);
        // println!("Required Feedstocks: {:?}", required_f);
        // for order in &orders {
        //     println!("Order: {:?} -> {:?} {:?}", order.process.name, order.amount, order.process.output);
        // }
        // println!("Produced: {:?}", produced);
        // println!("Consumed Resources: {:?}", consumed_r);
        // println!("Consumed Feedstocks: {:?}", feedstocks);
        assert!(produced.len() == expected.len());
        assert!(produced.iter().zip(expected).all(|(x1, x2)| approx_eq!(
            f32,
            *x1,
            x2,
            epsilon = 1e-2
        )));

        let expected = resources!(
            water: 80.
        );
        assert_eq!(consumed_r, expected);

        let expected = feedstocks!(
            oil: 50.,
            coal: 30.
        );
        assert_eq!(consumed_f, expected);

        // Should not have created enough to meet total demand
        assert!(produced.iter().sum::<f32>() < 200.);
    }

    #[test]
    fn test_calculated_required() {
        let processes = gen_processes();
        let demand = outputs!(fuel: 100., electricity: 100.);
        let orders: Vec<ProductionOrder> = processes
            .iter()
            .map(|p| p.production_order(&demand))
            .collect();

        let (required_r, required_f) = calculate_required(&orders);
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
}
