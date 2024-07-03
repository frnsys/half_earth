use super::{parts::IntensityBar, Credits};
use crate::{display::intensity, icons, state, state::Settings, t};
use js_sys::Date;
use leptos::*;
use leptos_use::{use_interval_fn, utils::Pausable};

#[component]
pub fn Menu(set_open: WriteSignal<bool>) -> impl IntoView {
    const LOCALES: &[&str] = &[
        "Havana",
        "Ouagadougou",
        "Port-au-Prince",
        "San Cristóbal de las Casas",
        "Paris",
        "Bandung",
        "Seattle",
        "Hanoi",
        "Dar es Salaam",
        "Ayn Issa",
        "Algiers",
        "Managua",
        "Prague",
    ];

    let (settings, set_settings) = Settings::get();
    let sound = move || {
        let settings = settings.get();
        settings.sound
    };
    let hide_help = move || {
        let settings = settings.get();
        settings.hide_help
    };

    let year = state!(|game, _| game.world.year);
    let pc = state!(|game, _| game.political_capital.max(0));
    let temp = state!(|game, _| format!("{:+.1}C", game.world.temperature));
    let emissions = state!(|game, _| format!("{:.1}Gt", game.emissions_gt()));
    let contentedness =
        state!(|game, _| { intensity::scale(game.outlook(), intensity::Variable::WorldOutlook) });
    let extinction = state!(|game, _| {
        intensity::scale(game.world.extinction_rate, intensity::Variable::Extinction)
    });
    let locale = state!(|game, ui| {
        let start_year = ui.start_year;
        let year = game.world.year;
        let idx = ((year - start_year) as f32 / 5.).round() as usize % LOCALES.len();
        &LOCALES[idx]
    });
    let time_place = move || format!("{}, {}", locale(), year());

    let (show_credits, set_show_credits) = create_signal(false);

    view! {
        <div class="dropdown-menu">
            <div class="dropdown-menu-content">
                <div
                    class="dropdown-menu-close dropdown-menu-button btn"
                    on:click=move |_| {
                        set_open.set(false);
                    }
                >

                    <img src=icons::CLOSE/>
                </div>
                <header>
                    <div class="dropdown-menu-inset">
                        <img src="/public/assets/gosplant.svg"/>
                    </div>
                </header>
                <Show
                    when=move || !show_credits.get()
                    fallback=move || {
                        view! {
                            <Credits
                                set_show_credits
                                on:click=move |_| {
                                    set_show_credits.set(false);
                                }
                            />
                        }
                    }
                >

                    <div class="dropdown-menu-time">
                        <Clock/>
                        <div class="dropdown-menu-inset dropdown-menu-year">
                            {time_place}
                        </div>
                    </div>
                    <div class="dropdown-menu-inset dropdown-menu-stats">
                        <div class="dropdown-menu-stat">
                            <img src=icons::POLITICAL_CAPITAL/>
                            <div class="dropdown-menu-stat-value">{pc}</div>
                        </div>
                        <div class="dropdown-menu-stat">
                            <img src=icons::EMISSIONS/>
                            <div class="dropdown-menu-stat-value">{emissions}</div>
                        </div>
                        <div class="dropdown-menu-stat">
                            <img src=icons::WARMING/>
                            <div class="dropdown-menu-stat-value">{temp}</div>
                        </div>
                    </div>
                    <div class="dropdown-menu-stats-labels">
                        <div class="dropdown-menu-stats-label">
                            {t!("Political Capital")}
                        </div>
                        <div class="dropdown-menu-stats-label">
                            {t!("CO2 Emissions/Yr")}
                        </div>
                        <div class="dropdown-menu-stats-label">
                            {t!("Temp. Anomaly")}
                        </div>
                    </div>
                    <div class="dropdown-menu-bars">
                        <div class="dropdown-menu-inset dropdown-menu-stat">
                            <img src=icons::EXTINCTION_RATE/>
                            <IntensityBar intensity=extinction.into_signal()/>
                        </div>
                        <div class="dropdown-menu-inset dropdown-menu-stat">
                            <img src=icons::CONTENTEDNESS/>
                            <IntensityBar
                                intensity=contentedness.into_signal()
                                invert=true
                            />
                        </div>
                    </div>
                    <div class="dropdown-menu-stats-labels">
                        <div class="dropdown-menu-stats-label">
                            {t!("Extinction Rate")}
                        </div>
                        <div class="dropdown-menu-stats-label">
                            {t!("Contentedness")}
                        </div>
                    </div>
                    <img class="motto" src="/public/assets/motto.png"/>
                    <div class="dropdown-menu-buttons">
                        <div
                            class="dropdown-menu-button"
                            class:active=sound
                            on:click=move |_| {
                                set_settings
                                    .update(|settings| {
                                        settings.sound = !settings.sound;
                                    });
                            }
                        >

                            {t!("Sound")}
                            :
                            {move || if sound() { t!("On") } else { t!("Off") }}
                        </div>
                        <div
                            class="dropdown-menu-button"
                            class:active=move || !hide_help()
                            on:click=move |_| {
                                set_settings
                                    .update(|settings| {
                                        settings.hide_help = !settings.hide_help;
                                    });
                            }
                        >

                            {t!("Tips")}
                            :
                            {move || if !hide_help() { t!("On") } else { t!("Off") }}
                        </div>
                        <div
                            class="dropdown-menu-button"
                            on:click=move |_| {
                                todo!();
                            }
                        >

                            {t!("Restart Game")}
                        </div>
                        <div
                            class="dropdown-menu-button"
                            on:click=move |_| {
                                set_show_credits.set(true);
                            }
                        >

                            {t!("Credits")}
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn Clock() -> impl IntoView {
    let now = Date::new_0();
    let (time, set_time) = create_signal((now.get_seconds(), now.get_minutes(), now.get_hours()));

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            let now = Date::new_0();
            let now = (now.get_seconds(), now.get_minutes(), now.get_hours());
            set_time.set(now);
        },
        1000,
    );

    let seconds_rotate = move || {
        let (secs, _, _) = time.get();
        let deg = secs as f32 / 60. * 360. + 90.;
        format!("rotate({}deg)", deg)
    };
    let minutes_rotate = move || {
        let (_, mins, _) = time.get();
        let deg = mins as f32 / 60. * 360. + 90.;
        format!("rotate({}deg)", deg)
    };
    let hours_rotate = move || {
        let (hours, mins, _) = time.get();
        let deg = (hours as f32 / 12. * 360.) + (mins as f32 / 60. * 360. + 90.);
        format!("rotate({}deg)", deg)
    };

    view! {
        <figure class="clock--outer">
            <div class="clock--inner">
                <div class="hand hour" style:transform=hours_rotate></div>
                <div class="hand min" style:transform=minutes_rotate></div>
                <div class="hand sec" style:transform=seconds_rotate></div>
            </div>
        </figure>
    }
}
