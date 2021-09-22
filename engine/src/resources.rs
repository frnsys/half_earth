use paste::paste;
use std::ops::{Add, AddAssign, Mul, Div, Index};

/*
 * NOTES:
 * - Do we need to distinguish resources from raw resources (things that are extracted from the
 * ground/from the environment)?
 *      - I think we do because we want raw resources to be attached to regions/cells, and the
 *      other ones not to be (e.g. we want to know how much sunlight a cell gets in each season but
 *      not how much steel it has)
 * - Do we need to add aditional outputs for processes (byproducts), e.g. CO2 for CCS, in addition
 * to impacts
 *      - Can probably change Input to be something more generic (Product?) that can be either an
 *      Input or an Output
 *
 * ENERGY/FOOD/MATERIALS
 */

/// Define an enum of the provided resources,
/// and define a ResourceMap which is indexed by
/// those resource enum variants.
///
/// For example: `define_resources!(Raw {A, B, C})`
/// would define an enum with the variants `Raw::A, Raw::B, Raw::C`;
/// a `RawMap<T>` with the fields `raw_map.a, raw_map.b, raw_map.c`
/// that can be accessed via e.g. `raw_map[Raw::A]`;
/// and can be built like `RawMap::default().a(10).b(12).c(16)`.
macro_rules! define_resources {
    ($name:ident { $($field:ident),* }) => {
        pub enum $name {
            $(
                $field,
            )*
        }

        paste! {
            // Define resource map
            #[derive(Default, Clone, Copy, Debug, PartialEq)]
            pub struct [<$name Map>]<T> {
                $(
                    [<$field:lower>]: T,
                )*
            }

            // Indexing by resource enum
            impl<T> Index<$name> for [<$name Map>]<T> {
                type Output = T;

                fn index(&self, resource: $name) -> &Self::Output {
                    match resource {
                        $(
                            $name::$field => &self.[<$field:lower>],
                        )*
                    }
                }
            }

            // Builder
            impl<T> [<$name Map>]<T> {
                $(
                    pub fn [<$field:lower>](&mut self, val: T) -> &mut [<$name Map>]<T> {
                        self.[<$field:lower>] = val;
                        self
                    }
                )*
            }

            impl [<$name Map>]<f32> {
                fn min(&self) -> f32 {
                    [$(
                        self.[<$field:lower>],
                    )*].iter().fold(1./0., |a, &b| f32::min(a, b))
                }
            }

            // Add and float multiplication ops
            impl Add for [<$name Map>]<f32> {
                type Output = Self;

                fn add(self, other: Self) -> Self {
                    Self {
                        $(
                            [<$field:lower>]: self.[<$field:lower>] + other.[<$field:lower>],
                        )*
                    }
                }
            }

            impl AddAssign for [<$name Map>]<f32> {
                fn add_assign(&mut self, other: Self) {
                    $(
                        self.[<$field:lower>] += other.[<$field:lower>];
                    )*
                }
            }

            impl Mul<f32> for [<$name Map>]<f32> {
                type Output = Self;

                fn mul(self, rhs: f32) -> Self {
                    Self {
                        $(
                            [<$field:lower>]: self.[<$field:lower>] * rhs,
                        )*
                    }
                }
            }

            impl Div for [<$name Map>]<f32> {
                type Output = Self;

                fn div(self, other: Self) -> Self {
                    Self {
                        $(
                            [<$field:lower>]: self.[<$field:lower>]/other.[<$field:lower>],
                        )*
                    }
                }
            }
        }
    }
}


define_resources!(Refined {
    Energy,     // MWh
    Concrete,   // tons
    Steel,      // tons
    Plastic,    // tons
    Livestock,  // tons? calories?
    Food,       // Calories
    CO2         // tons
});

// TODO units
// Low-level resources, i.e. those that are extracted from the environment
define_resources!(Raw {
    Water,
    Wind,
    Sunlight,
    Coal,
    Oil,
    Lumber,
    BaseMetals,
    PreciousMetals,
    REEs,
    Minerals
});

