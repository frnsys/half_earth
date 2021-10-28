use super::processes::{Process, ProcessStatus};
use crate::kinds::{ResourceMap, ByproductMap, FeedstockMap, OutputMap, Feedstock};
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};

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

    let mut filled_demand: Expression = 0.into();
    let amounts: Vec<Variable> = orders.iter().map(|order| {
        // Ran into issues where solutions couldn't be found if the min was set to
        // 0. I can't figure out why because it seems under the constraints provided
        // a valid solution will always be for all `amount_to_produce` to equal 0.
        let amount_to_produce = vars.add(variable().min(0.).max(order.amount));

        // Add 1. to avoid zero division issues
        filled_demand += amount_to_produce/(order.amount + 1.);

        for (k, v) in order.process.resources.items() {
            consumed_resources[k] += amount_to_produce * *v;
        }
        for (k, v) in order.process.byproducts.items() {
            created_byproducts[k] += amount_to_produce * *v;
        }

        // Ignore "Other" feedstock
        match order.process.feedstock.0 {
            Feedstock::Other | Feedstock::Soil => (),
            _ => {
                consumed_feedstocks[order.process.feedstock.0] += amount_to_produce * order.process.feedstock.1;
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

/// Calculate the ideal mix of production processes based on demand and resource weights.
/// Here "ideal" means one that minimizes resource usages, weighted by the provided resource
/// weights, while meeting demand.
/// This is intended to be used on a per-sector basis.
pub fn calculate_mix(processes: &[Process], demand: &OutputMap<f32>, resource_weights: &ResourceMap<f32>, feedstock_weights: &FeedstockMap<f32>) -> Vec<f32> {
    let mut vars = variables!();
    let mut total_intensity: Expression = 0.into();

    // Track production per output subtype
    let mut total_produced: OutputMap<Expression> = OutputMap::default();

    let amounts: Vec<Variable> = processes.iter().map(|process| {
        let amount_to_produce = if !process.locked && !process.is_banned() {
            vars.add(variable().min(0))
        } else {
            vars.add(variable().min(0).max(0))
        };
        total_produced[process.output] += amount_to_produce;
        for (k, v) in process.resources.items() {
            total_intensity += amount_to_produce * *v * resource_weights[k];
        }
        // TODO for some reason introducing this causes production to go to infinity?
        // let emissions = process.byproducts.co2 + (process.byproducts.n2o * 298.) + (process.byproducts.ch4 * 36.);
        // total_intensity += amount_to_produce * emissions;

        // let (feedstock, amount) = process.feedstock;
        // total_intensity += amount_to_produce * amount * feedstock_weights[feedstock];
        amount_to_produce
    }).collect();

    let mut problem = vars
        .minimise(total_intensity)
        .using(default_solver);

    let empty_expression: Expression = 0.into();
    for ((k, produced), demand) in total_produced.items().iter().zip(demand.values()) {
        if *demand > 0. {
            // Impossible to create a mix that satisfies this demand because
            // no provided processes produce the demanded output.
            // Just print a warning--in the final game all processes for a sector will
            // always be present, thus encompassing all outputs for that sector;
            // so there will (should) be no cases where this will occur.
            if **produced == empty_expression {
                println!("Non-zero demand for {:?} but no provided processes produce this output.", k);
            } else {
                let constraint = (*produced).clone().geq(*demand as f64);
                problem = problem.with(constraint);
            }
        }
    }

    let shares: Vec<f32> = match problem.solve() {
        Ok(solution) => {
            let total_produced: f64 = total_produced.values().iter().map(|produced| solution.eval(*produced)).sum();
            amounts.iter().map(|var| (solution.value(*var)/total_produced) as f32).collect()
        },
        Err(_) => {
            amounts.iter().map(|_| 0.).collect()
        }
    };
    shares
}


#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;
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
