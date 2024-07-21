mod update;

use crate::{
    audio,
    consts,
    debug::get_debug_opts,
    display,
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
use std::{collections::VecDeque, sync::LazyLock};

#[derive(Clone)]
struct Toast {
    id: usize,
    icon: String,
    desc: String,
}
impl Toast {
    fn new(ev: &IconEvent, region_name: &str) -> Self {
        let id =
            (js_sys::Math::random() * 1e10).round() as usize;
        Toast {
            id,
            icon: ev.icon.clone(),
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

#[derive(Default)]
struct DisasterStream {
    events: Vec<Disaster>,
}
impl DisasterStream {
    /// Get events scheduled for at or earlier than the provided time.
    fn pop_for_time(&mut self, time: f32) -> Vec<Disaster> {
        self.events.extract_if(|ev| ev.when <= time).collect()
    }

    fn roll_events(&mut self, game: &mut Game) {
        self.events.clear();
        self.events.extend(
            game.roll_events(EventPhase::Icon, None)
                .into_iter()
                .map(|ev| Disaster {
                    event_id: ev.id,
                    region: ev.region.clone(),
                    when: js_sys::Math::random() as f32,
                }),
        );
    }

    fn trigger_events(
        &mut self,
        globe: Option<GlobeRef>,
        time: f32,
    ) -> Vec<(&IconEvent, Id, Id, String)> {
        let mut events = vec![];
        for ev_meta in self.pop_for_time(time) {
            if let Disaster {
                event_id,
                region: Some((region_id, region_name)),
                ..
            } = ev_meta
            {
                if let Some(ev) = ICON_EVENTS.get(&event_id) {
                    if let Some(globe) = &globe {
                        globe.show_icon_event(
                            &region_name,
                            ev.is_over_water(),
                            &ev.icon,
                            ev.intensity,
                        );
                    }
                    events.push((
                        ev,
                        event_id,
                        region_id,
                        region_name,
                    ));
                }
            }
        }
        events
    }
}

#[component]
pub fn WorldEvents() -> impl IntoView {
    let state =
        expect_context::<RwSignal<crate::state::GameState>>();
    let disasters = store_value(DisasterStream::default());
    let events = create_rw_signal(vec![]);
    let updates = create_rw_signal::<VecDeque<EngineUpdate>>(
        VecDeque::new(),
    );
    let toasts = create_rw_signal::<Vec<Toast>>(vec![]);
    let (globe, set_globe) =
        create_signal::<Option<GlobeRef>>(None);
    let time_controls = create_rw_signal::<
        Option<(Callback<()>, Callback<()>)>,
    >(None);
    let ready_to_advance = store_value(false);

    create_effect(move |_| {
        update!(|state| {
            events.set(
                state
                    .game
                    .roll_events(EventPhase::WorldStart, None),
            );

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
    });

    let year = state!(world.year);
    let temp = state!(world.temperature);
    let cycle_start_year = ui!(cycle_start_state.year);

    let skipping = create_rw_signal(false);
    let skip = move |_| skipping.set(true);
    let bg_color = move || warming_colour(temp.get());

    let begin_year = move || {
        if let Some(globe) = globe.get() {
            globe.rotate(true);
        }

        logging::log!("Disaster rolling");
        update!(move |state, disasters| {
            disasters.roll_events(&mut state.game);
        });
        logging::log!("Done disaster rolling");

        if let Some((_, resume)) = time_controls.get() {
            resume.call(());
        }
    };
    let try_begin_year = move || {
        with!(|events, updates, globe| {
            logging::log!("events: {:?}", events);
            logging::log!("updates: {:?}", updates);
            if events.is_empty()
                && updates.is_empty()
                && globe.is_some()
            {
                begin_year();
            }
        });
    };

    create_effect(move |_| {
        try_begin_year();
    });

    let roll_event = move || {
        let cur_year = year.get();
        if cur_year > cycle_start_year.get()
            && cur_year % 5 == 0
        {
            ready_to_advance.set_value(true);

            if updates.with(|updates| updates.is_empty())
                || skipping.get()
            {
                state.update(|state| {
                    if let Some((pause, resume)) =
                        time_controls.get()
                    {
                        pause.call(());
                    }
                    state.game.finish_cycle();
                    state.ui.phase = Phase::Report;
                });
            }
        } else {
            state.update(|GameState { game, ui }| {
                let evs = game
                    .roll_events(EventPhase::WorldMain, None);
                for event in &evs {
                    ui.world_events.push(event.id);
                    // ui.events.push(event.id, region_id, ev["ref_id"]); // TODO
                }

                // TODO globe update surface?
                // or pass tgav to globe component as a signal so it automatically updates
                let past_emissions = ui.record_emissions(&game);
                spawn_local(async move {
                    let tgav =
                        calc_tgav(year.get(), past_emissions)
                            .await
                            .unwrap();
                    state.update(|state| {
                        state.game.set_tgav(tgav);
                    });
                });

                // If skipping, just apply all events.
                if skipping.get() {
                    for ev in evs {
                        game.apply_event(
                            ev.event.id,
                            ev.region
                                .as_ref()
                                .map(|(id, _)| *id),
                        );
                    }
                    begin_year();
                } else {
                    if let Some(globe) = globe.get() {
                        globe.rotate(false);
                    }
                    events.set(evs);
                }
                // TODO otherwise need to wait until after all events have been shown to start the
                // year
            });
        }
    };

    // At the end of the year,
    // advance the game engine and
    // check for any updates, then
    // roll for new events.
    let on_year_end = move |_| {
        if let Some((pause, resume)) = time_controls.get() {
            pause.call(());
            state.update(|state| {
                let updates = state.step_year();

                // If no updates or we're skipping we can
                // immediately start the next year.
                if skipping.get() {
                    // TODO
                    resume.call(());
                }
            });
        }
        roll_event();
    };

    // Called for each tick in the year.
    let on_tick = move |progress| {
        update!(|state, disasters, toasts| {
            // Trigger any scheduled disasters.
            let events = disasters.trigger_events(
                globe.get_untracked(),
                progress,
            );

            for (ev, event_id, region_id, region_name) in events
            {
                state.apply_disaster(ev, &event_id, &region_id);
                toasts.push(Toast::new(ev, &region_name));
            }
        });
    };

    // When all updates have been dismissed.
    let on_updates_finished = move |_| {
        if ready_to_advance.get_value() {
            update!(|state| {
                if let Some((pause, resume)) =
                    time_controls.get()
                {
                    pause.call(());
                }
                state.ui.phase = Phase::Report;
            });
        } else {
            try_begin_year();
        }
    };

    // When all events have been dismissed.
    let on_events_finished = move |_| {
        logging::log!("TRYING TO BEGIN YEAR AFTER EVENTS");
        try_begin_year();
    };

    let on_globe_ready = move |globe: GlobeRef| {
        globe.clear();
        globe.rotate(true);
        globe.clouds(true);
        set_globe.set(Some(globe));
    };

    view! {
        <Hud/>
        <div id="event-stream">
            <div id="event-stream--year">
                {year}
                <YearProgress skipping on_tick on_year_end controls=time_controls />
            </div>
            <Globe id="events-globe" on_ready=on_globe_ready bg_color/>
            <Updates updates on_done=on_updates_finished />
            <Events events on_done=on_events_finished />
            <Toasts toasts />
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
            style:width=progress
            ></div>
    }
}
