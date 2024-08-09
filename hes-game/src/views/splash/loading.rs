use crate::{audio, t};
use leptos::*;
use list_files_macro::list_files;

fn preload_assets() -> Vec<String> {
    const PRELOAD: &[&str] = &[
        "/assets/stamp.svg",
        "/assets/backgrounds/menu.jpg",
        "/assets/backgrounds/dashboard.png",
        "/assets/backgrounds/parliament.png",
        "/assets/backgrounds/plan.png",
        "/assets/backgrounds/regions.png",
        "/assets/backgrounds/report.png",
        "/assets/gosplant.svg",
        "/assets/clock.png",
        "/assets/motto.png",
    ];

    let mut preload = Vec::from(PRELOAD);
    preload.extend(list_files!(
        "../../../public/assets/content/images/*.png"
    ));
    preload.extend(list_files!(
        "../../../public/assets/characters/*.png"
    ));
    preload.extend(list_files!(
        "../../../public/assets/characters/*.webp"
    ));
    preload.extend(list_files!(
        "../../../public/assets/icons/*.png"
    ));
    preload.extend(list_files!(
        "../../../public/assets/icons/*.svg"
    ));
    preload.extend(list_files!(
        "../../../public/assets/icons/feedstocks/*.png"
    ));
    preload.extend(list_files!(
        "../../../public/assets/icons/features/*.png"
    ));
    preload.extend(list_files!(
        "../../../public/assets/icons/pips/*.png"
    ));
    preload.extend(list_files!(
        "../../../public/assets/icons/industries/*.png"
    ));
    preload.extend(list_files!(
        "../../../public/assets/icons/npcs/*.svg"
    ));
    preload.extend(list_files!(
        "../../../public/assets/icons/hud/*.svg"
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
