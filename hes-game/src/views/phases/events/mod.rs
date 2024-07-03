mod event;
mod update;

use crate::{
    consts,
    display::format,
    state,
    state::Phase,
    t,
    views::{
        globe::Globe, hud::Hud, phases::cutscene::Events as InnerEvents,
        phases::events::update::Update,
    },
    write_state,
};
use hes_engine::game::Update as EngineUpdate;
use leptos::*;
use std::collections::VecDeque;

#[derive(Clone)]
struct Toast {
    id: usize,
    icon: &'static str,
    desc: String,
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

#[component]
pub fn Events() -> impl IntoView {
    // TODO
    // let events = game.roll.world('Start');
    let events = vec![];

    let (updates, set_updates) = create_signal::<VecDeque<EngineUpdate>>(VecDeque::new());

    let year = state!(|state, ui| state.world.year);

    let (skipping, set_skipping) = create_signal(false);
    let skip = move |_| set_skipping.set(true);

    let bg_color = state!(|state, ui| {
        let temp = ui.cycle_start_state.temperature;
        warming_colour(temp)
    });
    let ms_per_year = move || {
        if skipping.get() {
            10
        } else {
            consts::MS_PER_YEAR
        }
    };
    let (time, set_time) = create_signal(0.);
    let progress = move || {
        let progress = time.get() / ms_per_year() as f32;
        format::percent(progress, false)
    };

    // TODO
    // let on_globe_ready = move |globe: ThaGlobe| {
    //     globe.clear();
    // };

    let (done, set_done) = create_signal(false);
    let (stopped, set_stopped) = create_signal(false);
    let show_update = move || !updates.with(|u| u.is_empty()) && !skipping.get();
    let next_update = move || updates.with(|u| u.front().cloned());
    let dismiss_update = write_state!(move |state, ui| {
        set_updates.update(|updates| {
            updates.pop_front();
            let no_updates = updates.is_empty();
            if no_updates && done.get() {
                ui.phase = Phase::Report;
            } else {
                set_stopped.set(!no_updates);
            }
        });
    });

    let (toasts, set_toasts) = create_signal::<Vec<Toast>>(vec![]);
    let n_toasts = move || toasts.with(|toasts| toasts.len() as f32);

    // TODO
    let (globe, set_globe) = create_signal(None);
    let on_globe_ready = move |globe| {
        // this.globe.clear();
        // this.globe.rotate = true;
        // if (this.globe.clouds) {
        //   this.globe.clouds.visible = true;
        // }
        // this.startYear();
        set_globe.set(Some(globe));
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
            <InnerEvents events on_advance=|_| {} on_done=|_| {}/>
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
