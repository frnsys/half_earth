#[derive(Debug, Copy, Clone)]
pub enum WorldVariable {
    Year,
    Population,
    Emissions,
    Biodiversity,
    Outlook,
    Temperature,
    Contentedness,
    WaterStress,
    SeaLevelRise,
    Precipitation,
    Health,
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
