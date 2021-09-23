use super::plan;
use super::regions::{Cell, CellIdx};
use super::{sectors, resources, byproducts};
use super::kinds::{Resource, Sector, SectorMap, ResourceMap, ByproductMap};
use arrayvec::ArrayVec;

enum Amount {
    None,
    VeryLow,
    Low,
    Mid,
    High,
    VeryHigh
}

struct Process<T> {
    unlocked: bool,
    banned: bool,
    reqs: ResourceMap<f32>,
    byproducts: ByproductMap<f32>,
    feats: T
}

// Modifiers are added to Industries as Event effects.
// For example: a labor requirement penalty, or additional emissions (e.g. a fugitive/black market
// emissions penalty).
// Modifiers are industry-wide; i.e. they apply to all processes for a given industry.
struct Modifier {
    // pub output: f32,
    pub active: bool,
    pub reqs: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
}

// An Industry is a bundle of Processes
// and Modifiers
struct Industry<T, const N: usize> {
    kind: Sector,
    processes: [Process<T>; N],
    mix: [f32; N],
    cells: Vec<CellIdx>,
    modifiers: Vec<Modifier> // TODO array?
}

impl<T, const N: usize> Industry<T, N> {
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

    fn production_orders(&self, demand: f32) -> ArrayVec<plan::ProductionOrder, N> {
        let (req_mods, byp_mods) = self.active_modifiers();

        // TODO calculate agg modifiers here, add to proc.reqs
        self.processes.iter().zip(self.mix).map(|(proc, p)| {
            // Locked processes should have p == 0.
            plan::ProductionOrder {
                reqs: proc.reqs + req_mods,
                byproducts: proc.byproducts + byp_mods,
                amount: demand * p,
                kind: self.kind,
            }
        }).collect()
    }

    // TODO transition_speed is per month, can increase at the expense of decommission risk (mine leakages and what not)
    fn update_mix(&mut self, orders: &[plan::ProductionOrder], transition_speed: f32, weights: &ResourceMap<f32>) {
        let target_mix = plan::calculate_mix(&orders, &weights); // This is the target mix
        for ((cur, target), process) in self.mix.iter_mut().zip(target_mix).zip(&self.processes) {
            if process.unlocked {
                // Phase out banned processes
                if process.banned && *cur > 0.{
                    *cur -= transition_speed;
                } else if !process.banned {
                    if *cur < target {
                        *cur += transition_speed;
                    } else if *cur > target {
                        *cur -= transition_speed;
                    }
                }
            }
        }
        // Renormalize
        let total: f32 = self.mix.iter().sum();
        for share in &mut self.mix {
            *share /= total;
        }
    }
}

struct Agriculture {
    soil_impact: bool, // degrades or builds soil
    pesticides: Amount,
    fertilizer: Amount,
    livestock: bool,
    meat: bool
}

struct Energy {
    intermittent: bool,
    portable: bool
}

// TODO the thing with something like housing is that it accumulates,
// so "demand" that's fed to the planning algorithm should really be demand
// for *new* housing
struct Housing {
    accumulates: bool,
}

