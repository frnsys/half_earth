use std::time::Duration;

use crate::{
    audio,
    state::{GameExt, Phase},
    t,
    ui_rw,
    util::to_ws_el,
    views::events::Events,
    write_state,
};
use hes_engine::{
    events::Phase as EventPhase,
    game::ResolvedEvent,
};
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

    audio::play_phase_music("/assets/music/intro.mp3", false);

    let (image_idx, set_image_idx) = create_signal(0);
    let background = move || {
        let image = IMAGES[image_idx.get()];
        format!("url('/assets/cutscenes/out/{image}')")
    };

    let (_, set_phase) = ui_rw!(phase);
    let main_ref = create_node_ref::<html::Div>();
    let fade_out = move || {
        if let Some(elem) = main_ref.get() {
            logging::log!("FADING OUT");
            let elem = elem.style(
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
        // TODO
        fade_out();
    };

    // Wait a beat before showing the event
    let events = create_rw_signal(vec![]);
    create_effect(move |_| {
        write_state!(move |state, _ui| {
            events.set(
                state.roll_events(
                    EventPhase::CutsceneIntro,
                    None,
                ),
            );
        })();
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
