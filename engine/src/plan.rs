use enum_map::EnumMap;
use super::sectors::Output;
use std::collections::HashMap;
use super::kinds::{ResourceMap, ByproductMap};
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};

// Requirements, Amount
#[derive(Debug)]
pub struct ProductionOrder<O: Output> {
    pub output: O,
    pub reqs: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub amount: f32
}

// TODO this is calculated on a per-sector basis, should it be per-process?
// TODO if we have resources on a per-process basis, then we don't really need this problem
pub fn calculate_production<O: Output>(orders: &[ProductionOrder<O>], limits: &ResourceMap<f32>) -> (Vec<f32>, ResourceMap<f32>, ByproductMap<f32>) {
    let mut vars = variables!();
    let mut consumed_resources: ResourceMap<Expression> = resources!();
    let mut created_byproducts: ByproductMap<Expression> = byproducts!();

    let mut filled_demand: Expression = 0.into();
    let amounts: Vec<Variable> = orders.iter().map(|order| {
        let amount_to_produce = vars.add(variable().min(0).max(order.amount));

        // TODO better objective function
        // Add 1. to avoid zero division issues
        filled_demand += (amount_to_produce/order.amount)/(order.byproducts.co2+order.byproducts.pollution+1.) + order.amount;

        for (k, v) in order.reqs.items() {
            consumed_resources[k] += amount_to_produce * v;
        }
        for (k, v) in order.byproducts.items() {
            created_byproducts[k] += amount_to_produce * v;
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

pub fn calculate_required<O: Output>(orders: &[ProductionOrder<O>]) -> ResourceMap<f32> {
    orders.iter().fold(resources!(), |mut acc, order| {
        acc += order.reqs * order.amount;
        acc
    })
}

pub fn calculate_mix<O: Output>(orders: &[ProductionOrder<O>], demand: &EnumMap<O, f32>, resource_weights: &ResourceMap<f32>) -> Vec<f32> {
    let mut vars = variables!();
    let mut total_intensity: Expression = 0.into();

    // Track production per output subtype
    // TODO would like to not use a HashMap here because then it requires Hash + PartialEq + Eq on
    // Output, kind of messy
    let mut total_produced: HashMap<O, Expression> = HashMap::default();

    let amounts: Vec<Variable> = orders.iter().map(|order| {
        let amount_to_produce = vars.add(variable().min(0));
        *total_produced.entry(order.output).or_default() += amount_to_produce;
        for (k, v) in order.reqs.items() {
            total_intensity += amount_to_produce * v * resource_weights[k];
        }
        amount_to_produce
    }).collect();

    let mut problem = vars
        .minimise(total_intensity)
        .using(default_solver);

    for (k, v) in total_produced.iter() {
        problem = problem.with(v.clone().geq(demand[*k] as f64));
    }

    let solution = problem.solve().unwrap();

    let total_produced: f64 = total_produced.values().map(|produced| solution.eval(produced)).sum();
    let shares: Vec<f32> = amounts.iter().map(|var| (solution.value(*var)/total_produced) as f32).collect();
    shares
}


#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;
    use enum_map::{enum_map, Enum};

    #[derive(Clone, Copy, Enum, Hash, PartialEq, Eq)]
    enum Widget {
        Basic,
        Advanced
    }
    impl Output for Widget {}

    fn gen_orders() -> Vec<ProductionOrder<Widget>> {
        let basic_demand = 10.;
        let advanced_demand = 5.;

        let basic_a = ProductionOrder {
            output: Widget::Basic,
            reqs: resources!(
                energy: 1.,
                land: 1.1),
            byproducts: byproducts!(),
            amount: basic_demand * 0.25,
        };

        let basic_b = ProductionOrder {
            output: Widget::Basic,
            reqs: resources!(
                energy: 1.1,
                land: 1.),
            byproducts: byproducts!(),
            amount: basic_demand * 0.75,
        };

        let advanced_a = ProductionOrder {
            output: Widget::Advanced,
            reqs: resources!(
                energy: 0.1,
                land: 2.),
            byproducts: byproducts!(),
            amount: advanced_demand * 1.
        };

        vec![basic_a, basic_b, advanced_a]
    }


    #[test]
    fn test_calculate_production_with_limits() {
        let orders = gen_orders();
        let available_resources = resources!(
            land: 12.,
            energy: 6.
        );

        let (produced, consumed, _byproducts) = calculate_production(&orders, &available_resources);

        let expected = [2.5, 2.89, 3.18];
        assert!(produced.len() == expected.len());
        assert!(produced.iter().zip(expected)
                .all(|(x1,x2)| approx_eq!(f32, *x1, x2, epsilon=1e-2)));

        let expected = resources!(
            land: 12.,
            energy: 6.
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
            land: 20.25,
            energy: 11.25
        );
        assert_eq!(required, expected);
    }

    #[test]
    fn test_calculate_mix() {
        let orders = gen_orders();

        // Bias towards minimizing land use
        let resource_weights = resources!(
            land: 1.,
            energy: 0.8
        );
        let demand = enum_map! {
            Widget::Basic => 10.,
            Widget::Advanced => 5.,
        };

        let shares = calculate_mix(&orders, &demand, &resource_weights);

        // Basic widgets should only be produced using the second process
        // because it's more land efficient.
        // Overall shares of each output subtype should remain unchanged.
        assert_eq!(shares, vec![0.0, 0.6666667, 0.33333334]);
    }
}
