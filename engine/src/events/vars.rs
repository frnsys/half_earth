use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    IntoStaticStr,
    EnumIter,
    EnumString,
    Display,
)]
pub enum WorldVariable {
    Year,
    Population,
    Emissions,
    ExtinctionRate,
    Outlook,
    Temperature,
    SeaLevelRise,
    SeaLevelRiseRate,
    Precipitation,
    PopulationGrowth,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    IntoStaticStr,
    EnumIter,
    EnumString,
    Display,
)]
pub enum LocalVariable {
    Population,
    Outlook,
    Habitability,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    IntoStaticStr,
    EnumIter,
    EnumString,
    Display,
)]
pub enum PlayerVariable {
    PoliticalCapital,
    ResearchPoints,
    YearsToDeath,
}
