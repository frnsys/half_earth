use leptos::*;

use crate::{
    eval::{eval_badges, summarize, Summary},
    i18n,
    icons,
    state::{GameExt, Settings},
    t,
    views::{tip, Events, HasTip},
};
use hes_engine::{events::Phase as EventPhase, Game};

#[server(prefix = "/compute", endpoint = "image")]
pub async fn generate_image(
    summary: Summary,
) -> Result<String, ServerFnError> {
    let image =
        crate::server::sharing::generate_image(&summary);
    Ok(image)
}

#[component]
pub fn End(lose: bool) -> impl IntoView {
    let events = create_rw_signal(vec![]);
    let game = expect_context::<RwSignal<Game>>();
    game.update_untracked(|game: &mut Game| {
        let evs = if lose {
            game.roll_events(EventPhase::BreakStart, None)
        } else {
            game.roll_events(EventPhase::EndStart, None)
        };
        events.set(evs);
    });

    let (_, set_settings) = Settings::rw();
    set_settings.update_untracked(|settings| {
        settings.runs_played += 1;
    });

    let message = if lose {
        "This is not the end..."
    } else {
        "Well Played!"
    };

    let share_image = create_rw_signal(String::new());
    spawn_local(async move {
        let summary = with!(|game| summarize(&game, !lose));
        let img = generate_image(summary).await.unwrap();
        share_image.set(img);
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
                    <h2>{t!(message)}</h2>
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
