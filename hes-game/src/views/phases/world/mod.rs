mod update;

use crate::{
    audio,
    consts,
    debug::get_debug_opts,
    display,
    icons,
    state,
    state::{GameExt, GameState, Phase},
    t,
    ui,
    views::{
        events::Events,
        globe::{Globe, GlobeRef},
        hud::Hud,
        phases::world::update::Updates,
    },
};
use hes_engine::{
    events::{IconEvent, Phase as EventPhase, ICON_EVENTS},
    game::Update as EngineUpdate,
    state::State,
    Game,
    Id,
};
use leptos::*;
use leptos_use::{
    use_raf_fn_with_options,
    utils::Pausable,
    UseRafFnCallbackArgs,
    UseRafFnOptions,
};
use std::sync::LazyLock;

#[derive(Clone)]
struct Toast {
    id: usize,
    icon: &'static str,
    desc: String,
}
impl Toast {
    fn new(ev: &IconEvent, region_name: &str) -> Self {
        let id =
            (js_sys::Math::random() * 1e10).round() as usize;
        Toast {
            id,
            icon: icons::disaster_icon(&ev.icon),
            desc: t!("{disaster} in {region}", disaster: t!(&ev.name), region: t!(region_name)),
        }
    }
}

#[server(prefix = "/compute", endpoint = "tgav")]
pub async fn calc_tgav(
    year: usize,
    annual_emissions: Vec<(f64, f64, f64)>,
) -> Result<f32, ServerFnError> {
    Ok(crate::server::compute_tgav(year, &annual_emissions)
        as f32)
}

fn warming_colour(mut temp: f32) -> String {
    if temp <= 0. {
        temp = 0.1;
    }

    let mut r = 250;
    let mut g = (255. / temp).round();
    let mut b = (230. / temp).round();
    if g >= 255. {
        g = 255.;
        r = 240;
    }
    if b >= 255. {
        b = 255.;
        r = 240;
    }
    format!("rgb({r}, {g}, {b})")

    // fallback color
    // return '#fadbae';
}

#[derive(Debug)]
struct Disaster {
    event_id: Id,
    region: Option<(Id, String)>,

    /// When in the year the event occurs.
    when: f32,
}

