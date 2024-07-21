use crate::inputs::*;
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
use leptos_use::{
    use_infinite_scroll_with_options,
    UseInfiniteScrollOptions,
};

#[component]
fn Event(
    event: (Signal<Event>, SignalSetter<Event>),
) -> impl IntoView {
    let (read, write) = event;
    let event = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(event.get());
    });

    view! {
        <div class="event">
            <div class="name">
                <TextInput signal=slice!(event.name) />
            </div>
            <div class="arc">
                <TextInput signal=slice!(event.flavor.arc) />
            </div>
            <OptionalImageInput signal=slice!(event.flavor.image) />
            <EnumInput
                label="Phase"
                help="What phase/screen the event can occur on."
                signal=slice!(event.phase) />
            <ToggleInput
                label="Locked"
                help="If this event is locked at the start."
                signal=slice!(event.locked) />
            <Effects
                effects=slice!(event.effects) />

            <div class="input-help">"Probabilities are checked in their defined order, and the first probability with all conditions satisfied is the one that is rolled."</div>
            <Probabilities
                probabilities=slice!(event.probabilities) />
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

#[component]
pub fn Events(world: RwSignal<World>) -> impl IntoView {
    let max_idx = create_rw_signal(10);
    let list = move || {
        with!(|world, max_idx| world
            .events
            .iter()
            .enumerate()
            .map(|(i, item)| (i, *item.id()))
            .take(*max_idx)
            .collect::<Vec<_>>())
    };
    let total = move || with!(|world| world.events.len());

    let el = create_node_ref::<html::Div>();
    let _ = use_infinite_scroll_with_options(
        el,
        move |_| async move {
            logging::log!("LOADING MORE");
            update!(|max_idx| {
                *max_idx += 10;
                *max_idx = (*max_idx).min(total());
            });
        },
        UseInfiniteScrollOptions::default().distance(10.0),
    );

    view! {
        <div ref=el class="scroll-list">
            <For each=list
                 key=|(_, id)| *id
                 children=move |(i, _)| {
                 view! {
                     <Event
                         event=create_slice(world,
                             move |world| world.events.by_idx(i).clone(),
                             move |world, val| *world.events.by_idx_mut(i) = val
                         ) />
                 }
                 } />
        </div>
    }
}
