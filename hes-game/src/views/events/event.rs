use std::collections::BTreeMap;

use super::{Dialogue, DisplayEvent};
use crate::views::cards::EventCard;
use leptos::*;

#[component]
pub fn Event(
    #[prop(into)] event: Signal<DisplayEvent>,
    #[prop(into)] on_done: Callback<()>,
    #[prop(into)] on_advance: Callback<()>,
) -> impl IntoView {
    let ctx = move || {
        with!(|event| {
            let mut ctx = BTreeMap::default();
            if let Some((_, name)) = &event.region {
                ctx.insert(
                    "region".to_string(),
                    name.to_string(),
                );
            }
            ctx
        })
    };

    let dialogue =
        move || with!(|event| event.flavor.dialogue.clone());
    let effects =
        move || with!(|event| Some(event.effects.clone()));
    let event_id = move || with!(|event| Some(event.id));

    view! {
        <div class="event">
            <EventCard event />
            <Dialogue
                dialogue=dialogue
                context=ctx
                on_advance=on_advance
                on_done=on_done
                event_id
                effects
            />
        </div>
    }
}
