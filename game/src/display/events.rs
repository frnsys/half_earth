use std::ops::Deref;

use hes_engine::*;
use rust_i18n::t;

use super::{
    DisplayEffect,
    icons::{HasIcon, Icon},
};

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayEvent {
    event: ResolvedEvent,
    pub factors: Vec<(Icon, String)>,
    pub effects: Vec<DisplayEffect>,
}
impl Deref for DisplayEvent {
    type Target = ResolvedEvent;
    fn deref(&self) -> &Self::Target {
        &self.event
    }
}
impl DisplayEvent {
    pub fn new(event: ResolvedEvent, state: &State) -> Self {
        let mut factors = event
            .probabilities
            .iter()
            .flat_map(|prob| {
                prob.conditions.iter().filter_map(|cond| {
                    describe_condition(cond, state)
                        .map(|desc| (cond.icon(), desc))
                })
            })
            .collect::<Vec<_>>();

        factors.sort();
        factors.dedup();

        let effects = event
            .effects
            .iter()
            .map(DisplayEffect::from)
            .collect::<Vec<_>>();

        DisplayEvent {
            event,
            factors,
            effects,
        }
    }

    pub fn has_visible_effects(&self) -> bool {
        if self.event.effects.is_empty() {
            false
        } else {
            self.event.effects.iter().any(|effect| match effect
            {
                Effect::AddEvent(..)
                | Effect::TriggerEvent(..) => false,
                _ => true,
            })
        }
    }

    pub fn show_as_card(&self) -> bool {
        self.flavor.image.is_some()
    }
}

fn describe_condition(
    condition: &Condition,
    state: &State,
) -> Option<String> {
    match condition {
        Condition::ProjectStatus(id, status) => {
            let name = state.world.projects[id].name.as_str();
            let label = match status {
                Status::Active | Status::Finished => "active",
                Status::Inactive => "inactive",
                Status::Building => "being built",
                Status::Stalled => "stalled",
                Status::Halted => "halted",
            };
            Some(t!(
                r#"This event can occur if "%{name}" is %{label}."#,
                name = t!(name),
                label = t!(label)
            ))
        }
        Condition::ProcessOutput(id, _, _) => {
            let name = state.world.processes[id].name.as_str();
            Some(t!(
                "This event is influenced by the output of %{name}.",
                name = t!(name)
            ))
        }
        Condition::ProcessMixShare(id, _, _) => {
            let name = state.world.processes[id].name.as_str();
            Some(t!(
                "This event is influenced by the mix share of %{name}.",
                name = t!(name)
            ))
        }
        Condition::NPCRelationship(id, rel_type) => {
            let name = state.npcs[id].name.as_str();
            Some(t!(
                "This event can occur if %{name} is your %{relType}.",
                name = t!(name),
                relType = t!(rel_type.to_string())
            ))
        }
        Condition::ProcessMixShareFeature(feat, _, _) => {
            match feat {
                ProcessFeature::IsCCS => Some(t!(
                    "This event is influenced by how much production involves carbon capture and storage."
                )),
                ProcessFeature::CanMeltdown => Some(t!(
                    "This event is influenced by how much energy production can meltdown."
                )),
                ProcessFeature::MakesNuclearWaste => Some(t!(
                    "This event is influenced by how much energy production produces nuclear waste."
                )),
                ProcessFeature::IsLaborIntensive => Some(t!(
                    "This event is influenced by how production is especially labor-intensive."
                )),
                ProcessFeature::IsFossil => Some(t!(
                    "This event is influenced by how much energy production uses fossil fuels."
                )),
                ProcessFeature::UsesPesticides => Some(t!(
                    "This event is influenced by how much food production uses pesticides."
                )),
                ProcessFeature::UsesLivestock => Some(t!(
                    "This event is influenced by how much food production uses livestock."
                )),
                ProcessFeature::IsIntermittent => Some(t!(
                    "This event is influenced by how much energy production is intermittent."
                )),
                _ => None,
            }
        }
        Condition::WorldVariable(var, _, _) => match var {
            WorldVariable::Temperature => Some(t!(
                "This event is influenced by the global temperature anomaly."
            )),
            WorldVariable::Outlook => Some(t!(
                "This event is influenced by how happy people are."
            )),
            WorldVariable::ExtinctionRate => Some(t!(
                "This event is influenced by biodiversity pressures."
            )),
            WorldVariable::SeaLevelRise => Some(t!(
                "This event is influenced by the amount of sea level rise."
            )),
            _ => None,
        },
        Condition::LocalVariable(var, _, _) => match var {
            LocalVariable::Outlook => Some(t!(
                "This event is influenced by how happy people are."
            )),
            LocalVariable::Habitability => Some(t!(
                "This event is influenced by the habitability of regions."
            )),
            _ => None,
        },
        Condition::Demand(output, _, _) => match output {
            Output::AnimalCalories => Some(t!(
                "This event is influenced by the demand for animal calories."
            )),
            Output::PlantCalories => Some(t!(
                "This event is influenced by the demand for plant calories."
            )),
            Output::Electricity => Some(t!(
                "This event is influenced by the demand for electricity."
            )),
            Output::Fuel => Some(t!(
                "This event is influenced by the demand for fuel."
            )),
        },
        _ => None,
    }.map(|val| val.to_string())
}
