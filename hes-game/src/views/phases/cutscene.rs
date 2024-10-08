use std::time::Duration;

use crate::{
    audio,
    state::{Phase, StateExt, UIState},
    t,
    views::events::Events,
};
use hes_engine::{EventPhase, State};
use leptos::*;

#[component]
pub fn Cutscene() -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

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

    audio::play_phase_music("/assets/music/intro.mp3", false);

    let (image_idx, set_image_idx) = create_signal(0);
    let background = move || {
        let image = IMAGES[image_idx.get()];
        format!("url('/assets/cutscenes/out/{image}')")
    };

    let (_, set_phase) = slice!(ui.phase);
    let main_ref = create_node_ref::<html::Div>();
    let fade_out = move || {
        if let Some(elem) = main_ref.get() {
            let _ = elem.style(
                "animation",
                "1s fade-out ease-out forwards",
            );
            set_timeout(
                move || {
                    set_phase.set(Phase::Interstitial);
                },
                Duration::from_secs(1),
            );
        }
    };

    let advance = move |_| {
        set_image_idx.update(|idx| *idx += 1);
    };

    let next_phase = move |_| {
        fade_out();
    };

    // Wait a beat before showing the event
    let events = create_rw_signal(vec![]);
    create_effect(move |_| {
        update!(move |game| {
            events.set(StateExt::roll_events(
                game,
                EventPhase::CutsceneIntro,
            ));
        });
    });

    view! {
        <div
            ref=main_ref
            class="cutscene"
            style:background-image=background
        >
            <Events
                delay=1500
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
