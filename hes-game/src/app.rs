use crate::{
    audio::init_audio,
    i18n::{get_preferred_language, load_language},
    state::UIState,
    tgav::HectorRef,
    views::{Game, Loading, Start, TipState, ToolTip},
};
use hes_engine::{State, World};
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
pub fn App() -> impl IntoView {
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

    view! {
        <Show when=move || lang.get().is_some()>
            <Show when=move || phase.get() == Phase::Start>
                <Start on_ready=move |_| {
                    update!(|phase| phase.advance());
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
    }
}
