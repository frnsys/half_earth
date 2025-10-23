use serde::{Deserialize, Serialize};

use super::{Flag, LocalVariable, PlayerVariable, RegionFlag, WorldVariable};
use crate::{
    Id,
    kinds::{Feedstock, Output, Resource},
    npcs::NPCRelation,
    production::ProcessFeature,
    projects::{Group, Status as ProjectStatus},
    state::State,
};
use strum::{Display, EnumDiscriminants, EnumIter, EnumString, IntoStaticStr};

const HEAVY_PROJECTS: [Group; 4] = [
    Group::Space,
    Group::Nuclear,
    Group::Geoengineering,
    Group::Electrification,
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter, EnumString, IntoStaticStr, Display))]
#[strum_discriminants(name(ConditionKind))]
pub enum Condition {
    LocalVariable(LocalVariable, Comparator, f32),
    WorldVariable(WorldVariable, Comparator, f32),
    PlayerVariable(PlayerVariable, Comparator, f32),
    ProcessOutput(Id, Comparator, f32),
    ProcessMixShare(Id, Comparator, f32),
    ProcessMixShareFeature(ProcessFeature, Comparator, f32),
    ResourcePressure(Resource, Comparator, f32),
    ResourceDemandGap(Resource, Comparator, f32),
    OutputDemandGap(Output, Comparator, f32),
    Demand(Output, Comparator, f32),
    ProjectStatus(Id, ProjectStatus),
    ActiveProjectUpgrades(Id, Comparator, usize),
    RunsPlayed(Comparator, usize),
    RegionFlag(RegionFlag),
    NPCRelationship(Id, NPCRelation),
    FeedstockYears(Feedstock, Comparator, f32),
    HasFlag(Flag),
    WithoutFlag(Flag),
    HeavyProjects(Comparator, usize),
    ProtectLand(Comparator, f32),
    WaterStress(Comparator, f32),
}

impl Condition {
    pub fn from_kind(
        kind: ConditionKind,
        default_process: Id,
        default_project: Id,
        default_npc: Id,
    ) -> Self {
        let comp = Comparator::GreaterEqual;
        match kind {
            ConditionKind::LocalVariable => Self::LocalVariable(LocalVariable::Outlook, comp, 0.),
            ConditionKind::WorldVariable => Self::WorldVariable(WorldVariable::Outlook, comp, 0.),
            ConditionKind::PlayerVariable => {
                Self::PlayerVariable(PlayerVariable::PoliticalCapital, comp, 0.)
            }
            ConditionKind::ProcessOutput => Self::ProcessOutput(default_process, comp, 0.),
            ConditionKind::ProcessMixShare => Self::ProcessMixShare(default_process, comp, 0.),
            ConditionKind::ProcessMixShareFeature => {
                Self::ProcessMixShareFeature(ProcessFeature::IsCCS, comp, 0.)
            }
            ConditionKind::ResourcePressure => Self::ResourcePressure(Resource::Land, comp, 0.),
            ConditionKind::ResourceDemandGap => Self::ResourceDemandGap(Resource::Land, comp, 0.),
            ConditionKind::OutputDemandGap => Self::OutputDemandGap(Output::Fuel, comp, 0.),
            ConditionKind::Demand => Self::Demand(Output::Fuel, comp, 0.),
            ConditionKind::ProjectStatus => {
                Self::ProjectStatus(default_project, ProjectStatus::Active)
            }
            ConditionKind::ActiveProjectUpgrades => {
                Self::ActiveProjectUpgrades(default_project, comp, 1)
            }
            ConditionKind::RunsPlayed => Self::RunsPlayed(comp, 1),
            ConditionKind::RegionFlag => Self::RegionFlag(RegionFlag::Protests),
            ConditionKind::NPCRelationship => Self::NPCRelationship(default_npc, NPCRelation::Ally),
            ConditionKind::FeedstockYears => Self::FeedstockYears(Feedstock::Coal, comp, 0.),
            ConditionKind::HasFlag => Self::HasFlag(Flag::Vegan),
            ConditionKind::WithoutFlag => Self::WithoutFlag(Flag::Vegan),
            ConditionKind::HeavyProjects => Self::HeavyProjects(comp, 1),
            ConditionKind::ProtectLand => Self::ProtectLand(comp, 0.),
            ConditionKind::WaterStress => Self::WaterStress(comp, 0.),
        }
    }

