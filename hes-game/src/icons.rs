use hes_engine::{
    events::{Condition, LocalVariable, WorldVariable},
    kinds::{Byproduct, Feedstock, Output, Resource},
    npcs::NPC,
    production::ProcessFeature,
    projects::Type,
};

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
    PRECIPITATION: "/icons/precipitation.svg",
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
    ARROW_RIGHT_LIGHT: "/icons/arrow_right_light.svg",
    CLOSED_BORDERS: "/icons/ban.png",
    RESEARCH: "/icons/pips/research.png",
    INITIATIVE: "/icons/pips/initiative.png",
    POLICY: "/icons/pips/political_capital.png",
    DEGROWTH: "/icons/degrowth.png",
    OCEAN: "/icons/ocean.png",
    LABOR: "/icons/labor.png",
    BIRB: "/icons/biodiversity.png",
    BIODIVERSITY: "/icons/extinction.png",
    SEA_LEVEL_RISE: "/icons/sea_level_rise.png",
    UNLOCKS: "/icons/unlocks.png",
    PROTECT: "/icons/protect.png",
    CHANCE: "/icons/chance.png",
    COST: "/icons/cost.png",
    REQUEST: "/icons/request.png",
    IMPLEMENT: "/icons/implement.png",
    BAN: "/icons/ban.png",
    DEMAND: "/icons/demand.png",
    ADD: "/icons/add.svg",
    CHECK: "/icons/check.png",
    CHECK_BLK: "/icons/check_blk.png",
    TIME: "/icons/time.svg",
    WARNING: "/icons/warning.svg",
    HALTED: "/icons/halted.png",
    SETTINGS: "/icons/settings.svg",

    HUD_POLITICAL_CAPITAL: "/icons/hud/political_capital.svg",
    HUD_EXTINCTION_RATE: "/icons/hud/extinction.svg",
    HUD_CONTENTEDNESS: "/icons/hud/contentedness.svg",
    HUD_WARMING: "/icons/hud/warming.svg",
    HUD_EMISSIONS: "/icons/hud/emissions.svg",

    // NPC relationships
    RELATIONSHIP: "/icons/relationship.png",
    RELATIONSHIP_EMPTY: "/icons/relationship_empty.png",
    ALLY: "/icons/npcs/ally.svg",
    NEUTRAL: "/icons/npcs/neutral.svg",
    FRIENDLY: "/icons/npcs/friendly.svg",
    NEMESIS: "/icons/npcs/nemesis.svg",

    // Industries
    AVIATION: "/icons/industries/aviation.png",
    BUILDINGS: "/icons/industries/buildings.png",
    CHEMICALS: "/icons/industries/chemicals.png",
    CONCRETE: "/icons/industries/concrete.png",
    IRON_AND_STEEL: "/icons/industries/iron_and_steel.png",
    OTHER_INDUSTRY: "/icons/industries/other_industry.png",
    ROAD_TRANSPORT: "/icons/industries/road_transport.png",
    SHIPPING: "/icons/industries/shipping.png",

    // Characters
    THE_AUTHORITARIAN: "/characters/The Authoritarian.png",
    THE_ECONOMIST: "/characters/The Economist.png",
    THE_ENVIRONMENTALIST: "/characters/The Environmentalist.png",
    THE_SCIENTIST: "/characters/The Scientist.png",
    THE_POPULIST: "/characters/The Populist.png",
    THE_ECOLOGIST: "/characters/The Ecologist.png",
    THE_MALTHUSIAN: "/characters/The Malthusian.png",
    THE_GEOENGINEER: "/characters/The Geoengineer.png",
    THE_POSADIST: "/characters/The Posadist.png",
    THE_WRETCHED: "/characters/The Wretched.png",
    THE_CONSUMERIST: "/characters/The Consumerist.png",
    THE_UTOPIAN: "/characters/The Utopian.png",
    THE_ACCELERATIONIST: "/characters/The Accelerationist.png",
    THE_ANIMAL_LIBERATIONIST: "/characters/The Animal Liberationist.png",
    THE_FARMER: "/characters/The Farmer.png",
    THE_ECOFEMINIST: "/characters/The Ecofeminist.png",
    THE_FANONIST: "/characters/The Fanonist.png",
    THE_PLACEHOLDER: "/characters/placeholder.png",

    // Process features
    IS_CSS: "/icons/features/is_ccs.png",
    IS_COMBUSTION: "/icons/features/is_combustion.png",
    IS_INTERMITTENT: "/icons/features/is_intermittent.png",
    MAKES_NUCLEAR_WASTE: "/icons/features/makes_nuclear_waste.png",
    CAN_MELTDOWN: "/icons/features/can_meltdown.png",
    IS_LABOR_INTENSIVE: "/icons/features/is_labor_intensive.png",
    IS_SOLAR: "/icons/features/is_solar.png",
    IS_FOSSIL: "/icons/features/is_fossil.png",
    USES_OIL: "/icons/feedstocks/oil.png",
    USES_LIVESTOCK: "/icons/features/uses_livestock.png",
    USES_PESTICIDES: "/icons/features/uses_pesticides.png",
    USES_SYN_FERTILIZER: "/icons/features/uses_syn_fertilizer.png",
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
            Type::Research => RESEARCH,
            Type::Initiative => INITIATIVE,
            Type::Policy => POLITICAL_CAPITAL,
        }
    }
}

