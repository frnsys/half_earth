use leptos::*;

use crate::{
    i18n, icons, t,
    views::{
        dialogue::Event,
        phases::cutscene::Events,
        tips::{tip, HasTip},
    },
};

#[component]
pub fn End(lose: bool) -> impl IntoView {
    // TODO
    let events = {
        if lose {
            vec![]
            // game.roll.break("Start")
        } else {
            vec![]
            // game.roll.end("Start")
        }
    };

    let message = if lose {
        "This is not the end..."
    } else {
        "Well Played!"
    };

    // TODO get share image
    // getShareImage() {
    //   share(!this.lose, (data) => {
    //     if (data.success) {
    //       let {badges, url, image} = data;
    //       this.shareImgUrl = image;
    //       this.shareUrl = url;
    //       this.badges = badges;
    //     }
    //   });
    // },

    // TODO
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

    let (show_start, set_show_start) = create_signal(events.is_empty());
    let (badges, set_badges) = create_signal::<Vec<Badge>>(vec![]);
    let (share_img_url, set_share_img_url) = create_signal::<Option<String>>(None);

    let start_run = move |_| {
        // TODO game.clearSave();
        window().location().reload();
    };

    view! {
        <div class="break">
            <Events
                events
                on_advance=|_| {}
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
                    <button class="try-again-button" on:click=start_run>
                        {t!("Try Again?")}
                    </button>
                </div>
                <Show when=move || share_img_url.with(|u| u.is_some())>
                    <div>
                        <img
                            class="share-image"
                            crossorigin="anonymous"
                            src=share_img_url
                        />
                    </div>
                </Show>
            </Show>
        </div>
    }
}
