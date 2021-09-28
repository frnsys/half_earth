use super::planner;
use crate::kinds::{ResourceMap, ByproductMap, OutputMap, Output};

pub enum Amount {
    None,
    VeryLow,
    Low,
    Mid,
    High,
    VeryHigh
}

// TODO better way of doing this instead of lumping all together?
pub struct ProcessDetails {
    soil_impact: bool, // degrades or builds soil
    pesticides: Amount,
    fertilizer: Amount,
    livestock: bool,
    intermittent: bool,
}

pub struct Process {
    mix_share: f32,
    unlocked: bool,
    banned: bool,
    reqs: ResourceMap<f32>,
    byproducts: ByproductMap<f32>,
    output: Output,
    details: ProcessDetails
}

// Modifiers are added to Industries as Event effects.
// For example: a labor requirement penalty, or additional emissions (e.g. a fugitive/black market
// emissions penalty).
// Modifiers are industry-wide; i.e. they apply to all processes for a given industry.
pub struct Modifier {
    // pub output: f32,
    pub active: bool,
    pub reqs: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
}

// An Sector is a bundle of Processes
// and Modifiers
pub struct Sector {
    processes: Vec<Process>,
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

    pub fn production_orders(&self, demand: &OutputMap<f32>) -> Vec<planner::ProductionOrder> {
        let (req_mods, byp_mods) = self.active_modifiers();

        let mut mixes_by_output: OutputMap<f32> = OutputMap::default();
        for proc in &self.processes {
            mixes_by_output[proc.output] += proc.mix_share;
        };

        self.processes.iter().map(|proc| {
            // Locked processes should have p == 0.
            planner::ProductionOrder {
                reqs: proc.reqs + req_mods,
                byproducts: proc.byproducts + byp_mods,
                amount: demand[proc.output] * proc.mix_share/mixes_by_output[proc.output],
                output: proc.output,
            }
        }).collect()
    }

