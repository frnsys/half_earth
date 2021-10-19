#[derive(Debug, Copy, Clone)]
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
}

#[derive(Debug, Copy, Clone)]
pub enum LocalVariable {
    Population,
    Outlook,
    Habitability,
}

#[derive(Debug, Copy, Clone)]
pub enum PlayerVariable {
    PoliticalCapital,
}
