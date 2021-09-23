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
                    pub [<$field:lower>]: T,
                )*
            }

            impl<T> [<$name Map>]<T> {
                pub fn keys(&self) -> [$name; count!($($field)*)] {
                    [$(
                        $name::$field,
                    )*]
                }
            }

            impl<T: Copy> [<$name Map>]<T> {
                pub fn items(&self) -> [($name, T); count!($($field)*)] {
                    [$(
                        ($name::$field, self.[<$field:lower>]),
                    )*]
                }

                pub fn values(&self) -> [T; count!($($field)*)] {
                    [$(
                        self.[<$field:lower>],
                    )*]
                }
            }

            // Indexing by enum variants
            impl<T> Index<$name> for [<$name Map>]<T> {
                type Output = T;

                fn index(&self, key: $name) -> &Self::Output {
                    match key {
                        $(
                            $name::$field => &self.[<$field:lower>],
                        )*
                    }
                }
            }

            impl<T> IndexMut<$name> for [<$name Map>]<T> {
                fn index_mut(&mut self, key: $name) -> &mut Self::Output {
                    match key {
                        $(
                            $name::$field => &mut self.[<$field:lower>],
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
                            [<$field:lower>]: self.[<$field:lower>] + rhs.[<$field:lower>],
                        )*
                    }
                }
            }

            // Map<f32> += Map<f32>
            impl AddAssign for [<$name Map>]<f32> {
                fn add_assign(&mut self, rhs: Self) {
                    $(
                        self.[<$field:lower>] += rhs.[<$field:lower>];
                    )*
                }
            }

            // Map<f32> - Map<f32>
            impl Sub for [<$name Map>]<f32> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self {
                    Self {
                        $(
                            [<$field:lower>]: self.[<$field:lower>] - rhs.[<$field:lower>],
                        )*
                    }
                }
            }

            // Map<f32> += Map<f32>
            impl SubAssign for [<$name Map>]<f32> {
                fn sub_assign(&mut self, rhs: Self) {
                    $(
                        self.[<$field:lower>] -= rhs.[<$field:lower>];
                    )*
                }
            }

            // Map * f32
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

            // Map / f32
            impl Div<f32> for [<$name Map>]<f32> {
                type Output = Self;

                fn div(self, rhs: f32) -> Self {
                    Self {
                        $(
                            [<$field:lower>]: self.[<$field:lower>]/rhs,
                        )*
                    }
                }
            }
        }
    }
}

define_enum_map!(Resource {
    Land,
    Energy,
    Material,
    Sun,
    Wind,
    Soil, // Fertile land
    Water,
    Labor
});

define_enum_map!(Byproduct {
    CO2,
    Pollution
});

define_enum_map!(Sector {
    Agriculture,        // Calories
    Materials,          // Tons
    Energy,             // MWh
    Water               // ?
});

// TODO would like to define these as part of the `define_enum_map`
// macro but it looks like nested macros aren't well supported?
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
macro_rules! sectors {
    () => {
        SectorMap::default()
    };
    ($($field:ident: $value:expr),*) => {
        {
            let mut map = SectorMap::default();
            $(
                map.$field = $value;
            )*
            map
        }
    };
}
