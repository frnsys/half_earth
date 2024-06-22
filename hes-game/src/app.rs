use crate::components::Start;
use crate::i18n::{get_preferred_language, load_language, Language, AVAILABLE_LANGUAGES};
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::{logging::log, *};
use leptos_router::*;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Router fallback=|| {
            view! {
                <div>TODO</div>
            }
            .into_view()
        }>
            <Routes>
                <Route path="" view=App />
            </Routes>
        </Router>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (started, set_started) = create_signal(false);
    let (loaded, set_loaded) = create_signal(false);

    let lang = create_resource(
        || (),
        |_| async move {
            let lang = get_preferred_language();
            load_language(&lang).await.unwrap()
        },
    );

    view! {
        <Show when={move || lang.get().is_some()}>
            <Show when={move || !started.get()}>
                <Start set_started />
            </Show>
            <Show when={move || started.get() && !loaded.get()}>
                <div>LOADING</div>
            </Show>
            <Show when={move || started.get() && loaded.get()}>
                <div>STARTED</div>
            </Show>
        </Show>
    }
}
