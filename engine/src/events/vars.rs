use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorldVariable {
    Year,
    Population,
    Emissions,
    ExtinctionRate,
    Outlook,
    Temperature,
    WaterStress,
    SeaLevelRise,
    SeaLevelRiseRate,
    Precipitation,
    PopulationGrowth,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocalVariable {
    Population,
    Outlook,
    Habitability,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerVariable {
    PoliticalCapital,
    ResearchPoints,
    YearsToDeath,
}
