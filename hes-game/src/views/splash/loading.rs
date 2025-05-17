use crate::{audio, t, util::load_images};

// we also need this macro or else load_images won't compile
use include_dir::include_dir;
use leptos::*;

fn preload_assets() -> Vec<String> {
    const PRELOAD: &[&str] = &[
        "$CARGO_MANIFEST_DIR/public/assets/stamp.svg",
        "$CARGO_MANIFEST_DIR/public/assets/backgrounds/menu.jpg",
        "$CARGO_MANIFEST_DIR/public/assets/backgrounds/dashboard.png",
        "$CARGO_MANIFEST_DIR/public/assets/backgrounds/parliament.png",
        "$CARGO_MANIFEST_DIR/public/assets/backgrounds/plan.png",
        "$CARGO_MANIFEST_DIR/public/assets/backgrounds/regions.png",
        "$CARGO_MANIFEST_DIR/public/assets/backgrounds/report.png",
        "$CARGO_MANIFEST_DIR/public/assets/gosplant.svg",
        "$CARGO_MANIFEST_DIR/public/assets/clock.png",
        "$CARGO_MANIFEST_DIR/public/assets/motto.png",
    ];

    let mut preload: Vec<Box<str>> = Vec::from(
        PRELOAD
            .into_iter()
            .map(|s| {
                s.replace(
                    "$CARGO_MANIFEST_DIR",
                    env!("CARGO_MANIFEST_DIR"),
                )
            })
            .map(|s| s.into_boxed_str())
            .collect::<Vec<_>>(),
    );
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/content/images/",
        "png"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/characters/",
        "png"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/characters/",
        "webp"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/icons/",
        "png"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/icons/",
        "svg"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/icons/feedstocks/",
        "png"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/icons/features/",
        "png"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/icons/pips/",
        "png"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/icons/industries/",
        "png"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/icons/npcs/",
        "svg"
    ));
    preload.extend(load_images!(
        "$CARGO_MANIFEST_DIR/public/assets/icons/hud/",
        "svg"
    ));

    // A little hacky (ideally we do this at compile time)
    // but turn the file paths into the proper urls.
    let root = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    preload
        .into_iter()
        .map(|path| path.replace(&root, "").to_string())
        .collect()
}

#[component]
pub fn Loading(
    #[prop(into)] on_ready: Callback<()>,
) -> impl IntoView {
    audio::play_phase_music("/assets/music/143208__klerrp__maxtor-diamondmax-d540x-5400rpm-bb.mp3", false);

    let preload = preload_assets();
    let n_images = preload.len();

    let (loaded, set_n_loaded) = create_signal(0);
    let loading_bar = move || {
        let p = loaded.get() as f32 / n_images as f32 * 100.;
        let width = format!("{p}%");
        view! {
            <div class="loading-bar">
                <div class="loading-bar-fill" style:width=width></div>
            </div>
        }
    };

    view! {
        <div id="loading">
            <div>
                <img src="/assets/gosplant.svg"/>
                <div class="loading-text">
                    {t!("Booting Up")}
                </div>
                {loading_bar}

                <div class="fonts">
                    <span style:font-family="W95FA">a</span>
                    <span style:font-family="Inter">a</span>
                    <span style:font-family="Times Ten">a</span>
                </div>

                {preload
                    .into_iter()
                    .map(|url| {
                        view! {
                            <img
                                style:display="none"
                                src=url.to_string()
                                on:load=move |_| {
                                    set_n_loaded.update(|count| *count += 1);
                                    if loaded.get() >= n_images {
                                        on_ready.call(());
                                    }
                                }
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
