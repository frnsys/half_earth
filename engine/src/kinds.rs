use paste::paste;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div, Index, IndexMut};

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

/// Define an enum (e.g. Foo) of the provided variants,
/// and define a struct (e.g. FooMap) which is indexed by
/// those enum variants.
macro_rules! define_enum_map {
    ($name:ident { $($field:ident),* }) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum $name {
            $(
                $field,
            )*
        }

        paste! {
            // Define map
            #[derive(Default, Clone, Copy, Debug, PartialEq)]
            pub struct [<$name Map>]<T> {
                $(
                    pub [<$field:snake>]: T,
                )*
            }

            impl<T> [<$name Map>]<T> {
                pub fn keys(&self) -> [$name; count!($($field)*)] {
                    [$(
                        $name::$field,
                    )*]
                }
            }

            impl<T> [<$name Map>]<T> {
                pub fn items(&self) -> [($name, &T); count!($($field)*)] {
                    [$(
                        ($name::$field, &self.[<$field:snake>]),
                    )*]
                }

                pub fn values(&self) -> [&T; count!($($field)*)] {
                    [$(
                        &self.[<$field:snake>],
                    )*]
                }

                pub fn items_mut(&mut self) -> [($name, &mut T); count!($($field)*)] {
                    [$(
                        ($name::$field, &mut self.[<$field:snake>]),
                    )*]
                }
            }

            // Indexing by enum variants
            impl<T> Index<$name> for [<$name Map>]<T> {
                type Output = T;

                fn index(&self, key: $name) -> &Self::Output {
                    match key {
                        $(
                            $name::$field => &self.[<$field:snake>],
                        )*
                    }
                }
            }

            impl<T> IndexMut<$name> for [<$name Map>]<T> {
                fn index_mut(&mut self, key: $name) -> &mut Self::Output {
                    match key {
                        $(
                            $name::$field => &mut self.[<$field:snake>],
                        )*
                    }
                }
            }

            // Map<f32> + Map<f32>
            impl Add for [<$name Map>]<f32> {
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
            impl AddAssign for [<$name Map>]<f32> {
                fn add_assign(&mut self, rhs: Self) {
                    $(
                        self.[<$field:snake>] += rhs.[<$field:snake>];
                    )*
                }
            }

            // Map<f32> - Map<f32>
            impl Sub for [<$name Map>]<f32> {
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
            impl SubAssign for [<$name Map>]<f32> {
                fn sub_assign(&mut self, rhs: Self) {
                    $(
                        self.[<$field:snake>] -= rhs.[<$field:snake>];
                    )*
                }
            }

            // Map * f32
            impl Mul<f32> for [<$name Map>]<f32> {
            // impl<T: Mul + Mul<Output = T>> Mul for [<$name Map>]<T> {
                type Output = Self;

                fn mul(self, rhs: f32) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>] * rhs,
                        )*
                    }
                }
            }

            // Map / f32
            impl Div<f32> for [<$name Map>]<f32> {
                type Output = Self;

                fn div(self, rhs: f32) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: self.[<$field:snake>]/rhs,
                        )*
                    }
                }
            }

            impl Div<[<$name Map>]<f32>> for [<$name Map>]<f32> {
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
    Sun,
    Wind,
    Soil, // Fertile land
    Water,
    Biomass, // Lumber?

    Coal,
    Oil,
    Uranium,
    Lithium,

    // Outputs from other sectors
    Labor,
    Energy,
    Feed,
    Material,
    Co2
});

define_enum_map!(Byproduct {
    Co2,
    Pollution
});


define_enum_map!(Output {
    Fuel,
    Electricity,
    PlantCalories,
    MeatCalories,
    Concrete,
    Steel,
    Project,
    General
});


// Would like to define these as part of the `define_enum_map`
// macro but it looks like nested macros aren't well supported.
/// Macro for quickly creating a maps with default values.
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
