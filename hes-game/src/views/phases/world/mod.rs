mod event;
mod update;

use crate::{
    audio,
    consts,
    debug::get_debug_opts,
    display,
    state,
    state::{GameState, Phase},
    t,
    ui,
    ui_rw,
    views::{
        events::Events,
        globe::{Globe, GlobeRef},
        hud::Hud,
        phases::world::update::Update,
    },
    write_state,
};
use hes_engine::{
    events::{Phase as EventPhase, ICON_EVENTS},
    game::Update as EngineUpdate,
};
use leptos::*;
use leptos_use::{
    use_raf_fn_with_options,
    utils::Pausable,
    UseRafFnCallbackArgs,
    UseRafFnOptions,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::LazyLock,
};

#[derive(Clone)]
struct Toast {
    id: usize,
    icon: String,
    desc: String,
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

struct WorldEvent {
    event_id: usize,
    region: Option<(usize, String)>,

    /// When in the year the event occurs.
    when: f32,
}

/// Get events scheduled for at or earlier than the provided time.
fn pop_icon_events(
    events: &mut Vec<WorldEvent>,
    time: f32,
) -> Vec<WorldEvent> {
    events.extract_if(|ev| ev.when <= time).collect()
}

#[component]
pub fn WorldEvents() -> impl IntoView {
    let state =
        expect_context::<RwSignal<crate::state::GameState>>();

    let (events, set_events) = create_signal(vec![]);
    let icon_events = store_value(vec![]);
    create_effect(move |_| {
        state.update(|state| {
            let events = state.game.roll_events_for_phase(
                EventPhase::WorldStart,
                None,
            );
            logging::log!("rolled world events: {:?}", events);
            set_events.set(events);
        });
    });

    let (updates, set_updates) = create_signal::<
        VecDeque<EngineUpdate>,
    >(VecDeque::new());

    let year = state!(world.year);
    let start_temp = ui!(cycle_start_state.temperature);
    let cycle_start_year = ui!(cycle_start_state.year);

    let skipping = store_value(false);
    let skip = move |_| skipping.set_value(true);
    let bg_color = move || warming_colour(start_temp.get());

    let debug = get_debug_opts();
    let ms_per_year = move || {
        if skipping.get_value() {
            10.
        } else if debug.fast_years {
            500.
        } else {
            consts::MS_PER_YEAR
        }
    };
    let (time, set_time) = create_signal(0.);
    let progress = move || {
        let progress = time.get() / ms_per_year() as f32;
        display::percent(progress, false)
    };

    let (_, set_phase) = ui_rw!(phase);
    let done = store_value(false);
    let stopped = store_value(false);

    let (toasts, set_toasts) =
        create_signal::<Vec<Toast>>(vec![]);
    let n_toasts =
        move || toasts.with(|toasts| toasts.len() as f32);

    let (globe, set_globe) =
        create_signal::<Option<GlobeRef>>(None);
    let show_event_on_globe =
        move |event_id: usize,
              region_id: usize,
              region_name: &str| {
            if let Some(ev) = ICON_EVENTS.get(&event_id) {
                if let Some(globe) = globe.get() {
                    write_state!(|state, ui| {
                        let region_events = ui
                            .annual_region_events
                            .entry(region_id)
                            .or_default();
                        region_events.push(ev.clone());
                    });

                    globe.show_icon_event(
                        region_name,
                        ev.is_over_water(),
                        &ev.icon,
                        ev.intensity,
                    );

                    let effect = ev.intensity as f32 * consts::EVENT_INTENSITY_TO_CONTENTEDNESS;
                    write_state!(|state, ui| {
                        state.change_habitability(
                            -effect.round() as isize,
                            region_id,
                        );
                        state.apply_event(
                            event_id,
                            Some(region_id),
                        );
                    });

                    let toast_id =
                        js_sys::Math::random() * 1e10;
                    let toast = Toast {
                        id: toast_id.round() as usize,
                        icon: ev.icon.clone(),
                        desc: t!("{disaster} in {region}", disaster: t!(&ev.name), region: t!(region_name)),
                    };
                    set_toasts.update(move |toasts| {
                        toasts.push(toast);
                        if toasts.len() > 3 {
                            toasts.remove(0);
                        }
                    });
                }
            }
        };

    let start_year = move || {
        set_time.set(0.);
        if let Some(globe) = globe.get() {
            globe.rotate(true);
        }

        state.update(move |state| {
            let disasters: Vec<_> = state
                .game
                .roll_events_for_phase(EventPhase::Icon, None)
                .into_iter()
                .map(|ev| WorldEvent {
                    event_id: ev.id,
                    region: ev.region,
                    when: js_sys::Math::random() as f32,
                })
                .collect();
            icon_events.set_value(disasters);
        });
    };

    let roll_event = move || {
        let cur_year = year.get();
        if cur_year > cycle_start_year.get()
            && cur_year % 5 == 0
        {
            stopped.set_value(true);
            done.set_value(true);

            if updates.with(|updates| updates.is_empty())
                || skipping.get_value()
            {
                write_state!(|state, ui| {
                    state.step_cycle();
                });
                set_phase.set(Phase::Report);
                return;
            }

            state.update(|GameState { game, ui }| {
                let events = game.roll_events_for_phase(
                    EventPhase::WorldMain,
                    None,
                );
                for event in &events {
                    ui.world_events.push(event.id);
                    // ui.events.push(event.id, region_id, ev["ref_id"]); // TODO
                }

                // TODO globe update surface?
                // or pass tgav to globe component as a signal so it automatically updates
                ui.past_emissions.push((
                    game.co2_emissions as f64,
                    game.ch4_emissions as f64,
                    game.n2o_emissions as f64,
                ));
                let past_emissions = ui.past_emissions.clone();
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
                if skipping.get_value() {
                    for ev in events {
                        game.apply_event(
                            ev.event.id,
                            ev.region.map(|(id, _)| id),
                        );
                    }
                    start_year();
                } else {
                    if let Some(globe) = globe.get() {
                        globe.rotate(false);
                    }
                    set_events.set(events);
                }
                // TODO otherwise need to wait until after all events have been shown to start the
                // year
            });
        }
    };

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_raf_fn_with_options(
        move |args: UseRafFnCallbackArgs| {
            if !stopped.get_value() {
                // if (!this.showingEvent) {
                let time = time.get() + args.delta as f32;

                if time >= ms_per_year() {
                    write_state!(|state, ui| {
                        let mut updates = state.step();
                        if year.get() + 1 % 5 == 0 {
                            let mut outcomes = state
                                .roll_new_policy_outcomes();
                            updates.append(&mut outcomes);
                        }
                        if !updates.is_empty()
                            && !skipping.get_value()
                        {
                            stopped.set_value(true);
                        }

                        let completed_projects =
                            updates.iter().filter_map(
                                |update| match update {
                                    EngineUpdate::Project {
                                        id,
                                    } => Some(id),
                                    _ => None,
                                },
                            );
                        ui.cycle_start_state
                            .completed_projects
                            .extend(completed_projects);
                    });

                    roll_event();
                }
                icon_events.update_value(|events| {
                    for ev_meta in pop_icon_events(
                        events,
                        time / ms_per_year(),
                    ) {
                        if let WorldEvent {
                            event_id,
                            region:
                                Some((region_id, region_name)),
                            ..
                        } = &ev_meta
                        {
                            show_event_on_globe(
                                *event_id,
                                *region_id,
                                region_name,
                            );
                        }
                    }
                });
                set_time.set(time);
            }
        },
        UseRafFnOptions::default().immediate(false),
    );

    let show_update = move || {
        !updates.with(|u| u.is_empty()) && !skipping.get_value()
    };
    let next_update =
        move || updates.with(|u| u.front().cloned());
    let dismiss_update = move || {
        set_updates.update(|updates| {
            updates.pop_front();
            let no_updates = updates.is_empty();
            if no_updates && done.get_value() {
                set_phase.set(Phase::Report);
            } else {
                stopped.set_value(!no_updates);
            }
        });
    };

    let temp = state!(world.temperature);
    let extinction = state!(world.extinction_rate);
    let emissions = state!(emissions_gt());
    create_effect(move |_| {
        if temp.get_untracked() <= 1.
            || extinction.get_untracked() <= 20.
            || emissions.get_untracked() <= 0.
        {
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

        state.update(|state| {
            state.ui.cycle_start_snapshot(&state.game);
        });
    });

    let on_globe_ready = move |globe: GlobeRef| {
        globe.clear();
        globe.rotate(true);
        globe.clouds(true);
        set_globe.set(Some(globe));
        start_year();
    };

    let update = move || {
        next_update().map(|update| {
            let (update, _) = create_signal(update);
            view! { <Update update on_done=move |_| dismiss_update()/> }
        })
    };

    view! {
        <Hud/>
        <div id="event-stream">
            <div id="event-stream--year">
                {year}
                <div
                    id="event-stream-timer-fill"
                    style:width=progress
                ></div>
            </div>
            <Globe id="events-globe" on_ready=on_globe_ready bg_color/>
            {update}
            <Events events on_advance=|_| {} on_done=|_| {}/>
            <div id="event-stream--toasts">
                <For
                    each=move || {
                        toasts.get().into_iter().enumerate().collect::<Vec<_>>()
                    }

                    key=|(i, _)| *i
                    children=move |(i, toast): (usize, Toast)| {
                        let opacity = (i as f32 + 1.) / (n_toasts() + 1.);
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
            <button class="events--skip btn" on:click=skip>
                {t!("Skip")}
            </button>
        </div>
    }
}