    pub fn process_id(&self) -> Option<Id> {
        match self {
            Condition::ProcessOutput(id, ..) | Condition::ProcessMixShare(id, ..) => Some(*id),
            _ => None,
        }
    }

    pub fn project_id(&self) -> Option<Id> {
        match self {
            Condition::ProjectStatus(id, ..) | Condition::ActiveProjectUpgrades(id, ..) => {
                Some(*id)
            }
            _ => None,
        }
    }
}

impl Condition {
    /// If this condition has any regional conditions.
    pub fn is_regional(&self) -> bool {
        matches!(self, Self::LocalVariable(..) | Self::RegionFlag(..))
    }

    pub fn eval(&self, state: &State, region_id: Option<Id>) -> bool {
        match self {
            Condition::LocalVariable(var, comp, other_val) => {
                if let Some(id) = &region_id {
                    let region = &state.world.regions[id];
                    let val = match var {
                        LocalVariable::Population => region.population,
                        LocalVariable::Outlook => region.outlook,
                        LocalVariable::Habitability => region.habitability(),
                    };
                    comp.eval(val, *other_val)
                } else {
                    false
                }
            }
            Condition::WorldVariable(var, comp, other_val) => {
                let val = match var {
                    WorldVariable::Year => state.world.year as f32,
                    WorldVariable::Population => state.world.regions.population(),
                    WorldVariable::PopulationGrowth => state.world.population_growth_modifier,
                    WorldVariable::Emissions => state.emissions.as_co2eq(),
                    WorldVariable::ExtinctionRate => state.world.extinction_rate,
                    WorldVariable::Outlook => state.outlook(),
                    WorldVariable::Temperature => state.world.temperature,
                    WorldVariable::SeaLevelRise => state.world.sea_level_rise,
                    WorldVariable::SeaLevelRiseRate => state.world.sea_level_rise_rate(),
                    WorldVariable::Precipitation => state.world.precipitation,
                };
                comp.eval(val, *other_val)
            }
            Condition::PlayerVariable(var, comp, other_val) => {
                let val = match var {
                    PlayerVariable::PoliticalCapital => state.political_capital as f32,
                    PlayerVariable::ResearchPoints => state.research_points as f32,
                    PlayerVariable::YearsToDeath => {
                        state.death_year as f32 - state.world.year as f32
                    }
                };
                comp.eval(val, *other_val)
            }
            Condition::ProcessOutput(id, comp, other_val) => {
                if let Some(val) = state.produced.by_process.get(id) {
                    comp.eval(*val, *other_val)
                } else {
                    false
                }
            }
            Condition::ProcessMixShare(id, comp, other_val) => {
                let val = state.world.processes[id].mix_percent();
                comp.eval(val, *other_val)
            }
            Condition::ProcessMixShareFeature(feat, comp, other_val) => {
                let val = state
                    .world
                    .processes
                    .iter()
                    .filter(|p| p.features.contains(feat))
                    .map(|p| p.mix_percent())
                    .sum();
                comp.eval(val, *other_val)
            }
            Condition::ResourcePressure(resource, comp, other_val) => {
                let val = state.resources[*resource] / state.resource_demand.of(*resource);
                comp.eval(val, *other_val)
            }
            Condition::ResourceDemandGap(resource, comp, other_val) => {
                let available = state.resources[*resource];
                let demand = state.resource_demand.of(*resource);
                let val = (available - demand) / demand;
                comp.eval(val, *other_val)
            }
            Condition::OutputDemandGap(output, comp, other_val) => {
                let available = state.produced.of(*output);
                let demand = state.output_demand.of(*output);
                let val = 1. - (available / demand).min(1.);
                comp.eval(val, *other_val)
            }
            Condition::Demand(output, comp, other_val) => {
                // Apply conversion to OUTPUT_UNITS
                let factor = match output {
                    Output::Fuel => 1e-9 / 1e3,           // per 1000 TWh
                    Output::Electricity => 1e-9 / 1e3,    // per 1000 TWh
                    Output::PlantCalories => 1e-9 / 2e4,  // per 20000 Tcals
                    Output::AnimalCalories => 1e-9 / 2e4, // per 20000 Tcals
                };
                let demand = state.output_demand.of(*output) * factor;
                comp.eval(demand, *other_val)
            }
            Condition::FeedstockYears(feedstock, comp, other_val) => {
                comp.eval(state.feedstocks.until_exhaustion(*feedstock), *other_val)
            }
            Condition::RunsPlayed(comp, runs) => comp.eval(state.runs as f32, *runs as f32),
            Condition::ProjectStatus(id, status) => match status {
                ProjectStatus::Active | ProjectStatus::Finished => {
                    matches!(
                        state.world.projects[id].status,
                        ProjectStatus::Active | ProjectStatus::Finished
                    )
                }
                _ => state.world.projects[id].status == *status,
            },
            Condition::ActiveProjectUpgrades(id, comp, upgrades) => {
                comp.eval(state.world.projects[id].level as f32, *upgrades as f32)
            }
            Condition::NPCRelationship(id, relation) => state.npcs[id].relation() == *relation,
            Condition::RegionFlag(flag) => {
                if let Some(id) = &region_id {
                    let region = &state.world.regions[id];
                    region.flags.contains(flag)
                } else {
                    false
                }
            }
            Condition::HasFlag(flag) => state.flags.contains(flag),
            Condition::WithoutFlag(flag) => !state.flags.contains(flag),
            Condition::HeavyProjects(comp, n) => {
                let heavy_projects = state
                    .world
                    .projects
                    .iter()
                    .filter(|p| {
                        p.status == ProjectStatus::Finished && HEAVY_PROJECTS.contains(&p.group)
                    })
                    .count();
                comp.eval(heavy_projects as f32, *n as f32)
            }
            Condition::ProtectLand(comp, n) => comp.eval(state.protected_land, *n),
            Condition::WaterStress(comp, n) => {
                let water_stress =
                    state.resource_demand.of(Resource::Water) / state.resources.available.water;
                comp.eval(water_stress, *n)
            }
        }
    }
}

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, PartialEq, EnumIter, EnumString, IntoStaticStr,
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
impl std::fmt::Display for Comparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Comparator::Less => "<",
                Comparator::LessEqual => "<=",
                Comparator::Equal => "==",
                Comparator::NotEqual => "!=",
                Comparator::GreaterEqual => ">=",
                Comparator::Greater => ">",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_demand_gap() {
        let cond =
            Condition::OutputDemandGap(Output::PlantCalories, Comparator::GreaterEqual, 0.15);
        let mut state = State::default();
        state.output_demand.base.plant_calories = 100.;
        state.produced.amount.plant_calories = 100.;
        assert_eq!(cond.eval(&state, None), false);

        state.output_demand.base.plant_calories = 100.;
        state.produced.amount.plant_calories = 99.;
        assert_eq!(cond.eval(&state, None), false);

        state.output_demand.base.plant_calories = 100.;
        state.produced.amount.plant_calories = 84.;
        assert_eq!(cond.eval(&state, None), true);

        state.produced.amount.plant_calories = 50.;
        assert_eq!(cond.eval(&state, None), true);
    }
}
