use crate::i18n::{get_preferred_language, load_language};
use crate::views::{Cutscene, End, Interstitial, Loading, Start, Tip, ToolTip};
use crate::{
    state,
    state::{GameState, Phase},
};
use leptos::*;
use leptos_animation::*;
use leptos_router::*;

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Router fallback=|| {
            view! { <div>TODO</div> }.into_view()
        }>
            <Routes>
                <Route path="" view=App/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn App() -> impl IntoView {
    AnimationContext::provide();
    provide_context(create_rw_signal::<Option<Tip>>(None));
    provide_context(create_signal::<GameState>(GameState::load()));

    let (started, set_started) = create_signal(false);
    let (loaded, set_loaded) = create_signal(false);

    let lang = create_resource(
        || (),
        |_| async move {
            let lang = get_preferred_language();
            load_language(&lang).await.unwrap()
        },
    );

    let phase = move || {
        state!(|state, ui| {
            match ui.phase {
                Phase::Intro => view! { <Cutscene/> }.into_view(),
                Phase::Interstitial => view! { <Interstitial/> }.into_view(),
                Phase::GameOver => view! { <End lose=true/> }.into_view(),
                Phase::GameWin => view! { <End lose=false/> }.into_view(),
                Phase::Planning => todo!(),
                Phase::Report => todo!(),
                Phase::Events => todo!(),
            }
        });
    };

    view! {
        <Show when=move || lang.get().is_some()>
            <Show when=move || !started.get()>
                <Start set_started/>
            </Show>
            <Show when=move || started.get() && !loaded.get()>
                <Loading set_loaded/>
            </Show>
            <Show when=move || started.get() && loaded.get()>
                <ToolTip/>
                {phase}
            </Show>
        </Show>
    }
}
