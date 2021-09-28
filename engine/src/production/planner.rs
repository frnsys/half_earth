use crate::kinds::{ResourceMap, ByproductMap, OutputMap, Output};
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};

#[derive(Debug)]
pub struct ProductionOrder {
    pub output: Output,
    pub reqs: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub amount: f32
}


/// With a constrained amount of resources, allocate the available resources across the provided
/// production orders/processes.
/// Returns the amount produced for each provided order, the consumed resources, and the byproducts
/// of production.
/// This is currently formulated as a linear programming problem, but ideally we have a non-linear
/// formulation instead (see dev notes).
pub fn calculate_production(orders: &[ProductionOrder], limits: &ResourceMap<f32>) -> (Vec<f32>, ResourceMap<f32>, ByproductMap<f32>) {
    let mut vars = variables!();
    let mut consumed_resources: ResourceMap<Expression> = resources!();
    let mut created_byproducts: ByproductMap<Expression> = byproducts!();

    let mut filled_demand: Expression = 0.into();
    let amounts: Vec<Variable> = orders.iter().map(|order| {
        let amount_to_produce = vars.add(variable().min(0).max(order.amount));

        // Add 1. to avoid zero division issues
        filled_demand += amount_to_produce/(order.amount + 1.);

        for (k, v) in order.reqs.items() {
            consumed_resources[k] += amount_to_produce * *v;
        }
        for (k, v) in order.byproducts.items() {
            created_byproducts[k] += amount_to_produce * *v;
        }
        amount_to_produce
    }).collect();

    let mut problem = vars
        .maximise(filled_demand)
        .using(default_solver);

    for k in consumed_resources.keys() {
        problem = problem.with(consumed_resources[k].clone().leq(limits[k]));
    }

    let solution =  problem.solve().unwrap();

    let produced: Vec<f32> = amounts.iter().map(|var| solution.value(*var) as f32).collect();
    let mut consumed = resources!();
    for k in consumed_resources.keys() {
        consumed[k] = solution.eval(consumed_resources[k].clone()) as f32;
    }
    let byproducts = byproducts!(
        co2: solution.eval(created_byproducts.co2) as f32,
        pollution: solution.eval(created_byproducts.pollution) as f32
    );
    (produced, consumed, byproducts)
}

/// Calculate the total required resources to completely
/// meet the demand of the provided production orders.
pub fn calculate_required(orders: &[ProductionOrder]) -> ResourceMap<f32> {
    orders.iter().fold(resources!(), |mut acc, order| {
        acc += order.reqs * order.amount;
        acc
    })
}

/// Calculate the ideal mix of production processes based on demand and resource weights.
/// Here "ideal" means one that minimizes resource usages, weighted by the provided resource
/// weights, while meeting demand.
/// This is intended to be used on a per-sector basis.
pub fn calculate_mix(orders: &[ProductionOrder], demand: &OutputMap<f32>, resource_weights: &ResourceMap<f32>) -> Vec<f32> {
    let mut vars = variables!();
    let mut total_intensity: Expression = 0.into();

    // Track production per output subtype
    let mut total_produced: OutputMap<Expression> = OutputMap::default();

    let amounts: Vec<Variable> = orders.iter().map(|order| {
        let amount_to_produce = vars.add(variable().min(0));
        total_produced[order.output] += amount_to_produce;
        for (k, v) in order.reqs.items() {
            total_intensity += amount_to_produce * *v * resource_weights[k];
        }
        amount_to_produce
    }).collect();

    let mut problem = vars
        .minimise(total_intensity)
        .using(default_solver);

    let empty_expression: Expression = 0.into();
    for ((k, produced), demand) in total_produced.items().iter().zip(demand.values()) {
        if *demand > 0. {
            // Impossible to create a mix that satisfies this demand because
            // no provided orders/processes produce the demanded output.
            // Just print a warning--in the final game all processes for a sector will
            // always be present, thus encompassing all outputs for that sector;
            // so there will (should) be no cases where this will occur.
            if **produced == empty_expression {
                println!("Non-zero demand for {:?} but no provided production orders produce this output.", k);
            } else {
                let constraint = (*produced).clone().geq(*demand as f64);
                problem = problem.with(constraint);
            }
        }
    }

    let solution = problem.solve().unwrap();

    let total_produced: f64 = total_produced.values().iter().map(|produced| solution.eval(*produced)).sum();
    let shares: Vec<f32> = amounts.iter().map(|var| (solution.value(*var)/total_produced) as f32).collect();
    shares
}


#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    fn gen_orders() -> Vec<ProductionOrder> {
        let fuel_demand = 10.;
        let electricity_demand = 5.;

        let a = ProductionOrder {
            output: Output::Fuel,
            reqs: resources!(
                sun: 1.,
                water: 1.1),
            byproducts: byproducts!(),
            amount: fuel_demand * 0.25,
        };

        let b = ProductionOrder {
            output: Output::Fuel,
            reqs: resources!(
                sun: 1.1,
                water: 1.),
            byproducts: byproducts!(),
            amount: fuel_demand * 0.75,
        };

        let c = ProductionOrder {
            output: Output::Electricity,
            reqs: resources!(
                sun: 0.1,
                water: 2.),
            byproducts: byproducts!(),
            amount: electricity_demand * 1.
        };

        vec![a, b, c]
    }


    #[test]
    fn test_calculate_production_with_limits() {
        let orders = gen_orders();
        let available_resources = resources!(
            sun: 6.,
            water: 12.
        );

        let (produced, consumed, _byproducts) = calculate_production(&orders, &available_resources);

        let expected = [2.5, 2.89, 3.18];
        assert!(produced.len() == expected.len());
        assert!(produced.iter().zip(expected)
                .all(|(x1,x2)| approx_eq!(f32, *x1, x2, epsilon=1e-2)));

        let expected = resources!(
            sun: 6.,
            water: 12.
        );
        assert_eq!(consumed, expected);

        // Should not have created enough to meet total demand
        assert!(produced.iter().sum::<f32>() < 15.);
    }

    #[test]
    fn test_calculated_required() {
        let orders = gen_orders();
        let required = calculate_required(&orders);
        let expected = resources!(
            sun: 11.25,
            water: 20.25
        );
        assert_eq!(required, expected);
    }

    #[test]
    fn test_calculate_mix() {
        let orders = gen_orders();

        // Bias towards minimizing land water
        let resource_weights = resources!(
            sun: 0.8,
            water: 1.
        );
        let demand = outputs!(
            fuel: 10.,
            electricity: 5.
        );

        let shares = calculate_mix(&orders, &demand, &resource_weights);

        // Fuel should only be produced using the second process
        // because it's more land efficient.
        // Overall shares of each output subtype should remain unchanged.
        assert_eq!(shares, vec![0.0, 0.6666667, 0.33333334]);
    }
}
