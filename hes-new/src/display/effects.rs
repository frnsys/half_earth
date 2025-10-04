use crate::{
    consts,
    display::{
        self,
        AsText,
        Icon,
        icons::{self, HasIcon},
    },
    views::{Tip, tip},
    // views::factors::factors_card, // TODO
};
use hes_engine::*;
use rust_i18n::t;
use serde::{Deserialize, Serialize};

trait AsKey {
    fn as_key(&self) -> &'static str;
}
impl AsKey for Resource {
    fn as_key(&self) -> &'static str {
        match self {
            Resource::Land => "land",
            Resource::Fuel => "fuel",
            Resource::Water => "water",
            Resource::Electricity => "electricity",
        }
    }
}
impl AsKey for Output {
    fn as_key(&self) -> &'static str {
        match self {
            Output::Electricity => "electricity",
            Output::Fuel => "fuel",
            Output::PlantCalories => "plant_calories",
            Output::AnimalCalories => "animal_calories",
        }
    }
}
impl AsKey for Byproduct {
    fn as_key(&self) -> &'static str {
        match self {
            Byproduct::Co2 => "co2",
            Byproduct::N2o => "n2o",
            Byproduct::Ch4 => "ch4",
            Byproduct::Biodiversity => "biodiversity",
        }
    }
}
impl AsKey for Feedstock {
    fn as_key(&self) -> &'static str {
        match self {
            Feedstock::Coal => "coal",
            Feedstock::Lithium => "lithium",
            Feedstock::NaturalGas => "natural_gas",
            Feedstock::Oil => "oil",
            Feedstock::Uranium => "uranium",
            Feedstock::Thorium => "thorium",
            Feedstock::Soil => "soil",
            Feedstock::Other => "other",
        }
    }
}
impl AsKey for ProjectType {
    fn as_key(&self) -> &'static str {
        match self {
            ProjectType::Policy => "policy",
            ProjectType::Research => "research",
            ProjectType::Initiative => "initiative",
        }
    }
}

