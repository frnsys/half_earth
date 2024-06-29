use hes_engine::{kinds::{Feedstock, Output, Resource}, projects::Type};

use crate::display::Var;

macro_rules! icons {
    ($($name:ident: $path:literal),* $(,)?) => {
        $(
            pub const $name: &'static str = concat!("/public/assets", $path);
        )*
    }
}

icons! {
    CLOSE: "/icons/close.svg",
    POLITICAL_CAPITAL: "/icons/pips/political_capital.png",
    EMISSIONS: "/icons/emissions.png",
    WARMING: "/icons/warming.png",
    CONTENTEDNESS: "/icons/contentedness.png",
    EXTINCTION_RATE: "/icons/extinction.png",
    LAND: "/icons/land.png",
    WATER: "/icons/water.png",
    ENERGY: "/icons/energy.png",
    FUEL: "/icons/fuel.png",
    ELECTRICITY: "/icons/electricity.png",
    PLANT_CALORIES: "/icons/plantcalories.png",
    ANIMAL_CALORIES: "/icons/animalcalories.png",
    WEALTH: "/icons/wealth.png",
    POPULATION: "/icons/population.png",
    PRECIPTATION: "/icons/precipitation.svg",
    HABITABILITY: "/icons/habitability.png",
    TEMPERATURE: "/icons/temperature.svg",
    DEVELOPMENT: "/icons/development.png",
    MIX_TOKEN: "/icons/mix_allocation.png",
    ALERT: "/icons/alert.png",
    HELP: "/icons/help.svg",
    PROJECT: "/icons/implement.png",
    DOWN_ARROW_SMALL: "/icons/down_arrow.svg",
    ARROW_RIGHT: "/icons/arrow_right.svg",
    ARROW_LEFT: "/icons/arrow_left.svg",
}

pub trait HasIcon {
    fn icon(&self) -> &'static str;
}

impl HasIcon for Output {
    fn icon(&self) -> &'static str {
        match self {
            Output::Fuel => FUEL,
            Output::Electricity => ELECTRICITY,
            Output::PlantCalories => PLANT_CALORIES,
            Output::AnimalCalories => ANIMAL_CALORIES,
        }
    }
}

impl HasIcon for Var {
    fn icon(&self) -> &'static str {
        match self {
            Var::Land => LAND,
            Var::Water => WATER,
            Var::Energy => ENERGY,
            Var::Emissions => EMISSIONS,
            Var::Biodiversity => EXTINCTION_RATE,
            Var::Contentedness => CONTENTEDNESS,
            Var::Fuel => FUEL,
            Var::Electricity => ELECTRICITY,
            Var::PlantCalories => PLANT_CALORIES,
            Var::AnimalCalories => ANIMAL_CALORIES,
        }
    }
}

impl HasIcon for Resource {
    fn icon(&self) -> &'static str {
        match self {
            Resource::Land => LAND,
            Resource::Water => WATER,
            Resource::Electricity => ELECTRICITY,
            Resource::Fuel => FUEL,
        }
    }
}

impl HasIcon for Feedstock {
    fn icon(&self) -> &'static str {
        match self {
            Feedstock::Coal => "/icons/feedstocks/coal.png",
            Feedstock::Lithium => "/icons/feedstocks/lithium.png",
            Feedstock::NaturalGas => "/icons/feedstocks/naturtl_gas.png",
            Feedstock::Oil => "/icons/feedstocks/oil.png",
            Feedstock::Uranium => "/icons/feedstocks/uranium.png",
            Feedstock::Thorium => "/icons/feedstocks/thorium.png",
            Feedstock::Soil => "/icons/feedstocks/soil.png",
            Feedstock::Other => "/icons/feedstocks/other.png",
        }
    }
}

impl HasIcon for Type {
    fn icon(&self) -> &'static str {
        match self {
            Type::Research => "/icons/pips/research.png",
            Type::Initiative => "/icons/pips/initiative.png",
            Type::Policy => POLITICAL_CAPITAL,
        }
    }
}
