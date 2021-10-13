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
    BaseHabitability,
}

#[derive(Debug, Copy, Clone)]
pub enum PlayerVariable {
    PoliticalCapital,
}
