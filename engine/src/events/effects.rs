use crate::state::State;
use crate::regions::Region;
use crate::production::ProcessFeature;
use crate::kinds::{Resource, Output, Feedstock, Byproduct};
use super::{WorldVariable, LocalVariable, PlayerVariable, EventPool};
use serde::Serialize;
use std::ops::Mul;

const MIGRATION_WAVE_PERCENT_POP: f32 = 0.1;

#[derive(Clone, Serialize)]
pub enum Request {
    Project,
    Process
}

#[derive(Serialize, PartialEq, Debug, Clone, Copy)]
pub enum Flag {
    Electrified,
    EnergyStorage1,
    EnergyStorage2,
    EnergyStorage3,
    Vegetarian,
    Vegan,
    ClosedBorders,
    EVs
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub enum Effect {
    LocalVariable(LocalVariable, f32),
    WorldVariable(WorldVariable, f32),
    PlayerVariable(PlayerVariable, f32),

    Resource(Resource, f32),
    Demand(Output, f32),
    Output(Output, f32),
    DemandAmount(Output, f32),
    OutputForFeature(ProcessFeature, f32),
    OutputForProcess(usize, f32),
    Feedstock(Feedstock, f32),

    AddEvent(usize),
    TriggerEvent(usize, usize),
    UnlocksProject(usize),
    UnlocksProcess(usize),
    UnlocksNPC(usize),

    ProjectRequest(usize, bool, usize),
    ProcessRequest(usize, bool, usize),

    Migration,
    RegionLeave,
    AddRegionFlag(String),

    AddFlag(Flag),
    AutoClick(usize, f32),
    NPCRelationship(usize, isize),

    ModifyIndustryByproducts(usize, Byproduct, f32),
    ModifyIndustryResources(usize, Resource, f32),
    ModifyEventProbability(usize, f32),
    ModifyIndustryDemand(usize, f32),
    DemandOutlookChange(Output, f32),
    IncomeOutlookChange(f32),
    ProjectCostModifier(usize, f32),

    ProtectLand(f32),
}

impl Effect {
    pub fn apply(&self, state: &mut State, event_pool: &mut EventPool, region_id: Option<usize>) {
        match self {
            Effect::LocalVariable(var, change) => {
                if let Some(id) = region_id {
                    let region = &mut state.world.regions[id];
                    match var {
                        LocalVariable::Population => region.population *= 1. + *change/100.,
                        LocalVariable::Outlook => region.outlook += *change,
                        LocalVariable::Habitability => region.base_habitability += *change,
                    }
                }
            },
            Effect::WorldVariable(var, change) => {
                match var {
                    WorldVariable::Year => state.world.year += *change as usize,
                    WorldVariable::Population => state.world.change_population(*change/100.),
                    WorldVariable::PopulationGrowth => state.world.population_growth_modifier += *change/100.,
                    WorldVariable::Emissions => {
                        state.world.byproduct_mods.co2 += *change * 1e15; // effect in Gt
                        state.world.co2_emissions += *change * 1e15; // Apply immediately
                    },
                    WorldVariable::ExtinctionRate => state.world.byproduct_mods.biodiversity -= *change,
                    WorldVariable::Outlook => state.world.change_outlook(*change),
                    WorldVariable::Temperature => state.world.temperature_modifier += *change,
                    WorldVariable::WaterStress => state.world.water_stress += *change,
                    WorldVariable::SeaLevelRise => state.world.sea_level_rise += *change,
                    WorldVariable::Precipitation => state.world.precipitation += *change,
                }
            }
            Effect::PlayerVariable(var, change) => {
                match var {
                    PlayerVariable::PoliticalCapital => state.political_capital += *change as isize,
                    PlayerVariable::MalthusianPoints => state.malthusian_points += *change as usize,
                    PlayerVariable::HESPoints => state.hes_points += *change as usize,
                    PlayerVariable::FALCPoints => state.falc_points += *change as usize,
                }
            },
            Effect::Resource(resource, pct_change) => {
                state.resources[*resource] *= 1. + pct_change;
            }
            Effect::Demand(output, pct_change) => {
                state.output_demand_modifier[*output] += pct_change;
            },
            Effect::DemandAmount(output, amount) => {
                state.output_demand_extras[*output] += amount;
            },
            Effect::Output(output, pct_change) => {
                state.output_modifier[*output] += pct_change;
            },
            Effect::OutputForFeature(feat, pct_change) => {
                for process in state.processes.iter_mut().filter(|p| p.features.contains(feat)) {
                    process.output_modifier += pct_change;
                }
            },
            Effect::OutputForProcess(id, pct_change) => {
                let process = &mut state.processes[*id];
                process.output_modifier += pct_change;
            },
            Effect::Feedstock(feedstock, pct_change) => {
                state.feedstocks[*feedstock] *= pct_change;
            },
            Effect::AddEvent(id) => {
                event_pool.events[*id].locked = false;
            },
            Effect::TriggerEvent(id, years) => {
                event_pool.queue_event(*id, region_id, *years);
            },
            Effect::UnlocksProject(id) => {
                state.projects[*id].locked = false;
            },
            Effect::UnlocksProcess(id) => {
                state.processes[*id].locked = false;
            },
            Effect::UnlocksNPC(id) => {
                state.npcs[*id].locked = false;
            },
            Effect::ProjectRequest(id, active, bounty) => {
                state.requests.push((Request::Project, *id, *active, *bounty));
            },
            Effect::ProcessRequest(id, active, bounty) => {
                state.requests.push((Request::Process, *id, *active, *bounty));
            },
            Effect::Migration => {
                if let Some(id) = region_id {
                    let leave_pop = state.world.regions[id].population * MIGRATION_WAVE_PERCENT_POP;
                    state.world.regions[id].population -= leave_pop;

                    // Find the most habitable regions
                    let mean_habitability: f32 = state.world.habitability();
                    let target_regions: Vec<&mut Region> = state.world.regions.iter_mut()
                        .filter(|r| r.id != id && r.habitability() > mean_habitability).collect();
                    let per_region = leave_pop/target_regions.len() as f32;
                    for region in target_regions {
                        region.population += per_region;
                    }
                }
                todo!()
            },
            Effect::RegionLeave => {
                if let Some(id) = region_id {
                    state.world.regions[id].seceded = true;
                }
            },
            Effect::AddRegionFlag(flag) => {
                if let Some(id) = region_id {
                    state.world.regions[id].flags.push(flag.to_string());
                }
            },
            Effect::AddFlag(flag) => {
                state.flags.push(*flag);
            },
            Effect::NPCRelationship(id, change) => {
                state.npcs[*id].relationship += change;
            },

            Effect::ModifyIndustryByproducts(id, byproduct, mult) => {
                state.industries[*id].byproducts[*byproduct] *= mult;
            },
            Effect::ModifyIndustryResources(id, resource, mult) => {
                state.industries[*id].resources[*resource] *= mult;
            },
            Effect::ModifyEventProbability(id, change) => {
                event_pool.events[*id].prob_modifier += change;
            },
            Effect::ModifyIndustryDemand(id, change) => {
                state.industries[*id].demand_modifier += change;
            },
            Effect::DemandOutlookChange(output, mult) => {
                for region in &mut state.world.regions {
                    region.outlook += (mult * region.demand()[*output]).floor();
                }
            },
            Effect::IncomeOutlookChange(mult) => {
                for region in &mut state.world.regions {
                    region.outlook += (mult * region.adjusted_income()).floor();
                }
            },
            Effect::ProjectCostModifier(id, change) => {
                state.projects[*id].cost_modifier += change;
            },
            Effect::ProtectLand(percent) => {
                state.protected_land += percent/100.;
            }

            // Effects like AutoClick have no impact in the engine side
            _ => ()
        }
    }

