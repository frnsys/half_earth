use super::planner;
use super::processes::{Process, ProcessDetails, Amount};
use crate::kinds::{ResourceMap, ByproductMap, OutputMap, Output};

// Modifiers are added to Sectors as Event effects.
// For example: a labor requirement penalty, or additional emissions (e.g. a fugitive/black market
// emissions penalty).
// Modifiers are industry-wide; i.e. they apply to all processes for a given industry.
pub struct Modifier {
    // pub output: f32,
    pub active: bool,
    pub reqs: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
}

// A Sector is a bundle of Processes
// and Modifiers
pub struct Sector {
    processes: Vec<(Process, f32)>,
    modifiers: Vec<Modifier>
}

impl Sector {
    fn active_modifiers(&self) -> (ResourceMap<f32>, ByproductMap<f32>) {
        self.modifiers.iter().filter(|m| m.active).fold(
            (resources!(), byproducts!()),
            |mut acc, modifier| {
                acc.0 += modifier.reqs;
                acc.1 += modifier.byproducts;
                acc
            }
        )
    }

    /// Generates production orders based on the provided demand
    /// and this sector's process mix.
    pub fn production_orders(&self, demand: &OutputMap<f32>) -> Vec<planner::ProductionOrder> {
        let (req_mods, byp_mods) = self.active_modifiers();

        let mut mixes_by_output: OutputMap<f32> = OutputMap::default();
        for (proc, mix_share) in &self.processes {
            mixes_by_output[proc.output] += mix_share;
        };

        self.processes.iter().map(|(proc, mix_share)| {
            // Locked processes should have p == 0.
            planner::ProductionOrder {
                reqs: proc.reqs + req_mods,
                byproducts: proc.byproducts + byp_mods,
                amount: demand[proc.output] * mix_share/mixes_by_output[proc.output],
                output: proc.output,
            }
        }).collect()
    }

