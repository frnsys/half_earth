use enum_map::Enum;
use hes_engine::*;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Impact-related variables.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Impact {
    Land,
    Water,
    Electricity,
    Fuel,
    Energy,
    Emissions,
    Biodiversity,
}
impl Into<Var> for Impact {
    fn into(self) -> Var {
        match self {
            Self::Land => Var::Land,
            Self::Water => Var::Water,
            Self::Electricity => Var::Electricity,
            Self::Fuel => Var::Fuel,
            Self::Energy => Var::Energy,
            Self::Emissions => Var::Emissions,
            Self::Biodiversity => Var::Biodiversity,
        }
    }
}
impl Impact {
    pub fn as_resource(&self) -> Option<Resource> {
        match self {
            Self::Land => Some(Resource::Land),
            Self::Water => Some(Resource::Water),
            Self::Electricity => Some(Resource::Electricity),
            Self::Fuel => Some(Resource::Fuel),
            _ => None,
        }
    }

    pub fn as_output(&self) -> Option<Output> {
        match self {
            Self::Electricity => Some(Output::Electricity),
            Self::Fuel => Some(Output::Fuel),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum OutputKind {
    Energy,
    Calories,
}
impl From<Output> for OutputKind {
    fn from(value: Output) -> Self {
        match value {
            Output::Fuel | Output::Electricity => {
                OutputKind::Energy
            }
            Output::AnimalCalories | Output::PlantCalories => {
                OutputKind::Calories
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Enum,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum Var {
    Land,
    Water,
    Energy,
    Emissions,
    Biodiversity,
    Contentedness,
    Fuel,
    Electricity,
    PlantCalories,
    AnimalCalories,
}
impl From<Resource> for Var {
    fn from(value: Resource) -> Self {
        match value {
            Resource::Land => Var::Land,
            Resource::Water => Var::Water,
            Resource::Fuel => Var::Fuel,
            Resource::Electricity => Var::Electricity,
        }
    }
}
impl From<Output> for Var {
    fn from(value: Output) -> Self {
        match value {
            Output::Fuel => Var::Fuel,
            Output::Electricity => Var::Electricity,
            Output::PlantCalories => Var::PlantCalories,
            Output::AnimalCalories => Var::AnimalCalories,
        }
    }
}
impl Var {
    pub fn is_demand_var(&self) -> bool {
        match self {
            Var::Energy
            | Var::Electricity
            | Var::Fuel
            | Var::PlantCalories
            | Var::AnimalCalories => true,
            _ => false,
        }
    }

    pub fn as_impact(&self) -> Option<Impact> {
        match self {
            Var::Land => Some(Impact::Land),
            Var::Water => Some(Impact::Water),
            Var::Electricity => Some(Impact::Electricity),
            Var::Fuel => Some(Impact::Fuel),
            Var::Energy => Some(Impact::Energy),
            Var::Emissions => Some(Impact::Emissions),
            Var::Biodiversity => Some(Impact::Biodiversity),
            _ => None,
        }
    }

    pub fn as_output(&self) -> Option<Output> {
        match self {
            Var::Electricity => Some(Output::Electricity),
            Var::Fuel => Some(Output::Fuel),
            Var::PlantCalories => Some(Output::PlantCalories),
            Var::AnimalCalories => Some(Output::AnimalCalories),
            _ => None,
        }
    }
}
