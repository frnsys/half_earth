use super::{tip, Tip};
use crate::{
    consts,
    display::{self, AsText},
    icons::{self, HasIcon},
    t,
    views::factors::factors_card,
};
use hes_engine::{
    events::*,
    kinds::{
        Byproduct,
        Feedstock,
        Output,
        OutputMap,
        Resource,
    },
    projects::Type,
    state::State,
};

// TODO ideally can get rid of this, just using for icons below
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
impl AsKey for Type {
    fn as_key(&self) -> &'static str {
        match self {
            Type::Policy => "policy",
            Type::Research => "research",
            Type::Initiative => "initiative",
        }
    }
}

fn icon_card_tag(name: &str, icon: &str) -> String {
    format!(
        r#"<div class="card-tag"><img src="{icon}">{name}</div>"#
    )
}
fn card_tag(name: &str) -> String {
    format!(r#"<div class="card-tag">{name}</div>"#)
}

fn lt1(value: f32) -> String {
    if value > 0. && value < 1. {
        "<1".into()
    } else {
        format!("{:.1}", value)
    }
}

macro_rules! tip {
        ($icon:expr, $template:expr $(, $key:ident : $value:expr)* $(,)?) => {
            tip($icon, t!($template $(, $key: $value)*))
        };
    }

macro_rules! text {
        ($icon:expr, $template:expr $(, $key:ident : $value:expr)* $(,)?) => {
            format!("[{}] {}", $icon, t!($template $(, $key: $value)*))
        };
    }

pub fn flag_tip(flag: Flag, demand: &OutputMap) -> Tip {
    match flag {
        Flag::Electrified => {
            let changed_demand = (demand.fuel * 0.8).round();
            tip! {
                icons::ELECTRICITY,
                r#"Fuel demand will change from <img src="{iconFuel}">{prevDemandFuel} to <img src="{iconFuel}">{nextDemandFuel} and electricity demand will change from <img src="{iconElec}">{prevDemandElec} to <img src="{iconElec}">{nextDemandElec}."#,
                iconFuel: icons::FUEL,
                iconElec: icons::ELECTRICITY,
                prevDemandFuel: demand.fuel,
                nextDemandFuel: demand.fuel - changed_demand,
                prevDemandElec: demand.electricity,
                nextDemandElec: demand.electricity + changed_demand,
            }
        }
        Flag::Vegan => {
            let changed_demand =
                (demand.animal_calories * 0.9).round();
            tip! {
                icons::PLANT_CALORIES,
                r#"Animal calorie demand will change from <img src="{iconACals}">{prevDemandACals} to <img src="{iconACals}">{nextDemandACals} and plant calorie demand will change from <img src="{iconPCals}">{prevDemandPCals} to <img src="{iconPCals}">{nextDemandPCals}."#,
                iconACals: icons::ANIMAL_CALORIES,
                iconPCals: icons::PLANT_CALORIES,
                prevDemandACals: demand.animal_calories,
                nextDemandACals: demand.animal_calories - changed_demand,
                prevDemandPCals: demand.plant_calories,
                nextDemandPCals: demand.plant_calories + changed_demand,
            }
        }
        Flag::Vegetarian => {
            let changed_demand =
                (demand.animal_calories * 0.75).round();
            tip! {
                icons::PLANT_CALORIES,
                r#"Animal calorie demand will change from <img src="{iconACals}">{prevDemandACals} to <img src="{iconACals}">{nextDemandACals} and plant calorie demand will change from <img src="{iconPCals}">{prevDemandPCals} to <img src="{iconPCals}">{nextDemandPCals}."#,
                iconACals: icons::ANIMAL_CALORIES,
                iconPCals: icons::PLANT_CALORIES,
                prevDemandACals: demand.animal_calories,
                nextDemandACals: demand.animal_calories - changed_demand,
                prevDemandPCals: demand.plant_calories,
                nextDemandPCals: demand.plant_calories + changed_demand,
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
                r#"Research points are 1<img src="{iconPC}"> cheaper."#,
                iconPC: icons::PLANT_CALORIES
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
                "Repeat the tutorial"
            }
        }
        Flag::SkipTutorial => {
            tip! {
                icons::ALERT,
                "Skip the tutorial"
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplayEffect {
    pub effect: Effect,
    pub likelihood: Option<Likelihood>,
    pub is_unknown: bool,
    pub is_hidden: bool,
}

pub struct EffectTip {
    pub tip: Tip,
    pub text: String,
}

impl DisplayEffect {
    fn fmt_param(&self, value: f32) -> String {
        if self.is_unknown {
            r#"<span class="unknown-param">?</span>"#.into()
        } else {
            format!("<strong>{}</strong>", value.abs())
        }
    }

    fn change_dir(&self, change: f32) -> String {
        if self.is_unknown {
            t!("Changes")
        } else if let Some(prob) = self.likelihood {
            let term =
                if change < 0. { "reduce" } else { "increase" };
            t!(&format!("{prob} {term}"))
        } else {
            let term = if change < 0. {
                t!("Reduces")
            } else {
                t!("Increases")
            };
            format!("<strong>{term}</strong>")
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
                        r#"Current world contentedness is {contentedness}<span class="type-total">/{maxContentedness}</span>."#,
                        contentedness: state.outlook().round(),
                        maxContentedness: consts::MAX_CONTENTEDNESS,
                    },
                    text! {
                        "contentedness",
                        "{changeDir} world contentedness by {amount}.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(*amount),
                    },
                ),
                WorldVariable::Emissions => (
                    tip! {
                        icons::EMISSIONS,
                        "This will directly change annual emissions by {amount}.{percent}",
                        amount: if self.is_unknown {
                            t!("an unknown amount")
                        } else {
                            format!("{:+}", amount)
                        },
                        percent: if self.is_unknown {
                            "".into()
                        } else {
                            let percent = (amount/state.emissions_gt() * 100.).round();
                            format!(" {}", t!("That's a {percent}% change.", percent: percent))
                        }
                    },
                    text! {
                        "emissions",
                        "{changeDir} emissions by {amount}.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(*amount)
                    },
                ),
                WorldVariable::ExtinctionRate => (
                    tip! {
                        icons::EXTINCTION_RATE,
                        r#"Current biodiversity pressure is {amount}<span class="type-total">/{maxAmount}</span>."#,
                        amount: state.world.extinction_rate.round(),
                        maxAmount: consts::MAX_BIODIVERSITY
                    },
                    text! {
                        "extinction_rate",
                        "{changeDir} biodiversity pressure by {amount}.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(*amount)
                    },
                ),
                WorldVariable::Temperature => (
                    tip! {
                        icons::WARMING,
                        "This will directly change the global temperature anomaly by {amount}<strong>°c</strong>.",
                        amount: format!("{:+}", amount)
                    },
                    text! {
                        "warming",
                        "{changeDir} the global temperature by {amount}<strong>°c</strong>.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(*amount)
                    },
                ),
                WorldVariable::Precipitation => (
                    tip! {
                        icons::WATER,
                        "This will directly change global precipitation by {amount}<strong>cm/yr</strong>.",
                        amount: format!("{:+}", amount)
                    },
                    text! {
                        "water",
                        "{changeDir} global precipitation by {amount}<strong>cm/yr</strong>.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(*amount)
                    },
                ),
                WorldVariable::PopulationGrowth => (
                    tip! {
                        icons::POPULATION,
                        "The number of people on the planet.",
                    },
                    text! {
                        "population",
                        "{changeDir} global population growth by {amount}<strong>%.</strong>",
                        changeDir: self.change_dir(*amount),
                        amount: display::percent(*amount, false)
                    },
                ),
                WorldVariable::Population => (
                    tip! {
                        icons::POPULATION,
                        "The number of people on the planet.",
                    },
                    text! {
                        "population",
                        "{changeDir} global population by {amount}.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(*amount)
                    },
                ),
                WorldVariable::SeaLevelRiseRate => (
                    tip! {
                        icons::SEA_LEVEL_RISE,
                        "The amount of sea level rise is currently {amount}m.",
                        amount: format!("{:.2}", state.world.sea_level_rise)
                    },
                    text! {
                        "sea_level_rise",
                        "{changeDir} the rate of sea level rise by {amount}mm/year.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(amount * 1000.)
                    },
                ),
                WorldVariable::SeaLevelRise => (
                    tip! {
                        icons::SEA_LEVEL_RISE,
                        "The amount of sea level rise is currently {amount}m.",
                        amount: format!("{:.2}", state.world.sea_level_rise)
                    },
                    text! {
                        "sea_level_rise",
                        "{changeDir} the amount of sea level rise by {amount}mm/year.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(*amount)
                    },
                ),
                WorldVariable::Year => (
                    tip! {
                        icons::HELP,
                        "The year is currently {year}.",
                        year: state.world.year,
                    },
                    text! {
                        "help",
                        "{changeDir} the year by {amount}.",
                        changeDir: self.change_dir(*amount),
                        amount: self.fmt_param(*amount)
                    },
                ),
            },
            Effect::PlayerVariable(var, amount) => match var {
                PlayerVariable::ResearchPoints => (
                    tip! {
                        icons::RESEARCH,
                        "Research points: Allocate them to research projects!",
                    },
                    text! {
                        "research",
                        "{random}+{amount} research points.",
                        random: if self.is_unknown {
                            format!("{} ", t!("Possible"))
                        } else {
                            "".into()
                        },
                        amount: self.fmt_param(*amount)
                    },
                ),
                PlayerVariable::PoliticalCapital => (
                    tip! {
                        icons::POLITICAL_CAPITAL,
                        r#"How much political capital you have. Political capital is what you spend to implement your plans. <b class="tip-warn">If you run out you\'ll be pushed out of government.</b>"#,
                    },
                    text! {
                        "political_capital",
                        "{random}+{amount} political capital.",
                        random: if self.is_unknown {
                            format!("{} ", t!("Possible"))
                        } else {
                            "".into()
                        },
                        amount: self.fmt_param(*amount)
                    },
                ),
                PlayerVariable::YearsToDeath => return Err(()),
            },
            Effect::ProcessLimit(id, amount) => {
                let process = &state.world.processes[id];
                let change = if let Some(limit) = process.limit
                {
                    let p = (amount / limit).abs();
                    display::percent(p, true)
                } else {
                    amount.round().to_string()
                };
                let text = t!("{changeDir} maximum output for {process} by <strong>{amount}</strong>",
                amount: change,
                process: t!(&process.name),
                changeDir: self.change_dir(*amount),
                );
                (tip(icons::ALERT, text.clone()), text)
            }
            Effect::RegionHabitability(lat, amount) => (
                tip! {
                    icons::HABITABILITY,
                    "Lower habitability means unhappier people who may need to migrate to more hospitable locales.",
                },
                text! {
                    "habitability",
                    "{changeDir} habitability in {type} regions by {amount}.",
                    changeDir: self.change_dir(*amount),
                    amount: self.fmt_param(*amount),
                    type: t!(lat.lower()),
                },
            ),
            Effect::Resource(resource, amount) => {
                let fmtted =
                    display::resource(*amount, *resource).abs();
                let percent = display::percent(
                    amount / state.resources[*resource],
                    true,
                );
                (
                        tip! {
                            resource.icon(),
                            r#"{changeDir} {name} supply by <img src="{icon}">{amount} ({percent}% of current supply)."#,
                            percent: percent,
                            amount: fmtted,
                            icon: resource.icon(),
                            name: t!(resource.lower()),
                            changeDir: self.change_dir(*amount),
                        }.card(factors_card(None, (*resource).into(), state)),
                        text! {
                            resource.as_key(),
                            "{changeDir} {name} supply by [{icon}]{amount}.",
                            amount: fmtted,
                            name: t!(resource.lower()),
                            changeDir: self.change_dir(*amount),
                            icon: resource.as_key(),

                        },
                    )
            }
            Effect::Output(output, amount) => {
                let base = display::output(*amount, *output);
                let changed = base * (1. + amount);
                (
                    tip! {
                        output.icon(),
                        r#"Global {name} output will change from <img src="{icon}">{base} to <img src="{icon}">{changed} with no change in impacts."#,
                        changed: changed,
                        base: base,
                        icon: output.icon(),
                        name: output.lower(),
                    },
                    text! {
                        output.as_key(),
                        "{changeDir} all {name} production by <strong>{percent}%.</strong>",
                        percent: display::percent(amount.abs(), true),
                        name: t!(output.lower()),
                        changeDir: self.change_dir(*amount),
                        icon: output.as_key(),
                    },
                )
            }
            Effect::OutputForProcess(id, amount) => {
                let process = &state.world.processes[id];
                (tip! {
                        process.output.icon(),
                        "Changes the output for this process by {percent}% with no change in impacts.",
                        percent: display::percent(*amount, true),
                    }.card(process.clone()), text!{
                        process.output.as_key(),
                        "{changeDir} {tag} output by <strong>{percent}%.</strong>",
                        percent: display::percent(amount.abs(), true),
                        tag: icon_card_tag(&t!(&process.name), process.output.icon()),
                        changeDir: self.change_dir(*amount),
                        icon: process.output.icon(),
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
                        "Changes the output for these processes by {percent}% without changing their impacts.",
                        percent: display::percent(*amount, true),
                    }.card(processes),
                    text! {
                        "output",
                        r#"{changeDir} output for <span><img class="effect-feature" src="{icon}" /><strong>{feature}</strong></span> by <strong>{percent}%.</strong>"#,
                        percent: display::percent(amount.abs(), true),
                        icon: feat.icon(),
                        feature: feat.lower(),
                        changeDir: self.change_dir(*amount),
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
                        "{changeDir} the CO2 emissions for these processes by <strong>{percent}%.</strong>",
                        percent: display::percent(amount.abs(), true),
                        changeDir: self.change_dir(*amount),
                    }.card(processes), text! {
                        "emissions",
                        r#"{changeDir} CO2 emissions for <span><img class="effect-feature" src="{icon}" />{feature}</span> by <strong>{percent}%.</strong>"#,
                        percent: display::percent(amount.abs(), true),
                        icon: feat.icon(),
                        feature: feat.lower(),
                        changeDir: self.change_dir(*amount),
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
                        "Changes the biodiversity pressure for these processes by <strong>{amount}.</strong>",
                        amount: amount.abs(),
                        changeDir: self.change_dir(*amount),
                    }.card(processes), text! {
                        "biodiversity",
                        r#"{changeDir} biodiversity pressure for <span><img class="effect-feature" src="{icon}" />{feature}</span> by <strong>{amount}.</strong>"#,
                        amount: amount.abs(),
                        icon: feat.icon(),
                        feature: feat.lower(),
                        changeDir: self.change_dir(*amount),
                    })
            }
            Effect::Demand(output, amount) => {
                let demand =
                    display::outputs(&state.output_demand);
                let current_demand = demand[*output];
                let after_demand =
                    demand[*output] * (1. + amount);
                (
                    tip! {
                            output.icon(),
                            r#"This changes {name} demand from <img src="{icon}">{currentDemand} to <img src="{icon}">{afterDemand}."#,
                            afterDemand: after_demand,
                            currentDemand: current_demand,
                            icon: output.icon(),
                            name: t!(output.lower()),
                    },
                    text! {
                        output.as_key(),
                        "{changeDir} demand for {name} by <strong>{percent}%</strong>.",
                        percent: display::percent(amount.abs(), true),
                        changeDir: self.change_dir(*amount),
                        name: t!(&output.lower()),
                    },
                )
            }
            Effect::DemandAmount(output, amount) => {
                let demand =
                    display::outputs(&state.output_demand);
                let amount = display::output(*amount, *output);
                let current_demand = demand[*output];
                let after_demand = demand[*output] + amount;
                let demand_change = (after_demand
                    - current_demand)
                    / state.world.demand()[*output];
                (
                    tip! {
                        output.icon(),
                        r#"This changes {name} demand from <img src="{icon}">{currentDemand} to <img src="{icon}">{afterDemand}. This is a {percent}% change of all {name} demand."#,
                        percent: display::percent(demand_change.abs(), true),
                        afterDemand: after_demand,
                        currentDemand: current_demand,
                        icon: output.icon(),
                        name: t!(output.lower()),
                    },
                    text! {
                        output.as_key(),
                        r#"{changeDir} demand for {name} by <img src="{icon}">{amount}."#,
                        amount: amount.abs(),
                        icon: output.icon(),
                        name: t!(output.lower()),
                        changeDir: self.change_dir(amount),
                    },
                )
            }
            Effect::UnlocksProject(id) => {
                let project = &state.world.projects[id];
                let prob = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    prob.to_string()
                } else {
                    "Will".into()
                };
                let tag = icon_card_tag(
                    &t!(&project.name),
                    project.kind.icon(),
                );
                let text = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    t!(&format!(
                        "{prob} unlock the {{tag}} project."
                    ))
                } else {
                    t!("<strong>Unlocks</strong> the {tag} project.")
                };

                (
                    tip! {
                        icons::UNLOCKS,
                        &format!("{prob} unlock this project:"),
                    }
                    .card(project.clone()),
                    text! {
                        "unlocks",
                        &text,
                        tag: tag,
                    },
                )
            }
            Effect::UnlocksProcess(id) => {
                let process = &state.world.processes[id];
                let prob = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    prob.to_string()
                } else {
                    "Will".into()
                };
                let tag = icon_card_tag(
                    &t!(&process.name),
                    process.output.icon(),
                );
                let text = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    t!(&format!(
                        "{prob} unlock the {{tag}} process."
                    ))
                } else {
                    t!("<strong>Unlocks</strong> the {tag} process.")
                };

                (
                    tip! {
                        icons::UNLOCKS,
                        &format!("{prob} unlock this process:"),
                    }
                    .card(process.clone()),
                    text! {
                        "unlocks",
                        &text,
                        tag: tag,
                    },
                )
            }
            Effect::UnlocksNPC(id) => {
                let npc = &state.npcs[id];
                let text = if self.is_unknown
                    && let Some(prob) = self.likelihood
                {
                    t!(&format!("{prob} unlock {{name}}."))
                } else {
                    t!("<strong>Unlocks</strong> {name}.")
                };
                (
                        tip! {
                            icons::UNLOCKS,
                            &format!("This new character will be unlocked:"),
                        }
                        .card(npc.clone()),
                        text! {
                            "unlocks",
                            &text,
                            name: t!(&npc.name),
                        },
                    )
            }
            Effect::ProjectCostModifier(id, amount) => {
                let project = &state.world.projects[id];
                let abs_amount = project.cost as f32 * amount;
                let tag = icon_card_tag(
                    &t!(&project.name),
                    project.kind.icon(),
                );
                let kind = match project.kind {
                    Type::Policy => t!("cost"),
                    Type::Research => t!("research time"),
                    Type::Initiative => {
                        t!("development time")
                    }
                };
                let unit = match project.kind {
                    Type::Policy => "".into(),
                    _ => format!(" {}", t!("years")),
                };
                let tip_icon = match project.kind {
                    Type::Policy => format!(
                        r#"<img src="{}" />"#,
                        icons::POLITICAL_CAPITAL
                    ),
                    _ => "".into(),
                };
                let tip_amount = if self.is_unknown {
                    t!("by an unknown amount")
                } else {
                    t!("from {tipIcon}{cost}{unit} to {tipIcon}{newCost}{unit}.",
                    newCost: (project.cost as f32 + abs_amount).round(),
                    unit: unit,
                    cost: project.cost,
                    tipIcon: tip_icon,
                    )
                };

                let icon = match project.kind {
                    Type::Policy => "[political_capital]",
                    _ => "",
                };

                (
                        tip! {
                            icons::COST,
                            "This effect {changeDir} the {kind} of this project {tipAmount}.",
                            tipAmount: tip_amount,
                            kind: kind,
                            changeDir: self.change_dir(*amount).to_lowercase(),
                        }
                        .card(project.clone()),
                        text! {
                            "cost",
                            "{changeDir} {kind} of {tag} by {icon}{amount}{unit}.",
                            changeDir: self.change_dir(*amount),
                            kind: kind,
                            tag: tag,
                            icon: icon,
                            unit: unit,
                            amount: self.fmt_param(amount.abs().ceil())
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
                                "I request that you implement {name}. (+{pc}PC)",
                                name: t!(&project.name),
                                pc: bounty,
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
                                "I request that you stop {name}. (+{pc}PC)",
                                name: t!(&project.name),
                                pc: bounty,
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
                                "I request that you implement {name}. (+{pc}PC)",
                                name: t!(&process.name),
                                pc: bounty,
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
                                "I request that you stop {name}. (+{pc}PC)",
                                name: t!(&process.name),
                                pc: bounty,
                            },
                        )
                }
            }
            Effect::ModifyIndustryDemand(id, amount) => {
                let industry = &state.world.industries[id];
                let tag = card_tag(&t!(&industry.name));
                let tip_text = if self.is_unknown {
                    t!("Changes demand for {name} by an unknown amount.",
                            name: t!(&industry.name))
                } else {
                    t!("Changes demand for {name} by <strong>{percent}%.</strong>",
                    name: t!(&industry.name),
                    percent: display::percent(*amount, true),
                    )
                };
                (
                    tip! {
                        icons::DEMAND,
                        &tip_text,
                    }
                    .card(industry.clone()),
                    text! {
                        "demand",
                        "{changeDir} demand for {tag} by {amount}.",
                        amount: if self.is_unknown {
                            self.fmt_param(*amount)
                        } else {
                            display::percent(amount.abs(), true)
                        },
                        changeDir: self.change_dir(*amount),
                        tag: tag,
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
                let current_demand = display::resource(
                    industry.resources[*resource]
                        * industry.demand(lic_pop),
                    *resource,
                );
                let after_demand =
                    current_demand * (1. + amount);
                let demand_change = (after_demand
                    - current_demand)
                    / state.resources_demand[*resource];
                let tag = card_tag(&t!(&industry.name));
                let tip = if self.is_unknown {
                    tip! {
                        resource.icon(),
                        "This will change {resource} demand for {name} by some unknown amount.",
                        name: t!(&industry.name),
                        resource: t!(resource.lower()),
                    }
                } else {
                    tip! {
                        resource.icon(),
                        r#"This will change {resource} demand for {name} from <img src="{icon}">{demandBefore} to <img src="{icon}">{demandAfter}. This is a {percent}% change of all {resource} demand."#,
                        name: t!(&industry.name),
                        resource: t!(resource.lower()),
                        icon: resource.icon(),
                        percent: display::percent(demand_change, true),
                        demandAfter: if after_demand < 1. {
                            "<1".into()
                        } else {
                            after_demand.round().to_string()
                        },
                        demandBefore: current_demand.round(),
                    }
                };
                (
                    tip.card(industry.clone()),
                    text! {
                        resource.as_key(),
                        "{changeDir} {resource} demand for {tag} by {amount}.",
                        tag: tag,
                        resource: t!(resource.lower()),
                        amount: if self.is_unknown {
                            self.fmt_param(*amount)
                        } else {
                            display::percent(amount.abs(), true)
                        },
                        changeDir: self.change_dir(*amount),
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
                );
                let after_demand = display::resource(
                    (industry.resources[*resource] + amount)
                        * demand,
                    *resource,
                );
                let demand_change = (after_demand
                    - current_demand)
                    / state.resources_demand[*resource];
                let tag = card_tag(&t!(&industry.name));
                let tip = if self.is_unknown {
                    tip! {
                        resource.icon(),
                        "This will change {resource} demand for {name} by some unknown amount.",
                        name: t!(&industry.name),
                        resource: t!(resource.lower()),
                    }
                } else {
                    tip! {
                        resource.icon(),
                        r#"This will change {resource} demand for {name} from <img src="{icon}">{demandBefore} to <img src="{icon}">{demandAfter}. This is a {percent}% change of all {resource} demand."#,
                        name: t!(&industry.name),
                        resource: t!(resource.lower()),
                        icon: resource.icon(),
                        percent: display::percent(demand_change, true),
                        demandAfter: if after_demand < 1. {
                            "<1".into()
                        } else {
                            after_demand.round().to_string()
                        },
                        demandBefore: current_demand.round(),
                    }
                };
                (
                    tip.card(industry.clone()),
                    text! {
                        resource.as_key(),
                        "{changeDir} {resource} demand for {tag} by {amount}.",
                        tag: tag,
                        resource: t!(resource.lower()),
                        amount: if self.is_unknown {
                            self.fmt_param(*amount)
                        } else {
                            (after_demand - current_demand).round().abs().to_string()
                        },
                        changeDir: self.change_dir(*amount),
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
                let demand = industry.demand(lic_pop);
                let current =
                    industry.byproducts.gtco2eq() * demand;
                let after = current * (1. + amount);
                let change =
                    (after - current) / state.emissions_gt();
                let tag = card_tag(&t!(&industry.name));
                let tip_text = if self.is_unknown {
                    t!("Changes emissions for {name} by an unknown amount.",
                            name: t!(&industry.name))
                } else {
                    t!(r#"This will change emissions for {name} from <img src="{icon}">{emissionsBefore} to <img src="{icon}">{emissionsAfter}. This is a {emissionsChange}% change of all emissions."#,
                    name: t!(&industry.name),
                    icon: icons::EMISSIONS,
                    emissionsChange: display::percent(change, true),
                    emissionsAfter: lt1(after),
                    emissionsBefore: lt1(current),
                    )
                };
                (
                    tip! {
                        icons::EMISSIONS,
                        &tip_text,
                    }
                    .card(industry.clone()),
                    text! {
                        "emissions",
                        "{changeDir} {type} emissions for {tag} by <strong>{percent}</strong>.",
                            tag: tag,
                            type: t!(byproduct.lower()),
                            percent: if self.is_unknown {
                                self.fmt_param(*amount)
                            } else {
                                display::percent(amount.abs(), true)
                            },
                            changeDir: self.change_dir(*amount),

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
                        t!("{type} emissions", type: t!("CO2"))
                    }
                    Byproduct::N2o => {
                        t!("{type} emissions", type: t!("N2O"))
                    }
                    Byproduct::Ch4 => {
                        t!("{type} emissions", type: t!("CH4"))
                    }
                };
                let tag = card_tag(&t!(&process.name));
                let tip_text = if self.is_unknown {
                    t!("Changes {label} for {name} by an unknown amount.",
                        name: t!(&process.name),
                        label: label,
                    )
                } else {
                    let change = match byproduct {
                        Byproduct::Biodiversity => {
                            t!(r#"{fromAmount} to {toAmount}<img src="{icon}">."#,
                                icon: byproduct.icon(),
                                toAmount: amount,
                                fromAmount: process.byproducts.biodiversity,
                            )
                        }
                        _ => {
                            let current =
                                process.byproducts.gtco2eq()
                                    * state
                                        .produced_by_process
                                        .get(id)
                                        .unwrap_or(&0.);
                            let after = current * (1. + amount);
                            let change = (after - current)
                                / state.emissions_gt();
                            t!(r#"{emissionsBefore} to <img src="{icon}">{emissionsAfter}. This is a {emissionsChange}% change of all emissions."#,
                                icon: icons::EMISSIONS,
                                emissionsChange: display::percent(change, true),
                                emissionsAfter: lt1(after),
                                emissionsBefore: lt1(current))
                        }
                    };

                    t!(r#"This will change {short} for {name} from <img src="{icon}">{change}"#,
                        name: t!(&process.name),
                        icon: byproduct.icon(),
                        short: match byproduct {
                            Byproduct::Biodiversity => t!("biodiversity pressure"),
                            _ => t!("emissions"),
                        },
                        change: change,
                    )
                };
                (
                    tip! {
                        byproduct.icon(),
                        &tip_text,
                    },
                    text! {
                        byproduct.as_key(),
                        "{changeDir} {label} for {tag} by <strong>{percent}</strong>.",
                        tag: tag,
                        label: label,
                        icon: byproduct.icon(),
                        percent: if self.is_unknown {
                            self.fmt_param(*amount)
                        } else {
                            display::percent(amount.abs(), true)
                        },
                        changeDir: self.change_dir(*amount),

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
                            r#"This changes regional contentedness based on demand for {name}. Current world contentedeness is {amount}<span class="type-total">/{maxAmount}</span>."#,
                            name: t!(output.lower()),
                            maxAmount: consts::MAX_CONTENTEDNESS,
                            amount: state.outlook().round(),
                        }.subicon(output.icon()),
                        text! {
                            "contentedness",
                            "{changeDir} world contentedness by <strong>{amount}</strong>.",
                            amount: change.abs(),
                            changeDir: self.change_dir(*amount),
                        }
                    )
            }
            Effect::IncomeOutlookChange(amount) => {
                let change =
                    mean_income_outlook_change(*amount, state)
                        .round();
                (tip! {
                        icons::CONTENTEDNESS,
                        r#"This changes regional contentedness by {amount} per income level (wealthier regions will feel it more). Current world contentedeness is {contentedness}<span class="type-total">/{maxContentedness}</span>."#,
                        maxContentedness: consts::MAX_CONTENTEDNESS,
                        contentedness: state.outlook().round(),
                        amount: amount,
                    }.subicon(icons::WEALTH), text! {
                        "contentedness",
                        "{changeDir} contentedness by <strong>{amount}</strong>.",
                        amount: change.abs(),
                        changeDir: self.change_dir(*amount),
                    })
            }
            Effect::ModifyEventProbability(id, amount) => {
                let event = &state.event_pool.events[id];
                let percent = if self.is_unknown {
                    self.fmt_param(*amount)
                } else {
                    display::percent(amount.abs(), true)
                };
                let text = t!(r#"{changeDir} the chance of "{event}" by {percent}%"#,
                event: t!(&event.name),
                percent: percent,
                changeDir: self.change_dir(*amount),
                );
                (
                    tip! {
                        icons::CHANCE,
                        &text,
                    },
                    text! {
                        "chance",
                        &text,
                    },
                )
            }
            Effect::AddFlag(flag) => {
                let demand =
                    display::outputs(&state.output_demand);
                let tip = flag_tip(*flag, &demand);
                let text = format!(
                    "<strong>{}</strong>",
                    t!(&flag.to_string())
                );
                (tip, text)
            }
            Effect::ProtectLand(amount) => (
                tip! {
                    icons::LAND,
                    "This will limit the amount of land that processes can use.",
                },
                text! {
                    "land",
                    "Place <strong>{percent}%</strong> of land under protection.",
                    percent: display::percent(*amount, true),
                },
            ),
            Effect::Feedstock(feedstock, amount) => {
                let estimate = match feedstock {
                    Feedstock::Other | Feedstock::Soil => None,
                    other => {
                        let est = state.feedstocks[*other]
                            / state.consumed_feedstocks[*other];
                        Some(est.round())
                    }
                };
                let text = match estimate {
                    None => {
                        t!("We aren't tracking this feedstock.")
                    }
                    Some(0.) => {
                        t!("This feedstock has been depleted.")
                    }
                    Some(est) => {
                        if est.is_infinite() {
                            t!("At current usage rates the estimated supply is expected to last indefinitely.")
                        } else {
                            t!("At current usage rates the estimated supply is expected to last {estimate} years.", estimate: est)
                        }
                    }
                };
                (
                    tip! {
                        feedstock.icon(),
                        &text,
                    },
                    text! {
                        feedstock.as_key(),
                        "{changeDir} {name} supply by <strong>{percent}%.</strong>",
                        name: t!(feedstock.lower()),
                        percent: display::percent(*amount, true),
                        changeDir: self.change_dir(*amount),
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
                            "{name} will be unavailable while this project is active.",
                            name: t!(&project.name),
                        }
                        .card(project.clone()),
                        text! {
                            "locks",
                            "<strong>Locks</strong> {tag}",
                            tag: tag,
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
                            "This will directly change the global temperature anomaly by {amount}<strong>°c</strong>.",
                            amount: format!("{:+}", temp_change),
                        },
                        text! {
                            "warming",
                            "{changeDir} the global temperature by {amount}<strong>°c</strong>.",
                            changeDir: self.change_dir(temp_change),
                            amount: self.fmt_param(temp_change),
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
