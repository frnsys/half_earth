use serde::{Deserialize, Serialize};

use super::{
    Flag, LocalVariable, PlayerVariable, WorldVariable,
};
use crate::kinds::{Feedstock, Output, Resource};
use crate::npcs::NPCRelation;
use crate::production::ProcessFeature;
use crate::projects::{Group, Status as ProjectStatus};
use crate::state::State;

const HEAVY_PROJECTS: [Group; 4] = [
    Group::Space,
    Group::Nuclear,
    Group::Geoengineering,
    Group::Electrification,
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Condition {
    LocalVariable(LocalVariable, Comparator, f32),
    WorldVariable(WorldVariable, Comparator, f32),
    PlayerVariable(PlayerVariable, Comparator, f32),
    ProcessOutput(usize, Comparator, f32),
    ProcessMixShare(usize, Comparator, f32),
    ProcessMixShareFeature(ProcessFeature, Comparator, f32),
    ResourcePressure(Resource, Comparator, f32),
    ResourceDemandGap(Resource, Comparator, f32),
    OutputDemandGap(Output, Comparator, f32),
    Demand(Output, Comparator, f32),
    ProjectStatus(usize, ProjectStatus),
    ActiveProjectUpgrades(usize, Comparator, usize),
    RunsPlayed(Comparator, usize),
    RegionFlag(String),
    NPCRelationship(usize, NPCRelation),
    FeedstockYears(Feedstock, Comparator, f32),
    HasFlag(Flag),
    WithoutFlag(Flag),
    HeavyProjects(Comparator, usize),
    ProtectLand(Comparator, f32),
}

impl Condition {
    pub fn eval(
        &self,
        state: &State,
        region_id: Option<usize>,
    ) -> bool {
        match self {
            Condition::LocalVariable(var, comp, other_val) => {
                if let Some(id) = region_id {
                    let region = &state.world.regions[id];
                    let val = match var {
                        LocalVariable::Population => {
                            region.population
                        }
                        LocalVariable::Outlook => {
                            region.outlook
                        }
                        LocalVariable::Habitability => {
                            region.habitability()
                        }
                    };
                    comp.eval(val, *other_val)
                } else {
                    false
                }
            }
            Condition::WorldVariable(var, comp, other_val) => {
                let val = match var {
                    WorldVariable::Year => {
                        state.world.year as f32
                    }
                    WorldVariable::Population => {
                        state.world.population()
                    }
                    WorldVariable::PopulationGrowth => {
                        state.population_growth_modifier
                    }
                    WorldVariable::Emissions => {
                        state.emissions()
                    }
                    WorldVariable::ExtinctionRate => {
                        state.world.extinction_rate
                    }
                    WorldVariable::Outlook => state.outlook(),
                    WorldVariable::Temperature => {
                        state.world.temperature
                    }
                    WorldVariable::WaterStress => {
                        state.water_stress
                    }
                    WorldVariable::SeaLevelRise => {
                        state.world.sea_level_rise
                    }
                    WorldVariable::SeaLevelRiseRate => {
                        state.sea_level_rise_rate()
                    }
                    WorldVariable::Precipitation => {
                        state.precipitation
                    }
                };
                comp.eval(val, *other_val)
            }
            Condition::PlayerVariable(var, comp, other_val) => {
                let val = match var {
                    PlayerVariable::PoliticalCapital => {
                        state.political_capital as f32
                    }
                    PlayerVariable::ResearchPoints => {
                        state.research_points as f32
                    }
                    PlayerVariable::YearsToDeath => {
                        state.death_year as f32
                            - state.world.year as f32
                    }
                };
                comp.eval(val, *other_val)
            }
            Condition::ProcessOutput(id, comp, other_val) => {
                let val = state.produced_by_process[*id];
                comp.eval(val, *other_val)
            }
            Condition::ProcessMixShare(id, comp, other_val) => {
                let val =
                    state.world.processes[*id].mix_percent();
                comp.eval(val, *other_val)
            }
            Condition::ProcessMixShareFeature(
                feat,
                comp,
                other_val,
            ) => {
                let val = state
                    .world
                    .processes
                    .iter()
                    .filter(|p| p.features.contains(feat))
                    .map(|p| p.mix_percent())
                    .sum();
                comp.eval(val, *other_val)
            }
            Condition::ResourcePressure(
                resource,
                comp,
                other_val,
            ) => {
                let val = state.resources[*resource]
                    / state.resources_demand[*resource];
                comp.eval(val, *other_val)
            }
            Condition::ResourceDemandGap(
                resource,
                comp,
                other_val,
            ) => {
                let available = state.resources[*resource];
                let demand = state.resources_demand[*resource];
                let val = (available - demand) / demand;
                comp.eval(val, *other_val)
            }
            Condition::OutputDemandGap(
                output,
                comp,
                other_val,
            ) => {
                let available = state.produced[*output];
                let demand = state.output_demand[*output];
                let val = 1. - (available / demand).min(1.);
                comp.eval(val, *other_val)
            }
            Condition::Demand(output, comp, other_val) => {
                // Apply conversion to OUTPUT_UNITS
                let factor = match output {
                    Output::Fuel => 1e-9 / 1e3, // per 1000 TWh
                    Output::Electricity => 1e-9 / 1e3, // per 1000 TWh
                    Output::PlantCalories => 1e-9 / 2e4, // per 20000 Tcals
                    Output::AnimalCalories => 1e-9 / 2e4, // per 20000 Tcals
                };
                let demand =
                    state.output_demand[*output] * factor;
                comp.eval(demand, *other_val)
            }
            Condition::FeedstockYears(
                feedstock,
                comp,
                other_val,
            ) => comp.eval(
                state.feedstocks[*feedstock]
                    / state.consumed_feedstocks[*feedstock],
                *other_val,
            ),
            Condition::RunsPlayed(comp, runs) => {
                comp.eval(state.runs as f32, *runs as f32)
            }
            Condition::ProjectStatus(id, status) => {
                match status {
                    ProjectStatus::Active
                    | ProjectStatus::Finished => {
                        match state.world.projects[*id].status {
                            ProjectStatus::Active => true,
                            ProjectStatus::Finished => true,
                            _ => false,
                        }
                    }
                    _ => {
                        state.world.projects[*id].status
                            == *status
                    }
                }
            }
            Condition::ActiveProjectUpgrades(
                id,
                comp,
                upgrades,
            ) => comp.eval(
                state.world.projects[*id].level as f32,
                *upgrades as f32,
            ),
            Condition::NPCRelationship(id, relation) => {
                state.npcs[*id].relation() == *relation
            }
            Condition::RegionFlag(flag) => {
                if let Some(id) = region_id {
                    let region = &state.world.regions[id];
                    region.flags.contains(flag)
                } else {
                    false
                }
            }
            Condition::HasFlag(flag) => {
                state.flags.contains(flag)
            }
            Condition::WithoutFlag(flag) => {
                !state.flags.contains(flag)
            }
            Condition::HeavyProjects(comp, n) => {
                let heavy_projects = state
                    .world
                    .projects
                    .iter()
                    .filter(|p| {
                        p.status == ProjectStatus::Finished
                            && HEAVY_PROJECTS.contains(&p.group)
                    })
                    .count();
                comp.eval(heavy_projects as f32, *n as f32)
            }
            Condition::ProtectLand(comp, n) => {
                comp.eval(state.protected_land, *n)
            }
        }
    }
}

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq,
)]
pub enum Comparator {
    Less,
    LessEqual,
    Equal,
    NotEqual,
    GreaterEqual,
    Greater,
}

impl Comparator {
    fn eval(&self, a: f32, b: f32) -> bool {
        match self {
            Comparator::Less => a < b,
            Comparator::LessEqual => a <= b,
            Comparator::Equal => a == b,
            Comparator::NotEqual => a != b,
            Comparator::GreaterEqual => a >= b,
            Comparator::Greater => a > b,
        }
    }
}
