#[derive(Debug, Copy, Clone)]
pub enum WorldVariable {
    Year,
    Population,
    Emissions,
    ExtinctionRate,
    Health,
    Outlook,
    Temperature,
    Contentedness,
    WaterStress,
    SeaLevelRise,
    Precipitation,
}

#[derive(Debug, Copy, Clone)]
pub enum LocalVariable {
    Population,
    Health,
    Outlook,
    Contentedness,
    Habitability,
}

#[derive(Debug, Copy, Clone)]
pub enum PlayerVariable {
    PoliticalCapital,
}
