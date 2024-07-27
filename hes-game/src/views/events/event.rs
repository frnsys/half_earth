use std::{collections::BTreeMap, ops::Deref};

use super::{Dialogue, DisplayEvent};
use crate::{
    t,
    util::ImageExt,
    views::{
        cards::EventCard,
        tip,
        Effects,
        Events,
        HasTip,
        Help,
    },
};
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

    view! {
        <div class="event">
            <EventCard event />
            <Dialogue
                dialogue=dialogue
                context=ctx
                on_advance=on_advance
                on_done=on_done
            />
        </div>
    }
}
