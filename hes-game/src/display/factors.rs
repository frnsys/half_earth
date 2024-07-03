use enum_map::EnumMap;
use hes_engine::{
    events::{mean_demand_outlook_change, mean_income_outlook_change, Effect, WorldVariable},
    industries::Industry,
    kinds::{ByproductMap, Output, Resource, ResourceMap},
    production::Process,
    state::State,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::{display::format, icons, icons::HasIcon, views::FactorsCard};

use super::{intensity, Impact, OutputKind, Var};

fn effects_factor<E: AsRef<Effect>>(var: Var, effects: &[E], state: &State) -> f32 {
    let effects = effects.iter().map(|e| e.as_ref());
    match var {
        Var::Emissions => effects
            .filter_map(|e| match e {
                Effect::WorldVariable(WorldVariable::Emissions, val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
        Var::Water => effects
            .filter_map(|e| match e {
                Effect::Resource(Resource::Water, val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
        Var::Land => effects
            .filter_map(|e| match e {
                Effect::ProtectLand(val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
        Var::Energy => effects
            .filter_map(|e| match e {
                Effect::DemandAmount(Output::Electricity | Output::Fuel, val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
        Var::Electricity => effects
            .filter_map(|e| match e {
                Effect::DemandAmount(Output::Electricity, val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
        Var::Fuel => effects
            .filter_map(|e| match e {
                Effect::DemandAmount(Output::Fuel, val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
        Var::PlantCalories => effects
            .filter_map(|e| match e {
                Effect::DemandAmount(Output::PlantCalories, val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
        Var::AnimalCalories => effects
            .filter_map(|e| match e {
                Effect::DemandAmount(Output::AnimalCalories, val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
        Var::Contentedness => effects
            .filter_map(|e| match e {
                Effect::WorldVariable(WorldVariable::Outlook, val) => Some(*val),
                Effect::IncomeOutlookChange(val) => {
                    let change = mean_income_outlook_change(*val, state);
                    Some(change)
                }
                Effect::DemandOutlookChange(output, val) => {
                    let change = mean_demand_outlook_change(*val, output, state);
                    Some(change)
                }
                _ => None,
            })
            .sum::<f32>(),
        Var::Biodiversity => effects
            .filter_map(|e| match e {
                Effect::WorldVariable(WorldVariable::ExtinctionRate, val) => Some(val),
                _ => None,
            })
            .sum::<f32>(),
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum Factor {
    Project {
        name: String,
        amount: f32,
        display: String,
    },
    Region {
        name: String,
        amount: f32,
        display: String,
        intensity: usize,
    },
    Process {
        name: String,
        amount: f32,
        display: String,
        intensity: usize,
        output: Output,
        produced: f32,
        display_produced: String,
    },
    Industry {
        name: String,
        amount: f32,
        produced: f32,
        intensity: usize,
        display: String,
    },
    Event {
        name: String,
        amount: f32,
        display: Option<String>,
    },
}
impl Factor {
    pub fn name(&self) -> &str {
        match self {
            Self::Project { name, .. } => name,
            Self::Region { name, .. } => name,
            Self::Process { name, .. } => name,
            Self::Industry { name, .. } => name,
            Self::Event { name, .. } => name,
        }
    }

    pub fn amount(&self) -> f32 {
        match self {
            Self::Project { amount, .. } => *amount,
            Self::Region { amount, .. } => *amount,
            Self::Process { amount, .. } => *amount,
            Self::Industry { amount, .. } => *amount,
            Self::Event { amount, .. } => *amount,
        }
    }
}

fn event_factors(var: Var, state: &State) -> Vec<Factor> {
    state
        .events
        .iter()
        .map(|event| Factor::Event {
            name: event.name.clone(),
            amount: effects_factor(var, &event.effects, state),
            display: None,
        })
        .filter(|fac| fac.amount() != 0.)
        .collect()
}

fn project_factors(var: Var, state: &State) -> Vec<Factor> {
    state
        .world
        .projects
        .iter()
        .filter(|p| p.is_active() || p.is_finished())
        .map(|p| {
            let effects = p.active_effects_with_outcomes();
            let amount = effects_factor(var, &effects, state);

            let display = if var.is_demand_var() {
                let demand = match var {
                    Var::Energy => state.output_demand.energy(),
                    Var::Electricity => state.output_demand.electricity,
                    Var::Fuel => state.output_demand.fuel,
                    Var::PlantCalories => state.output_demand.plant_calories,
                    Var::AnimalCalories => state.output_demand.animal_calories,
                    _ => unreachable!(),
                };
                format::percent(amount / demand, true)
            } else {
                amount.to_string()
            };
            Factor::Project {
                name: p.name.clone(),
                amount,
                display,
            }
        })
        .filter(|fac| fac.amount() != 0.)
        .collect()
}

fn regional_factors(output: Output, state: &State) -> Vec<Factor> {
    let per_capita_output_demand = &state.world.output_demand;
    let output_demand = &state.output_demand;
    state
        .world
        .regions
        .iter()
        .map(|region| {
            let intensity = region.income_level() + 1;
            let amount = region.demand(per_capita_output_demand)[output];

            // What percent of total demand this region's demand represents.
            let display = format::percent(amount / output_demand[output], true);
            Factor::Region {
                name: region.name.clone(),
                intensity,
                amount: format::output(amount, output),
                display,
            }
        })
        .filter(|fac| fac.amount() != 0.)
        .collect()
}

trait HasImpacts {
    fn resources(&self) -> &ResourceMap;
    fn byproducts(&self) -> &ByproductMap;
    fn extinction_rate(&self, total_land: f32) -> f32;
    fn demand(&self, state: &State) -> f32;
}
impl HasImpacts for Process {
    fn resources(&self) -> &ResourceMap {
        &self.resources
    }

    fn byproducts(&self) -> &ByproductMap {
        &self.byproducts
    }

    fn extinction_rate(&self, total_land: f32) -> f32 {
        self.extinction_rate(total_land)
    }

    fn demand(&self, state: &State) -> f32 {
        state.produced_by_process[self.id]
    }
}

impl HasImpacts for Industry {
    fn resources(&self) -> &ResourceMap {
        &self.resources
    }

    fn byproducts(&self) -> &ByproductMap {
        &self.byproducts
    }

    fn extinction_rate(&self, total_land: f32) -> f32 {
        self.extinction_rate(total_land)
    }

    fn demand(&self, state: &State) -> f32 {
        let lic_pop = state.world.lic_population();
        self.demand(lic_pop)
    }
}

fn impact_factor<S: HasImpacts>(
    source: &S,
    impact: Impact,
    state: &State,
) -> (f32, f32, f32, String) {
    let base = match impact {
        Impact::Land | Impact::Water | Impact::Electricity | Impact::Fuel => {
            let res = impact.as_resource().expect("Checked that it's a resource.");
            source.resources()[res]
        }
        Impact::Energy => {
            source.resources()[Resource::Electricity] + source.resources()[Resource::Fuel]
        }
        Impact::Emissions => source.byproducts().co2eq(),
        Impact::Biodiversity => {
            let total_land = state.world.starting_resources.land;
            source.extinction_rate(total_land)
        }
    };

    let demand = source.demand(state);

    let total = base
        * demand
        * impact
            .as_output()
            .map(|o| state.output_demand_modifier[o])
            .unwrap_or(1.);

    let display = match impact {
        Impact::Energy => {
            let demand = state.output_demand.electricity + state.output_demand.fuel;
            format::percent(total / demand, true)
        }
        Impact::Electricity => {
            let demand = state.output_demand.electricity;
            format::percent(total / demand, true)
        }
        Impact::Fuel => {
            let demand = state.output_demand.fuel;
            format::percent(total / demand, true)
        }
        _ => format::format_impact(impact, total),
    };

    (base, demand, total, display)
}

fn production_factors(impact: Impact, state: &State) -> Vec<Factor> {
    state
        .world
        .processes
        .iter()
        .map(|proc| {
            let (base, demand, total, display) = impact_factor(proc, impact, state);

            let kind: OutputKind = proc.output.into();
            let inten = intensity::impact_intensity(base, impact, kind);
            let total_demand = state.output_demand[proc.output];
            let display_produced = format::percent(demand / total_demand, true);

            Factor::Process {
                name: proc.name.to_string(),
                produced: demand,
                output: proc.output,
                amount: total,
                display,
                intensity: inten,
                display_produced,
            }
        })
        .chain(state.world.industries.iter().map(|ind| {
            let (base, demand, total, display) = impact_factor(ind, impact, state);
            let inten = intensity::impact_intensity(base, impact, OutputKind::Energy);
            Factor::Industry {
                name: ind.name.to_string(),
                produced: demand,
                display,
                intensity: inten,
                amount: total,
            }
        }))
        .filter(|fac| fac.amount() != 0.)
        .collect()
}

pub fn rank(state: &State) -> EnumMap<Var, Vec<Factor>> {
    let mut factors = EnumMap::default();
    for var in Var::iter() {
        let mut rankings = project_factors(var, state);
        rankings.extend(event_factors(var, state));
        if let Some(impact) = var.as_impact() {
            rankings.extend(production_factors(impact, state));
        }
        if let Some(output) = var.as_output() {
            rankings.extend(regional_factors(output, state));
        }

        // Additional factors
        match var {
            Var::Contentedness => {
                if state.temp_outlook != 0. {
                    rankings.push(Factor::Event {
                        name: "Temperature Change".into(),
                        amount: state.temp_outlook.round(),
                        display: None,
                    })
                }
                if state.shortages_outlook != 0. {
                    rankings.push(Factor::Event {
                        name: "Production Shortages".into(),
                        amount: state.shortages_outlook.round(),
                        display: None,
                    })
                }
                rankings.push(Factor::Event {
                    name: "Post-Revolution Optimism".into(),
                    amount: 30.,
                    display: None,
                })
            }
            Var::Land => {
                rankings.push(Factor::Event {
                    name: "Nature Preserves".into(),
                    display: Some("10%".into()),
                    amount: 0.1 * state.world.starting_resources.land,
                });
            }
            Var::Biodiversity => {
                rankings.push(Factor::Event {
                    name: "Sea Level Rise".into(),
                    amount: state.world.slr_extinction_rate().round(),
                    display: None,
                });
                rankings.push(Factor::Event {
                    name: "Temperature Change".into(),
                    amount: state.world.tgav_extinction_rate().round(),
                    display: None,
                });
            }
            _ => {}
        }

        // NOTE TODO I do not remember what this is about
        // Split into modifiers (which don't make up demand percentages)
        // and contributors (who do make up demand percentages);
        // let [modifiers, contribs] = partition(rankings, (r) => {
        //     let amount = r.displayAmount || r.amount;
        //     return typeof amount === 'string' && (amount.startsWith('+') || amount.startsWith('-'));
        // });
        // modifiers.sort();
        // contribs.sort((a, b) => Math.abs(a.amount) > Math.abs(b.amount) ? -1 : 1)
        // factors[k] = modifiers.concat(contribs);

        rankings.sort_by(|a, b| a.amount().partial_cmp(&b.amount()).unwrap());
        factors[var] = rankings;
    }
    factors
}

pub fn factors_card(current_name: Option<String>, var: Var, state: &State) -> FactorsCard {
    FactorsCard {
        icon: var.icon(),
        kind: var,
        current: current_name,
        total: match var {
            // TODO total: `${state.gameState.world.emissions_gt().toFixed(1)}Gt`,
            Var::Emissions => state.emissions_gt(),
            Var::Biodiversity => state.world.extinction_rate.round().max(0.),

            // TODO total: `${Math.round(format.landUsePercent(state.gameState.resources_demand.land))}%`,
            Var::Land => format::land_use_percent(state.resources_demand.land),
            Var::Energy => {
                let demand = state.output_demand;
                // TODO total: `${format.twh(demand.electricity + demand.fuel)}TWh`,
                format::twh(demand.electricity + demand.fuel)
            }
            Var::Water => {
                // TODO total: `${format.output(state.gameState.resources_demand.water, 'water')}/${format.output(state.gameState.resources.water, 'water')}`,
                format::resource(state.resources_demand.water, Resource::Water)
                    / format::resource(state.resources.water, Resource::Water)
            }
            Var::Contentedness => state.outlook().round(),
            Var::Electricity => {
                format::resource(state.output_demand.electricity, Resource::Electricity)
            }
            Var::Fuel => format::resource(state.output_demand.fuel, Resource::Fuel),
            Var::PlantCalories => {
                format::output(state.output_demand.plant_calories, Output::PlantCalories)
            }
            Var::AnimalCalories => {
                format::output(state.output_demand.animal_calories, Output::AnimalCalories)
            }
        },
    }
}
