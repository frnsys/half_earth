use super::plan;
use std::hash::Hash;
use enum_map::{Enum, EnumMap, enum_map};
use super::regions::{CellIdx, CellGrid};
use super::kinds::{ResourceMap, ByproductMap};
use arrayvec::ArrayVec;

// Sectors:
// Agriculture,        // Calories
// Materials,          // Tons
// Energy,             // MWh
// Projects,           // Units of progress/maintenance
// Other               // All other needs, each output is a "person"
//                     // whose per-capita resource needs have been met


pub trait Output: Copy + Enum<f32> + Hash + PartialEq + Eq {}

#[derive(Clone, Copy, Enum, Hash, PartialEq, Eq)]
enum Power {
    Fuel,
    Electricity
}
impl Output for Power {}

#[derive(Clone, Copy, Enum, Hash, PartialEq, Eq)]
enum Calories {
    Plant,
    Meat
}
impl Output for Calories {}

trait SectorDetails {
    type Output : Output;
}

enum Amount {
    None,
    VeryLow,
    Low,
    Mid,
    High,
    VeryHigh
}

struct Process<T: SectorDetails> {
    unlocked: bool,
    banned: bool,
    reqs: ResourceMap<f32>,
    byproducts: ByproductMap<f32>,
    output: T::Output,
    details: T
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
struct Industry<T: SectorDetails, const N: usize> {
    processes: [Process<T>; N],
    mix: [f32; N],
    cells: Vec<CellIdx>,
    modifiers: Vec<Modifier>
}

impl<T: SectorDetails, const N: usize> Industry<T, N> {
    pub fn produce<const M: usize>(&mut self, demand: &EnumMap<T::Output, f32>, grid: &mut CellGrid<M>) -> (EnumMap<T::Output, f32>, ByproductMap<f32>) {
        // Generate production orders based on current process mix and demand
        let orders = self.production_orders(&demand);

        // Get active resources for this sector
        let resources = grid.resources_for_cells(&self.cells);

        // TODO check/test this now that we have output subtypes
        // Calculate the sector's output
        let (produced, consumed, byproducts) = plan::calculate_production(&orders, &resources);

        // TODO deduct consumed resources from cells
        // TODO renewable resources need a replenish rate

        // Get resource deficit/surplus
        let required = plan::calculate_required(&orders);
        let gap = required - consumed;

        // Adjust resource amounts
        // TODO expand/contract should be combined, since there may very well be a mix
        // of some resources in surplus and some in deficit
        let growth_rate = 0.2; // TODO
        let n_expansions = (self.cells.len() as f32 * growth_rate) as usize;
        self.cells = grid.expand_resources(&self.cells, &gap, n_expansions);

        let transition_speed = 0.1;
        self.cells = grid.contract_resources(&self.cells, &gap, transition_speed);

        // Weigh resources by scarcity
        let resource_weights = required/resources;

        // Update mix according to resource scarcity
        let transition_speed = 0.1; // TODO
        self.update_mix(&orders, &demand, &resource_weights, transition_speed);

        // Calculate production per output type
        let mut produced_by_type = enum_map! { _ => 0. };
        for (amount, process) in produced.iter().zip(&self.processes) {
            produced_by_type[process.output] += amount;
        }

        (produced_by_type, byproducts)
    }

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

    // TODO demand per output type
    fn production_orders(&self, demand: &EnumMap<T::Output, f32>) -> ArrayVec<plan::ProductionOrder<T::Output>, N> {
        let (req_mods, byp_mods) = self.active_modifiers();

        let mut mixes_by_output = enum_map! { _ => 0. };
        for (proc, p) in self.processes.iter().zip(self.mix) {
            mixes_by_output[proc.output] += p;
        };

        self.processes.iter().zip(self.mix).map(|(proc, p)| {
            // Locked processes should have p == 0.
            plan::ProductionOrder {
                reqs: proc.reqs + req_mods,
                byproducts: proc.byproducts + byp_mods,
                amount: demand[proc.output] * p/mixes_by_output[proc.output],
                output: proc.output,
            }
        }).collect()
    }

