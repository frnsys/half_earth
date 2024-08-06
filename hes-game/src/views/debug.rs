use crate::{
    memo,
    views::{DisplayEvent, Events},
};
use hes_engine::{ResolvedEvent, State};
use leptos::*;

#[component]
pub fn DebugEvents() -> impl IntoView {
    let events = create_rw_signal(vec![]);
    let game = expect_context::<RwSignal<State>>();

    let region = game.with_untracked(|game| {
        let region = game.world.regions.first();
        (region.id, region.name.clone())
    });
    let game_events = memo!(game.world.events);
    let event_views = move || {
        game_events.get().iter().map(|event| {
            let name = event.name.clone();
            let event = ResolvedEvent {
                event: event.clone(),
                region: if event.is_regional() {
                    Some(region.clone())
                } else {
                    None
                }
            };

            view! {
                <div class="debug-event" on:click=move |_| {
                    let event = game.with_untracked(|game| DisplayEvent::new(event.clone(), game));
                    update!(|events| events.push(event));
                }>
                    {name}
                </div>
            }
        }).collect::<Vec<_>>()
    };

    view! {
        <div class="debug-events">
            {event_views}
        </div>
        <Events events />
    }
}
