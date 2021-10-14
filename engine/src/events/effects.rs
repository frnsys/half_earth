use crate::game::Game;
use crate::regions::Region;
use crate::production::ProcessFeature;
use crate::kinds::{Resource, Output, Feedstock};
use super::{WorldVariable, LocalVariable, PlayerVariable};

const MIGRATION_WAVE_PERCENT_POP: f32 = 0.1;

#[derive(Debug, Clone)]
pub enum Effect {
    LocalVariable(LocalVariable, f32),
    WorldVariable(WorldVariable, f32),
    PlayerVariable(PlayerVariable, f32),

    Resource(Resource, f32),
    Demand(Output, f32),
    Output(Output, f32),
    OutputForFeature(ProcessFeature, f32),
    Feedstock(Feedstock, f32),

    AddEvent(usize),
    TriggerEvent(usize, usize),
    UnlocksProject(usize),
    UnlocksProcess(usize),

    Migration,
    RegionLeave,
}

impl Effect {
    pub fn apply(&self, game: &mut Game, region_id: Option<usize>) {
        match self {
            Effect::LocalVariable(var, change) => {
                if let Some(id) = region_id {
                    let region = &mut game.state.world.regions[id];
                    match var {
                        LocalVariable::Population => region.population *= 1. + *change/100.,
                        LocalVariable::Health => region.health += *change,
                        LocalVariable::Outlook => region.outlook += *change,
                        LocalVariable::Contentedness => region.base_contentedness += *change,
                        LocalVariable::Habitability => region.base_habitability += *change,
                    }
                }
            },
            Effect::WorldVariable(var, change) => {
                match var {
                    WorldVariable::Year => game.state.world.year += *change as usize,
                    WorldVariable::Population => game.state.world.change_population(*change/100.),
                    WorldVariable::Emissions => game.state.world.change_emissions(*change/100.),
                    WorldVariable::ExtinctionRate => game.state.world.extinction_rate += *change,
                    WorldVariable::Outlook => game.state.world.change_outlook(*change),
                    WorldVariable::Temperature => game.state.world.temperature += *change,
                    WorldVariable::Contentedness => game.state.world.change_contentedness(*change),
                    WorldVariable::WaterStress => game.state.world.water_stress += *change,
                    WorldVariable::SeaLevelRise => game.state.world.sea_level_rise += *change,
                    WorldVariable::Precipitation => game.state.world.precipitation += *change,
                    WorldVariable::Health => game.state.world.change_health(*change),
                }
            }
            Effect::PlayerVariable(var, change) => {
                match var {
                    PlayerVariable::PoliticalCapital => game.state.political_capital += *change as usize,
                }
            },
            Effect::Resource(resource, pct_change) => {
                game.state.resources[*resource] *= 1. + pct_change;
            }
            Effect::Demand(output, pct_change) => {
                game.state.output_demand_modifier[*output] *= 1. + pct_change;
            },
            Effect::Output(output, pct_change) => {
                game.state.output_modifier[*output] *= 1. + pct_change;
            },
            Effect::OutputForFeature(feat, pct_change) => {
                for process in game.state.processes.iter_mut().filter(|p| p.features.contains(feat)) {
                    process.output_modifier *= 1. + pct_change;
                }
            },
            Effect::Feedstock(feedstock, pct_change) => {
                game.state.feedstocks[*feedstock] *= pct_change;
            },
            Effect::AddEvent(id) => {
                game.event_pool.events[*id].locked = false;
            },
            Effect::TriggerEvent(id, months) => {
                game.event_pool.queue.push((*id, region_id, *months));
            },
            Effect::UnlocksProject(id) => {
                game.state.projects[*id].locked = false;
            },
            Effect::UnlocksProcess(id) => {
                game.state.processes[*id].locked = false;
            },
            Effect::Migration => {
                if let Some(id) = region_id {
                    let leave_pop = game.state.world.regions[id].population * MIGRATION_WAVE_PERCENT_POP;
                    game.state.world.regions[id].population -= leave_pop;

                    // Find the most habitable regions
                    let mean_habitability: f32 = game.state.world.habitability();
                    let target_regions: Vec<&mut Region> = game.state.world.regions.iter_mut()
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
                    game.state.world.regions[id].seceded = true;
                }
            }
        }
    }
}
