use crate::{infinite_list, inputs::*};
use hes_engine::{
    events::{Event, Probability},
    kinds::Output,
    npcs::NPC,
    projects::{
        Cost,
        Factor,
        FactorKind,
        Group,
        Project,
        Type,
    },
    world::World,
    Collection,
    HasId,
};
use leptos::*;

#[component]
fn Event(
    signal: (Signal<Event>, SignalSetter<Event>),
) -> impl IntoView {
    let (read, write) = signal;
    let event = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(event.get());
    });

    view! {
        <div class="event">
            <div class="name">
                <TextInput signal=slice!(event.name) />
                <div class="item-lock">
                    <ToggleInput
                        label="Locked"
                        tooltip=true
                        icons=("🔒Locked", "🔓Unlocked")
                        help="If this event is locked at the start."
                        signal=slice!(event.locked) />
                </div>
            </div>
            <div class="item-form">
                <div class="input-groups">
                    <OptionalImageInput signal=slice!(event.flavor.image) />
                </div>
                <div class="input-groups event-meta">
                    <div class="arc">
                        <TextInput signal=slice!(event.flavor.arc) />
                        <div class="input-help">Optional story arc name.</div>
                    </div>
                    <EnumInput
                        label="Phase"
                        help="What phase/screen the event can occur on."
                        signal=slice!(event.phase) />
                </div>
            </div>

            <div class="item-form effects-form">
                <Effects
                    effects=slice!(event.effects) />
            </div>

            <div class="item-form probabilities-form">
                <div class="input-help">"Probabilities are checked in their defined order, and the first probability with all conditions satisfied is the one that is rolled."</div>
                <Probabilities
                    probabilities=slice!(event.probabilities) />
            </div>
        </div>
    }
}

#[component]
fn Probability(
    probability: (
        Signal<Probability>,
        SignalSetter<Probability>,
    ),
) -> impl IntoView {
    let (read, write) = probability;
    let probability = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(probability.get());
    });

    view! {
        <div class="probability">
            <EnumInput
                label="Likelihood"
                help="The likelihood when all conditions are met."
                signal=slice!(probability.likelihood) />
            <Conditions
                conditions=slice!(probability.conditions) />
        </div>
    }
}

#[component]
fn Probabilities(
    probabilities: (
        Signal<Vec<Probability>>,
        SignalSetter<Vec<Probability>>,
    ),
) -> impl IntoView {
    let (read, write) = probabilities;
    let probabilities = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(probabilities.get());
    });

    let n_probabilities =
        with!(|probabilities| probabilities.len());
    view! {
        <div class="probabilities">
        {move || {
             (0..n_probabilities).map(|i| {
                 view! {
                     <Probability
                         probability=create_slice(probabilities,
                             move |probabilities| probabilities[i].clone(),
                             move |probabilities, val| probabilities[i] = val
                         ) />
                 }
             }).collect::<Vec<_>>()
         }}
        </div>
    }
}

infinite_list!(Events, Event, events);
