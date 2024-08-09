use leptos::*;

use crate::{
    eval::{eval_badges, summarize, Summary},
    i18n,
    icons,
    state::{Settings, StateExt},
    t,
    views::{tip, Events, HasTip},
};
use hes_engine::{EventPhase, State};

#[component]
pub fn End(lose: bool) -> impl IntoView {
    let events = create_rw_signal(vec![]);
    let game = expect_context::<RwSignal<State>>();
    game.update_untracked(|game: &mut State| {
        let evs = if lose {
            StateExt::roll_events(game, EventPhase::BreakStart)
        } else {
            StateExt::roll_events(game, EventPhase::EndStart)
        };
        events.set(evs);
    });

    let (_, set_settings) = Settings::rw();
    set_settings.update_untracked(|settings| {
        settings.runs_played += 1;
    });

    let message = if lose {
        t!("This is not the end...")
    } else {
        t!("Well Played!")
    };

    let share_image = create_rw_signal(String::new());
    spawn_local(async move {
        let summary = with!(|game| summarize(&game, !lose));
        // TODO
        // let img = generate_image(summary).await.unwrap();
        // share_image.set(img);
    });

    let (show_start, set_show_start) = create_signal(false);

    let badges_view = move || {
        let badges =
            game.with_untracked(|game| eval_badges(&game));
        badges
            .into_iter()
            .map(|badge| {
                let text = i18n::t(&badge.to_string());
                let tip = tip(icons::HELP, text);
                view! {
                    <HasTip tip>
                        <img src=badge.image_url()/>
                        </HasTip>
                }
            })
            .collect::<Vec<_>>()
    };

    let start_new_run = move |_| {
        crate::state::start_new_run();
    };

    view! {
        <div class="break">
            <Events
                events
                on_done=move |_| set_show_start.set(true)
            />
            <Show when=move || {
                show_start.get()
            }>
                <div class="badges-section">
                    <div class="badges">
                        {badges_view}
                    </div>
                </div>
            </Show>
            <Show when=move || show_start.get()>
                <div class="break--actions">
                    <h2>{&message}</h2>
                    <button class="try-again-button" on:click=start_new_run>
                        {t!("Try Again?")}
                    </button>
                </div>
                <div>
                    <img class="share-image" src={share_image} />
                </div>
            </Show>
        </div>
    }
}
