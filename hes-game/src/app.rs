use crate::{
    audio::init_audio,
    i18n::{get_preferred_language, load_language},
    state::{GameState, Phase},
    ui,
    views::{
        Cutscene,
        End,
        Interstitial,
        Loading,
        Planning,
        Report,
        Start,
        TipState,
        ToolTip,
        WorldEvents,
    },
};
use hes_engine::world::World;
use leptos::*;
use leptos_animation::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Root() -> impl IntoView {
    provide_meta_context();

    // id=leptos means cargo-leptos will hot-reload this stylesheet
    view! {
        <Title text="Half-Earth Socialism"/>
        <Stylesheet id="leptos" href="/pkg/hes-game.css"/>
        <Link rel="icon" type_="image/png" href="/assets/favicon/16.png" sizes="16x16" />
        <Link rel="icon" type_="image/png" href="/assets/favicon/32.png" sizes="32x32" />

        <Meta property="og:site_name" content="Half-Earth Socialism" />
        <Meta property="og:type" content="website" />
        <Meta property="og:description" content="Play as a planetary planner and decide what we should do about the climate, biodiversity, and human welfare. Can you bring the world safely to a better place?" />
        <Meta property="og:title" content="Half-Earth Socialism: The Game" />
        <Meta property="og:image" content="https://play.half.earth/assets/social.jpg" />
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:title" content="Half-Earth Socialism: The Game" />
        <Meta name="twitter:description" content="Play as a planetary planner and decide what we should do about the climate, biodiversity, and human welfare. Can you bring the world safely to a better place?" />
        <Meta name="twitter:image" content="https://play.half.earth/assets/social.jpg" />
        <Meta name="twitter:image:alt" content="A hot pink logo of the earth surrounded by grains. Underneath is the text 'Half Earth Socialism: A Planetary Crisis Planning Game'. The background is a pixelated mixture of marbled liquid." />
        <Meta name="twitter:creator" content="@VersoBooks" />
        <Meta name="twitter:site" content="@VersoBooks" />

        <Router>
            <Routes>
                <Route path="" view=App/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn App() -> impl IntoView {
    AnimationContext::provide();
    provide_context(create_rw_signal::<TipState>(
        TipState::default(),
    ));
    provide_context(create_rw_signal::<GameState>(
        GameState::new(World::default()),
    ));
    init_audio();

    let (started, set_started) = create_signal(false);
    let (loaded, set_loaded) = create_signal(false);

    let lang = create_resource(
        || (),
        |_| async move {
            let lang = get_preferred_language();
            load_language(&lang).await.unwrap()
        },
    );

    let cur_phase = ui!(phase);

    // It feels a little hacky to use `create_memo`
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
        tracing::debug!("Phase changed to {phase:?}.");
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
                {move || game_view.get()}
            </Show>
        </Show>
    }
}
