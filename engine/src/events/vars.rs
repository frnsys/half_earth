use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub enum WorldVariable {
    Year,
    Population,
    Emissions,
    ExtinctionRate,
    Outlook,
    Temperature,
    WaterStress,
    SeaLevelRise,
    Precipitation,
    PopulationGrowth,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub enum LocalVariable {
    Population,
    Outlook,
    Habitability,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub enum PlayerVariable {
    PoliticalCapital,
    MalthusianPoints,
    HESPoints,
    FALCPoints,
}
