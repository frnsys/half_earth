use leptos::*;

use crate::{
    eval::{summarize, Summary},
    i18n,
    icons,
    state::{GameExt, GameState, Settings},
    t,
    views::{tip, Events, HasTip},
};
use hes_engine::events::Phase as EventPhase;

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
    let state =
        expect_context::<RwSignal<crate::state::GameState>>();
    state.update_untracked(|state: &mut GameState| {
        let evs = if lose {
            state.game.roll_events(EventPhase::BreakStart, None)
        } else {
            state.game.roll_events(EventPhase::EndStart, None)
        };
        events.set(evs);
    });

    let (settings, set_settings) = Settings::rw();
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
        let state = with!(|state| state.game.clone());
        let summary = summarize(&state, !lose);
        let img = generate_image(summary).await.unwrap();
        share_image.set(img);
    });

    #[derive(Clone)]
    struct Badge {
        name: String,
        desc: String,
    }
    impl Badge {
        pub fn image_url(&self) -> String {
            format!("/public/assets/badges/{}.png", self.name)
        }
    }

    let (show_start, set_show_start) = create_signal(false);
    let (badges, set_badges) =
        create_signal::<Vec<Badge>>(vec![]);
    let (share_img_url, set_share_img_url) =
        create_signal::<Option<String>>(None);

    let start_new_run = move |_| {
        GameState::start_new_run();
    };

    view! {
        <div class="break">
            <Events
                events
                on_done=move |_| set_show_start.set(true)
            />
            <Show when=move || {
                !badges.with(|b| b.is_empty()) && show_start.get()
            }>
                <div class="badges-section">
                    <div class="badges">
                        <For
                            each=move || badges.get()
                            key=|badge: &Badge| badge.name.clone()
                            children=|badge| {
                                let text = i18n::t(&badge.desc);
                                let tip = tip(icons::HELP, text);
                                view! {
                                    <HasTip tip>
                                        <img src=badge.image_url()/>
                                    </HasTip>
                                }
                            }
                        />

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