enum Resource {
    Refined(Refined),
    Raw(Raw)
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct ResourceMap<T> {
    raw: RawMap<T>,
    refined: RefinedMap<T>,
}

impl ResourceMap<f32> {
    fn min(&self) -> f32 {
        f32::min(self.raw.min(), self.refined.min())
    }
}

impl<T> Index<Resource> for ResourceMap<T> {
    type Output = T;

    fn index(&self, resource: Resource) -> &Self::Output {
        match resource {
            Resource::Raw(r) => &self.raw[r],
            Resource::Refined(r) => &self.refined[r],
        }
    }
}

impl Add for ResourceMap<f32> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            raw: self.raw + other.raw,
            refined: self.refined + other.refined,
        }
    }
}

impl AddAssign for ResourceMap<f32> {
    fn add_assign(&mut self, other: Self) {
        self.raw += other.raw;
        self.refined += other.refined;
    }
}

impl Mul<f32> for ResourceMap<f32> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            raw: self.raw * rhs,
            refined: self.refined * rhs,
        }
    }
}

impl Div for ResourceMap<f32> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            raw: self.raw/other.raw,
            refined: self.refined/other.refined,
        }
    }
}


// All per unit of the Resource
struct Process<'a> {
    name: &'a str,
    impacts: Impacts,
    inputs: ResourceMap<f32>,
    byproducts: ResourceMap<f32>
}

impl<'a> Process<'a> {
    pub fn new(name: &'a str, impacts: Impacts) -> Process<'a> {
        Process {
            name, impacts,
            inputs: ResourceMap::default(),
            byproducts: ResourceMap::default(),
        }
    }
}

struct ProcessMix<'a, const N: usize> {
    target: [f32; N],
    percents: [f32; N],
    processes: [Process<'a>; N]
}

#[derive(Default, Debug, PartialEq)]
struct Impacts {
    emissions: f32, // tCO2eq; May need to break down into specific chemicals
    land_use: f32,  // ha
    pollution: f32, // TODO ppm?; catch-all for air, land, water pollution that affects local health
}

impl<'a, const N: usize> ProcessMix<'a, N> {
    // Impacts per unit produced, based on current mix
    pub fn impacts(&self) -> Impacts {
        self.weighted_processes().fold(Impacts::default(), |mut acc, (proc, p)| {
            acc.emissions += proc.impacts.emissions * p;
            acc.land_use += proc.impacts.land_use * p;
            acc.pollution += proc.impacts.pollution * p;
            acc
        })
    }

    fn weighted_processes(&self) -> impl Iterator<Item=(&Process<'a>, f32)> {
        self.processes.iter().zip(self.percents)
    }

    // Weighted inputs required across the entire mix, per unit output
    fn inputs_required(&self) -> ResourceMap<f32> {
        self.weighted_processes().fold(ResourceMap::default(), |mut acc, (proc, p)|{
            acc += proc.inputs * p;
            acc
        })
    }

    // Byproducts per unit output
    fn byproducts(&self) -> ResourceMap<f32> {
        self.weighted_processes().fold(ResourceMap::default(), |mut acc, (proc, p)|{
            acc += proc.byproducts * p;
            acc
        })
    }

    // How many units of output is generated for the given inputs,
    // how many inputs are actually consumed, and the byproducts generated
    pub fn output_for_inputs(&self, inputs: ResourceMap<f32>) -> (f32, ResourceMap<f32>, ResourceMap<f32>) {
        let requirements = self.inputs_required();
        let output = (inputs/requirements).min();
        let byproducts = self.byproducts() * output;
        (output, requirements * output, byproducts)
    }

