use super::credits::Credits;
use crate::{
    audio,
    i18n,
    state::{Settings, UIState},
    t,
    util::is_steam,
};
use hes_engine::{world::World, Game};
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};

#[derive(Clone)]
enum WorldStatus {
    Default,
    Custom(String, World),
    FailedToParse,
}
impl WorldStatus {
    fn is_custom(&self) -> bool {
        match self {
            Self::Custom(..) => true,
            _ => false,
        }
    }
}

#[component]
pub fn Start(set_started: WriteSignal<bool>) -> impl IntoView {
    let lang = expect_context::<Rc<i18n::Language>>();
    let cur_lang = lang.locale;

    let show_book_link = is_steam();
    let (show_credits, set_show_credits) = create_signal(false);

    let (settings, set_settings) = Settings::rw();
    let sound = move || {
        let settings = settings.get();
        settings.sound
    };

    let game = expect_context::<RwSignal<Game>>();
    let ui = expect_context::<RwSignal<UIState>>();
    let world = create_rw_signal(WorldStatus::Default);
    view! {
        <div>
            <div id="start-bg"></div>
            <div id="start-screen">
                <div id="lang-select">
                    <select on:change=move |ev| {
                        let lang = event_target_value(&ev);
                        spawn_local(async move {
                            let query = format!("?lang={}", lang);
                            let _ = window().location().set_search(&query);
                        });
                    }>
                        <For
                            each=move || i18n::AVAILABLE_LANGUAGES.iter()
                            key=|s: &&&str| s.to_string()
                            children=move |s: &&str| {
                                let label = s.to_uppercase();
                                view! {
                                    <option value=*s selected=move || &cur_lang == s>
                                        {&label}
                                    </option>
                                }
                            }
                        />
                    </select>
                </div>
                <Show when=move || show_credits.get()>
                    <Credits
                        set_show_credits
                        on:click=move |_| {
                            set_show_credits.set(false);
                        }
                    />
                </Show>
                <div id="start-screen-inset">
                    <div id="start-inner">
                        <img src="/assets/intro.svg"/>
                        <div class="start-subtitle">{t!("A Planetary Crisis Planning Game")}</div>
                        <Show when=|| crate::state::has_save()>
                            <button
                                class="continue-button"
                                on:click=move |_| {
                                    let state = crate::state::load();
                                    game.set(state.0);
                                    ui.set(state.1);
                                    set_started.set(true);
                                }
                            >
                                {t!("Continue")}
                            </button>
                        </Show>
                        <div>
                            <button
                                class="start-button"
                                on:click=move |_| {
                                    let world = match world.get() {
                                        WorldStatus::Custom(_, world) => world,
                                        _ => World::default()
                                    };
                                    let state = crate::state::new_game(world);
                                    game.set(state.0);
                                    ui.set(state.1);
                                    set_started.set(true);
                                }
                            >
                                {t!("New Game")}
                            </button>
                            <div class="world-picker"
                                class:world-selected={move || with!(|world| world.is_custom())}>
                                <label>
                                   <img src="/assets/world.png"/>
                                    <input
                                        type="file"
                                        multiple=false
                                        on:input=move |ev| {
                                            let files = ev.target().unwrap()
                                                .unchecked_ref::<web_sys::HtmlInputElement>()
                                                .files().unwrap();
                                            if let Some(file) = files.get(0) {
                                                let name = file.name();
                                                let reader = web_sys::FileReader::new().unwrap();
                                                let reader_clone = reader.clone();
                                                let onloadend = Closure::wrap(Box::new(move || {
                                                    if let Ok(result) = reader_clone.result() {
                                                        if let Some(text) = result.as_string() {
                                                            let w = serde_json::from_str::<World>(&text);
                                                            if let Ok(w) = w {
                                                                world.set(WorldStatus::Custom(name.clone(), w));
                                                            } else {
                                                                world.set(WorldStatus::FailedToParse);
                                                            }
                                                        }
                                                    }
                                                }) as Box<dyn Fn()>);

                                                reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                                                reader.read_as_text(&file).unwrap();
                                                onloadend.forget();
                                            }
                                        }
                                    />
                                    <span class="world-status">
                                        {move || {
                                                     with!(|world| {
                                                         match world {
                                                             WorldStatus::Default => "Default World".into(),
                                                             WorldStatus::Custom(name, _world) => format!("Custom: {name}"),
                                                             WorldStatus::FailedToParse => "Failed to parse provided world.".into(),
                                                         }
                                                     })
                                                 }}
                                    </span>
                                </label>
                                <div class="world-details">
                                    Click to load a custom world.<br />
                                    New worlds can be made using the editor.
                                </div>
                            </div>
                        </div>
                        <div class="two-buttons">
                            <button
                                class="start-button"
                                on:click=move |_| {
                                    set_settings
                                        .update(|settings| {
                                            settings.sound = !settings.sound;
                                            if settings.sound {
                                                audio::play_one_shot("/assets/sounds/notification.wav");
                                            }
                                        });
                                }
                            >
                                {t!("Sound")}
                                :
                                {move || if sound() { t!("On") } else { t!("Off") }}
                            </button>
                            <hr/>
                            <button
                                class="start-button"
                                on:click=move |_| {
                                    set_show_credits.set(true);
                                }
                            >
                                {t!("Credits")}
                            </button>
                        </div>
                        <Show
                            when=move || { show_book_link }
                            fallback=|| {
                                view! {
                                    <a class="book-line">
                                        <span>
                                            {t!("Based on the book")} :&nbsp;
                                            <em>Half-Earth Socialism</em> .
                                        </span>
                                    </a>
                                }
                            }
                        >
                            <a
                                class="book-line "
                                target="_blank"
                                href="https://www.versobooks.com/books/3818-half-earth-socialism"
                                v-else
                            >
                                <span>{t!("Read the book")} : <em>Half-Earth Socialism</em> .</span>
                            </a>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}
