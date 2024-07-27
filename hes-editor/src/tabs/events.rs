use crate::{infinite_list, inputs::*, subsignal};
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
    let event = signal;
    view! {
        <div class="event" id={move || signal.0.with(|signal| signal.id.to_string())}>
            <div class="name">
                <TextInput signal=subsignal!(event.name) />
                <div class="item-lock">
                    <ToggleInput
                        label="Locked"
                        tooltip=true
                        icons=("🔒Locked", "🔓Unlocked")
                        help="If this event is locked at the start."
                        signal=subsignal!(event.locked) />
                </div>
            </div>
            <div class="item-form">
                <div class="input-groups">
                    <OptionalImageInput signal=subsignal!(event.flavor.image) />
                </div>
                <div class="input-groups event-meta">
                    <div class="arc">
                        <TextInput signal=subsignal!(event.flavor.arc) />
                        <div class="input-help">Optional story arc name.</div>
                    </div>
                    <EnumInput
                        label="Phase"
                        help="What phase/screen the event can occur on."
                        signal=subsignal!(event.phase) />
                </div>
            </div>

            <div class="item-form effects-form">
                <Effects
                    double_col=true
                    effects=subsignal!(event.effects) />
            </div>

            <div class="item-form probabilities-form">
                <Probabilities
                    probabilities=subsignal!(event.probabilities) />
            </div>

             <div class="item-form notes-form">
                 <TextArea label="Notes" help="Optional notes" signal=subsignal!(event.notes) />
             </div>
        </div>
    }
}

#[component]
fn Probability<F>(
    probability: (
        Signal<Probability>,
        SignalSetter<Probability>,
    ),
    on_remove: F,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <div class="mutable-list-item-header">
            <EnumInput
                label="Likelihood"
                help="The likelihood when all conditions are met."
                signal=subsignal!(probability.likelihood) />
            <div class="mutable-list-item-remove" title="Ctrl-click to remove without confirmation." on:click=on_remove>"✗"</div>
        </div>
        <Conditions
            conditions=subsignal!(probability.conditions) />
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

    view! {
        <div class="probabilities mutable-list mutable-list-double-col mutable-list-sortable">
            <div class="mutable-list-header">
                <h2>Probabilities</h2>
                <div class="mutable-list-add">
                    <div class="mutable-list-add-button" on:click=move |_| {
                        let probability = Probability::default();
                        let mut probabilities = read.get();
                        probabilities.insert(0, probability);
                        write.set(probabilities);
                    }>+Add</div>
                </div>
            </div>
            <div class="input-help">"Probabilities are checked in their defined order, and the first probability with all conditions satisfied is the one that is rolled."</div>
            {move || {
                 let empty = with!(|read| read.is_empty());
                 if empty {
                     Some(view! {
                         <div class="empty">No probabilities defined.</div>
                     })
                 } else {
                     None
                 }
            }}
            <div class="mutable-list-items">
                {move || {
                     let n_probabilities =
                         with!(|read| read.len());
                     (0..n_probabilities).map(|i| {
                         view! {
                             <div class="probability mutable-list-item">
                                 <div class="mutable-list-item-index">
                                    <div class="move-up" class:hidden={i == 0} on:click=move |_| {
                                         let mut probabilities = read.get();
                                         probabilities.swap(i, i-1);
                                         write.set(probabilities);
                                    }>"⯅"</div>
                                    {i}
                                    <div class="move-down" class:hidden={i == n_probabilities - 1} on:click=move |_| {
                                         let mut probabilities = read.get();
                                         probabilities.swap(i, i+1);
                                         write.set(probabilities);
                                    }>"⯆"</div>
                                 </div>
                                 <Probability
                                     on_remove=move |ev: ev::MouseEvent| {
                                         let msg = "Are you sure you want to remove this probability?";
                                         if ev.ctrl_key() || window().confirm_with_message(msg).unwrap() {
                                             let mut probabilities = read.get();
                                             probabilities.remove(i);
                                             write.set(probabilities);
                                         }
                                     }
                                     probability=subsignal!(probabilities[i]) />
                             </div>
                         }
                     }).collect::<Vec<_>>()
                 }}
            </div>
        </div>
    }
}

infinite_list!(Events, Event, events);