fn planetary_production(cells: &[Cell], demand: &SectorMap<f32>) {
    // TODO could probably update this as part of the use_change function
    let mut resources_per_sector: SectorMap<ResourceMap<f32>> = sectors!();
    let mut planned_resources_per_sector: SectorMap<ResourceMap<f32>> = sectors!();
    // for cell in cells {
    //     for user in cell.users {
    //         match user {
    //             // Needs to be built-up enough before it contributes resources
    //             Some((sec, readiness)) => {
    //                 if readiness > 3 {
    //                     resources_per_sector[sec] += cell.resources/3.;
    //                 }
    //                 planned_resources_per_sector[sec] += cell.resources/3.;
    //             },
    //             None => (),
    //         }
    //     }
    // }

    let ind_ag = Process {
        unlocked: true,
        banned: false,
        reqs: resources!(land: 1.0, energy: 2.0),
        byproducts: ByproductMap {co2: 1.0, pollution: 0.5},
        feats: Agriculture {
            soil_impact: true,
            pesticides: Amount::High,
            fertilizer: Amount::High,
            livestock: true,
            meat: true
        }
    };
    let regen_ag = Process {
        unlocked: true,
        banned: false,
        reqs: resources!(land: 2.0, energy: 1.0),
        byproducts: ByproductMap {co2: 0.2, pollution: 0.2},
        feats: Agriculture {
            soil_impact: false,
            pesticides: Amount::Low,
            fertilizer: Amount::Low,
            livestock: true,
            meat: true
        }
    };
    let ind = Industry {
        cells: vec![],
        kind: Sector::Agriculture,
        processes: [ind_ag, regen_ag],
        mix: [0.8, 0.2],
        modifiers: vec![]
    };

    let mut sector_capacities: SectorMap<f32> = sectors!();

    let orders = ind.production_orders(demand[ind.kind]);
    let (produced, consumed, byproducts) = plan::calculate_production(&orders, &resources_per_sector[ind.kind]);
    let required = plan::calculate_required(&orders);
    sector_capacities[ind.kind] = produced.iter().sum();
    let gap = required - consumed;

    // TODO resource depletion, need to modify from cells

    // TODO expand/contract cell use
    // let next_available = available.clone();
    // for (k, v) in requirement_gap.items() {
    //     if v > 0. {
    //         match k {
    //             Resource::Land => {
    //                 // TODO check available resources in regions
    //                 // also apply limit, e.g. if only 50% of land can be used
    //                 next_available[k] += v * 0.1; // transition speed
    //             },
    //             _ => () // TODO
    //         }
    //     } else {
    //         // Take some out of commission
    //     }
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;

    fn gen_industry() -> Industry<Agriculture, 2> {
        let ind_ag = Process {
            unlocked: true,
            banned: false,
            reqs: resources!(land: 1.0, energy: 2.0),
            byproducts: ByproductMap {co2: 1.0, pollution: 0.5},
            feats: Agriculture {
                soil_impact: true,
                pesticides: Amount::High,
                fertilizer: Amount::High,
                livestock: true,
                meat: true
            }
        };
        let regen_ag = Process {
            unlocked: true,
            banned: false,
            reqs: resources!(land: 2.0, energy: 1.0),
            byproducts: ByproductMap {co2: 0.2, pollution: 0.2},
            feats: Agriculture {
                soil_impact: false,
                pesticides: Amount::Low,
                fertilizer: Amount::Low,
                livestock: true,
                meat: true
            }
        };
        let agriculture = Industry {
            kind: Sector::Agriculture,
            processes: [ind_ag, regen_ag],
            mix: [0.8, 0.2],
            cells: vec![],
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
        let demand = 1000.;
        let mut agriculture = gen_industry();
        let orders = agriculture.production_orders(demand);

        // Weight land to be more important than energy
        let weights = resources!(
            land: 1.0,
            energy: 0.5
        );
        assert_eq!(agriculture.mix, [0.8, 0.2]);

        // The process is less land intensive but more energy intensive
        // than the second. Because of our weighting, this first process
        // should gain a greater share of the production mix.
        let transition_speed = 0.1;
        agriculture.update_mix(&orders, transition_speed, &weights);
        assert_approx_eq!(f32, agriculture.mix[0], 0.9);
        assert_approx_eq!(f32, agriculture.mix[1], 0.1);
    }

    #[test]
    fn test_mix_transition_locked() {
        let demand = 1000.;
        let mut agriculture = gen_industry();
        agriculture.mix = [1.0, 0.0];
        agriculture.processes[1].unlocked = false;
        agriculture.processes[1].reqs.land = 0.;

        let orders = agriculture.production_orders(demand);

        // Weight land to be more important than energy
        let weights = resources!(
            land: 1.0,
            energy: 0.5
        );

        // Although the second process requires no land,
        // the mix shouldn't have changed because that process is locked.
        let transition_speed = 0.1;
        agriculture.update_mix(&orders, transition_speed, &weights);
        assert_approx_eq!(f32, agriculture.mix[0], 1.0);
        assert_approx_eq!(f32, agriculture.mix[1], 0.0);
    }

    #[test]
    fn test_resource_gap() {
        let demand = 1000.;
        let agriculture = gen_industry();
        let orders = agriculture.production_orders(demand);

        let available = resources!(
            land: 200.,
            energy: 200.
        );

        let (produced, _consumed, _byproducts) = plan::calculate_production(&orders, &available);

        // Not enough resources, should have produced less than demand
        assert!(produced.iter().sum::<f32>() < demand);

        let required = plan::calculate_required(&orders);

        assert_eq!(required, resources!(
            land: 1200.,
            energy: 1800.
        ));

        let gap = required - available;
        assert_eq!(gap, resources!(
            land: 1000.,
            energy: 1600.
        ));
    }

    #[test]
    fn test_active_modifiers() {
        let mut agriculture = gen_industry();
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
        let mut agriculture = gen_industry();

        // Higher labor requirement
        agriculture.modifiers[0].active = true;

        let demand = 1000.;
        let orders = agriculture.production_orders(demand);

        let available = resources!(
            land: 2000.,
            energy: 2000.,
            labor: 0.
        );
        let (produced, _consumed, _byproducts) = plan::calculate_production(&orders, &available);

        // Nothing should be produced b/c we have no labor
        assert_eq!(produced, [0., 0.]);
    }

    // #[test]
    // fn test_gen() {
    //     let ind_ag = Process {
    //         unlocked: true,
    //         permitted: true,
    //         reqs: resources!(land: 1.1, energy: 1.2),
    //         byproducts: ByproductMap {co2: 0.1, pollution: 0.1},
    //         feats: Agriculture {
    //             soil_impact: true,
    //             pesticides: Amount::High,
    //             fertilizer: Amount::High,
    //             livestock: true,
    //             meat: true
    //         }
    //     };
    //     let regen_ag = Process {
    //         unlocked: true,
    //         permitted: true,
    //         reqs: resources!(land: 1.2, energy: 1.0),
    //         byproducts: ByproductMap {co2: 0.2, pollution: 0.2},
    //         feats: Agriculture {
    //             soil_impact: false,
    //             pesticides: Amount::Low,
    //             fertilizer: Amount::Low,
    //             livestock: true,
    //             meat: true
    //         }
    //     };
    //     let agriculture = Industry {
    //         kind: Sector::Agriculture,
    //         processes: [ind_ag, regen_ag],
    //         mix: [0.8, 0.2]
    //     };

    //     let demand = sectors!(agriculture: 1000.);
    //     let ag_orders = agriculture.production_orders(demand.agriculture);
    //     let available = resources!(
    //         land: 200.,
    //         energy: 129.
    //     );
    //     println!("{:?}", ag_orders);
    //     let (produced, consumed, byproducts) = plan::calculate_production(&ag_orders, Some(available));
    //     println!("{:?}", consumed);
    //     println!("{:?}", byproducts);
    //     let mut sector_capacities: SectorMap<f32> = sectors!();
    //     let (_, required, _) = plan::calculate_production(&ag_orders, None);
    //     for (amount, order) in produced.iter().zip(ag_orders) {
    //         println!("AMOUNT: {:?}", amount);
    //         println!("{:?}", order.kind);
    //         sector_capacities[order.kind] += amount;
    //     }
    //     println!("REQUIRED {:?}", required);
    //     println!("{:?}", sector_capacities);
    //     // TODO close gap
    //     let requirement_gap = required - consumed; // TODO

    //     let next_available = available.clone();
    //     for (k, v) in requirement_gap.items() {
    //         if v > 0. {
    //             match k {
    //                 Resource::Land => {
    //                     // TODO check available resources in regions
    //                     // also apply limit, e.g. if only 50% of land can be used
    //                     next_available[k] += v * 0.1; // transition speed
    //                 },
    //                 _ => () // TODO
    //             }
    //         } else {
    //             // Take some out of commission
    //         }
    //     }

    //     // Weight based on resource scarcity
    //     let weights = resources!(
    //         land: required.land/next_available.land,
    //         energy: required.energy/next_available.energy
    //     );

    //     // TODO This is what would happen in a step
    //     // Mix transition
    //     // TODO test this
    //     // Weight based on resource scarcity
    //     // let transition_speed = 0.01; // per month, can increase at the expense of decommission risk (mine leakages and what not)
    //     // let target_mix = plan::calculate_mix(&ag_orders, &weights); // This is the target mix
    //     // for (cur, target) in agriculture.mix.iter_mut().zip(target_mix) {
    //     //     if *cur < target {
    //     //         *cur += transition_speed;
    //     //     } else if *cur > target {
    //     //         *cur -= transition_speed;
    //     //     }
    //     // }

    //     // TODO want to get event probabilities (event system)
    //     // TODO calculate demand (population projections etc)
    //     // TODO regions: how are they selected/related to industry?
    //         // Are different regions "claimed" by an industry? and then only go into that resource
    //         // pool?
    // }
}

