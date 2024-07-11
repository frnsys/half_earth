mod dialogue;

use std::collections::HashMap;

pub use dialogue::Dialogue;
use hes_engine::game::ResolvedEvent;
use leptos::*;

#[component]
pub fn Events(
    #[prop(into)] on_advance: Callback<()>,
    #[prop(into)] on_done: Callback<()>,
    #[prop(into)] events: Signal<Vec<ResolvedEvent>>,
) -> impl IntoView {
    let (idx, set_idx) = create_signal(0);
    let (ctx, set_ctx) = create_signal::<HashMap<String, String>>(
        HashMap::default(),
    );

    let dialogue = move || {
        let ev = &events.get()[idx.get()];
        ev.flavor.dialogue.clone()
    };
    let has_event = move || {
        events
            .try_with(|events| idx.get() < events.len())
            .unwrap_or(false)
    };

    let next_event = move |_| {
        let next_idx = idx.get() + 1;
        let n_events = events.with(|events| events.len());
        if next_idx < n_events {
            events.with(|events| {
                let event = &events[next_idx];
                let mut ctx = HashMap::default();
                if let Some(name) = &event.region {
                    ctx.insert(
                        "region".to_string(),
                        name.to_string(),
                    );
                }
                set_ctx.set(ctx);
            });
            set_idx.set(next_idx);
        } else {
            logging::log!("Done dialogue");
            on_done.call(());
        }
    };

    create_effect(move |_| {
        if !has_event() {
            on_done.call(());
        }
    });

    let view = move || {
        if has_event() {
            Some(view! {
                <Dialogue
                    dialogue=dialogue
                    context=ctx
                    on_advance=on_advance
                    on_done=next_event
                />
            })
        } else {
            None
        }
    };

    view! { {view} }
}
