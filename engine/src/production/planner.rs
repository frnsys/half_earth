use serde::Serialize;
use super::processes::{Process, ProcessStatus};
use crate::kinds::{ResourceMap, ByproductMap, FeedstockMap, Output, OutputMap, Feedstock};
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Clone)]
pub enum Priority {
    Scarcity,
    Land,
    Emissions,
    Energy,
    Labor,
    Water,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Scarcity
    }
}

#[derive(Debug)]
pub struct ProductionOrder<'a> {
    pub process: &'a Process,
    pub amount: f32
}

/// With a constrained amount of resources, allocate the available resources across the provided
/// production orders/processes.
/// Returns the amount produced for each provided order, the consumed resources, and the byproducts
/// of production.
/// This is currently formulated as a linear programming problem, but ideally we have a non-linear
/// formulation instead (see dev notes).
pub fn calculate_production(orders: &[ProductionOrder], resources: &ResourceMap<f32>, feedstocks: &FeedstockMap<f32>) -> (Vec<f32>, ResourceMap<f32>, FeedstockMap<f32>, ByproductMap<f32>) {
    let mut vars = variables!();
    let mut consumed_resources: ResourceMap<Expression> = resources!();
    let mut consumed_feedstocks: FeedstockMap<Expression> = feedstocks!();
    let mut created_byproducts: ByproductMap<Expression> = byproducts!();

    // println!("PLANNER:");
    // println!("Resources: {:?}", resources);
    // println!("Orders: {:?}", orders);

    let mut filled_demand: Expression = 0.into();
    let amounts: Vec<Variable> = orders.iter().map(|order| {
        // Ran into issues where solutions couldn't be found if the min was set to
        // 0. I can't figure out why because it seems under the constraints provided
        // a valid solution will always be for all `amount_to_produce` to equal 0.
        let amount_to_produce = vars.add(variable().min(0.).max(order.amount));

        // Add 1. to avoid zero division issues
        filled_demand += amount_to_produce/(order.amount + 1.);

        // Calculate consumed resources and produced byproducts.
        // Apply output modifiers as a reduction in resource costs
        // and byproducts emitted.
        for (k, v) in order.process.resources.items() {
            consumed_resources[k] += (amount_to_produce * *v)/order.process.output_modifier;
        }
        for (k, v) in order.process.byproducts.items() {
            created_byproducts[k] += (amount_to_produce * *v)/order.process.output_modifier;
        }

        // Ignore "Other" feedstock
        match order.process.feedstock.0 {
            Feedstock::Other | Feedstock::Soil => (),
            _ => {
                consumed_feedstocks[order.process.feedstock.0] += (amount_to_produce * order.process.feedstock.1)/(order.process.output_modifier);
            }
        }
        amount_to_produce
    }).collect();

    let mut problem = vars
        .maximise(filled_demand)
        .using(default_solver);

    for k in consumed_resources.keys() {
        problem = problem.with(consumed_resources[k].clone().leq(resources[k]));
    }
    // for k in consumed_feedstocks.keys() {
    //     // Ignore "Other" feedstock
    //     match k {
    //         Feedstock::Other | Feedstock::Soil => (),
    //         _ => {
    //             problem = problem.with(consumed_feedstocks[k].clone().leq(feedstocks[k]));
    //         }
    //     }
    // }

    let mut consumed_r = resources!();
    let mut consumed_f = feedstocks!();
    let mut byproducts = byproducts!();

    // Ensure values are min 0,
    // slight negatives might occur because of the -1 min constraint above,
    // but these are usually negligible amounts
    let produced: Vec<f32> = match problem.solve() {
        Ok(solution) => {
            for k in consumed_resources.keys() {
                consumed_r[k] = f32::max(solution.eval(consumed_resources[k].clone()) as f32, 0.);
            }
            for k in consumed_feedstocks.keys() {
                consumed_f[k] = f32::max(solution.eval(consumed_feedstocks[k].clone()) as f32, 0.);
            }
            // Byproducts are ok to be negative (e.g. CO2 sequestration)
            for k in created_byproducts.keys() {
                byproducts[k] = solution.eval(created_byproducts[k].clone()) as f32;
            }
            amounts.iter().map(|var| f32::max(solution.value(*var) as f32, 0.)).collect()
        },
        Err(err) => {
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            println!("Planner error: {:?}", err);
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            amounts.iter().map(|_| 0.).collect()
        }
    };
    (produced, consumed_r, consumed_f, byproducts)
}

/// Calculate the total required resources to completely
/// meet the demand of the provided production orders.
pub fn calculate_required(orders: &[ProductionOrder]) -> (ResourceMap<f32>, FeedstockMap<f32>) {
    let mut resources = resources!();
    let mut feedstocks = feedstocks!();
    for order in orders {
        let (feedstock, amount) = order.process.feedstock;
        feedstocks[feedstock] += amount * order.amount;
        resources += order.process.resources * order.amount;
    }
    (resources, feedstocks)
}

fn score_process(p: &Process, priority: &Priority, resource_weights: &ResourceMap<f32>) -> f32 {
    let score = match priority {
        Priority::Scarcity => {
            p.resources.items().map(|(k, v)| v * resource_weights[k]).iter().sum::<f32>()
        },
        Priority::Land => {
            p.resources.land
        },
        Priority::Labor => {
            // TODO labor
            0.
        },
        Priority::Water => {
            p.resources.water
        },
        Priority::Energy => {
            p.resources.electricity + p.resources.fuel
        },
        Priority::Emissions => {
            let emissions = p.byproducts.co2 + (p.byproducts.n2o * 298.) + (p.byproducts.ch4 * 36.);
            emissions
        },
    };
    -score/p.output_modifier
}