    pub fn unapply(&self, state: &mut State, event_pool: &mut EventPool, region_id: Option<usize>) {
        match self {
            Effect::LocalVariable(var, change) => {
                if let Some(id) = region_id {
                    let region = &mut state.world.regions[id];
                    match var {
                        LocalVariable::Population => region.population /= 1. + *change/100.,
                        LocalVariable::Outlook => region.outlook -= *change,
                        LocalVariable::Habitability => region.base_habitability -= *change,
                    }
                }
            },
            Effect::WorldVariable(var, change) => {
                match var {
                    WorldVariable::Year => state.world.year -= *change as usize,
                    WorldVariable::Population => state.world.change_population(1./(*change/100.)),
                    WorldVariable::PopulationGrowth => state.world.population_growth_modifier -= *change/100.,
                    WorldVariable::Emissions => {
                        state.world.byproduct_mods.co2 -= *change * 1e15;
                        state.world.co2_emissions -= *change * 1e15; // Apply immediately
                    },
                    WorldVariable::ExtinctionRate => state.world.byproduct_mods.biodiversity += *change,
                    WorldVariable::Outlook => state.world.change_outlook(-*change),
                    WorldVariable::Temperature => state.world.temperature_modifier -= *change,
                    WorldVariable::WaterStress => state.world.water_stress -= *change,
                    WorldVariable::SeaLevelRise => state.world.sea_level_rise -= *change,
                    WorldVariable::Precipitation => state.world.precipitation -= *change,
                }
            }
            Effect::PlayerVariable(var, change) => {
                match var {
                    PlayerVariable::PoliticalCapital => state.political_capital -= *change as isize,
                    PlayerVariable::MalthusianPoints => state.malthusian_points -= *change as usize,
                    PlayerVariable::HESPoints => state.hes_points -= *change as usize,
                    PlayerVariable::FALCPoints => state.falc_points -= *change as usize,
                }
            },
            Effect::Resource(resource, pct_change) => {
                state.resources[*resource] /= 1. + pct_change;
            }
            Effect::Demand(output, pct_change) => {
                state.output_demand_modifier[*output] -= pct_change;
            },
            Effect::DemandAmount(output, amount) => {
                state.output_demand_extras[*output] -= amount;
            },
            Effect::Output(output, pct_change) => {
                state.output_modifier[*output] -= pct_change;
            },
            Effect::OutputForFeature(feat, pct_change) => {
                for process in state.processes.iter_mut().filter(|p| p.features.contains(feat)) {
                    process.output_modifier -= pct_change;
                }
            },
            Effect::OutputForProcess(id, pct_change) => {
                let process = &mut state.processes[*id];
                process.output_modifier -= pct_change;
            },
            Effect::Feedstock(feedstock, pct_change) => {
                state.feedstocks[*feedstock] /= pct_change;
            },
            Effect::NPCRelationship(id, change) => {
                state.npcs[*id].relationship -= change;
            },
            Effect::ModifyIndustryByproducts(id, byproduct, mult) => {
                state.industries[*id].byproducts[*byproduct] /= mult;
            },
            Effect::ModifyIndustryResources(id, resource, mult) => {
                state.industries[*id].resources[*resource] /= mult;
            },
            Effect::ModifyEventProbability(id, change) => {
                event_pool.events[*id].prob_modifier -= change;
            },
            Effect::ModifyIndustryDemand(id, change) => {
                state.industries[*id].demand_modifier -= change;
            },
            Effect::DemandOutlookChange(output, mult) => {
                for region in &mut state.world.regions {
                    region.outlook -= (mult * region.demand()[*output]).round();
                }
            },
            Effect::IncomeOutlookChange(mult) => {
                for region in &mut state.world.regions {
                    region.outlook -= (mult * region.adjusted_income()).round();
                }
            },
            Effect::ProjectCostModifier(id, change) => {
                state.projects[*id].cost_modifier -= change;
            },
            Effect::ProtectLand(percent) => {
                state.protected_land -= percent/100.;
            }

            // Other effects aren't reversible
            _ => ()
        }
    }
}

// For scaling effects by float
impl Mul<f32> for Effect {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        match self {
            Effect::LocalVariable(var, val) => Effect::LocalVariable(var, val * rhs),
            Effect::WorldVariable(var, val) => Effect::WorldVariable(var, val * rhs),
            Effect::PlayerVariable(var, val) => Effect::PlayerVariable(var, val * rhs),
            Effect::Resource(resource, val) => Effect::Resource(resource, val * rhs),
            Effect::Demand(output, val) => Effect::Demand(output, val * rhs),
            Effect::Output(output, val) => Effect::Output(output, val * rhs),
            Effect::DemandAmount(output, val) => Effect::DemandAmount(output, val * rhs),
            Effect::OutputForFeature(feat, val) => Effect::OutputForFeature(feat, val * rhs),
            Effect::OutputForProcess(id, val) => Effect::OutputForProcess(id, val * rhs),
            Effect::Feedstock(feedstock, val) => Effect::Feedstock(feedstock, val * rhs),
            Effect::ModifyIndustryByproducts(id, byproduct, val) => Effect::ModifyIndustryByproducts(id, byproduct, val * rhs),
            Effect::ModifyIndustryResources(id, resource, val) => Effect::ModifyIndustryResources(id, resource, val * rhs),
            Effect::ModifyIndustryDemand(id, val) => Effect::ModifyIndustryDemand(id, val * rhs),
            Effect::ModifyEventProbability(id, val) => Effect::ModifyEventProbability(id, val * rhs),
            Effect::DemandOutlookChange(output, val) => Effect::DemandOutlookChange(output, val * rhs),
            Effect::IncomeOutlookChange(val) => Effect::IncomeOutlookChange(val * rhs),
            Effect::ProjectCostModifier(id, val) => Effect::ProjectCostModifier(id, val * rhs),
            Effect::ProtectLand(val) => Effect::ProtectLand(val * rhs),
            _ => self
        }
    }
}
