use crate::{
    i18n,
    state::{GameState, Settings},
    t,
};
use leptos::*;
use list_files_macro::list_files;

fn preload_assets() -> Vec<String> {
    const PRELOAD: &[&str] = &[
        "/public/assets/stamp.svg",
        "/public/assets/backgrounds/menu.jpg",
        "/public/assets/backgrounds/dashboard.png",
        "/public/assets/backgrounds/parliament.png",
        "/public/assets/backgrounds/plan.png",
        "/public/assets/backgrounds/regions.png",
        "/public/assets/backgrounds/report.png",
        "/public/assets/gosplant.svg",
        "/public/assets/clock.png",
        "/public/assets/motto.png",
    ];

    let mut preload = Vec::from(PRELOAD);
    preload.extend(list_files!("../../../public/assets/content/images/*.png"));
    preload.extend(list_files!("../../../public/assets/characters/*.png"));
    preload.extend(list_files!("../../../public/assets/icons/*.png"));
    preload.extend(list_files!("../../../public/assets/icons/*.svg"));
    preload.extend(list_files!("../../../public/assets/icons/feedstocks/*.png"));
    preload.extend(list_files!("../../../public/assets/icons/features/*.png"));
    preload.extend(list_files!("../../../public/assets/icons/pips/*.png"));
    preload.extend(list_files!("../../../public/assets/icons/industries/*.png"));
    preload.extend(list_files!("../../../public/assets/icons/npcs/*.svg"));
    preload.extend(list_files!("../../../public/assets/icons/hud/*.svg"));

    // A little hacky (ideally we do this at compile time)
    // but turn the file paths into the proper urls.
    preload
        .into_iter()
        .map(|path| path.replace(env!("CARGO_MANIFEST_DIR"), "").to_string())
        .collect()
}

#[component]
pub fn Loading(set_loaded: WriteSignal<bool>) -> impl IntoView {

    // TODO
    // window.audioManager.startSoundtrack('/assets/music/143208__klerrp__maxtor-diamondmax-d540x-5400rpm-bb.mp3');

    on_cleanup(|| {
        // TODO
        // window.audioManager.stopSoundtrack();
    });

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
                <img src="/public/assets/gosplant.svg"/>
                <div class="loading-text">
                    {if GameState::has_save() {
                        t!("Loading saved data")
                    } else {
                        t!("Booting Up")
                    }}

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
                                        set_loaded.set(true);
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