    pub fn update_mix(&mut self,
                  orders: &[planner::ProductionOrder],
                  demand: &OutputMap<f32>,
                  resource_weights: &ResourceMap<f32>,
                  transition_speed: f32) {
        let target_mix = planner::calculate_mix(&orders, &demand, &resource_weights);
        for (process, target) in self.processes.iter_mut().zip(target_mix) {
            if process.unlocked {
                // Phase out banned processes
                if process.banned && process.mix_share > 0.{
                    process.mix_share -= transition_speed;
                } else if !process.banned {
                    if process.mix_share < target {
                        process.mix_share += transition_speed;
                    } else if process.mix_share > target {
                        process.mix_share -= transition_speed;
                    }
                }
            }
        }

        // Renormalize
        let total: f32 = self.processes.iter().map(|proc| proc.mix_share).sum();
        for process in &mut self.processes {
            process.mix_share /= total;
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use float_cmp::assert_approx_eq;

//     fn gen_industry() -> Sector<Agriculture, 2> {
//         let ind_ag = Process {
//             unlocked: true,
//             banned: false,
//             output: Calories::Plant,
//             reqs: resources!(land: 1.0, energy: 2.0),
//             byproducts: ByproductMap {co2: 1.0, pollution: 0.5},
//             details: Agriculture {
//                 soil_impact: true,
//                 pesticides: Amount::High,
//                 fertilizer: Amount::High,
//                 livestock: true,
//                 meat: true
//             }
//         };
//         let regen_ag = Process {
//             unlocked: true,
//             banned: false,
//             output: Calories::Plant,
//             reqs: resources!(land: 2.0, energy: 1.0),
//             byproducts: ByproductMap {co2: 0.2, pollution: 0.2},
//             details: Agriculture {
//                 soil_impact: false,
//                 pesticides: Amount::Low,
//                 fertilizer: Amount::Low,
//                 livestock: true,
//                 meat: true
//             }
//         };
//         let agriculture = Sector {
//             kind: Sector::Agriculture,
//             processes: [ind_ag, regen_ag],
//             mix: [0.8, 0.2],
//             modifiers: vec![Modifier {
//                 active: false,
//                 reqs: resources!(labor: 1.),
//                 byproducts: byproducts!(),
//             }, Modifier {
//                 active: false,
//                 reqs: resources!(labor: 0.5),
//                 byproducts: byproducts!(pollution: 1.),
//             }]
//         };
//         agriculture
//     }

//     #[test]
//     fn test_mix_transition() {
//         let demand = enum_map! {
//             Calories::Plant => 1000.,
//             Calories::Meat => 0.
//         };
//         let mut agriculture = gen_industry();
//         let orders = agriculture.production_orders(&demand);

//         // Weight land to be more important than energy
//         let resource_weights = resources!(
//             land: 1.0,
//             energy: 0.5
//         );
//         assert_eq!(agriculture.mix, [0.8, 0.2]);

//         // The process is less land intensive but more energy intensive
//         // than the second. Because of our weighting, this first process
//         // should gain a greater share of the production mix.
//         let transition_speed = 0.1;
//         agriculture.update_mix(&orders, &demand, &resource_weights, transition_speed);
//         assert_approx_eq!(f32, agriculture.mix[0], 0.9);
//         assert_approx_eq!(f32, agriculture.mix[1], 0.1);
//     }

//     #[test]
//     fn test_mix_transition_locked() {
//         let demand = enum_map! {
//             Calories::Plant => 1000.,
//             Calories::Meat => 0.
//         };
//         let mut agriculture = gen_industry();
//         agriculture.mix = [1.0, 0.0];
//         agriculture.processes[1].unlocked = false;
//         agriculture.processes[1].reqs.land = 0.;

//         let orders = agriculture.production_orders(&demand);

//         // Weight land to be more important than energy
//         let resource_weights = resources!(
//             land: 1.0,
//             energy: 0.5
//         );

//         // Although the second process requires no land,
//         // the mix shouldn't have changed because that process is locked.
//         let transition_speed = 0.1;
//         agriculture.update_mix(&orders, &demand, &resource_weights, transition_speed);
//         assert_approx_eq!(f32, agriculture.mix[0], 1.0);
//         assert_approx_eq!(f32, agriculture.mix[1], 0.0);
//     }

//     #[test]
//     fn test_resource_gap() {
//         let demand = enum_map! {
//             Calories::Plant => 1000.,
//             Calories::Meat => 0.
//         };
//         let agriculture = gen_industry();
//         let orders = agriculture.production_orders(&demand);

//         let available = resources!(
//             land: 200.,
//             energy: 200.
//         );

//         let (produced, _consumed, _byproducts) = plan::calculate_production(&orders, &available);

//         // Not enough resources, should have produced less than demand
//         assert!(produced.iter().sum::<f32>() < 1000.);

//         let required = plan::calculate_required(&orders);

//         assert_eq!(required, resources!(
//             land: 1200.,
//             energy: 1800.
//         ));

//         let gap = required - available;
//         assert_eq!(gap, resources!(
//             land: 1000.,
//             energy: 1600.
//         ));
//     }

//     #[test]
//     fn test_active_modifiers() {
//         let mut agriculture = gen_industry();
//         let (req_mods, byp_mods) = agriculture.active_modifiers();

//         // All inactive by default
//         assert_eq!(req_mods, resources!());
//         assert_eq!(byp_mods, byproducts!());

//         // Higher labor requirement
//         agriculture.modifiers[0].active = true;
//         let (req_mods, byp_mods) = agriculture.active_modifiers();
//         assert_eq!(req_mods, resources!(labor: 1.));
//         assert_eq!(byp_mods, byproducts!());

//         agriculture.modifiers[1].active = true;
//         let (req_mods, byp_mods) = agriculture.active_modifiers();
//         assert_eq!(req_mods, resources!(labor: 1.5));
//         assert_eq!(byp_mods, byproducts!(pollution: 1.));
//     }

//     #[test]
//     fn test_production_with_modifiers() {
//         let mut agriculture = gen_industry();

//         // Higher labor requirement
//         agriculture.modifiers[0].active = true;

//         let demand = enum_map! {
//             Calories::Plant => 1000.,
//             Calories::Meat => 0.
//         };
//         let orders = agriculture.production_orders(&demand);

//         let available = resources!(
//             land: 2000.,
//             energy: 2000.,
//             labor: 0.
//         );
//         let (produced, _consumed, _byproducts) = plan::calculate_production(&orders, &available);

//         // Nothing should be produced b/c we have no labor
//         assert_eq!(produced, [0., 0.]);
//     }
// }

