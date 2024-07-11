use std::time::Duration;

use crate::{
    state::Phase,
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

    // window.audioManager.startSoundtrack('/assets/music/intro.mp3', false);

    let (image_idx, set_image_idx) = create_signal(0);
    let background = move || {
        let image = IMAGES[image_idx.get()];
        format!("url('/public/assets/cutscenes/out/{image}')")
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
                    logging::log!("CHANGING PHASE");
                    set_phase.set(Phase::Interstitial);

                    // TODO no idea why but leptos will not clean
                    // up this view, so do it manually I guess
                    to_ws_el(elem).remove();
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
        // window.audioManager.stopSoundtrack(true);
        fade_out();
    };

    // Wait a beat before showing the event
    let (events, set_events) =
        create_signal::<Vec<ResolvedEvent>>(vec![]);
    create_effect(move |_| {
        write_state!(move |state, _ui| {
            let events = state.roll_events_for_phase(
                EventPhase::CutsceneIntro,
                None,
            );
            set_events.set(events);
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
