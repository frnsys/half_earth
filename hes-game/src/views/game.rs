use leptos::*;

use crate::{
    debug::get_debug_opts,
    memo,
    state::{Phase, UIState},
    views::{
        debug::DebugEvents,
        Cutscene,
        End,
        Interstitial,
        Planning,
        Report,
        WorldEvents,
    },
};

#[component]
pub fn Game() -> impl IntoView {
    let ui = expect_context::<RwSignal<UIState>>();
    let cur_phase = memo!(ui.phase);

    // HACK: It feels a little hacky to use `create_memo`
    // here but I ran into a weird bug where at
    // the second planning phase the `cur_phase`
    // signal would trigger twice even though I'm
    // only setting the phase once. This double-triggering
    // would then cause a weird signal disposal bug that
    // I couldn't figure out. Wrapping this in a memo
    // ensures that it will only be called when the value changes,
    // so a double-triggering of the same phase will still
    // only render once.
    let game_view = create_memo(move |_| {
        let phase = cur_phase.get();
        match phase {
            Phase::Intro => view! { <Cutscene/> }.into_view(),
            Phase::Interstitial => {
                view! { <Interstitial/> }.into_view()
            }
            Phase::GameOver => {
                view! { <End lose=true/> }.into_view()
            }
            Phase::GameWin => {
                view! { <End lose=false/> }.into_view()
            }
            Phase::Planning => {
                view! { <Planning/> }.into_view()
            }
            Phase::Report => view! { <Report/> }.into_view(),
            Phase::Events => {
                view! { <WorldEvents /> }.into_view()
            }
        }
    });

    if get_debug_opts().check_events {
        view! {
            <DebugEvents />
        }
    } else {
        game_view.into_view()
    }
}
