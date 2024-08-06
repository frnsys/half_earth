use crate::{
    audio,
    icons,
    memo,
    state::{start_new_run, Settings, UIState},
    t,
    views::{
        intensity::{self, IntensityBar},
        splash::Credits,
    },
};
use hes_engine::State;
use js_sys::Date;
use leptos::*;
use leptos_use::use_interval_fn;

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

#[component]
pub fn Menu(set_open: WriteSignal<bool>) -> impl IntoView {
    let (settings, set_settings) = Settings::rw();
    let sound = memo!(settings.sound);
    let hide_help = memo!(settings.hide_help);

    let (show_credits, set_show_credits) = create_signal(false);

    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();
    let year = memo!(game.world.year);
    let pc = memo!(game.political_capital.max(0));
    let outlook = memo!(game.outlook());
    let emissions = memo!(game.emissions.as_gtco2eq());
    let extinction = memo!(game.world.extinction_rate);
    let temperature = memo!(game.world.temperature);
    let start_year = memo!(ui.start_year);

    let temp = move || format!("{:+.1}C", temperature.get());
    let emissions = move || format!("{:.1}Gt", emissions.get());
    let contentedness = move || {
        intensity::scale(
            outlook.get(),
            intensity::Variable::WorldOutlook,
        )
    };
    let extinction = move || {
        intensity::scale(
            extinction.get(),
            intensity::Variable::Extinction,
        )
    };
    let locale = move || {
        let elapsed = year.get() - start_year.get();
        let idx = (elapsed as f32 / 5.).round() as usize
            % LOCALES.len();
        &LOCALES[idx]
    };
    let time_place =
        move || format!("{}, {}", locale(), year.get());

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
                        <img src="/assets/gosplant.svg"/>
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
                    <img class="motto" src="/assets/motto.png"/>
                    <div class="dropdown-menu-buttons">
                        <div
                            class="dropdown-menu-button"
                            class:active=sound
                            on:click=move |_| {
                                set_settings
                                    .update(|settings| {
                                        settings.sound = !settings.sound;
                                        if !settings.sound {
                                            audio::mute();
                                        } else {
                                            audio::unmute();
                                        }
                                    });
                            }
                        >
                            {t!("Sound")}
                            :
                            {move || if sound.get() { t!("On") } else { t!("Off") }}
                        </div>
                        <div
                            class="dropdown-menu-button"
                            class:active=move || !hide_help.get()
                            on:click=move |_| {
                                set_settings
                                    .update(|settings| {
                                        settings.hide_help = !settings.hide_help;
                                    });
                            }
                        >
                            {t!("Tips")}
                            :
                            {move || if !hide_help.get() { t!("On") } else { t!("Off") }}
                        </div>
                        <div
                            class="dropdown-menu-button"
                            on:click=move |_| {
                                start_new_run();
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
    let (time, set_time) = create_signal((
        now.get_seconds(),
        now.get_minutes(),
        now.get_hours(),
    ));

    use_interval_fn(
        move || {
            let now = Date::new_0();
            let now = (
                now.get_seconds(),
                now.get_minutes(),
                now.get_hours(),
            );
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
        let (_, mins, hours) = time.get();
        let deg = (hours as f32 / 12. * 360.)
            + (mins as f32 / 60. * 360. + 90.);
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