fn score_shares(mix: &[f32], scores: &[f32]) -> Vec<f32> {
    let total: f32 = mix.iter().sum();
    mix.iter().zip(scores).map(|(share, score)| {
        // Probably a more elegant way of doing this
        // x^e * score
        // This is to guard against any one process
        // from becoming 100% of production
        let norm_share = share/total;
        norm_share.powf(2.71828) * score
    }).collect()
}

fn score_mix(mix: &[f32], scores: &[f32]) -> f32 {
    score_shares(mix, scores).iter().sum()
}

const TRANSITION_SPEED: f32 = 0.01;

/// Calculate the ideal mix of production processes based on demand and resource weights.
/// Here "ideal" means one that minimizes resource usages, weighted by the provided resource
/// weights, while meeting demand.
/// This is intended to be used on a per-output basis.
pub fn calculate_mix(mut mix: Vec<f32>, processes: &Vec<&mut Process>, weights: &ResourceMap<f32>, feedstock_weights: &FeedstockMap<f32>, priority: &Priority) -> Vec<f32> {
    let mut changes: Vec<(usize, f32)> = vec![];
    let scores: Vec<f32> = processes.iter().map(|p| score_process(p, priority, weights)).collect();

    let base_score = score_mix(&mix, &scores);
    let change_amounts = [TRANSITION_SPEED, -TRANSITION_SPEED];
    for i in 0..mix.len() {
        if processes[i].locked {
            continue;
        } else if processes[i].is_banned() {
            changes.push((i, -TRANSITION_SPEED*2.));
        } else if processes[i].is_promoted() {
            changes.push((i, TRANSITION_SPEED*2.));
        } else {
            for change in &change_amounts {
                let changed = f32::max(mix[i] + change, 0.);
                let orig = std::mem::replace(&mut mix[i], changed);
                let score = score_mix(&mix, &scores);
                let _ = std::mem::replace(&mut mix[i], orig);
                if score > base_score {
                    changes.push((i, *change));
                    break;
                }
            }
        }
    }

    for (idx, change) in changes {
        mix[idx] = f32::max(mix[idx] + change, 0.);
    }
    let total: f32 = mix.iter().sum();
    for share in &mut mix {
        *share /= total;
    }
    mix
}


#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;
    use crate::production::ProcessChange;
    use crate::kinds::{Feedstock, Output};

    fn gen_processes() -> Vec<Process> {
        vec![Process {
            id: 0,
            name: "Test Process A",
            mix_share: 0.5,
            output: Output::Fuel,
            output_modifier: 1.,
            resources: resources!(water: 1.),
            byproducts: byproducts!(),
            feedstock: (Feedstock::Oil, 1.),
            features: vec![],
            locked: false,
            status: ProcessStatus::Neutral,
            change: ProcessChange::Neutral,
            opposers: vec![],
            supporters: vec![],
        }, Process {
            id: 1,
            name: "Test Process B",
            mix_share: 0.5,
            output: Output::Fuel,
            output_modifier: 1.,
            resources: resources!(water: 1.),
            byproducts: byproducts!(),
            feedstock: (Feedstock::Oil, 1.),
            features: vec![],
            locked: false,
            status: ProcessStatus::Neutral,
            change: ProcessChange::Neutral,
            opposers: vec![],
            supporters: vec![],
        }, Process {
            id: 2,
            name: "Test Process C",
            mix_share: 1.0,
            output: Output::Electricity,
            output_modifier: 1.,
            resources: resources!(water: 1.),
            byproducts: byproducts!(),
            feedstock: (Feedstock::Coal, 1.),
            features: vec![],
            locked: false,
            status: ProcessStatus::Neutral,
            change: ProcessChange::Neutral,
            opposers: vec![],
            supporters: vec![],
        }]
    }

    #[test]
    fn test_calculate_production_with_limits() {
        let processes = gen_processes();
        let demand = outputs!(fuel: 100., electricity: 100.);
        let orders: Vec<ProductionOrder> = processes.iter()
            .map(|p| p.production_order(&demand)).collect();

        let resources = resources!(water: 80.);
        let feedstocks = feedstocks!(oil: 100., coal: 100.);
        let (produced, consumed_r, consumed_f, _byproducts) = calculate_production(&orders, &resources, &feedstocks);

        let expected = [30., 50., 0.];
        assert!(produced.len() == expected.len());
        assert!(produced.iter().zip(expected)
                .all(|(x1,x2)| approx_eq!(f32, *x1, x2, epsilon=1e-2)));

        let expected = resources!(
            water: 80.
        );
        assert_eq!(consumed_r, expected);

        let expected = feedstocks!(
            oil: 80.
        );
        assert_eq!(consumed_f, expected);

        // Should not have created enough to meet total demand
        assert!(produced.iter().sum::<f32>() < 200.);
    }

    #[test]
    fn test_calculated_required() {
        let processes = gen_processes();
        let demand = outputs!(fuel: 100., electricity: 100.);
        let orders: Vec<ProductionOrder> = processes.iter()
            .map(|p| p.production_order(&demand)).collect();

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
