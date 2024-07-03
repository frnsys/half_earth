use std::fmt::Display;

use hes_engine::{
    events::Flag,
    kinds::{Byproduct, Feedstock, Output, Resource},
    production::ProcessFeature,
    regions::{Income, Latitude},
    ProjectType,
};

use super::Var;

pub trait AsText {
    fn lower(&self) -> &'static str;
    fn title(&self) -> &'static str;
}

impl AsText for Resource {
    fn lower(&self) -> &'static str {
        match self {
            Resource::Land => "land",
            Resource::Water => "water",
            Resource::Fuel => "fuel",
            Resource::Electricity => "electricity",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Resource::Land => "Land",
            Resource::Water => "Water",
            Resource::Fuel => "Fuel",
            Resource::Electricity => "Electricity",
        }
    }
}

impl AsText for Income {
    fn lower(&self) -> &'static str {
        match self {
            Income::Low => "low",
            Income::High => "high",
            Income::LowerMiddle => "lower-middle",
            Income::UpperMiddle => "upper-middle",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Income::Low => "Low",
            Income::High => "High",
            Income::LowerMiddle => "Lower-Middle",
            Income::UpperMiddle => "Upper-Middle",
        }
    }
}

impl AsText for Output {
    fn lower(&self) -> &'static str {
        match self {
            Output::Fuel => "fuel",
            Output::Electricity => "electricity",
            Output::AnimalCalories => "animal calories",
            Output::PlantCalories => "plant calories",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Output::Fuel => "Fuel",
            Output::Electricity => "Electricity",
            Output::AnimalCalories => "Animal Calories",
            Output::PlantCalories => "Plant Calories",
        }
    }
}

impl AsText for Feedstock {
    fn lower(&self) -> &'static str {
        match self {
            Feedstock::Coal => "coal",
            Feedstock::Lithium => "lithium",
            Feedstock::NaturalGas => "natural gas",
            Feedstock::Oil => "oil",
            Feedstock::Uranium => "uranium",
            Feedstock::Thorium => "thorium",
            Feedstock::Soil => "soil",
            Feedstock::Other => "other",
        }
    }

    fn title(&self) -> &'static str {
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
    }
}

impl AsText for ProcessFeature {
    fn title(&self) -> &'static str {
        match self {
            ProcessFeature::UsesPesticides => "This process use pesticides.",
            ProcessFeature::UsesSynFertilizer => "This process uses synthetic fertilizers.",
            ProcessFeature::UsesLivestock => "This process uses livestock.",
            ProcessFeature::UsesOil => "This process uses oil.",
            ProcessFeature::IsIntermittent => "This process is intermittent.",
            ProcessFeature::CanMeltdown => "This process can meltdown.",
            ProcessFeature::MakesNuclearWaste => "This process produces nuclear waste.",
            ProcessFeature::IsSolar => "This process relies on the sun.",
            ProcessFeature::IsCCS => "This process captures and stores carbon.",
            ProcessFeature::IsCombustion => "This process involves combustion.",
            ProcessFeature::IsFossil => "This process uses fossil fuels.",
            ProcessFeature::IsLaborIntensive => "This process is especially labor-intensive.",
        }
    }

    fn lower(&self) -> &'static str {
        match self {
            ProcessFeature::IsSolar => "solar processes",
            ProcessFeature::IsIntermittent => "intermittent processes",
            ProcessFeature::CanMeltdown => "processes that may meltdown",
            ProcessFeature::MakesNuclearWaste => "processes that produce nuclear waste",
            ProcessFeature::IsLaborIntensive => "especially labor-intensive processes",
            ProcessFeature::IsCombustion => "combustion processes",
            ProcessFeature::IsFossil => "fossil fuel processes",
            ProcessFeature::UsesOil => "oil processes",
            ProcessFeature::IsCCS => "carbon capture processes",
            ProcessFeature::UsesLivestock => "processes that use livestock",
            ProcessFeature::UsesPesticides => "processes that use pesticides",
            ProcessFeature::UsesSynFertilizer => "processes that use synthetic fertilizers",
        }
    }
}

impl AsText for Latitude {
    fn lower(&self) -> &'static str {
        match self {
            Latitude::Tropic => "tropic",
            Latitude::Subtropic => "subtropic",
            Latitude::Temperate => "temperate",
            Latitude::Frigid => "frigid",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Latitude::Tropic => "Tropic",
            Latitude::Subtropic => "Subtropic",
            Latitude::Temperate => "Temperate",
            Latitude::Frigid => "Frigid",
        }
    }
}

impl AsText for Byproduct {
    fn lower(&self) -> &'static str {
        match self {
            Byproduct::Co2 => "CO2",
            Byproduct::Ch4 => "CH4",
            Byproduct::N2o => "N2O",
            Byproduct::Biodiversity => "biodiversity",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Byproduct::Co2 => "CO2",
            Byproduct::Ch4 => "CH4",
            Byproduct::N2o => "N2O",
            Byproduct::Biodiversity => "Biodiversity",
        }
    }
}

impl AsText for ProjectType {
    fn lower(&self) -> &'static str {
        match self {
            ProjectType::Policy => "policy",
            ProjectType::Research => "research",
            ProjectType::Initiative => "infrastructure",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            ProjectType::Policy => "Policy",
            ProjectType::Research => "Research",
            ProjectType::Initiative => "Infrastructure",
        }
    }
}

impl AsText for Var {
    fn title(&self) -> &'static str {
        match self {
            Var::Land => "Land",
            Var::Water => "Water",
            Var::Energy => "Energy",
            Var::Emissions => "Emissions",
            Var::Biodiversity => "Biodiversity",
            Var::Contentedness => "Contentedness",
            Var::Fuel => "Fuel",
            Var::Electricity => "Electricity",
            Var::PlantCalories => "Plant Calories",
            Var::AnimalCalories => "Animal Calories",
        }
    }

    fn lower(&self) -> &'static str {
        match self {
            Var::Land => "land",
            Var::Water => "water",
            Var::Energy => "energy",
            Var::Emissions => "emissions",
            Var::Biodiversity => "biodiversity",
            Var::Contentedness => "contentedness",
            Var::Fuel => "fuel",
            Var::Electricity => "electricity",
            Var::PlantCalories => "plant calories",
            Var::AnimalCalories => "animal calories",
        }
    }
}