    /// Update this sector's process mix to better match
    /// the demand and resource weights (by scarcity).
    /// This mix adjustment happens at a speed of `transition_speed`.
    pub fn update_mix(&mut self,
                  orders: &[planner::ProductionOrder],
                  demand: &OutputMap<f32>,
                  resource_weights: &ResourceMap<f32>,
                  transition_speed: f32) {
        let target_mix = planner::calculate_mix(&orders, &demand, &resource_weights);
        for ((process, mix_share), target) in self.processes.iter_mut().zip(target_mix) {
            if process.unlocked {
                // Phase out banned processes
                if process.banned && *mix_share > 0.{
                    *mix_share = transition_speed;
                } else if !process.banned {
                    if *mix_share < target {
                        *mix_share += transition_speed;
                    } else if *mix_share > target {
                        *mix_share -= transition_speed;
                    }
                }
                *mix_share = f32::max(*mix_share, 0.);
            }
        }

        // Renormalize
        let total: f32 = self.processes.iter().map(|(_, mix_share)| mix_share).sum();
        for (_, mix_share) in &mut self.processes {
            *mix_share /= total;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;

    fn gen_sector() -> Sector {
        let ind_ag = Process {
            unlocked: true,
            banned: false,
            output: Output::PlantCalories,
            reqs: resources!(water: 1.0, sun: 2.0),
            byproducts: byproducts!(co2: 1.0, pollution: 0.5),
            details: details!(
                soil_impact: true,
                pesticides: Amount::High,
                fertilizer: Amount::High,
                livestock: true
            )
        };
        let regen_ag = Process {
            unlocked: true,
            banned: false,
            output: Output::PlantCalories,
            reqs: resources!(water: 2.0, sun: 1.0),
            byproducts: byproducts!(co2: 0.2, pollution: 0.2),
            details: details!(
                soil_impact: false,
                pesticides: Amount::Low,
                fertilizer: Amount::Low,
                livestock: true
            )
        };
        let agriculture = Sector {
            processes: vec![(ind_ag, 0.8), (regen_ag, 0.2)],
            modifiers: vec![Modifier {
                active: false,
                reqs: resources!(labor: 1.),
                byproducts: byproducts!(),
            }, Modifier {
                active: false,
                reqs: resources!(labor: 0.5),
                byproducts: byproducts!(pollution: 1.),
            }]
        };
        agriculture
    }

    #[test]
    fn test_mix_transition() {
        let demand = outputs!(
            plant_calories: 1000.
        );
        let mut agriculture = gen_sector();
        let orders = agriculture.production_orders(&demand);

        // Weight land to be more important than energy
        let resource_weights = resources!(
            water: 1.0,
            sun: 0.5
        );
        let mix: Vec<f32> = agriculture.processes.iter().map(|(_, mix_share)| *mix_share).collect();
        assert_eq!(mix, [0.8, 0.2]);

        // The process is less water intensive but more sun intensive
        // than the second. Because of our weighting, this first process
        // should gain a greater share of the production mix.
        let transition_speed = 0.1;
        agriculture.update_mix(&orders, &demand, &resource_weights, transition_speed);
        let mix: Vec<f32> = agriculture.processes.iter().map(|(_, mix_share)| *mix_share).collect();
        assert_approx_eq!(f32, mix[0], 0.9);
        assert_approx_eq!(f32, mix[1], 0.1);
    }

    #[test]
    fn test_mix_transition_locked() {
        let demand = outputs!(
            plant_calories: 1000.
        );
        let mut agriculture = gen_sector();
        agriculture.processes[0].1 = 1.0;
        agriculture.processes[1].1 = 0.0;
        agriculture.processes[1].0.unlocked = false;
        agriculture.processes[1].0.reqs.water = 0.;

        let orders = agriculture.production_orders(&demand);

        // Weight land to be more important than energy
        let resource_weights = resources!(
            water: 1.0,
            sun: 0.5
        );

        // Although the second process requires no land,
        // the mix shouldn't have changed because that process is locked.
        let transition_speed = 0.1;
        agriculture.update_mix(&orders, &demand, &resource_weights, transition_speed);
        let mix: Vec<f32> = agriculture.processes.iter().map(|(_, mix_share)| *mix_share).collect();
        assert_approx_eq!(f32, mix[0], 1.0);
        assert_approx_eq!(f32, mix[1], 0.0);
    }

    #[test]
    fn test_resource_gap() {
        let demand = outputs!(
            plant_calories: 1000.
        );
        let agriculture = gen_sector();
        let orders = agriculture.production_orders(&demand);

        let available = resources!(
            water: 200.,
            sun: 200.
        );

        let (produced, _consumed, _byproducts) = planner::calculate_production(&orders, &available);

        // Not enough resources, should have produced less than demand
        assert!(produced.iter().sum::<f32>() < 1000.);

        let required = planner::calculate_required(&orders);

        assert_eq!(required, resources!(
            water: 1200.,
            sun: 1800.
        ));

        let gap = required - available;
        assert_eq!(gap, resources!(
            water: 1000.,
            sun: 1600.
        ));
    }

    #[test]
    fn test_active_modifiers() {
        let mut agriculture = gen_sector();
        let (req_mods, byp_mods) = agriculture.active_modifiers();

        // All inactive by default
        assert_eq!(req_mods, resources!());
        assert_eq!(byp_mods, byproducts!());

        // Higher labor requirement
        agriculture.modifiers[0].active = true;
        let (req_mods, byp_mods) = agriculture.active_modifiers();
        assert_eq!(req_mods, resources!(labor: 1.));
        assert_eq!(byp_mods, byproducts!());

        agriculture.modifiers[1].active = true;
        let (req_mods, byp_mods) = agriculture.active_modifiers();
        assert_eq!(req_mods, resources!(labor: 1.5));
        assert_eq!(byp_mods, byproducts!(pollution: 1.));
    }

    #[test]
    fn test_production_with_modifiers() {
        let mut agriculture = gen_sector();

        // Higher labor requirement
        agriculture.modifiers[0].active = true;

        let demand = outputs!(
            plant_calories: 1000.
        );
        let orders = agriculture.production_orders(&demand);

        let available = resources!(
            water: 2000.,
            sun: 2000.,
            labor: 0.
        );
        let (produced, _consumed, _byproducts) = planner::calculate_production(&orders, &available);

        // Nothing should be produced b/c we have no labor
        assert_eq!(produced, [0., 0.]);
    }
}

