use std::cmp::Ordering;

use hes_engine::*;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ending {
    Win,
    Died,
    Coup,
    LostOther,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, EnumIter,
)]
pub enum Badge {
    Seceded,
    Aliens,
    Biodiversity,
    Electrification,
    Extinction,
    FossilFuels,
    Meat,
    Nuclear,
    Renewables,
    Space,
    Vegan,
}
impl Badge {
    fn applies(&self, state: &State) -> bool {
        match self {
            Self::Seceded => state
                .world
                .regions
                .iter()
                .any(|reg| reg.seceded),
            Self::Aliens => {
                state.flags.contains(&Flag::AlienEncounter)
            }
            Self::Biodiversity => {
                state.world.extinction_rate <= 15.
            }
            Self::Extinction => {
                state.world.extinction_rate >= 60.
            }
            Self::Electrification => {
                state.world.projects.iter().any(|proj| {
                    proj.name == "Mass Electrification"
                        && (proj.status == Status::Finished
                            || proj.status == Status::Active)
                })
            }
            Self::FossilFuels => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| {
                        proc.features
                            .contains(&ProcessFeature::IsFossil)
                    })
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    > 0
            }
            Self::Meat => {
                // Animal calories demand at least 80% of starting value
                state.output_demand.of(Output::AnimalCalories)
                    >= 2e15
            }
            Self::Nuclear => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| {
                        proc.features.contains(
                            &ProcessFeature::CanMeltdown,
                        ) || proc.features.contains(
                            &ProcessFeature::MakesNuclearWaste,
                        )
                    })
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    >= 10
            }
            Self::Renewables => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| {
                        proc.features.contains(
                            &ProcessFeature::IsIntermittent,
                        )
                    })
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    >= 10
            }
            Self::Space => {
                state
                    .world
                    .projects
                    .iter()
                    .map(|proj| {
                        proj.group == Group::Space
                            && (proj.status == Status::Finished
                                || proj.status
                                    == Status::Active)
                    })
                    .count()
                    >= 3
            }
            Self::Vegan => {
                // Animal calories demand down to less than 10% of starting val
                state.output_demand.of(Output::AnimalCalories)
                    < 2e14
            }
        }
    }

    pub fn image_url(&self) -> String {
        let name = match self {
            Self::Seceded => "seceded",
            Self::Aliens => "aliens",
            Self::Biodiversity => "biodiversity",
            Self::Electrification => "electrification",
            Self::Extinction => "extinction",
            Self::FossilFuels => "fossil_fuels",
            Self::Meat => "meat",
            Self::Nuclear => "nuclear",
            Self::Renewables => "renewables",
            Self::Space => "space",
            Self::Vegan => "vegan",
        };
        format!("/assets/badges/{}.png", name)
    }
}
impl std::fmt::Display for Badge {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let desc = match self {
            Self::Seceded => {
                "At least one region seceded from Gosplant."
            }
            Self::Aliens => {
                "You had an extraterrestrial encounter."
            }
            Self::Biodiversity => {
                "Planetary life flourished under your tenure."
            }
            Self::Electrification => {
                "You helped electrify the world."
            }
            Self::Extinction => {
                "Planetary life suffered under your tenure."
            }
            Self::FossilFuels => {
                "You kept on using fossil fuels."
            }
            Self::Meat => "Carnivorous diets were left intact.",
            Self::Nuclear => {
                "Nuclear was your preferred form of energy."
            }
            Self::Renewables => {
                "Renewables dominated energy production."
            }
            Self::Space => {
                "You pushed humanity towards the stars."
            }
            Self::Vegan => {
                "Global diets shifted towards vegan."
            }
        };
        write!(f, "{}", desc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub ending: Ending,
    pub faction: String,
    pub badges: Vec<Badge>,
}

pub fn eval_badges(state: &State) -> Vec<Badge> {
    Badge::iter().filter(|badge| badge.applies(state)).collect()
}

pub fn summarize(state: &State, win: bool) -> Summary {
    let badges = eval_badges(state);
    let closest = state
        .npcs
        .iter()
        .max_by(|x, y| {
            x.relationship
                .partial_cmp(&y.relationship)
                .unwrap_or(Ordering::Equal)
        })
        .unwrap();
    let faction = closest.name.to_string();

    Summary {
        badges,
        faction,
        ending: if win {
            Ending::Win
        } else {
            if state.world.year >= state.death_year {
                Ending::Died
            } else if state.political_capital <= 0 {
                Ending::Coup
            } else {
                Ending::LostOther
            }
        },
    }
}
