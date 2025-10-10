use crate::vars::Var;
use egui::Color32;
use hes_engine::*;

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
            ProcessFeature::UsesPesticides => {
                "This process use pesticides."
            }
            ProcessFeature::UsesSynFertilizer => {
                "This process uses synthetic fertilizers."
            }
            ProcessFeature::UsesLivestock => {
                "This process uses livestock."
            }
            ProcessFeature::UsesOil => "This process uses oil.",
            ProcessFeature::IsIntermittent => {
                "This process is intermittent."
            }
            ProcessFeature::CanMeltdown => {
                "This process can meltdown."
            }
            ProcessFeature::MakesNuclearWaste => {
                "This process produces nuclear waste."
            }
            ProcessFeature::IsSolar => {
                "This process relies on the sun."
            }
            ProcessFeature::IsCCS => {
                "This process captures and stores carbon."
            }
            ProcessFeature::IsCombustion => {
                "This process involves combustion."
            }
            ProcessFeature::IsFossil => {
                "This process uses fossil fuels."
            }
            ProcessFeature::IsLaborIntensive => {
                "This process is especially labor-intensive."
            }
        }
    }

    fn lower(&self) -> &'static str {
        match self {
            ProcessFeature::IsSolar => "solar processes",
            ProcessFeature::IsIntermittent => {
                "intermittent processes"
            }
            ProcessFeature::CanMeltdown => {
                "processes that may meltdown"
            }
            ProcessFeature::MakesNuclearWaste => {
                "processes that produce nuclear waste"
            }
            ProcessFeature::IsLaborIntensive => {
                "especially labor-intensive processes"
            }
            ProcessFeature::IsCombustion => {
                "combustion processes"
            }
            ProcessFeature::IsFossil => "fossil fuel processes",
            ProcessFeature::UsesOil => "oil processes",
            ProcessFeature::IsCCS => "carbon capture processes",
            ProcessFeature::UsesLivestock => {
                "processes that use livestock"
            }
            ProcessFeature::UsesPesticides => {
                "processes that use pesticides"
            }
            ProcessFeature::UsesSynFertilizer => {
                "processes that use synthetic fertilizers"
            }
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

pub fn group_color(group: &Group) -> (Color32, Color32) {
    match group {
        Group::Restoration => (
            Color32::from_rgb(0x24, 0x7f, 0x24),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Protection => (
            Color32::from_rgb(0x53, 0xa5, 0x53),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Nuclear => (
            Color32::from_rgb(0xff, 0xa5, 0x00),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Agriculture => (
            Color32::from_rgb(0xf5, 0xde, 0xb3),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Control => (
            Color32::from_rgb(0xd8, 0x35, 0x35),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Population => (
            Color32::from_rgb(0x6b, 0x6b, 0xec),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Food => (
            Color32::from_rgb(0xf3, 0xff, 0x56),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Space => (
            Color32::from_rgb(0x25, 0x04, 0x41),
            Color32::from_rgb(0xd0, 0xc0, 0xe4),
        ),
        Group::Geoengineering => (
            Color32::from_rgb(0x61, 0x68, 0x8b),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Electrification => (
            Color32::from_rgb(0xfc, 0xba, 0x03),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Behavior => (
            Color32::from_rgb(0xb8, 0xad, 0x91),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Limits => (
            Color32::from_rgb(0x4B, 0x5A, 0x85),
            Color32::from_rgb(0xff, 0xff, 0xff),
        ),
        Group::Energy => (
            Color32::from_rgb(0xfe, 0xe9, 0x4a),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Materials => (
            Color32::from_rgb(0x5f, 0x29, 0x29),
            Color32::from_rgb(0xff, 0xff, 0xff),
        ),
        Group::Buildings => (
            Color32::from_rgb(0x8f, 0x7e, 0xa9),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Cities => (
            Color32::from_rgb(0x56, 0x6b, 0x6a),
            Color32::from_rgb(0xff, 0xff, 0xff),
        ),
        Group::Other => (
            Color32::from_rgb(0xe0, 0xe0, 0xe0),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
    }
}