    pub fn transition_to_target(&self) {
        // transition costs? e.g. for building out a new grid or whatever
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_mix_impacts() {
        let mut mix = ProcessMix {
            target: [0.; 3],
            percents: [1., 0., 0.],
            processes: [
                Process::new("Coal",
                    Impacts {
                        emissions: 7.09,
                        land_use: 7.7,
                        pollution: 10.,
                    }),
                Process::new("Coal with CCS",
                    Impacts {
                        emissions: 2.09,
                        land_use: 7.7,
                        pollution: 8.
                    }),
                Process::new("Solar",
                    Impacts {
                        emissions: 0.3,
                        land_use: 4.1,
                        pollution: 1.
                })
            ]
        };
        let impacts = mix.impacts();
        assert_eq!(impacts, Impacts {
            emissions: 7.09,
            land_use: 7.7,
            pollution: 10.,
        });

        mix.percents[0] = 0.5;
        mix.percents[1] = 0.5;
        let impacts = mix.impacts();
        assert_eq!(impacts, Impacts {
            emissions: 4.59,
            land_use: 7.7,
            pollution: 9.,
        });
    }

    #[test]
    fn test_process_builder() {
        let mut coal = Process::new("Coal",
            Impacts {
                emissions: 7.09,
                land_use: 7.7,
                pollution: 10.,
            });
        coal.inputs.raw.coal(10.);
        assert_eq!(coal.inputs.raw.coal, 10.);

        let mut solar = Process::new("Solar",
            Impacts {
                emissions: 0.3,
                land_use: 4.1,
                pollution: 1.
        });
        solar.inputs.raw.sunlight(12.);
        assert_eq!(solar.inputs.raw.sunlight, 12.);
    }

    #[test]
    fn test_process_mix_inputs_required() {
        let mut coal = Process::new("Coal",
            Impacts {
                emissions: 7.09,
                land_use: 7.7,
                pollution: 10.,
            });
        coal.inputs.raw.coal(10.);

        let mut solar = Process::new("Solar",
            Impacts {
                emissions: 0.3,
                land_use: 4.1,
                pollution: 1.
        });
        solar.inputs.raw.sunlight(12.);

        let mut mix = ProcessMix {
            target: [0.; 2],
            percents: [1., 0.],
            processes: [coal, solar]
        };
        let required = mix.inputs_required();
        let mut expected = ResourceMap::default();
        expected.raw.coal(10.);
        assert_eq!(required, expected);

        mix.percents[0] = 0.5;
        mix.percents[1] = 0.5;
        let required = mix.inputs_required();
        let mut expected = ResourceMap::default();
        expected.raw.coal(5.).sunlight(6.);
        assert_eq!(required, expected);
    }

    #[test]
    fn test_process_mix_outputs() {
        let mut coal = Process::new("Coal",
            Impacts {
                emissions: 7.09,
                land_use: 7.7,
                pollution: 10.,
            });
        coal.inputs.raw.coal(10.);
        coal.byproducts.refined.co2(5.);

        let mut solar = Process::new("Solar",
            Impacts {
                emissions: 0.3,
                land_use: 4.1,
                pollution: 1.
        });
        solar.inputs.raw.sunlight(12.);

        let mut mix = ProcessMix {
            target: [0.; 2],
            percents: [1., 0.],
            processes: [coal, solar]
        };

        let mut inputs = ResourceMap::default();
        inputs.raw.coal(20.);
        let (outputs, consumed, byproducts) = mix.output_for_inputs(inputs);
        assert_eq!(outputs, 2.);

        let mut expected = ResourceMap::default();
        expected.raw.coal(20.);
        assert_eq!(consumed, expected);

        let mut expected = ResourceMap::default();
        expected.refined.co2(10.);
        assert_eq!(byproducts, expected);

        mix.percents[0] = 0.5;
        mix.percents[1] = 0.5;
        let mut inputs = ResourceMap::default();
        inputs.raw.coal(20.);
        inputs.raw.sunlight(6.);
        let (outputs, consumed, byproducts) = mix.output_for_inputs(inputs);
        assert_eq!(outputs, 2.5);
    }
}
