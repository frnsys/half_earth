mod update;

use std::collections::HashMap;
pub use update::Updates;

use crate::{
    audio,
    consts,
    debug::get_debug_opts,
    display,
    icons,
    memo,
    state::{Phase, StateExt, UIState},
    t,
    tgav::HectorRef,
    views::{
        events::Events,
        globe::{Globe, GlobeRef},
        hud::Hud,
    },
};
use hes_engine::{
    EventPhase,
    IconEvent,
    Id,
    State,
    Update as EngineUpdate,
    ICON_EVENTS,
};
use leptos::*;
use leptos_use::{
    use_raf_fn_with_options,
    UseRafFnCallbackArgs,
    UseRafFnOptions,
};

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

fn get_emissions(state: &State) -> HashMap<&'static str, f64> {
    // Set an upper cap to the amount of emissions we pass to hector,
    // because very large numbers end up breaking it.
    let emissions_factor = (consts::MAX_EMISSIONS
        / state.emissions.as_gtco2eq().abs())
    .min(1.0);

    let (co2, ch4, n2o) = state.emissions.for_hector();
    let mut emissions = HashMap::default();
    emissions.insert(
        "ffi_emissions",
        (co2 * emissions_factor) as f64,
    );
    emissions.insert(
        "CH4_emissions",
        (ch4 * emissions_factor) as f64,
    );
    emissions.insert(
        "N2O_emissions",
        (n2o * emissions_factor) as f64,
    );
    emissions
}

#[derive(Debug)]
struct Disaster {
    event_id: Id,
    region: Option<(Id, String)>,

    /// When in the year the event occurs.
    when: f32,
}

