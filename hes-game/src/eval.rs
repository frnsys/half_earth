use hes_engine::{
    events::Flag,
    kinds::Output,
    production::ProcessFeature,
    projects::{Group, Status},
    state::State,
};
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
                state.demand_for_output(&Output::AnimalCalories)
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
                state.demand_for_output(&Output::AnimalCalories)
                    < 2e14
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub ending: Ending,
    pub faction: String,
    pub badges: Vec<Badge>,
}

pub fn summarize(state: &State, win: bool) -> Summary {
    let badges: Vec<_> = Badge::iter()
        .filter(|badge| badge.applies(state))
        .collect();

    let closest = state
        .npcs
        .iter()
        .max_by(|x, y| {
            x.relationship.partial_cmp(&y.relationship).unwrap()
        })
        .unwrap();
    let faction = format!("{}s", closest.name);

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