impl HasIcon for ProcessFeature {
    fn icon(&self) -> &'static str {
        match self {
            ProcessFeature::UsesPesticides => USES_PESTICIDES,
            ProcessFeature::UsesSynFertilizer => USES_SYN_FERTILIZER,
            ProcessFeature::UsesLivestock => USES_LIVESTOCK,
            ProcessFeature::UsesOil => USES_OIL,
            ProcessFeature::IsIntermittent => IS_INTERMITTENT,
            ProcessFeature::CanMeltdown => CAN_MELTDOWN,
            ProcessFeature::MakesNuclearWaste => MAKES_NUCLEAR_WASTE,
            ProcessFeature::IsSolar => IS_SOLAR,
            ProcessFeature::IsCCS => IS_CSS,
            ProcessFeature::IsCombustion => IS_COMBUSTION,
            ProcessFeature::IsFossil => IS_FOSSIL,
            ProcessFeature::IsLaborIntensive => IS_LABOR_INTENSIVE,
        }
    }
}

impl HasIcon for Byproduct {
    fn icon(&self) -> &'static str {
        match self {
            Byproduct::Biodiversity => BIODIVERSITY,
            _ => EMISSIONS,
        }
    }
}

impl HasIcon for NPC {
    fn icon(&self) -> &'static str {
        match self.name.as_str() {
            "The Authoritarian" => THE_AUTHORITARIAN,
            "The Economist" => THE_ECONOMIST,
            "The Environmentalist" => THE_ENVIRONMENTALIST,
            "The Scientist" => THE_SCIENTIST,
            "The Populist" => THE_POPULIST,
            "The Ecologist" => THE_ECOLOGIST,
            "The Malthusian" => THE_MALTHUSIAN,
            "The Geoengineer" => THE_GEOENGINEER,
            "The Posadist" => THE_POSADIST,
            "The Wretched" => THE_WRETCHED,
            "The Consumerist" => THE_CONSUMERIST,
            "The Utopian" => THE_UTOPIAN,
            "The Accelerationist" => THE_ACCELERATIONIST,
            "The Animal Liberationist" => THE_ANIMAL_LIBERATIONIST,
            "The Farmer" => THE_FARMER,
            "The Ecofeminist" => THE_ECOFEMINIST,
            "The Fanonist" => THE_FANONIST,
            _ => THE_PLACEHOLDER,
        }
    }
}

impl HasIcon for Condition {
    fn icon(&self) -> &'static str {
        match self {
            Condition::Demand(output, ..) => output.icon(),
            Condition::OutputDemandGap(output, ..) => output.icon(),
            Condition::ResourceDemandGap(resource, ..) => resource.icon(),
            Condition::ResourcePressure(resource, ..) => resource.icon(),
            Condition::ProcessMixShareFeature(feat, ..) => feat.icon(),
            Condition::FeedstockYears(feedstock, ..) => feedstock.icon(),
            Condition::LocalVariable(var, ..) => match var {
                LocalVariable::Outlook => CONTENTEDNESS,
                LocalVariable::Habitability => HABITABILITY,
                LocalVariable::Population => POPULATION,
            },
            Condition::WorldVariable(var, ..) => match var {
                WorldVariable::Temperature => WARMING,
                WorldVariable::SeaLevelRise => SEA_LEVEL_RISE,
                WorldVariable::SeaLevelRiseRate => SEA_LEVEL_RISE,
                WorldVariable::Outlook => CONTENTEDNESS,
                WorldVariable::Emissions => EMISSIONS,
                WorldVariable::WaterStress => WATER,
                WorldVariable::Precipitation => PRECIPITATION,
                WorldVariable::Population => POPULATION,
                WorldVariable::PopulationGrowth => POPULATION,
                WorldVariable::ExtinctionRate => EXTINCTION_RATE,
                _ => HELP,
            },
            Condition::ProtectLand(..) => PROTECT,
            _ => HELP,
        }
    }
}
