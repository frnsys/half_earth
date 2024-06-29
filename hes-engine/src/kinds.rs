use enum_map::Enum;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Sub,
    SubAssign,
};
use wasm_bindgen::prelude::*;

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

/// Define an enum (e.g. Foo) of the provided variants,
/// and define a struct (e.g. FooMap) which is indexed by
/// those enum variants.
macro_rules! define_enum_map {
    ($name:ident { $($field:ident),* }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
        pub enum $name {
            $(
                $field,
            )*
        }

        paste! {
            // Define map
            #[wasm_bindgen]
            #[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
            pub struct [<$name Map>] {
                $(
                    pub [<$field:snake>]: f32,
                )*
            }

            impl [<$name Map>] {
                pub fn keys(&self) -> [$name; count!($($field)*)] {
                    [$(
                        $name::$field,
                    )*]
                }

                pub fn items(&self) -> [($name, f32); count!($($field)*)] {
                    [$(
                        ($name::$field, self.[<$field:snake>]),
                    )*]
                }

                pub fn values(&self) -> [f32; count!($($field)*)] {
                    [$(
                        self.[<$field:snake>],
                    )*]
                }

                pub fn items_mut(&mut self) -> [($name, &mut f32); count!($($field)*)] {
                    [$(
                        ($name::$field, &mut self.[<$field:snake>]),
                    )*]
                }

                pub fn sum(&self) -> f32 {
                    0. $(
                        + self.[<$field:snake>]
                    )*
                }
            }

            // Indexing by enum variants
            impl Index<$name> for [<$name Map>] {
                type Output = f32;

                fn index(&self, key: $name) -> &Self::Output {
                    match key {
                        $(
                            $name::$field => &self.[<$field:snake>],
                        )*
                    }
                }
            }

            impl IndexMut<$name> for [<$name Map>] {
                fn index_mut(&mut self, key: $name) -> &mut Self::Output {
                    match key {
                        $(
                            $name::$field => &mut self.[<$field:snake>],
                        )*
                    }
                }
            }

            // Map<f32> + Map<f32>
            impl Add for [<$name Map>] {
                type Output = Self;

                fn add(self, rhs: Self) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>] + rhs.[<$field:snake>],
                        )*
                    }
                }
            }

            // Map<f32> += Map<f32>
            impl AddAssign for [<$name Map>] {
                fn add_assign(&mut self, rhs: Self) {
                    $(
                        self.[<$field:snake>] += rhs.[<$field:snake>];
                    )*
                }
            }

            // Map<f32> - Map<f32>
            impl Sub for [<$name Map>] {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>] - rhs.[<$field:snake>],
                        )*
                    }
                }
            }

            // Map<f32> -= Map<f32>
            impl SubAssign for [<$name Map>] {
                fn sub_assign(&mut self, rhs: Self) {
                    $(
                        self.[<$field:snake>] -= rhs.[<$field:snake>];
                    )*
                }
            }

            // Map<f32> + f32
            impl Add<f32> for [<$name Map>] {
                type Output = Self;

                fn add(self, rhs: f32) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>] + rhs,
                        )*
                    }
                }
            }

            // Map * f32
            impl Mul<f32> for [<$name Map>] {
                type Output = Self;

                fn mul(self, rhs: f32) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>] * rhs,
                        )*
                    }
                }
            }

            // Map * Map
            impl Mul for [<$name Map>] {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>] * rhs.[<$field:snake>],
                        )*
                    }
                }
            }

            // Map<f32> *= Map<f32>
            impl MulAssign for [<$name Map>] {
                fn mul_assign(&mut self, rhs: Self) {
                    $(
                        self.[<$field:snake>] *= rhs.[<$field:snake>];
                    )*
                }
            }

            // Map / f32
            impl Div<f32> for [<$name Map>] {
                type Output = Self;

                fn div(self, rhs: f32) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>]/rhs,
                        )*
                    }
                }
            }

            impl Div<[<$name Map>]> for [<$name Map>] {
                type Output = Self;

                fn div(self, rhs: Self) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>]/rhs.[<$field:snake>],
                        )*
                    }
                }
            }


            // See: <https://github.com/rust-lang/rust/issues/35853>
            // macro_rules! [<$name:snake s>] {
            //     () => {
            //         [<$name Map>]::default()
            //     };
            //     ($($subfield:ident: $subvalue:expr),*) => {
            //         {
            //             let mut map = [<$name Map>]::default();
            //             $(
            //                 map.$subfield = $subvalue;
            //             )*
            //             map
            //         }
            //     };
            // }
        }
    }
}

define_enum_map!(Resource {
    Land,
    Water,
    Electricity,
    Fuel
});

define_enum_map!(Byproduct {
    Co2,
    Ch4,
    N2o,
    Biodiversity
});

define_enum_map!(Output {
    Fuel,
    Electricity,
    PlantCalories,
    AnimalCalories
});

define_enum_map!(Feedstock {
    Soil,
    Oil,
    Coal,
    Uranium,
    Lithium,
    Thorium,
    NaturalGas,
    Other
});

// Would like to define these as part of the `define_enum_map`
// macro but it looks like nested macros aren't well supported.
/// Macro for quickly creating a maps with default values.
#[macro_export]
macro_rules! resources {
    () => {
        ResourceMap::default()
    };
    ($($field:ident: $value:expr),*) => {
        {
            let mut map = ResourceMap::default();
            $(
                map.$field = $value;
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! byproducts {
    () => {
        ByproductMap::default()
    };
    ($($field:ident: $value:expr),*) => {
        {
            let mut map = ByproductMap::default();
            $(
                map.$field = $value;
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! outputs {
    () => {
        OutputMap::default()
    };
    ($($field:ident: $value:expr),*) => {
        {
            let mut map = OutputMap::default();
            $(
                map.$field = $value;
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! feedstocks {
    () => {
        FeedstockMap::default()
    };
    ($($field:ident: $value:expr),*) => {
        {
            let mut map = FeedstockMap::default();
            $(
                map.$field = $value;
            )*
            map
        }
    };
}

impl ByproductMap {
    pub fn co2eq(&self) -> f32 {
        self.co2 + self.ch4 * 36. + self.n2o * 298.
    }

    pub fn gtco2eq(&self) -> f32 {
        self.co2eq() * 1e-15
    }
}

impl OutputMap {
    pub fn energy(&self) -> f32 {
        self.electricity + self.fuel
    }
}

impl ResourceMap {
    pub fn energy(&self) -> f32 {
        self.electricity + self.fuel
    }
}
