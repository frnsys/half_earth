use std::{collections::HashMap, time::Duration};

use crate::{anim::fade_out, state::Phase, t, views::Dialogue, write_state};
use hes_engine::flavor;
use leptos::*;

#[component]
pub fn Cutscene() -> impl IntoView {
    // One per line of dialogue
    const IMAGES: &[&str] = &[
        "pexels-lt-chan-2833366.jpg",
        "gosplant_world.jpg",
        "gosplant_world.jpg",
        "gosplant_world.jpg",
        "pexels-marco-allasio-4275996.jpg",
        "pexels-mentatdgt-1185433.jpg",
        "hasan-almasi-OwqLxCvoVxI-unsplash.jpg",
        "matthew-tenbruggencate-0HJWobhGhJs-unsplash.jpg",
        "hasan-almasi-OwqLxCvoVxI-unsplash.jpg",
        "kelly-sikkema-VpcSDucAYjw-unsplash.jpg",
    ];

    // window.audioManager.startSoundtrack('/assets/music/intro.mp3', false);

    let (image_idx, set_image_idx) = create_signal(0);
    let background = move || {
        let image = IMAGES[image_idx.get()];
        format!("url('/assets/cutscenes/out/{image}')")
    };

    let (start_anim, opacity) = fade_out(
        1000.,
        write_state!(move |_, ui| {
            ui.phase = Phase::Interstitial;
        }),
    );

    let advance = move |_| {
        set_image_idx.update(|idx| *idx += 1);
    };
    let next_phase = move |_| {
        // TODO
        // window.audioManager.stopSoundtrack(true);
        start_anim();
    };

    // Wait a beat before showing the event
    let (events, set_events) = create_signal::<Vec<flavor::EventFlavor>>(vec![]);
    set_timeout(
        || {
            // let events = game.roll.cutscene("Intro")// TODO
        },
        Duration::from_millis(1500),
    );

    view! {
        <div
            class="cutscene"
            style:background-image=background
            style:opacity=opacity
        >
            <Events
                on_advance=advance
                on_done=next_phase.clone()
                events=events
            />
            <button
                class="cutscene--skip btn"
                on:click=move |_| next_phase(())
            >
                {t!("Skip")}
            </button>
        </div>
    }
}

#[component]
pub fn Events(
    #[prop(into)] on_advance: Callback<()>,
    #[prop(into)] on_done: Callback<()>,
    #[prop(into)] events: MaybeSignal<Vec<flavor::EventFlavor>>,
) -> impl IntoView {
    let (idx, set_idx) = create_signal(0);
    let (ctx, set_ctx) = create_signal::<HashMap<String, String>>(HashMap::default());
    let (dialogue, set_dialogue) = create_signal::<Option<flavor::Dialogue>>(None);
    let next_event = move |_| {
        // TODO apply game effects when rolling?
        events.with(|events| {
            update!(|set_idx, set_ctx, set_dialogue| {
                *set_idx += 1;

                if *set_idx < events.len() {
                    let event = &events[*set_idx];
                    *set_dialogue = Some(event.dialogue.clone());

                    // TODO
                    // if let Some(region_id) =
                    // if (regionId !== undefined) {
                    //   ctx['region'] = regions[regionId].name;
                    // };
                    let mut ctx = HashMap::default();
                    *set_ctx = ctx;
                } else {
                    on_done.call(());
                }
            });
        });
    };

    let view = move || {
        dialogue.get().map(|dialogue| {
            let (dialogue, _) = create_signal(dialogue);
            view! {
                <Dialogue
                    dialogue=dialogue
                    context=ctx
                    on_advance=on_advance
                    on_done=next_event.clone()
                />
            }
        })
    };

    view! { {view} }
}
