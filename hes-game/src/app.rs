use crate::{
    audio::init_audio,
    debug::get_debug_opts,
    i18n::{get_preferred_language, load_language},
    state::UIState,
    tgav::HectorRef,
    views::{
        DebugEvents,
        Game,
        Loading,
        Start,
        TipState,
        ToolTip,
    },
};
use hes_engine::{State, World};
use leptos::*;
use leptos_animation::*;
use leptos_router::*;

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="" view=App/>
            </Routes>
        </Router>
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Start,
    Loading,
    Ready,
}
impl Phase {
    pub fn advance(&mut self) {
        let next = match self {
            Self::Start => Self::Loading,
            Self::Loading => Self::Ready,
            Self::Ready => Self::Ready,
        };
        *self = next;
    }
}

#[component]
fn App() -> impl IntoView {
    AnimationContext::provide();
    provide_context(create_rw_signal::<TipState>(
        TipState::default(),
    ));

    let (game, ui) = crate::state::new_game(World::default());
    let year = game.world.year;

    provide_context(create_rw_signal::<State>(game));
    provide_context(create_rw_signal::<UIState>(ui));
    provide_context(store_value(HectorRef::new(year)));

    init_audio();

    let phase = create_rw_signal(Phase::Start);

    let lang = create_resource(
        || (),
        |_| async move {
            let lang = get_preferred_language();
            load_language(&lang).await.unwrap()
        },
    );

    if get_debug_opts().check_events {
        view! {
            <ToolTip />
            <DebugEvents />
        }
        .into_view()
    } else {
        view! {
            <Show when=move || lang.get().is_some()>
                <Show when=move || phase.get() == Phase::Start>
                    <Start on_ready=move |_| {
                        if get_debug_opts().skip_to_planning {
                            update!(|phase| *phase = Phase::Ready);
                        } else {
                            update!(|phase| phase.advance());
                        }
                    } />
                </Show>
                <Show when=move || phase.get() == Phase::Loading>
                    <Loading on_ready=move |_| {
                        update!(|phase| phase.advance());
                    }/>
                </Show>
                <Show when=move || phase.get() == Phase::Ready>
                    <ToolTip/>
                    <Game />
                </Show>
            </Show>
        }.into_view()
    }
}
