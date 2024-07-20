mod dialogue;
mod display;
mod event;

use std::{collections::HashMap, time::Duration};

pub use dialogue::Dialogue;
pub use display::DisplayEvent;
use event::Event;
use leptos::*;

#[component]
pub fn Events(
    #[prop(into, optional, default=(|_| {}).into())]
    on_done: Callback<()>,
    #[prop(into, optional, default=(|_| {}).into())] on_advance: Callback<()>,
    #[prop(into)] events: RwSignal<Vec<DisplayEvent>>,
    #[prop(optional, default = 0)] delay: u64,
) -> impl IntoView {
    let (ready, set_ready) = create_signal(delay == 0);
    let (idx, set_idx) = create_signal(0);

    let has_event = move || {
        events
            .try_with(|events| idx.get() < events.len())
            .unwrap_or(false)
    };

    let advance_event = move |_| {
        let next_idx = idx.get() + 1;
        let n_events = events.with(|events| events.len());
        if next_idx < n_events {
            set_idx.set(next_idx);
        } else {
            update!(|events| events.clear());
            on_done.call(());
        }
    };
    let event = move || events.get()[idx.get()].clone();

    create_effect(move |_| {
        if delay > 0 && !ready.get() {
            set_timeout(
                move || {
                    set_ready.set(true);
                },
                Duration::from_millis(delay),
            );
        }
        if ready.get() && !has_event() {
            on_done.call(());
        }
    });

    move || {
        if has_event() && ready.get() {
            Some(view! {
                <Event event on_advance on_done=advance_event />
            })
        } else {
            None
        }
    }
}