    // TODO transition_speed is per month, can increase at the expense of decommission risk (mine leakages and what not)
    fn update_mix(&mut self,
                  orders: &[plan::ProductionOrder<T::Output>],
                  demand: &EnumMap<T::Output, f32>,
                  resource_weights: &ResourceMap<f32>,
                  transition_speed: f32) {
        let target_mix = plan::calculate_mix(&orders, &demand, &resource_weights);
        for ((cur, target), process) in self.mix.iter_mut().zip(target_mix).zip(&self.processes) {
            if process.unlocked {
                let cur = *cur;

                // Phase out banned processes
                if process.banned && cur > 0.{
                    cur -= transition_speed;
                } else if !process.banned {
                    if cur < target {
                        cur += transition_speed;
                    } else if cur > target {
                        cur -= transition_speed;
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
impl SectorDetails for Agriculture {
    type Output = Calories;
}

struct Energy {
    intermittent: bool,
}
impl SectorDetails for Energy {
    type Output = Power;
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;

    fn gen_industry() -> Industry<Agriculture, 2> {
        let ind_ag = Process {
            unlocked: true,
            banned: false,
            output: Calories::Plant,
            reqs: resources!(land: 1.0, energy: 2.0),
            byproducts: ByproductMap {co2: 1.0, pollution: 0.5},
            details: Agriculture {
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
            output: Calories::Plant,
            reqs: resources!(land: 2.0, energy: 1.0),
            byproducts: ByproductMap {co2: 0.2, pollution: 0.2},
            details: Agriculture {
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
        let demand = enum_map! {
            Calories::Plant => 1000.,
            Calories::Meat => 0.
        };
        let mut agriculture = gen_industry();
        let orders = agriculture.production_orders(&demand);

        // Weight land to be more important than energy
        let resource_weights = resources!(
            land: 1.0,
            energy: 0.5
        );
        assert_eq!(agriculture.mix, [0.8, 0.2]);

        // The process is less land intensive but more energy intensive
        // than the second. Because of our weighting, this first process
        // should gain a greater share of the production mix.
        let transition_speed = 0.1;
        agriculture.update_mix(&orders, &demand, &resource_weights, transition_speed);
        assert_approx_eq!(f32, agriculture.mix[0], 0.9);
        assert_approx_eq!(f32, agriculture.mix[1], 0.1);
    }

    #[test]
    fn test_mix_transition_locked() {
        let demand = enum_map! {
            Calories::Plant => 1000.,
            Calories::Meat => 0.
        };
        let mut agriculture = gen_industry();
        agriculture.mix = [1.0, 0.0];
        agriculture.processes[1].unlocked = false;
        agriculture.processes[1].reqs.land = 0.;

        let orders = agriculture.production_orders(&demand);

        // Weight land to be more important than energy
        let resource_weights = resources!(
            land: 1.0,
            energy: 0.5
        );

        // Although the second process requires no land,
        // the mix shouldn't have changed because that process is locked.
        let transition_speed = 0.1;
        agriculture.update_mix(&orders, &demand, &resource_weights, transition_speed);
        assert_approx_eq!(f32, agriculture.mix[0], 1.0);
        assert_approx_eq!(f32, agriculture.mix[1], 0.0);
    }

    #[test]
    fn test_resource_gap() {
        let demand = enum_map! {
            Calories::Plant => 1000.,
            Calories::Meat => 0.
        };
        let agriculture = gen_industry();
        let orders = agriculture.production_orders(&demand);

        let available = resources!(
            land: 200.,
            energy: 200.
        );

        let (produced, _consumed, _byproducts) = plan::calculate_production(&orders, &available);

        // Not enough resources, should have produced less than demand
        assert!(produced.iter().sum::<f32>() < 1000.);

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

        let demand = enum_map! {
            Calories::Plant => 1000.,
            Calories::Meat => 0.
        };
        let orders = agriculture.production_orders(&demand);

        let available = resources!(
            land: 2000.,
            energy: 2000.,
            labor: 0.
        );
        let (produced, _consumed, _byproducts) = plan::calculate_production(&orders, &available);

        // Nothing should be produced b/c we have no labor
        assert_eq!(produced, [0., 0.]);
    }
}

