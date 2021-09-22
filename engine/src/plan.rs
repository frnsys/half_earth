use super::{resources, byproducts};
use super::kinds::{Sector, ResourceMap, ByproductMap};
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};

// Requirements, Amount
#[derive(Debug)]
pub struct ProductionOrder {
    pub kind: Sector,
    pub reqs: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub amount: f32
}

pub fn calculate_production(orders: &[ProductionOrder], limits: Option<ResourceMap<f32>>) -> (Vec<f32>, ResourceMap<f32>, ByproductMap<f32>) {
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

    let solution = match limits {
        Some(available) => {
            let mut problem = vars
                .maximise(filled_demand)
                .using(default_solver);

            for k in consumed_resources.keys() {
                problem = problem.with(consumed_resources[k].clone().leq(available[k]));
            }
            problem.solve().unwrap()
        },
        None => {
            vars
                .maximise(filled_demand)
                .using(default_solver)
                .solve()
                .unwrap()
        }
    };

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


pub fn calculate_mix(orders: &[ProductionOrder], weights: &ResourceMap<f32>) -> Vec<f32> {
    let mut vars = variables!();
    let mut total_produced: Expression = 0.into();
    let mut total_intensity: Expression = 0.into();
    let amounts: Vec<Variable> = orders.iter().map(|order| {
        let amount_to_produce = vars.add(variable().min(0));
        total_produced += amount_to_produce;
        total_intensity += amount_to_produce * order.reqs.energy * weights.energy;
        total_intensity += amount_to_produce * order.reqs.land * weights.land;
        amount_to_produce
    }).collect();

    let solution = vars
        .minimise(total_intensity)
        .using(default_solver)
        .with(total_produced.clone().geq(1000.)) // Just so non-zero values are returned
        .solve()
        .unwrap();

    let total_produced = solution.eval(total_produced);
    let shares: Vec<f32> = amounts.iter().map(|var| (solution.value(*var)/total_produced) as f32).collect();
    shares
}


#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    fn gen_orders() -> Vec<ProductionOrder> {
        let steel_demand = 10.;
        let energy_demand = 5.;

        let steel = ProductionOrder {
            kind: Sector::Materials,
            reqs: resources!(
                energy: 1.,
                land: 1.1),
            byproducts: byproducts!(),
            amount: steel_demand * 0.25,
        };

        let stainless_steel = ProductionOrder {
            kind: Sector::Materials,
            reqs: resources!(
                energy: 1.1,
                land: 1.),
            byproducts: byproducts!(),
            amount: steel_demand * 0.75,
        };

        let solar = ProductionOrder {
            kind: Sector::Energy,
            reqs: resources!(
                energy: 0.1,
                land: 2.),
            byproducts: byproducts!(),
            amount: energy_demand * 1.
        };

        vec![steel, stainless_steel, solar]
    }


    #[test]
    fn test_calculate_production_with_limits() {
        let orders = gen_orders();
        let available_resources = resources!(
            land: 12.,
            energy: 6.
        );

        let (produced, consumed, _byproducts) = calculate_production(&orders, Some(available_resources));

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
    fn test_calculate_production_without_limits() {
        let orders = gen_orders();
        let (produced, consumed, _byproducts) = calculate_production(&orders, None);

        let expected = [2.5, 7.5, 5.0];
        assert!(produced.len() == expected.len());
        assert!(produced.iter().zip(expected)
                .all(|(x1,x2)| approx_eq!(f32, *x1, x2, epsilon=1e-2)));

        let expected = resources!(
            land: 20.25,
            energy: 11.25
        );
        assert_eq!(consumed, expected);

        // Should have created enough to meet total demand
        assert_eq!(produced.iter().sum::<f32>(), 15.);
    }

    #[test]
    fn test_calculate_mix() {
        let orders = gen_orders();

        // Bias towards minimizing land use
        let weights = resources!(
            land: 1.,
            energy: 0.8
        );

        let shares = calculate_mix(&orders[..2], &weights);
        assert_eq!(shares, vec![0.0, 1.0]);
    }
}