#[component]
pub fn Disasters(
    phase: RwSignal<Subphase>,
    events: RwSignal<Vec<Disaster>>,
    #[prop(into)] skipping: Signal<bool>,
    #[prop(into)] on_done: Callback<()>,
) -> impl IntoView {
    let state =
        expect_context::<RwSignal<crate::state::GameState>>();
    let toasts = create_rw_signal::<Vec<Toast>>(vec![]);

    let (globe, set_globe) =
        create_signal::<Option<GlobeRef>>(None);
    let bg_color = move || {
        with!(|state| warming_colour(
            state.game.world.temperature
        ))
    };
    let on_globe_ready = move |globe: GlobeRef| {
        globe.clear();
        globe.rotate(true);
        globe.clouds(true);
        set_globe.set(Some(globe));
    };

    // Called for each tick in the year.
    let on_tick = move |progress| {
        update!(|state, events, toasts| {
            // Trigger any scheduled disasters.
            // Get events scheduled for at or earlier than the provided time.
            let popped: Vec<_> = events
                .extract_if(|ev| ev.when <= progress)
                .collect();

            let mut occurring = vec![];
            for ev_meta in popped {
                if let Disaster {
                    event_id,
                    region: Some((region_id, region_name)),
                    ..
                } = ev_meta
                {
                    if let Some(ev) = ICON_EVENTS.get(&event_id)
                    {
                        if let Some(globe) =
                            &globe.get_untracked()
                        {
                            globe.show_icon_event(
                                &region_name,
                                ev.is_over_water(),
                                &ev.icon,
                                ev.intensity,
                            );
                        }
                        occurring.push((
                            ev,
                            event_id,
                            region_id,
                            region_name,
                        ));
                    }
                }
            }

            for (ev, event_id, region_id, region_name) in
                occurring
            {
                state.apply_disaster(ev, &event_id, &region_id);
                toasts.push(Toast::new(ev, &region_name));
            }
        });
    };

    let time_controls = create_rw_signal::<
        Option<(Callback<()>, Callback<()>)>,
    >(None);

    create_effect(move |prev| {
        if phase.get() == Subphase::Disasters {
            if let Some(globe) = globe.get() {
                globe.rotate(true);
            }
            if let Some((_, resume)) = time_controls.get() {
                resume.call(());
            }
        }
    });

    let on_year_end = move |_| {
        if let Some(globe) = globe.get() {
            globe.rotate(false);
        }
        if let Some((pause, resume)) = time_controls.get() {
            pause.call(());
        }
        on_done.call(());
    };

    let year = state!(world.year);

    view! {
        <div id="event-stream--year">
            {year}
            <YearProgress skipping on_tick on_year_end controls=time_controls />
        </div>
        <Globe id="events-globe" on_ready=on_globe_ready bg_color/>
        <Toasts toasts />
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Subphase {
    Events,
    Disasters,
    Updates,
    Done,
}

#[component]
pub fn WorldEvents() -> impl IntoView {
    let state =
        expect_context::<RwSignal<crate::state::GameState>>();
    let phase = create_rw_signal(Subphase::Events);

    let disasters = create_rw_signal::<Vec<Disaster>>(vec![]);
    let updates = create_rw_signal::<Vec<EngineUpdate>>(vec![]);
    let events = create_rw_signal(vec![]);

    state.update_untracked(|state| {
        state.ui.cycle_start_snapshot(&state.game);

        let good = state.game.things_are_good();
        if good {
            audio::play_phase_music(
                "/assets/music/report_good.mp3",
                true,
            );
        } else {
            audio::play_phase_music(
                "/assets/music/report_bad.mp3",
                true,
            );
        }
    });
    state.update_untracked(|state| {
        events.set(
            state
                .game
                .roll_events(EventPhase::WorldStart, None),
        );
    });

    let skipping = create_rw_signal(false);
    let skip = move |_| skipping.set(true);

    let year = state!(world.year);
    let cycle_start_year = ui!(cycle_start_state.year);
    let next_phase = move || {
        let mut next = match phase.get_untracked() {
            Subphase::Disasters => Subphase::Updates,
            Subphase::Updates => Subphase::Events,
            Subphase::Events => Subphase::Disasters,
            Subphase::Done => Subphase::Done,
        };

        if next == Subphase::Updates {
            state.update_untracked(|game| {
                let step_updates = game.step_year();
                if step_updates.is_empty() || skipping.get() {
                    // Skip to next phase.
                    next = Subphase::Events;
                } else {
                    updates.set(step_updates.into());
                }

                // TODO globe update surface?
                // or pass tgav to globe component as a signal so it automatically updates
                let year = game.game.world.year;
                let past_emissions =
                    game.ui.record_emissions(&game.game);
                spawn_local(async move {
                    let tgav = calc_tgav(year, past_emissions)
                        .await
                        .unwrap();
                    state.update(|state| {
                        state.game.set_tgav(tgav);
                    });
                });
            });
        }

        if next == Subphase::Events {
            state.update_untracked(|GameState { game, ui }| {
                let evs = game
                    .roll_events(EventPhase::WorldMain, None);
                for event in &evs {
                    ui.world_events.push(event.clone());
                }

                if evs.is_empty() || skipping.get() {
                    next = Subphase::Disasters;
                } else {
                    events.set(evs);
                }
            });
        }

        // This phase is never skipped.
        if next == Subphase::Disasters {
            let cur_year = year.get();
            if cur_year > cycle_start_year.get()
                && cur_year % 5 == 0
            {
                update!(|state| {
                    state.game.finish_cycle();
                    state.ui.phase = Phase::Report;
                });
                next = Subphase::Done;
            } else {
                state.update_untracked(
                    |GameState { game, ui }| {
                        let evs: Vec<_> = game
                            .roll_events(EventPhase::Icon, None)
                            .into_iter()
                            .map(|ev| Disaster {
                                event_id: ev.id,
                                region: ev.region.clone(),
                                when: js_sys::Math::random()
                                    as f32,
                            })
                            .collect();
                        disasters.set(evs);
                    },
                );
            }
        }

        phase.set(next);
    };

    view! {
        <Hud/>
        <div id="event-stream">
            <Disasters phase skipping events=disasters on_done=move |_| {
                next_phase();
            } />
            <Show when=move || phase.get() == Subphase::Updates>
                <Updates updates on_done=move |_| {
                    next_phase();
                } />
            </Show>
            <Show when=move || phase.get() == Subphase::Events>
                <Events events on_done=move |_| {
                    next_phase();
                } />
            </Show>
            <button class="events--skip btn" on:click=skip>
                {t!("Skip")}
            </button>
        </div>
    }
}

#[component]
fn Toasts(toasts: RwSignal<Vec<Toast>>) -> impl IntoView {
    let n_toasts = move || toasts.with(|toasts| toasts.len());

    create_effect(move |_| {
        toasts.update(|toasts| {
            if toasts.len() > 3 {
                toasts.remove(0);
            }
        });
    });

    view! {
        <div id="event-stream--toasts">
            <For
            each=move || {
                toasts.get().into_iter().enumerate().collect::<Vec<_>>()
            }

        key=|(i, _)| *i
            children=move |(i, toast): (usize, Toast)| {
                let opacity = (i + 1) as f32 / (n_toasts() + 1) as f32;
                view! {
                    <div class="toast" style:opacity=opacity>
                        <div class="toast--body">
                            <img src=toast.icon/>
                            {toast.desc}
                        </div>
                    </div>
                }
            }
        />

            </div>
    }
}

#[component]
fn YearProgress(
    #[prop(into)] skipping: Signal<bool>,
    #[prop(into)] on_tick: Callback<f32>,
    #[prop(into)] on_year_end: Callback<()>,
    #[prop(into)] controls: RwSignal<
        Option<(Callback<()>, Callback<()>)>,
    >,
) -> impl IntoView {
    let time = create_rw_signal(0.);
    let ms_per_year = move || {
        if skipping.get() {
            10.
        } else if get_debug_opts().fast_years {
            500.
        } else {
            consts::MS_PER_YEAR
        }
    };
    let progress = move || {
        let progress = time.get() / ms_per_year() as f32;
        display::percent(progress, false)
    };

    let raf = use_raf_fn_with_options(
        move |args: UseRafFnCallbackArgs| {
            time.try_update(|time| {
                *time += args.delta as f32;
                let progress = *time / ms_per_year();
                on_tick.call(progress);
                if *time >= ms_per_year() {
                    on_year_end.call(());
                    *time = 0.;
                }
            });
        },
        UseRafFnOptions::default().immediate(false),
    );
    let pause = move |_| {
        (raf.pause)();
    };
    let resume = move |_| {
        (raf.resume)();
    };
    controls.set(Some((pause.into(), resume.into())));

    view! {
        <div
            id="event-stream-timer-fill"
            style:width=progress />
    }
}
