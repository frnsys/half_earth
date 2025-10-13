use enum_map::Enum;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{
        Add,
        AddAssign,
        Div,
        Index,
        IndexMut,
        Mul,
        MulAssign,
        Sub,
        SubAssign,
    },
};
use strum::{EnumIter, EnumString, IntoStaticStr};

pub trait KindMap<const SIZE: usize>:
    Index<Self::Key, Output = f32>
    + IndexMut<Self::Key>
    + Add<Output = Self>
    + Mul<Output = Self>
    + SubAssign
    + Copy
    + Default
{
    type Key: Copy + Display;

    fn splat(val: f32) -> Self;
    fn keys(&self) -> [Self::Key; SIZE];
    fn items(&self) -> [(Self::Key, f32); SIZE];
    fn items_mut(&mut self) -> [(Self::Key, &mut f32); SIZE];
    fn values(&self) -> [&f32; SIZE];
    fn values_mut(&mut self) -> [&mut f32; SIZE];
}

/// A consumable, exhaustible supply of something.
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Default, Debug,
)]
pub struct Reserve<M: KindMap<N>, const N: usize> {
    /// Current un-consumed stock.
    pub available: M,

    /// Actual annual consumption.
    pub consumed: M,

    /// Total production requirement for the resource,
    /// which may be more than what's available.
    pub required: M,
}
impl<M: KindMap<N>, const N: usize> Index<M::Key>
    for Reserve<M, N>
{
    type Output = f32;
    fn index(&self, index: M::Key) -> &Self::Output {
        &self.available[index]
    }
}
impl<M: KindMap<N>, const N: usize> From<M> for Reserve<M, N> {
    fn from(value: M) -> Self {
        Self {
            available: value,
            ..Default::default()
        }
    }
}
impl<M: KindMap<N>, const N: usize> Reserve<M, N> {
    pub fn until_exhaustion(&self, key: M::Key) -> f32 {
        if self.available[key] == 0. {
            0.
        } else {
            self.available[key] / self.consumed[key]
        }
    }

    /// Apply annual consumption.
    pub fn consume(&mut self, consumed: M) {
        self.consumed = consumed;

        // Float imprecision sometimes causes these values
        // to be slightly negative, so ensure they aren't
        self.available -= consumed;
        for val in self.available.values_mut() {
            *val = val.max(0.);
        }
    }

    /// Weigh resources by scarcity;
    /// higher weight = higher scarcity.
    pub fn scarcity(&self) -> M {
        let mut weights = M::default();
        for (k, v) in self.required.items() {
            weights[k] = f32::min(
                f32::max(v / self.available[k], 0.),
                1.,
            );
        }
        weights
    }

    pub fn shortage_of(&self, key: M::Key) -> f32 {
        self.required[key] - self.available[key]
    }

    pub fn has_shortage(&self, key: M::Key) -> bool {
        self.shortage_of(key) > 0.
    }
}

/// A map that can be modified by factors (multiplication)
/// and modifiers (addition).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Modifiable<M: KindMap<N>, const N: usize> {
    pub base: M,
    pub factor: M,
    pub modifier: M,
}
impl<M: KindMap<N>, const N: usize> Default
    for Modifiable<M, N>
{
    fn default() -> Self {
        Self {
            base: M::default(),
            factor: M::splat(1.),
            modifier: M::default(),
        }
    }
}
impl<M: KindMap<N>, const N: usize> Modifiable<M, N> {
    pub fn total(&self) -> M {
        (self.base + self.modifier) * self.factor
    }

    pub fn of(&self, key: M::Key) -> f32 {
        self.total()[key]
    }
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

/// Define an enum (e.g. Foo) of the provided variants,
/// and define a struct (e.g. FooMap) which is indexed by
/// those enum variants.
macro_rules! define_enum_map {
    ($name:ident { $($field:ident),* }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Enum, EnumIter, EnumString, IntoStaticStr)]
        pub enum $name {
            $(
                $field,
            )*
        }