fn icon_card_tag(name: &str, icon: Icon) -> String {
    format!(r#"[c][i]{icon}[/i]{name}[/c]"#)
}
fn card_tag(name: &str) -> String {
    format!(r#"[c]{name}[/c]"#)
}

macro_rules! tip {
    ($icon:expr, $template:expr $(, $key:ident = $value:expr)* $(,)?) => {
        tip($icon, t!($template $(, $key = $value)*))
    };
}

macro_rules! text {
    ($icon:expr, $template:expr $(, $key:ident = $value:expr)* $(,)?) => {
        format!("[{}] {}", $icon, t!($template $(, $key = $value)*))
    };
}

pub fn flag_tip(flag: Flag, demand: &OutputMap) -> Tip {
    let demand = display::outputs(demand);
    match flag {
        Flag::Electrified => {
            let changed_demand = demand.fuel * 0.8;
            tip! {
                icons::ELECTRICITY,
                r#"Fuel demand will change from [i]%{iconFuel}[/i]%{prevDemandFuel} to [i]%{iconFuel}[/i]%{nextDemandFuel} and electricity demand will change from [i]%{iconElec}[/i]%{prevDemandElec} to [i]%{iconElec}[/i]%{nextDemandElec}."#,
                iconFuel = icons::FUEL,
                iconElec = icons::ELECTRICITY,
                prevDemandFuel = display::rounded(demand.fuel),
                nextDemandFuel = display::rounded(demand.fuel - changed_demand),
                prevDemandElec = display::rounded(demand.electricity),
                nextDemandElec = display::rounded(demand.electricity + changed_demand),
            }
        }
        Flag::Vegan => {
            let changed_demand = demand.animal_calories * 0.9;
            tip! {
                icons::PLANT_CALORIES,
                r#"Animal calorie demand will change from [i]%{iconACals}[/i]%{prevDemandACals} to [i]%{iconACals}[/i]%{nextDemandACals} and plant calorie demand will change from [i]%{iconPCals}[/i]%{prevDemandPCals} to [i]%{iconPCals}[/i]%{nextDemandPCals}."#,
                iconACals = icons::ANIMAL_CALORIES,
                iconPCals = icons::PLANT_CALORIES,
                prevDemandACals = display::rounded(demand.animal_calories),
                nextDemandACals = display::rounded(demand.animal_calories - changed_demand),
                prevDemandPCals = display::rounded(demand.plant_calories),
                nextDemandPCals = display::rounded(demand.plant_calories + changed_demand),
            }
        }
        Flag::Vegetarian => {
            let changed_demand = demand.animal_calories * 0.75;
            tip! {
                icons::PLANT_CALORIES,
                r#"Animal calorie demand will change from [i]%{iconACals}[/i]%{prevDemandACals} to [i]%{iconACals}[/i]%{nextDemandACals} and plant calorie demand will change from [i]%{iconPCals}[/i]%{prevDemandPCals} to [i]%{iconPCals}[/i]%{nextDemandPCals}."#,
                iconACals = icons::ANIMAL_CALORIES,
                iconPCals = icons::PLANT_CALORIES,
                prevDemandACals = display::rounded(demand.animal_calories),
                nextDemandACals = display::rounded(demand.animal_calories - changed_demand),
                prevDemandPCals = display::rounded(demand.plant_calories),
                nextDemandPCals = display::rounded(demand.plant_calories + changed_demand),
            }
        }
        Flag::ClosedBorders => {
            tip! {
                icons::CLOSED_BORDERS,
                "Migrations will have less of an impact when they occur. But there might be other consequences.",
            }
        }
        Flag::HyperResearch => {
            tip! {
                icons::RESEARCH,
                r#"Research points are 1[i]%{iconPC}[/i] cheaper."#,
                iconPC = icons::PLANT_CALORIES
            }
        }
        Flag::StopDevelopment => {
            tip! {
                icons::BAN,
                "Stops regional development throughout the world.",
            }
        }
        Flag::FastDevelopment => {
            tip! {
                icons::DEVELOPMENT,
                "Accelerates regional development throughout the world.",
            }
        }
        Flag::Degrowth => {
            tip! {
                icons::DEGROWTH,
                "Contract the economies of the wealthiest regions.",
            }
        }
        Flag::DeepSeaMining => {
            tip! {
                icons::OCEAN,
                "Prevents or stops metal shortages.",
            }
        }
        Flag::MetalsShortage => {
            tip! {
                icons::IRON_AND_STEEL,
                "Infrastructure projects take 20% longer to finish.",
            }
        }
        Flag::ParliamentSuspended => {
            tip! {
                icons::THE_AUTHORITARIAN,
                "A parliamentary majority is no longer required for any project.",
            }
        }
        Flag::MoreLabor => {
            tip! {
                icons::LABOR,
                "Research and infrastructure take 10% less time to complete.",
            }
        }
        Flag::MoreLeisure => {
            tip! {
                icons::LABOR,
                "Research and infrastructure take 10% more time to complete.",
            }
        }
        Flag::MoreAutomation => {
            tip! {
                icons::LABOR,
                "Research and infrastructure take 10% less time to complete.",
            }
        }
        Flag::EcosystemModeling => {
            tip! {
                icons::BIRB,
                "Restoration projects take 10% less time to complete.",
            }
        }
        Flag::LaborResistance => {
            tip! {
                icons::LABOR,
                "Research and infrastructure take 5% more time to complete.",
            }
        }
        Flag::LaborSabotage => {
            tip! {
                icons::LABOR,
                "Research and infrastructure take 5% more time to complete.",
            }
        }
        Flag::AlienEncounter => {
            tip! {
                icons::ALERT,
                "You had an extraterrestrial encounter."
            }
        }
        Flag::BailedOut => {
            tip! {
                icons::ALERT,
                "You were bailed out."
            }
        }
        Flag::RepeatTutorial => {
            tip! {
                icons::ALERT,
                "Repeat the tutorial."
            }
        }
        Flag::SkipTutorial => {
            tip! {
                icons::ALERT,
                "Skip the tutorial."
            }
        }
        Flag::LifeGoesOn => {
            tip! {
                icons::ALERT,
                "The game never ends."
            }
        }
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize,
)]
pub struct DisplayEffect {
    pub effect: Effect,
    pub likelihood: Option<Likelihood>,
    pub is_unknown: bool,
    pub is_hidden: bool,
}

impl From<&Effect> for DisplayEffect {
    fn from(value: &Effect) -> Self {
        DisplayEffect {
            effect: value.clone(),
            likelihood: None,
            is_hidden: false,
            is_unknown: false,
        }
    }
}

pub struct EffectTip {
    pub tip: Tip,
    pub text: String,
}

// We use this to avoid doing
// `t!(&format!(...))` as with this it's harder
// to parse out the translation strings with need.
macro_rules! prefix_probs {
    ($prob:ident, $post:literal) => {
        match $prob {
            Likelihood::Guaranteed => {
                t!(concat!("Will", $post))
            }
            Likelihood::Likely => {
                t!(concat!("Likely to", $post))
            }
            Likelihood::Random => t!(concat!("Could", $post)),
            Likelihood::Unlikely => {
                t!(concat!("Unlikely to", $post))
            }
            Likelihood::Rare => {
                t!(concat!("Small chance to", $post))
            }
            Likelihood::Improbable => {
                t!(concat!("Tiny chance to", $post))
            }
            Likelihood::Impossible => {
                t!(concat!("Won't", $post))
            }
        }
    };
}

impl DisplayEffect {
    fn fmt_param(&self, value: f32) -> String {
        if self.is_unknown {
            r#"[u]?[/u]"#.into()
        } else {
            format!("[b]{}[/b]", value.abs())
        }
    }

    fn change_dir(&self, change: f32) -> String {
        if self.is_unknown {
            t!("Changes").to_string()
        } else if let Some(prob) = self.likelihood {
            if change < 0. {
                prefix_probs!(prob, " reduce").to_string()
            } else {
                prefix_probs!(prob, " increase").to_string()
            }
        } else {
            let term = if change < 0. {
                t!("Reduces")
            } else {
                t!("Increases")
            };
            format!("[b]{term}[/b]")
        }
    }

    // Hacky, but using `Result` as a way to short-circuit an
    // effect that shouldn't be displayed.
    pub fn tip(&self, state: &State) -> Result<EffectTip, ()> {
        let (tip, text) = match &self.effect {
            Effect::WorldVariable(var, amount) => match var {
                WorldVariable::Outlook => (
                    tip! {
                        icons::CONTENTEDNESS,
                        r#"Current world contentedness is %{contentedness}[t]/%{maxContentedness}[/t]."#,
                        contentedness = state.outlook().round(),
                        maxContentedness = consts::MAX_CONTENTEDNESS,
                    },
                    text! {
                        "contentedness",
                        "%{changeDir} world contentedness by %{amount}.",
                        changeDir = self.change_dir(*amount),
                        amount = self.fmt_param(*amount),
                    },
                ),
                WorldVariable::Emissions => {
                    let emissions =
                        state.emissions.as_gtco2eq();
                    (
                        tip! {
                            icons::EMISSIONS,
                            "This will directly change annual emissions by %{amount}.%{percent}",
                            amount = if self.is_unknown {
                                t!("an unknown amount").to_string()
                            } else {
                                format!("{:+}", amount)
                            },
                            percent = if self.is_unknown || emissions == 0. {
                                "".into()
                            } else {
                                let percent = display::signed_percent(amount / state.emissions.as_gtco2eq(), true);
                                format!(" {}", t!("That's a {percent}% change.", percent = percent))
                            }
                        },
                        text! {
                            "emissions",
                            "%{changeDir} emissions by %{amount}.",
                            changeDir = self.change_dir(*amount),
                            amount = self.fmt_param(*amount)
                        },
                    )
                }
                WorldVariable::ExtinctionRate => (
                    tip! {
                        icons::EXTINCTION_RATE,
                        r#"Current biodiversity pressure is %{amount}[t]/%{maxAmount}[/t]."#,
                        amount = state.world.extinction_rate.round(),
                        maxAmount = consts::MAX_BIODIVERSITY
                    },
                    text! {
                        "extinction_rate",
                        "%{changeDir} biodiversity pressure by %{amount}.",
                        changeDir = self.change_dir(*amount),
                        amount = self.fmt_param(*amount)
                    },
                ),
                WorldVariable::Temperature => (
                    tip! {
                        icons::WARMING,
                        "This will directly change the global temperature anomaly by %{amount}[b]°c[/b].",
                        amount = format!("{:+}", amount)
                    },
                    text! {
                        "warming",
                        "%{changeDir} the global temperature by %{amount}[b]°c[/b].",
                        changeDir = self.change_dir(*amount),
                        amount = self.fmt_param(*amount)
                    },
                ),
                WorldVariable::Precipitation => (
                    tip! {
                        icons::WATER,
                        "This will directly change global precipitation by %{amount}[b]cm/yr[/b].",
                        amount = format!("{:+}", amount)
                    },
                    text! {
                        "water",
                        "%{changeDir} global precipitation by %{amount}[b]cm/yr[/b].",
                        changeDir = self.change_dir(*amount),
                        amount = self.fmt_param(*amount)
                    },
                ),
                WorldVariable::PopulationGrowth => (
                    tip! {
                        icons::POPULATION,
                        "The number of people on the planet.",
                    },
                    text! {
                        "population",
                        "%{changeDir} global population growth by %{amount}[b]%.[/b]",
                        changeDir = self.change_dir(*amount),
                        amount = display::percent(amount.abs(), false)
                    },
                ),
                WorldVariable::Population => (
                    tip! {
                        icons::POPULATION,
                        "The number of people on the planet.",
                    },
                    text! {
                        "population",
                        "%{changeDir} global population by %{amount}.",
                        changeDir = self.change_dir(*amount),
                        amount = self.fmt_param(*amount)
                    },
                ),
                WorldVariable::SeaLevelRiseRate => (
                    tip! {
                        icons::SEA_LEVEL_RISE,
                        "The amount of sea level rise is currently %{amount}m.",
                        amount = format!("{:.2}", state.world.sea_level_rise)
                    },
                    text! {
                        "sea_level_rise",
                        "%{changeDir} the rate of sea level rise by %{amount}mm/year.",
                        changeDir = self.change_dir(*amount),
                        amount = self.fmt_param(amount * 1000.)
                    },
                ),
                WorldVariable::SeaLevelRise => (
                    tip! {
                        icons::SEA_LEVEL_RISE,
                        "The amount of sea level rise is currently %{amount}m.",
                        amount = format!("{:.2}", state.world.sea_level_rise)
                    },
                    text! {
                        "sea_level_rise",
                        "%{changeDir} the amount of sea level rise by %{amount}mm/year.",
                        changeDir = self.change_dir(*amount),
                        amount = self.fmt_param(amount * 1000.)
                    },
                ),
                WorldVariable::Year => return Err(()),
            },
            Effect::PlayerVariable(var, amount) => match var {
                PlayerVariable::ResearchPoints => (
                    tip! {
                        icons::RESEARCH,
                        "Research points: Allocate them to research projects!",
                    },
                    text! {
                        "research",
                        if self.is_unknown {
                            t!("Possible +%{amount} research points.", amount = self.fmt_param(*amount))
                        } else {
                            t!("+%{amount} research points.", amount = self.fmt_param(*amount))
                        }.to_string()
                    },
                ),
                PlayerVariable::PoliticalCapital => (
                    tip! {
                        icons::POLITICAL_CAPITAL,
                        r#"How much political capital you have. Political capital is what you spend to implement your plans. [w]If you run out you'll be pushed out of government.[/w]"#,
                    },
                    text! {
                        "political_capital",
                        if self.is_unknown {
                            t!("Possible +%{amount} political capital.", amount = self.fmt_param(*amount))
                        } else {
                            t!("+%{amount} political capital.", amount = self.fmt_param(*amount))
                        }.to_string(),
                    },
                ),
                PlayerVariable::YearsToDeath => return Err(()),
            },
            Effect::ProcessLimit(id, amount) => {
                let process = &state.world.processes[id];
                let change = if let Some(limit) = process.limit
                {
                    let p = (amount / limit).abs();
                    format!("{}%", display::percent(p, true))
                } else {
                    amount.round().abs().to_string()
                };
                let text = t!(
                    "%{changeDir} maximum output for %{process} by [b]%{amount}[/b].",
                    amount = change,
                    process = t!(&process.name),
                    changeDir = self.change_dir(*amount),
                );
                (
                    tip(icons::ALERT, text.clone()),
                    text.to_string(),
                )
            }
            Effect::RegionHabitability(lat, amount) => (
                tip! {
                    icons::HABITABILITY,
                    "Lower habitability means unhappier people who may need to migrate to more hospitable locales.",
                },
                text! {
                    "habitability",
                    "%{changeDir} habitability in %{kind} regions by %{amount}.",
                    changeDir = self.change_dir(*amount),
                    amount = self.fmt_param(*amount),
                    kind = t!(lat.lower()),
                },
            ),
            Effect::Resource(resource, amount) => {
                let fmtted = display::resource(
                    *amount,
                    *resource,
                    state.resources.available,
                )
                .abs();
                let fmtted = if matches!(
                    resource,
                    Resource::Water | Resource::Land
                ) {
                    format!("{}%", fmtted.round())
                } else {
                    fmtted.to_string()
                };
                (
                    tip! {
                                resource.icon(),
                                r#"%{changeDir} %{name} supply by [i]%{icon}[/i]%{amount}."#,
                                amount = fmtted,
                                icon = resource.icon(),
                                name = t!(resource.lower()),
                                changeDir = self.change_dir(*amount),
                    }, // TODO
                    // }.card(factors_card(None, (*resource).into(), state)),
                    text! {
                        resource.as_key(),
                        "%{changeDir} %{name} supply by [i]%{icon}[/i]%{amount}.",
                        amount = fmtted,
                        name = t!(resource.lower()),
                        changeDir = self.change_dir(*amount),
                        icon = resource.as_key(),

                    },
                )
            }
            Effect::Output(output, amount) => {
                let base = display::output(
                    state.produced.of(*output),
                    *output,
                );
                let changed = base * (1. + amount);
                (
                    tip! {
                        output.icon(),
                        r#"Global %{name} output will change from [i]%{icon}[/i]%{base} to [i]%{icon}[/i]%{changed} with no change in impacts."#,
                        changed = changed.round(),
                        base = base.round(),
                        icon = output.icon(),
                        name = output.lower(),
                    },
                    text! {
                        output.as_key(),
                        "%{changeDir} all %{name} production by [b]%{percent}%.[/b]",
                        percent = display::percent(amount.abs(), true),
                        name = t!(output.lower()),
                        changeDir = self.change_dir(*amount),
                        icon = output.as_key(),
                    },
                )
            }
            Effect::OutputForProcess(id, amount) => {
                let process = &state.world.processes[id];
                (tip! {
                        process.output.icon(),
                        "Changes the output for this process by %{percent}% with no change in impacts.",
                        percent = display::signed_percent(*amount, true),
                    }.card(process.clone()), text!{
                        process.output.as_key(),
                        "%{changeDir} %{tag} output by [b]%{percent}%.[/b]",
                        percent = display::percent(amount.abs(), true),
                        tag = icon_card_tag(&t!(&process.name), process.output.icon()),
                        changeDir = self.change_dir(*amount),
                        icon = process.output.icon(),
                    })
            }
            Effect::OutputForFeature(feat, amount) => {
                let processes: Vec<_> = state
                    .world
                    .processes
                    .iter()
                    .filter(|p| {
                        !p.locked && p.features.contains(feat)
                    })
                    .cloned()
                    .collect();
                (tip! {
                        feat.icon(),
                        "Changes the output for these processes by %{percent}% without changing their impacts.",
                        percent = display::signed_percent(*amount, true),
                    }.card(processes),
                    text! {
                        "output",
                        r#"%{changeDir} output for [e][i]%{icon}[/i][b]%{feature}[/b][/e] by [b]%{percent}%.[/b]"#,
                        percent = display::percent(amount.abs(), true),
                        icon = feat.icon(),
                        feature = feat.lower(),
                        changeDir = self.change_dir(*amount),
                    })
            }
            Effect::CO2ForFeature(feat, amount) => {
                let processes: Vec<_> = state
                    .world
                    .processes
                    .iter()
                    .filter(|p| {
                        !p.locked && p.features.contains(feat)
                    })
                    .cloned()
                    .collect();
                (tip! {
                        feat.icon(),
                        "%{changeDir} the CO2 emissions for these processes by [b]%{percent}%.[/b]",
                        percent = display::percent(amount.abs(), true),
                        changeDir = self.change_dir(*amount),
                    }.card(processes), text! {
                        "emissions",
                        r#"%{changeDir} CO2 emissions for [e][i]%{icon}[/i]%{feature}[/e] by [b]%{percent}%.[/b]"#,
                        percent = display::percent(amount.abs(), true),
                        icon = feat.icon(),
                        feature = feat.lower(),
                        changeDir = self.change_dir(*amount),
                    })
            }
            Effect::BiodiversityPressureForFeature(
                feat,
                amount,
            ) => {
                let processes: Vec<_> = state
                    .world
                    .processes
                    .iter()
                    .filter(|p| {
                        !p.locked && p.features.contains(feat)
                    })
                    .cloned()
                    .collect();
                (tip! {
                        feat.icon(),
                        "Changes the biodiversity pressure for these processes by [b]%{amount}.[/b]",
                        amount = format!("{:+}", amount),
                    }.card(processes), text! {
                        "biodiversity",
                        r#"%{changeDir} biodiversity pressure for [e][i]%{icon}[/i]%{feature}[/e] by [b]%{amount}.[/b]"#,
                        amount = amount.abs(),
                        icon = feat.icon(),
                        feature = feat.lower(),
                        changeDir = self.change_dir(*amount),
                    })
            }
            Effect::Demand(output, amount) => {
                let demand = display::outputs(
                    &state.output_demand.total(),
                );
                let current_demand = demand[*output];
                let after_demand =
                    demand[*output] * (1. + amount);
                (
                    tip! {
                            output.icon(),
                            r#"This changes %{name} demand from [i]%{icon}[/i]%{currentDemand} to [i]%{icon}[/i]%{afterDemand}."#,
                            afterDemand = after_demand.round(),
                            currentDemand = current_demand.round(),
                            icon = output.icon(),
                            name = t!(output.lower()),
                    },
                    text! {
                        output.as_key(),
                        "%{changeDir} demand for %{name} by [b]%{percent}%[/b].",
                        percent = display::percent(amount.abs(), true),
                        changeDir = self.change_dir(*amount),
                        name = t!(&output.lower()),
                    },
                )
            }
            Effect::DemandAmount(output, amount) => {
                let demand = display::outputs(
                    &state.output_demand.total(),
                );
                let amount = display::output(*amount, *output);
                let current_demand = demand[*output];
                let after_demand = demand[*output] + amount;
                let demand_change = (after_demand
                    - current_demand)
                    / current_demand;
                (
                    tip! {
                        output.icon(),
                        r#"This changes %{name} demand from [i]%{icon}[/i]%{currentDemand} to [i]%{icon}[/i]%{afterDemand}. This is a %{percent}% change of all %{name} demand."#,
                        percent = display::signed_percent(demand_change.abs(), true),
                        afterDemand = after_demand,
                        currentDemand = current_demand,
                        icon = output.icon(),
                        name = t!(output.lower()),
                    },
                    text! {
                        output.as_key(),
                        r#"%{changeDir} demand for %{name} by [i]%{icon}[/i]%{amount}."#,
                        amount = amount.abs(),
                        icon = output.icon(),
                        name = t!(output.lower()),
                        changeDir = self.change_dir(amount),
                    },
                )
            }
            Effect::UnlocksProject(id) => {
                let project = &state.world.projects[id];
                let prob = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    prob
                } else {
                    Likelihood::Guaranteed
                };
                let tag = icon_card_tag(
                    &t!(&project.name),
                    project.kind.icon(),
                );
                let text = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    prefix_probs!(
                        prob,
                        " unlock the %{tag} project."
                    )
                } else {
                    t!("[b]Unlocks[/b] the %{tag} project.")
                };

                (
                    tip(
                        icons::UNLOCKS,
                        prefix_probs!(
                            prob,
                            " unlock this project:"
                        ),
                    )
                    .card(project.clone()),
                    text! {
                        "unlocks",
                        text.to_string(),
                        tag = tag,
                    },
                )
            }
            Effect::UnlocksProcess(id) => {
                let process = &state.world.processes[id];
                let prob = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    prob
                } else {
                    Likelihood::Guaranteed
                };
                let tag = icon_card_tag(
                    &t!(&process.name),
                    process.output.icon(),
                );
                let text = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    prefix_probs!(
                        prob,
                        " unlock the %{tag} process."
                    )
                } else {
                    t!("[b]Unlocks[/b] the %{tag} process.")
                };

                (
                    tip(
                        icons::UNLOCKS,
                        prefix_probs!(
                            prob,
                            " unlock this process:"
                        ),
                    )
                    .card(process.clone()),
                    text! {
                        "unlocks",
                        text.to_string(),
                        tag = tag,
                    },
                )
            }
            Effect::UnlocksNPC(id) => {
                let npc = &state.npcs[id];
                let text = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    prefix_probs!(prob, " unlock {name}.")
                } else {
                    t!("[b]Unlocks[/b] {name}.")
                };
                (
                    tip! {
                        icons::UNLOCKS,
                        "This new character will be unlocked:",
                    }
                    .card(npc.clone()),
                    text! {
                        "unlocks",
                        text.to_string(),
                        name = t!(&npc.name),
                    },
                )
            }
            Effect::ProjectCostModifier(id, amount) => {
                let project = &state.world.projects[id];

                let tag = icon_card_tag(
                    &t!(&project.name),
                    project.kind.icon(),
                );
                let kind = match project.kind {
                    ProjectType::Policy => t!("cost"),
                    ProjectType::Research => {
                        t!("research time")
                    }
                    ProjectType::Initiative => {
                        t!("development time")
                    }
                };
                let tip_amount = if self.is_unknown {
                    t!("an unknown amount")
                } else if project.is_policy() {
                    format!(
                        "{}% [i]political_capital[/i]",
                        display::percent(amount.abs(), true)
                    )
                    .into()
                } else {
                    format!(
                        "{}%",
                        display::percent(amount.abs(), true)
                    )
                    .into()
                };
                (
                        tip! {
                            icons::COST,
                            "This effect %{changeDir} the %{kind} of this project by %{tipAmount}.",
                            tipAmount = tip_amount,
                            kind = kind,
                            changeDir = self.change_dir(*amount).to_lowercase(),
                        }
                        .card(project.clone()),
                        text! {
                            "cost",
                            "%{changeDir} %{kind} of %{tag} by [b]%{tipAmount}[/b].",
                            changeDir = self.change_dir(*amount),
                            kind = kind,
                            tag = tag,
                            tipAmount = tip_amount,
                        },
                    )
            }
            Effect::ProjectRequest(id, active, bounty) => {
                let project = &state.world.projects[id];
                if *active {
                    (
                            tip! {
                                icons::REQUEST,
                                "You received a request to implement this project:",
                            }
                            .card(project.clone()),
                            text! {
                                "implement",
                                "I request that you implement %{name}. (+%{pc}PC)",
                                name = t!(&project.name),
                                pc = bounty,
                            },
                        )
                } else {
                    (
                            tip! {
                                icons::REQUEST,
                                "You received a request to stop this project:",
                            }
                            .card(project.clone()),
                            text! {
                                "ban",
                                "I request that you stop %{name}. (+%{pc}PC)",
                                name = t!(&project.name),
                                pc = bounty,
                            },
                        )
                }
            }
            Effect::ProcessRequest(id, active, bounty) => {
                let process = &state.world.processes[id];
                if *active {
                    (
                            tip! {
                                icons::REQUEST,
                                "You received a request to promote this process:",
                            }
                            .card(process.clone()),
                            text! {
                                "implement",
                                "I request that you implement %{name}. (+%{pc}PC)",
                                name = t!(&process.name),
                                pc = bounty,
                            },
                        )
                } else {
                    (
                            tip! {
                                icons::REQUEST,
                                "You received a request to ban this process:",
                            }
                            .card(process.clone()),
                            text! {
                                "ban",
                                "I request that you stop %{name}. (+%{pc}PC)",
                                name = t!(&process.name),
                                pc = bounty,
                            },
                        )
                }
            }
            Effect::ModifyIndustryDemand(id, amount) => {
                let industry = &state.world.industries[id];
                let tag = card_tag(&t!(&industry.name));
                let tip_text = if self.is_unknown {
                    t!(
                        "Changes demand for %{name} by an unknown amount.",
                        name = t!(&industry.name)
                    )
                } else {
                    t!(
                        "Changes demand for %{name} by [b]%{percent}%.[/b]",
                        name = t!(&industry.name),
                        percent =
                            display::percent(*amount, true),
                    )
                };
                (
                    tip(icons::DEMAND, tip_text)
                        .card(industry.clone()),
                    text! {
                        "demand",
                        "%{changeDir} demand for %{tag} by %{amount}.",
                        amount = if self.is_unknown {
                            self.fmt_param(*amount)
                        } else {
                            format!("{}%", display::percent(amount.abs(), true))
                        },
                        changeDir = self.change_dir(*amount),
                        tag = tag,
                    },
                )
            }
            Effect::ModifyIndustryResources(
                id,
                resource,
                amount,
            ) => {
                let industry = &state.world.industries[id];
                let lic_pop = state.world.lic_population();
                let current_demand = industry
                    .total_demand_for_resource(
                        lic_pop, *resource,
                    );
                let current_demand = display::resource(
                    current_demand,
                    *resource,
                    state.resources.available,
                );
                let after_demand =
                    current_demand * (1. + amount);
                let total_demand = display::resource(
                    state.resource_demand.of(*resource),
                    *resource,
                    state.resources.available,
                );
                let change = after_demand - current_demand;
                let demand_change =
                    (total_demand + change) / total_demand - 1.;
                let tag = card_tag(&t!(&industry.name));
                let tip = if self.is_unknown {
                    tip! {
                        resource.icon(),
                        "This will change %{resource} demand for %{name} by some unknown amount.",
                        name = t!(&industry.name),
                        resource = t!(resource.lower()),
                    }
                } else {
                    tip! {
                        resource.icon(),
                        r#"This will change %{resource} demand for %{name} from [i]%{icon}[/i]%{demandBefore} to [i]%{icon}[/i]%{demandAfter}. This is a %{percent}% change of all %{resource} demand."#,
                        name = t!(&industry.name),
                        resource = t!(resource.lower()),
                        icon = resource.icon(),
                        percent = display::signed_percent(demand_change, true),
                        demandAfter = if after_demand < 1. {
                            "<1".into()
                        } else {
                            after_demand.round().to_string()
                        },
                        demandBefore = current_demand.round(),
                    }
                };
                (
                    tip.card(industry.clone()),
                    text! {
                        resource.as_key(),
                        "%{changeDir} %{resource} demand for %{tag} by %{amount}.",
                        tag = tag,
                        resource = t!(resource.lower()),
                        amount = if self.is_unknown {
                            self.fmt_param(*amount)
                        } else {
                            format!("{}%", display::percent(amount.abs(), true))
                        },
                        changeDir = self.change_dir(*amount),
                    },
                )
            }
            Effect::ModifyIndustryResourcesAmount(
                id,
                resource,
                amount,
            ) => {
                let industry = &state.world.industries[id];
                let lic_pop = state.world.lic_population();
                let demand = industry.demand(lic_pop);
                let current_demand = display::resource(
                    industry.resources[*resource] * demand,
                    *resource,
                    state.resources.available,
                );
                let after_demand = display::resource(
                    (industry.resources[*resource] + amount)
                        * demand,
                    *resource,
                    state.resources.available,
                );
                let total_demand = display::resource(
                    state.resource_demand.of(*resource),
                    *resource,
                    state.resources.available,
                );
                let change = after_demand - current_demand;
                let demand_change =
                    (total_demand + change) / total_demand - 1.;
                let tag = card_tag(&t!(&industry.name));
                let tip = if self.is_unknown {
                    tip! {
                        resource.icon(),
                        "This will change %{resource} demand for %{name} by some unknown amount.",
                        name = t!(&industry.name),
                        resource = t!(resource.lower()),
                    }
                } else {
                    tip! {
                        resource.icon(),
                        r#"This will change %{resource} demand for %{name} from [i]%{icon}[/i]%{demandBefore} to [i]%{icon}[/i]%{demandAfter}. This is a %{percent}% change of all %{resource} demand."#,
                        name = t!(&industry.name),
                        resource = t!(resource.lower()),
                        icon = resource.icon(),
                        percent = display::signed_percent(demand_change, true),
                        demandAfter = if after_demand < 1. {
                            "<1".into()
                        } else {
                            after_demand.round().to_string()
                        },
                        demandBefore = current_demand.round(),
                    }
                };
                (
                    tip.card(industry.clone()),
                    text! {
                        resource.as_key(),
                        "%{changeDir} %{resource} demand for %{tag} by %{amount}.",
                        tag = tag,
                        resource = t!(resource.lower()),
                        amount = if self.is_unknown {
                            self.fmt_param(*amount)
                        } else {
                            (after_demand - current_demand).round().abs().to_string()
                        },
                        changeDir = self.change_dir(*amount),
                    },
                )
            }
            Effect::ModifyIndustryByproducts(
                id,
                byproduct,
                amount,
            ) => {
                let industry = &state.world.industries[id];
                let lic_pop = state.world.lic_population();
                let current = industry
                    .total_byproducts(lic_pop)
                    .gtco2eq();
                let after = current * (1. + amount);
                let change = (after - current)
                    / state.emissions.as_gtco2eq();
                let tag = card_tag(&t!(&industry.name));
                let tip_text = if self.is_unknown {
                    t!(
                        "Changes emissions for %{name} by an unknown amount.",
                        name = t!(&industry.name)
                    )
                } else {
                    t!(
                        r#"This will change emissions for %{name} from [i]%{icon}[/i]%{emissionsBefore} to [i]%{icon}[/i]%{emissionsAfter}. This is a %{emissionsChange}% change of all emissions."#,
                        name = t!(&industry.name),
                        icon = icons::EMISSIONS,
                        emissionsChange =
                            display::percent(change, true),
                        emissionsAfter =
                            display::rounded(after),
                        emissionsBefore =
                            display::rounded(current),
                    )
                };
                (
                    tip(icons::EMISSIONS, tip_text)
                        .card(industry.clone()),
                    text! {
                        "emissions",
                        "%{changeDir} %{kind} emissions for %{tag} by [b]%{amount}[/b].",
                            tag = tag,
                            kind = t!(byproduct.lower()),
                            amount = if self.is_unknown {
                                self.fmt_param(*amount)
                            } else {
                                format!("{}%", display::percent(amount.abs(), true))
                            },
                            changeDir = self.change_dir(*amount),

                    },
                )
            }
            Effect::ModifyProcessByproducts(
                id,
                byproduct,
                amount,
            ) => {
                let process = &state.world.processes[id];
                let label = match byproduct {
                    Byproduct::Biodiversity => {
                        t!("biodiversity pressure")
                    }
                    Byproduct::Co2 => {
                        t!(
                            "%{kind} emissions",
                            kind = t!("CO2")
                        )
                    }
                    Byproduct::N2o => {
                        t!(
                            "%{kind} emissions",
                            kind = t!("N2O")
                        )
                    }
                    Byproduct::Ch4 => {
                        t!(
                            "%{kind} emissions",
                            kind = t!("CH4")
                        )
                    }
                };
                let tag = card_tag(&t!(&process.name));
                let tip_text = if self.is_unknown {
                    t!(
                        "Changes %{label} for %{name} by an unknown amount.",
                        name = t!(&process.name),
                        label = label,
                    )
                } else {
                    let change = match byproduct {
                        Byproduct::Biodiversity => {
                            let current = process
                                .adj_byproducts()
                                .biodiversity;
                            let mut after_process =
                                process.clone();
                            after_process
                                .byproduct_modifiers
                                .biodiversity += amount;
                            let after = after_process
                                .adj_byproducts()
                                .biodiversity;
                            t!(
                                r#"%{fromAmount} to %{toAmount}[i]%{icon}[/i]."#,
                                icon = byproduct.icon(),
                                toAmount = after,
                                fromAmount = current,
                            )
                        }
                        _ => {
                            let current = process
                                .adj_byproducts()
                                .gtco2eq()
                                * state
                                    .produced
                                    .by_process
                                    .get(id)
                                    .unwrap_or(&0.);
                            let after = current * (1. + amount);
                            let change = (after - current)
                                / state.emissions.as_gtco2eq();
                            t!(
                                r#"%{emissionsBefore} to [i]%{icon}[/i]%{emissionsAfter}. This is a %{emissionsChange}% change of all emissions."#,
                                icon = icons::EMISSIONS,
                                emissionsChange =
                                    display::percent(
                                        change, true
                                    ),
                                emissionsAfter =
                                    display::rounded(after),
                                emissionsBefore =
                                    display::rounded(current)
                            )
                        }
                    };

                    t!(
                        r#"This will change %{short} for %{name} from [i]%{icon}[/i]%{change}"#,
                        name = t!(&process.name),
                        icon = byproduct.icon(),
                        short = match byproduct {
                            Byproduct::Biodiversity =>
                                t!("biodiversity pressure"),
                            _ => t!("emissions"),
                        },
                        change = change,
                    )
                };
                (
                    tip(byproduct.icon(), tip_text),
                    text! {
                        match byproduct {
                            Byproduct::Biodiversity => "biodiversity",
                            _ => "emissions",
                        },
                        "%{changeDir} %{label} for %{tag} by [b]%{amount}[/b].",
                        tag = tag,
                        label = label,
                        icon = byproduct.icon(),
                        amount = if self.is_unknown {
                            self.fmt_param(*amount)
                        } else {
                            format!("{}%", display::percent(amount.abs(), true))
                        },
                        changeDir = self.change_dir(*amount),

                    },
                )
            }
            Effect::DemandOutlookChange(output, amount) => {
                let change = mean_demand_outlook_change(
                    *amount, output, state,
                )
                .round();
                (
                        tip! {
                            icons::CONTENTEDNESS,
                            r#"This changes regional contentedness based on demand for %{name}. Current world contentedeness is %{amount}[t]/%{maxAmount}[/t]."#,
                            name = t!(output.lower()),
                            maxAmount = consts::MAX_CONTENTEDNESS,
                            amount = state.outlook().round(),
                        }.subicon(output.icon()),
                        text! {
                            "contentedness",
                            "%{changeDir} world contentedness by [b]%{amount}[/b].",
                            amount = change.abs(),
                            changeDir = self.change_dir(*amount),
                        }
                    )
            }
            Effect::IncomeOutlookChange(amount) => {
                let change =
                    mean_income_outlook_change(*amount, state)
                        .round();
                (tip! {
                        icons::CONTENTEDNESS,
                        r#"This changes regional contentedness based on income level (wealthier regions will feel it more). Current world contentedeness is %{contentedness}[t]/%{maxContentedness}[/t]."#,
                        maxContentedness = consts::MAX_CONTENTEDNESS,
                        contentedness = state.outlook().round(),
                        amount = amount,
                    }.subicon(icons::WEALTH), text! {
                        "contentedness",
                        "%{changeDir} contentedness by [b]%{amount}[/b].",
                        amount = change.abs(),
                        changeDir = self.change_dir(*amount),
                    })
            }
            Effect::ModifyEventProbability(id, amount) => {
                let event = &state.event_pool.events[id];
                let amount_label = if self.is_unknown {
                    self.fmt_param(*amount)
                } else {
                    format!(
                        "{}%",
                        display::percent(amount.abs(), true)
                    )
                };
                let text = t!(
                    r#"%{changeDir} the chance of "%{event}" by %{amount}."#,
                    event = t!(&event.name),
                    amount = amount_label,
                    changeDir = self.change_dir(*amount),
                );
                (
                    tip! {
                        icons::CHANCE,
                        text.to_string(),
                    },
                    text! {
                        "chance",
                        text.to_string(),
                    },
                )
            }
            Effect::AddFlag(flag) => {
                let tip = flag_tip(
                    *flag,
                    &state.output_demand.total(),
                );
                let text =
                    format!("[b]{}[/b]", t!(flag.to_string()));
                (tip, text)
            }
            Effect::ProtectLand(amount) => {
                let before = state.protected_land;
                let after = state.protected_land + amount;
                (
                    tip! {
                        icons::LAND,
                        "This will limit the amount of land that processes can use. The amount of land under protection will change from %{before}% to %{after}%.",
                        before = display::percent(before, true),
                        after = display::percent(after, true),
                    },
                    text! {
                        "land",
                        "Change the amount of land under protection by [b]%{percent}%[/b].",
                        percent = display::signed_percent(*amount, true),
                    },
                )
            }
            Effect::Feedstock(feedstock, amount) => {
                let estimate = match feedstock {
                    Feedstock::Other | Feedstock::Soil => None,
                    other => {
                        let est = state
                            .feedstocks
                            .until_exhaustion(*other);
                        Some(est.round())
                    }
                };
                let available =
                    state.feedstocks.available[*feedstock];
                let text = match estimate {
                    None => {
                        t!("We aren't tracking this feedstock.")
                    }
                    Some(0.) => {
                        t!("This feedstock has been depleted.")
                    }
                    Some(est) => {
                        if est.is_infinite() {
                            t!(
                                "At current usage rates the estimated supply is expected to last indefinitely."
                            )
                        } else {
                            t!(
                                "At current usage rates the estimated supply is expected to last %{years} year(s).",
                                years = est
                            )
                        }
                    }
                };
                (
                    tip! {
                        feedstock.icon(),
                        text.to_string(),
                    },
                    text! {
                        feedstock.as_key(),
                        "%{changeDir} %{name} supply by [b]%{percent}%.[/b]",
                        name = t!(feedstock.lower()),
                        percent = display::percent((amount/available).abs(), true),
                        changeDir = self.change_dir(*amount),
                    },
                )
            }
            Effect::LocksProject(id) => {
                let project = &state.world.projects[id];
                let tag = icon_card_tag(
                    &t!(&project.name),
                    project.kind.icon(),
                );
                (
                        tip! {
                            icons::ALERT,
                            "%{name} will be unavailable while this project is active.",
                            name = t!(&project.name),
                        }
                        .card(project.clone()),
                        text! {
                            "locks",
                            "[b]Locks[/b] %{tag}",
                            tag = tag,
                        },
                    )
            }
            Effect::TerminationShock => {
                if let Some(project) =
                    state.world.projects.iter().find(|p| {
                        p.name.contains(
                            "Solar Radiation Management",
                        )
                    })
                {
                    let effects = project.active_effects();
                    let mut temp_change = 0.;
                    if let Some(temp_effect) = effects
                        .iter()
                        .find_map(|eff| match eff {
                            Effect::WorldVariable(
                                WorldVariable::Temperature,
                                val,
                            ) => Some(*val),
                            _ => None,
                        })
                    {
                        temp_change -= temp_effect;
                    }
                    (
                        tip! {
                            icons::WARMING,
                            "This will directly change the global temperature anomaly by %{amount}[b]°c[/b].",
                            amount = format!("{:+}", temp_change),
                        },
                        text! {
                            "warming",
                            "%{changeDir} the global temperature by %{amount}[b]°c[/b].",
                            changeDir = self.change_dir(temp_change),
                            amount = self.fmt_param(temp_change),
                        },
                    )
                } else {
                    return Err(());
                }
            }
            Effect::AddEvent(..)
            | Effect::TriggerEvent(..)
            | Effect::RegionLeave
            | Effect::Migration
            | Effect::AddRegionFlag(..)
            | Effect::GameOver
            | Effect::BailOut(..)
            | Effect::NPCRelationship(..) => {
                return Err(());
            }
        };
        Ok(EffectTip { tip, text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex_lite::Regex;

    /// For comparing against floats
    /// but also "<1" which is what we use to
    /// indicate a small but non-zero value.
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Value {
        Small, // <1
        Exact(f32),
    }
    impl PartialEq<f32> for Value {
        fn eq(&self, other: &f32) -> bool {
            match self {
                Self::Small => false,
                Self::Exact(value) => value == other,
            }
        }
    }
    const SMALL: Value = Value::Small;

    fn extract_numbers(input: &str) -> Vec<Value> {
        let re = Regex::new(r"<?-?\d+(\.\d+)?").unwrap();
        let mut numbers = vec![];
        for cap in re.captures_iter(input) {
            if let Some(matched) = cap.get(0) {
                let s = matched.as_str();
                if s == "<1" {
                    numbers.push(Value::Small);
                } else if let Ok(num) = s.parse::<f32>() {
                    numbers.push(Value::Exact(num));
                }
            }
        }
        numbers
    }

    fn effect_values(
        state: &State,
        effect: Effect,
    ) -> (Vec<Value>, Vec<Value>) {
        println!("{:?}", effect);
        let effect = DisplayEffect {
            effect,
            is_unknown: false,
            ..Default::default()
        };
        let tip = effect.tip(&state).unwrap();
        println!("  {}", tip.text);
        println!("  {}", tip.tip.text);
        (
            extract_numbers(&tip.text),
            extract_numbers(&tip.tip.text),
        )
    }

    #[test]
    fn test_electrified_flag_tip() {
        // Formatting is in PWh, i.e. 1e12 kWh.
        let mut demand = OutputMap::default();
        demand.fuel = 1.5e12;
        demand.electricity = 2.5e12;

        // Expect 80% of fuel to go to electricity.
        // let expected_fuel =
        let tip = flag_tip(Flag::Electrified, &demand);
        println!("{}", tip.text);

        // Because of rounding these values will be
        // slightly different.
        let vals = extract_numbers(&tip.text);
        let prev_fuel_demand = vals[0];
        let next_fuel_demand = vals[1];
        let prev_elec_demand = vals[2];
        let next_elec_demand = vals[3];
        assert_eq!(prev_fuel_demand, 2.);
        assert_eq!(next_fuel_demand, SMALL);
        assert_eq!(prev_elec_demand, 3.);
        assert_eq!(next_elec_demand, 4.);
    }

    #[test]
    fn test_vegan_flag_tip() {
        // Formatting is per 20,000 TCals.
        let mut demand = OutputMap::default();
        demand.animal_calories = 1.5e9 * 2e4;
        demand.plant_calories = 2.5e9 * 2e4;

        // Expect 80% of fuel to go to electricity.
        // let expected_fuel =
        let tip = flag_tip(Flag::Vegan, &demand);
        println!("{}", tip.text);

        let vals = extract_numbers(&tip.text);
        let prev_anim_demand = vals[0];
        let next_anim_demand = vals[1];
        let prev_plant_demand = vals[2];
        let next_plant_demand = vals[3];
        assert_eq!(prev_anim_demand, 2.);
        assert_eq!(next_anim_demand, SMALL);
        assert_eq!(prev_plant_demand, 3.);
        assert_eq!(next_plant_demand, 4.);
    }

    #[test]
    fn test_vegetarian_flag_tip() {
        // Formatting is per 20,000 TCals.
        let mut demand = OutputMap::default();
        demand.animal_calories = 3e9 * 2e4;
        demand.plant_calories = 5e9 * 2e4;

        // Expect 80% of fuel to go to electricity.
        // let expected_fuel =
        let tip = flag_tip(Flag::Vegetarian, &demand);
        println!("{}", tip.text);

        let vals = extract_numbers(&tip.text);
        let prev_anim_demand = vals[0];
        let next_anim_demand = vals[1];
        let prev_plant_demand = vals[2];
        let next_plant_demand = vals[3];
        assert_eq!(prev_anim_demand, 3.);
        assert_eq!(next_anim_demand, 1.);
        assert_eq!(prev_plant_demand, 5.);
        assert_eq!(next_plant_demand, 7.);
    }

    #[test]
    fn test_world_variables() {
        let get_values =
            move |var: WorldVariable, amount: f32| {
                let state = State::default();
                effect_values(
                    &state,
                    Effect::WorldVariable(var, amount),
                )
            };

        let (text_vals, _) =
            get_values(WorldVariable::Outlook, 12.);
        assert_eq!(text_vals[0], 12.);

        let (text_vals, tip_vals) =
            get_values(WorldVariable::Emissions, 2.5);
        assert_eq!(text_vals[0], 2.5);
        assert_eq!(tip_vals[0], 2.5);
        assert_eq!(tip_vals[1], 5.0);

        let (text_vals, tip_vals) =
            get_values(WorldVariable::ExtinctionRate, 7.);
        assert_eq!(text_vals[0], 7.);
        assert_eq!(tip_vals[0], 90.);

        let (text_vals, tip_vals) =
            get_values(WorldVariable::Temperature, 1.5);
        assert_eq!(text_vals[0], 1.5);
        assert_eq!(tip_vals[0], 1.5);

        let (text_vals, tip_vals) =
            get_values(WorldVariable::Precipitation, 0.2);
        assert_eq!(text_vals[0], 0.2);
        assert_eq!(tip_vals[0], 0.2);

        let (text_vals, _) =
            get_values(WorldVariable::PopulationGrowth, 0.5);
        assert_eq!(text_vals[0], 50.);

        let (text_vals, _) =
            get_values(WorldVariable::Population, 120.);
        assert_eq!(text_vals[0], 120.);

        let (text_vals, tip_vals) =
            get_values(WorldVariable::SeaLevelRiseRate, 0.5);
        assert_eq!(text_vals[0], 500.);
        assert_eq!(tip_vals[0], 0.09);

        let (text_vals, tip_vals) =
            get_values(WorldVariable::SeaLevelRise, 0.25);
        assert_eq!(text_vals[0], 250.);
        assert_eq!(tip_vals[0], 0.09);
    }

    #[test]
    fn test_process_limit() {
        let state = State::default();
        let process_id = state.world.processes.by_idx(3).id;
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ProcessLimit(process_id, 1.539e15),
        );
        assert_eq!(text_vals[0], 50.);
        assert_eq!(tip_vals[0], 50.);
    }

    #[test]
    fn test_regional_habitability() {
        let state = State::default();
        let (text_vals, _) = effect_values(
            &state,
            Effect::RegionHabitability(Latitude::Tropic, 3.),
        );
        assert_eq!(text_vals[0], 3.);
    }

    #[test]
    fn test_resource() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::Resource(Resource::Water, 1e15),
        );
        assert_eq!(text_vals[0], 2.);
        assert_eq!(tip_vals[0], 2.);
    }

    #[test]
    fn test_output() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::Output(Output::Fuel, 0.2),
        );
        assert_eq!(text_vals[0], 20.);
        assert_eq!(tip_vals[0], 89.);
        assert_eq!(tip_vals[1], 106.);
    }

    #[test]
    fn test_output_for_process() {
        let state = State::default();
        let process_id = state.world.processes.by_idx(7).id;
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::OutputForProcess(process_id, 0.2),
        );
        assert_eq!(text_vals[0], 20.);
        assert_eq!(tip_vals[0], 20.);
    }

    #[test]
    fn test_output_for_feature() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::OutputForFeature(
                ProcessFeature::UsesLivestock,
                0.25,
            ),
        );
        assert_eq!(text_vals[0], 25.);
        assert_eq!(tip_vals[0], 25.);
    }

    #[test]
    fn test_co2_for_feature() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::CO2ForFeature(
                ProcessFeature::UsesLivestock,
                0.15,
            ),
        );
        assert_eq!(text_vals[1], 15.);
        assert_eq!(tip_vals[1], 15.);
    }

    #[test]
    fn test_biodiversity_pressure_for_feature() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::BiodiversityPressureForFeature(
                ProcessFeature::UsesLivestock,
                2.,
            ),
        );
        assert_eq!(text_vals[0], 2.);
        assert_eq!(tip_vals[0], 2.);
    }

    #[test]
    fn test_demand() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::Demand(Output::Electricity, 0.25),
        );
        assert_eq!(text_vals[0], 25.);
        assert_eq!(tip_vals[0], 26.);
        assert_eq!(tip_vals[1], 33.);
    }

    #[test]
    fn test_demand_amount() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::DemandAmount(Output::Electricity, 3e12),
        );
        assert_eq!(text_vals[0], 3.);
        assert_eq!(tip_vals[0], 26.);
        assert_eq!(tip_vals[1], 29.);
        assert_eq!(tip_vals[2], 12.);
    }

    #[test]
    fn test_project_cost_modifier() {
        let state = State::default();

        let project = state.world.projects.by_idx(7);
        assert_eq!(project.kind, ProjectType::Initiative);
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ProjectCostModifier(project.id, 0.5),
        );
        assert_eq!(text_vals[0], 50.);
        assert_eq!(tip_vals[0], 50.);

        let project = state.world.projects.by_idx(8);
        assert_eq!(project.kind, ProjectType::Research);
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ProjectCostModifier(project.id, 0.25),
        );
        assert_eq!(text_vals[0], 25.);
        assert_eq!(tip_vals[0], 25.);

        let project = state.world.projects.by_idx(1);
        assert_eq!(project.kind, ProjectType::Policy);
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ProjectCostModifier(project.id, 2.),
        );
        assert_eq!(text_vals[0], 200.);
        assert_eq!(tip_vals[0], 200.);
    }

    #[test]
    fn test_modify_industry_demand() {
        let state = State::default();
        let industry_id = state.world.industries.by_idx(2).id;
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ModifyIndustryDemand(industry_id, 0.05),
        );
        assert_eq!(text_vals[0], 5.);
        assert_eq!(tip_vals[0], 5.);
    }

    #[test]
    fn test_modify_industry_resources() {
        let state = State::default();
        let industry_id = state.world.industries.by_idx(2).id;
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ModifyIndustryResources(
                industry_id,
                Resource::Fuel,
                0.2,
            ),
        );
        assert_eq!(text_vals[0], 20.);
        assert_eq!(tip_vals[0], 4.);
        assert_eq!(tip_vals[1], 5.);
        assert_eq!(tip_vals[2], SMALL);
    }

    #[test]
    fn test_modify_industry_resources_amount() {
        let state = State::default();
        let industry_id = state.world.industries.by_idx(2).id;
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ModifyIndustryResourcesAmount(
                industry_id,
                Resource::Fuel,
                1000.,
            ),
        );
        assert_eq!(text_vals[0], 42.);
        assert_eq!(tip_vals[0], 4.);
        assert_eq!(tip_vals[1], 46.);
        assert_eq!(tip_vals[2], 48.);
    }

    #[test]
    fn test_modify_industry_byproducts() {
        let mut state = State::default();

        // Should be the concrete industry, which has
        // direct CO2 emissions. But even then we need to
        // increase direct emissions to have a more noticeable effect.
        let industry = state.world.industries.by_idx_mut(7);
        let industry_id = industry.id;
        industry.byproducts.co2 *= 10000.;

        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ModifyIndustryByproducts(
                industry_id,
                Byproduct::Co2,
                0.6,
            ),
        );
        assert_eq!(text_vals[1], 60.);
        assert_eq!(tip_vals[0], 16.);
        assert_eq!(tip_vals[1], 26.);
        assert_eq!(tip_vals[2], 19.);
    }

    #[test]
    fn test_modify_process_byproducts() {
        let state = State::default();
        let process_id = state.world.processes.by_idx(15).id;
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ModifyProcessByproducts(
                process_id,
                Byproduct::Co2,
                0.6,
            ),
        );
        assert_eq!(text_vals[1], 60.);
        assert_eq!(tip_vals[0], 4.);
        assert_eq!(tip_vals[1], 6.);
        assert_eq!(tip_vals[2], 4.);

        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ModifyProcessByproducts(
                process_id,
                Byproduct::Biodiversity,
                0.6,
            ),
        );
        assert_eq!(text_vals[0], 60.);
        assert_eq!(tip_vals[0], 1.);
        assert_eq!(tip_vals[1], 1.6);
    }

    #[test]
    fn test_demand_outlook_change() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::DemandOutlookChange(Output::Fuel, 1.),
        );
        assert_eq!(text_vals[0], 3.);
        assert_eq!(tip_vals[0], 30.);
        assert_eq!(tip_vals[1], 40.);
    }

    #[test]
    fn test_income_outlook_change() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::IncomeOutlookChange(3.),
        );
        assert_eq!(text_vals[0], 5.);
        assert_eq!(tip_vals[0], 30.);
        assert_eq!(tip_vals[1], 40.);
    }

    #[test]
    fn test_modify_event_probability() {
        let state = State::default();
        let event_id = state.world.events.by_idx(15).id;
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::ModifyEventProbability(event_id, 0.2),
        );
        assert_eq!(text_vals[0], 20.);
        assert_eq!(tip_vals[0], 20.);
    }

    #[test]
    fn test_protect_land() {
        let state = State::default();
        let (text_vals, tip_vals) =
            effect_values(&state, Effect::ProtectLand(0.25));
        assert_eq!(text_vals[0], 25.);
        assert_eq!(tip_vals[0], 10.);
        assert_eq!(tip_vals[1], 35.);
    }

    #[test]
    fn test_feedstock() {
        let state = State::default();
        let (text_vals, tip_vals) = effect_values(
            &state,
            Effect::Feedstock(Feedstock::Oil, 7e15),
        );
        assert_eq!(text_vals[0], 865.);
        assert_eq!(tip_vals[0], 54.);
    }

    #[test]
    fn test_termination_shock() {
        let state = State::default();
        let (text_vals, tip_vals) =
            effect_values(&state, Effect::TerminationShock);
        assert_eq!(text_vals[0], 0.5);
        assert_eq!(tip_vals[0], 0.5);
    }
}
