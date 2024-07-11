use crate::{
    icons::HasIcon,
    t,
    views::{
        effects::DisplayEffect,
        tip,
        Effects,
        Events,
        HasTip,
        Help,
    },
};
use hes_engine::{
    events::{
        Condition,
        Effect as HesEffect,
        LocalVariable,
        WorldVariable,
    },
    kinds::Output,
    production::ProcessFeature,
    projects::Status,
    state::State,
};
use leptos::*;

fn describe_condition(
    condition: &Condition,
    state: &State,
) -> Option<String> {
    match condition {
        Condition::ProjectStatus(id, status) => {
            let name = &state.world.projects[*id].name;
            let label = match status {
                Status::Active | Status::Finished => "active",
                Status::Inactive => "inactive",
                Status::Building => "being built",
                Status::Stalled => "stalled",
                Status::Halted => "halted",
            };
            Some(t!(r#"This event can occur if "{name}" is {label}."#, name: t!(name), label: t!(label)))
        }
        Condition::ProcessOutput(id, _, _) => {
            let name = &state.world.processes[*id].name;
            Some(t!("This event is influenced by the output of {name}.", name: t!(name)))
        }
        Condition::ProcessMixShare(id, _, _) => {
            let name = &state.world.processes[*id].name;
            Some(t!("This event is influenced by the mix share of {name}.", name: t!(name)))
        }
        Condition::NPCRelationship(id, rel_type) => {
            let name = &state.npcs[*id].name;
            Some(t!("This event can occur if {name} is your {relType}.", name: t!(name), relType: t!(&rel_type.to_string())))
        }
        Condition::ProcessMixShareFeature(feat, _, _) => {
            match feat {
                ProcessFeature::IsCCS => Some(t!("This event is influenced by how much production involves carbon capture and storage.")),
                ProcessFeature::CanMeltdown => Some(t!("This event is influenced by how much energy production can meltdown.")),
                ProcessFeature::MakesNuclearWaste => Some(t!("This event is influenced by how much energy production produces nuclear waste.")),
                ProcessFeature::IsLaborIntensive => Some(t!("This event is influenced by how production is especially labor-intensive.")),
                ProcessFeature::IsFossil => Some(t!("This event is influenced by how much energy production uses fossil fuels.")),
                ProcessFeature::UsesPesticides => Some(t!("This event is influenced by how much food production uses pesticides.")),
                ProcessFeature::UsesLivestock => Some(t!("This event is influenced by how much food production uses livestock.")),
                ProcessFeature::IsIntermittent => Some(t!("This event is influenced by how much energy production is intermittent.")),
                _ => None
            }
        }
        Condition::WorldVariable(var, _, _) => {
            match var {
                WorldVariable::Temperature => {
                    Some(t!("This event is influenced by the global temperature anomaly."))
                }
                WorldVariable::Outlook => {
                    Some(t!("This event is influenced by how happy people are."))
                }
                WorldVariable::ExtinctionRate => {
                    Some(t!("This event is influenced by biodiversity pressures."))
                }
                WorldVariable::SeaLevelRise => {
                    Some(t!("This event is influenced by the amount of sea level rise."))
                }
                _ => None
            }
        }
        Condition::LocalVariable(var, _, _) => {
            match var {
                LocalVariable::Outlook => {
                    Some(t!("This event is influenced by how happy people are."))
                }
                LocalVariable::Habitability => {
                    Some(t!("This event is influenced by the habitability of regions."))
                }
                _ => None,
            }
        }
        Condition::Demand(output, _, _) => {
            match output {
                Output::AnimalCalories => Some(t!("This event is influenced by the demand for animal calories.")),
                Output::PlantCalories => Some(t!("This event is influenced by the demand for plant calories.")),
                Output::Electricity => Some(t!("This event is influenced by the demand for electricity.")),
                Output::Fuel => Some(t!("This event is influenced by the demand for fuel."))
            }
        }
        _ => None,
    }
}

#[component]
pub fn Event(
    #[prop(into)] event: Signal<
        hes_engine::game::ResolvedEvent,
    >,
    #[prop(into, optional)] as_card: Signal<bool>,
    #[prop(into)] on_done: Callback<()>,
) -> impl IntoView {
    let effect_image_url = move || {
        event.with(|event| {
            format!(
                "url(/public/assets/content/images/{})",
                event.flavor.image.fname
            )
        })
    };
    let has_visible_effects = move || {
        event.with(|event| {
            if event.effects.is_empty() {
                false
            } else {
                event.effects.iter().any(
                    |effect| match effect {
                        HesEffect::AddEvent(..)
                        | HesEffect::TriggerEvent(..) => false,
                        _ => true,
                    },
                )
            }
        })
    };

    let state =
        expect_context::<RwSignal<crate::state::GameState>>();
    let factors = move || {
        with!(move |event, state| {
            event
                .probabilities
                .iter()
                .flat_map(|prob| {
                    prob.conditions.iter().filter_map(|cond| {
                        describe_condition(cond, &state.game)
                            .map(|desc| (cond.icon(), desc))
                    })
                })
                .collect::<Vec<_>>()
        })
    };
    let effects = move || {
        event.with(|event| {
            event
                .effects
                .iter()
                .map(DisplayEffect::from)
                .collect::<Vec<_>>()
        })
    };
    let image_attrib = move || {
        event.with(|event| {
            event.flavor.image.attribution.clone()
        })
    };

    let factor_tip = t!("The factors behind this event.↓");
    on_cleanup(|| {
        // TODO
        // settings.hide_help[factor_tip] = true
    });

    let arc = move || event.with(|event| t!(&event.flavor.arc));
    let name = move || event.with(|event| t!(&event.name));
    let factors_list = move || {
        factors()
            .into_iter()
            .map(|(icon, factor)| {
                let tip = tip(icon, factor);
                view! {
                    <HasTip tip>
                        <img class="event--factor" src=icon/>
                    </HasTip>
                }
            })
            .collect::<Vec<_>>()
    };
    let events = move || vec![event.get()];

    view! {
        <div class="event">
            <div
                class="event--body"
                style:background-image=effect_image_url
            >
                <Help text=factor_tip x=0.55 y=-18.0 center=false/>
                <div class="arc">{arc}</div>
                <div class="event--factors">{factors_list}</div>
                <div class="image-attribution">
                    {t!("Image:")} {image_attrib}
                </div>
                <div class="event--name">{name}</div>
                <Show when=has_visible_effects>
                    <div class="event--effects">
                        <Effects effects/>
                    </div>
                </Show>
            </div>
            <Show when=move || as_card.get()>
                <Events
                    on_done
                    on_advance=|_| {}
                    events=events.into_signal()
                />
            </Show>
        </div>
    }
}