        paste! {
            #[derive(Default, Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
            pub struct [<$name Map>] {
                $(
                    pub [<$field:snake>]: f32,
                )*
            }

            impl [<$name Map>] {
                const N: usize = count!($($field)*);

                pub fn values(&self) -> [f32; count!($($field)*)] {
                    [$(
                        self.[<$field:snake>],
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

            impl KindMap<{count!($($field)*)}> for [<$name Map>] {
                type Key = $name;

                fn splat(val: f32) -> Self {
                    Self {
                        $(
                            [<$field:snake>]: val,
                        )*
                    }
                }

                fn keys(&self) -> [Self::Key; count!($($field)*)] {
                    [$(
                        $name::$field,
                    )*]
                }

                fn values(&self) -> [&f32; count!($($field)*)] {
                    [$(
                        &self.[<$field:snake>],
                    )*]
                }

                fn values_mut(&mut self) -> [&mut f32; count!($($field)*)] {
                    [$(
                        &mut self.[<$field:snake>],
                    )*]
                }

                fn items(&self) -> [(Self::Key, f32); count!($($field)*)] {
                    [$(
                        ($name::$field, self.[<$field:snake>]),
                    )*]
                }

                fn items_mut(&mut self) -> [(Self::Key, &mut f32); count!($($field)*)] {
                    [$(
                        ($name::$field, &mut self.[<$field:snake>]),
                    )*]
                }
            }

            impl crate::Diff for [<$name Map>] {
                fn diff(&self, other: &Self) -> Vec<crate::Change> {
                    self.items()
                        .iter()
                        .zip(other.values())
                        .filter_map(|((key, a), b)| {
                            (*a != b).then(|| {
                                crate::Change::Simple(format!(
                                        "{}: {:.2} -> {:.2}",
                                        key, a, b
                                ))
                            })
                        })
                    .collect()
                }
            }
        }
    }
}

// Hacky to use consts like this, there are nicer ways in theory
// but the compiler doesn't yet support them.
pub type Resources = Reserve<ResourceMap, { ResourceMap::N }>;
pub type Feedstocks =
    Reserve<FeedstockMap, { FeedstockMap::N }>;
pub type OutputDemand = Modifiable<OutputMap, { OutputMap::N }>;
pub type ResourceDemand =
    Modifiable<ResourceMap, { ResourceMap::N }>;
pub type Byproducts =
    Modifiable<ByproductMap, { ByproductMap::N }>;

define_enum_map!(Resource {
    Land,
    Water,
    Electricity,
    Fuel
});

impl Resource {
    pub fn as_output(&self) -> Option<Output> {
        match self {
            Resource::Electricity => Some(Output::Electricity),
            Resource::Fuel => Some(Output::Fuel),
            _ => None,
        }
    }
}

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
        crate::kinds::OutputMap::default()
    };
    ($($field:ident: $value:expr),*) => {
        {
            let mut map = crate::kinds::OutputMap::default();
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

    pub fn short_units(&self) -> OutputMap {
        outputs!(
            fuel: to_energy_units(self.fuel),
            electricity: to_energy_units(self.electricity),
            animal_calories: to_calorie_units(self.animal_calories),
            plant_calories: to_calorie_units(self.plant_calories)
        )
    }
}

/// Per 20000 Tcals
fn to_calorie_units(amount: f32) -> f32 {
    amount * (1e-9 / 2e4)
}

/// Per PWh
fn to_energy_units(amount: f32) -> f32 {
    amount * 1e-12
}

fn g_to_megatons(amount: f32) -> f32 {
    amount * 1e-12
}

fn l_to_million_megaliters(amount: f32) -> f32 {
    amount * 1e-12
}

fn m2_to_million_km2(amount: f32) -> f32 {
    amount * 1e-12
}

impl ResourceMap {
    pub fn energy(&self) -> f32 {
        self.electricity + self.fuel
    }

    pub fn short_units(&self) -> ResourceMap {
        resources!(
            fuel: to_energy_units(self.fuel),
            electricity: to_energy_units(self.electricity),
            land: m2_to_million_km2(self.land),
            water: l_to_million_megaliters(self.water)
        )
    }
}

impl FeedstockMap {
    pub fn short_units(&self) -> FeedstockMap {
        feedstocks!(
            coal: g_to_megatons(self.coal),
            thorium: g_to_megatons(self.thorium),
            uranium: g_to_megatons(self.uranium),
            lithium: g_to_megatons(self.lithium),
            oil: l_to_million_megaliters(self.oil),
            natural_gas: l_to_million_megaliters(self.natural_gas)
        )
    }
}

impl Display for Output {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fuel => "Fuel",
                Self::Electricity => "Electricity",
                Self::PlantCalories => "Plant Calories",
                Self::AnimalCalories => "Animal Calories",
            }
        )
    }
}
impl Default for Output {
    fn default() -> Self {
        Self::Fuel
    }
}

impl Display for Feedstock {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Feedstock::Coal => "Coal",
                Feedstock::Lithium => "Lithium",
                Feedstock::NaturalGas => "Natural Gas",
                Feedstock::Oil => "Oil",
                Feedstock::Uranium => "Uranium",
                Feedstock::Thorium => "Thorium",
                Feedstock::Soil => "Soil",
                Feedstock::Other => "Other",
            }
        )
    }
}

impl Default for Feedstock {
    fn default() -> Self {
        Self::Other
    }
}

impl Display for Resource {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Resource::Land => "Land",
                Resource::Water => "Water",
                Resource::Fuel => "Fuel",
                Resource::Electricity => "Electricity",
            }
        )
    }
}

impl Display for Byproduct {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Byproduct::Co2 => "CO2",
                Byproduct::N2o => "N2O",
                Byproduct::Ch4 => "CH4 (Methane)",
                Byproduct::Biodiversity =>
                    "Biodiversity Pressure",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedstock_exhaustion_estimate() {
        let feedstocks = feedstocks!(coal: 100.);
        let mut reserves = Reserve::from(feedstocks);
        assert_eq!(reserves.available.coal, 100.);

        // No consumption = lasts indefinitely
        reserves.consumed.coal = 0.;
        let estimate =
            reserves.until_exhaustion(Feedstock::Coal);
        assert!(estimate.is_infinite());

        // Consumption = lasts according to rate of consumption
        reserves.consumed.coal = 10.;
        let estimate =
            reserves.until_exhaustion(Feedstock::Coal);
        assert_eq!(estimate, 10.);

        reserves.available.coal = 10.;
        let estimate =
            reserves.until_exhaustion(Feedstock::Coal);
        assert_eq!(estimate, 1.);

        // Exhausted
        reserves.consumed.coal = 10.;
        reserves.available.coal = 0.;
        let estimate =
            reserves.until_exhaustion(Feedstock::Coal);
        assert_eq!(estimate, 0.);

        // Exhausted, even if no consumption
        reserves.consumed.coal = 0.;
        reserves.available.coal = 0.;
        let estimate =
            reserves.until_exhaustion(Feedstock::Coal);
        assert_eq!(estimate, 0.);
    }
}