#[component]
fn Disasters(
    year: RwSignal<usize>,
    phase: RwSignal<Subphase>,
    events: RwSignal<Vec<Disaster>>,
    #[prop(into)] skipping: Signal<bool>,
    #[prop(into)] on_done: Callback<()>,
) -> impl IntoView {
    let ui = expect_context::<RwSignal<UIState>>();
    let game = expect_context::<RwSignal<State>>();
    let toasts = create_rw_signal::<Vec<Toast>>(vec![]);

    let (globe, set_globe) =
        create_signal::<Option<GlobeRef>>(None);

    let temp = memo!(game.world.temperature);
    let bg_color = move || with!(|temp| warming_colour(*temp));
    let on_globe_ready = move |globe: GlobeRef| {
        globe.clear();
        globe.rotate(true);
        globe.clouds(true);
        set_globe.set(Some(globe));
    };

    let time_controls = create_rw_signal::<
        Option<(Callback<()>, Callback<()>)>,
    >(None);

    // Called for each tick in the year.
    let on_tick = move |progress| {
        update!(|events, toasts| {
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
                ui.update_untracked(|ui| {
                    let region_events = ui
                        .annual_region_events
                        .entry(region_id)
                        .or_default();
                    region_events.push(ev.clone());
                });
                game.update(|game| {
                    StateExt::apply_disaster(
                        game, ev, &event_id, &region_id,
                    );
                });
                toasts.push(Toast::new(ev, &region_name));
            }
        });
    };

    create_effect(move |_| {
        let phase = phase.get();
        if phase == Subphase::Disasters {
            if let Some(globe) = globe.get() {
                globe.rotate(true);
            }
            if let Some((_, resume)) = time_controls.get() {
                resume.call(());
            }
        } else {
            if let Some((pause, _)) = time_controls.get() {
                pause.call(());
            }
        }
    });

    let on_year_end = move |_| {
        if let Some(globe) = globe.get_untracked() {
            globe.rotate(false);
        }
        if let Some((pause, _)) = time_controls.get_untracked()
        {
            pause.call(());
        }
        on_done.call(());
    };

    view! {
        <div id="event-stream--year">
            {year}
            <YearProgress skipping on_tick on_year_end controls=time_controls />
        </div>
        <Globe id="events-globe" on_ready=on_globe_ready bg_color/>
        <Toasts toasts />
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Subphase {
    Events,
    Disasters,
    StepYear,
    Updates,
    Done,
}

#[component]
pub fn WorldEvents() -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

    let phase = create_rw_signal(Subphase::Events);

    let disasters = create_rw_signal::<Vec<Disaster>>(vec![]);
    let updates = create_rw_signal::<Vec<EngineUpdate>>(vec![]);
    let events = create_rw_signal(vec![]);

    game.update_untracked(|game| {
        ui.update_untracked(|ui| {
            ui.cycle_start_snapshot(&game);

            let good = game.things_are_good();
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
    game.update_untracked(|game| {
        events.set(StateExt::roll_events(
            game,
            EventPhase::WorldStart,
        ));
    });

    let skipping = create_rw_signal(get_debug_opts().always_skip_world);
    let skip = move |_| skipping.set(true);

    let year = create_rw_signal(with!(|game| game.world.year));
    let cycle_start_year = memo!(ui.cycle_start_state.year);
    let (_, set_game_phase) = slice!(ui.phase);

    let hector = expect_context::<StoredValue<HectorRef>>();

    let next_phase = move || {
        let mut next = match phase.get_untracked() {
            Subphase::Disasters => Subphase::StepYear,
            Subphase::StepYear => Subphase::Updates,
            Subphase::Updates => Subphase::Events,
            Subphase::Events => Subphase::Disasters,
            Subphase::Done => Subphase::Done,
        };

        if next == Subphase::StepYear {
            // Update the temp anomaly.
            spawn_local(async move {
                let emissions = game
                    .with_untracked(|game| get_emissions(game));
                let hector = hector.get_value();
                hector.add_emissions(emissions);
                let tgav = hector.calc_tgav().await as f32;

                // Advance the year.
                game.update_untracked(|game| {
                    let step_updates = game.step_year(tgav);
                    let completed_projects = step_updates
                        .iter()
                        .filter_map(|update| match update {
                            EngineUpdate::Project { id } => {
                                Some(id)
                            }
                            EngineUpdate::Policy { id } => {
                                Some(id)
                            }
                            _ => None,
                        });

                    ui.update_untracked(|ui| {
                        ui.cycle_start_state
                            .completed_projects
                            .extend(completed_projects);
                    });

                    updates.set(step_updates.into());
                });

                let cur_year =
                    game.with_untracked(|game| game.world.year);
                year.set(cur_year);
            });
        }

        if next == Subphase::Updates {
            if with!(|updates| updates.is_empty())
                || skipping.get()
            {
                next = Subphase::Events;
            }
        }

        if next == Subphase::Events {
            game.update_untracked(|game| {
                let evs = StateExt::roll_events(
                    game,
                    EventPhase::WorldMain,
                );

                ui.update_untracked(|ui| {
                    for event in &evs {
                        ui.world_events.push(event.clone());
                    }
                });

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
                update!(|game| {
                    game.finish_cycle();
                });
                set_game_phase.set(Phase::Report);
                next = Subphase::Done;
            } else {
                game.update_untracked(|game| {
                    let evs: Vec<_> = StateExt::roll_events(
                        game,
                        EventPhase::Icon,
                    )
                    .into_iter()
                    .map(|ev| Disaster {
                        event_id: ev.id,
                        region: ev.region.clone(),
                        when: js_sys::Math::random() as f32,
                    })
                    .collect();
                    disasters.set(evs);
                });
            }
        }

        phase.set(next);
    };

    create_effect(move |_| {
        year.track();

        // A hack to call `next_phase` outside of this effect,
        // to avoid borrow conflicts.
        set_timeout(
            move || {
                next_phase();
            },
            std::time::Duration::from_millis(10),
        );
    });

    view! {
        <Hud/>
        <div id="event-stream">
            <Disasters year phase skipping events=disasters on_done=move |_| {
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
            key=|(_, toast)| toast.id
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
        let percent = display::percent(progress, false);
        format!("{percent}%")
    };

    let raf = use_raf_fn_with_options(
        move |args: UseRafFnCallbackArgs| {
            time.try_update(|time| {
                let delta = args.delta.min(30.) as f32;
                *time += delta;
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
