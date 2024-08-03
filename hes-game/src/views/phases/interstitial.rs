use std::time::Duration;

use leptos::*;

use crate::{
    audio,
    memo,
    state::{GameExt, Phase, UIState},
    t,
    views::{events::Events, intensity, rank_factors},
};
use hes_engine::{events::Phase as EventPhase, Game};

struct Locale {
    name: &'static str,
    background: &'static str,
    ambience: &'static str,
    credit: &'static str,
}

// List from Troy:
// Bandung, Hanoi, Mexico City, Budapest, Thiruvananthapuram, Luanda, Ayn Issa, Ferrara, Vienna, Beijing, Aden, Caracas, Algiers, Belgrade, Moscow, Managua, Buenos Aires, Trier, Prague, Porto Alegre, Seattle/Burlington/Bronx, Dar es Salaam
const LOCALES: &[Locale] = &[Locale {
  name: "Havana",
  background: "pexels-matthias-oben-3687869.jpg",
  ambience: "city_noise.mp3",
  credit: "Matthias Oben",
}, Locale {
  name: "Ouagadougou",
  background: "2560px-Ouagadougou_BCEAO_day.jpg",
  ambience: "city_noise.mp3",
  credit: "Wegmann, CC BY-SA 3.0, via Wikimedia Commons",
}, Locale {
  name: "Port-au-Prince",
  background: "robin-canfield-CkCV7vTmmz4-unsplash.jpg",
  ambience: "city_noise.mp3",
  credit: "Robin Canfield",
}, Locale {
  name: "San Cristóbal de las Casas",
  background: "1536px-Street_Scene_with_Church_Cupola_-_San_Cristobal_de_las_Casas_-_Chiapas_-_Mexico.jpg",
  ambience: "city_noise.mp3",
  credit: "Adam Jones, CC BY 2.0, via Wikimedia Commons",
}, Locale {
  name: "Paris",
  background: "pexels-pierre-blache-3073666.jpg",
  ambience: "city_noise.mp3",
  credit: "Pierre Blaché",
}, Locale {
  name: "Bandung",
  background: "Street_Braga,_Bandung_City,_West_Java,_Indonesia.jpg",
  ambience: "city_noise.mp3",
  credit: "PACARNYAKEYES, CC BY-SA 4.0, via Wikimedia Commons",
}, Locale {
  name: "Seattle",
  background: "2560px-Seattle_4.jpg",
  ambience: "city_noise.mp3",
  credit: "Daniel Schwen, CC BY-SA 4.0, via Wikimedia Commons",
}, Locale {
  name: "Hanoi",
  background: "2560px-Vietnam,_Hanoi,_Streets_of_central_Hanoi_2.jpg",
  ambience: "city_noise.mp3",
  credit: "© Vyacheslav Argenberg / http://www.vascoplanet.com/, CC BY 4.0, via Wikimedia Commons",
}, Locale {
  name: "Dar es Salaam",
  background: "Dar_es_Salaam_before_dusk.jpg",
  ambience: "city_noise.mp3",
  credit: "Muhammad Mahdi Karim, GFDL 1.2, via Wikimedia Commons",
}, Locale {
  name: "Ayn Issa",
  background: "2560px-Another_Year_Without_Daesh.jpg",
  ambience: "city_noise.mp3",
  credit: "Combined Joint Task Force - Operation Inherent Resolve/Sgt. Raymond Boyington, Public domain, via Wikimedia Commons",
}, Locale {
  name: "Algiers",
  background: "2560px-Martyrs_Memorial,_A_cloudy_day_in_Algiers.jpg",
  ambience: "city_noise.mp3",
  credit: "EL Hacene Boulkroune, CC BY-SA 4.0, via Wikimedia Commons",
}, Locale {
  name: "Managua",
  background: "Old_Managua_Cathedral_from_Highway_2.jpg",
  ambience: "city_noise.mp3",
  credit: "Byralaal, CC BY-SA 4.0, via Wikimedia Commons",
}, Locale {
  name: "Prague",
  background: "2560px-Vltava_river_in_Prague.jpg",
  ambience: "city_noise.mp3",
  credit: "Dmitry A. Mottl, CC BY-SA 4.0, via Wikimedia Commons",
}, Locale {
  name: "Havana",
  background: "pexels-matthias-oben-3687869.jpg",
  ambience: "city_noise.mp3",
  credit: "Matthias Oben",
}];

fn describe_parliament(pc: isize) -> String {
    let desc = if pc <= 20 {
        "is conspiring against you"
    } else if pc <= 200 {
        "is ready to work with you"
    } else {
        "trusts you completely"
    };
    let text = format!("Parliament {}.", desc);
    t!(&text)
}

fn describe_warming(emissions: f32, temp: f32) -> String {
    let desc = if emissions > 0. {
        "still warming"
    } else if emissions <= 0. {
        "recovering"
    } else if temp >= 2. {
        "becoming unbearable"
    } else if temp > 3. {
        "hostile to life"
    } else {
        ""
    };
    let text = format!("The world is {}.", desc);
    t!(&text)
}

fn describe_extinction(extinction_rate: f32) -> String {
    let idx = intensity::scale(
        extinction_rate,
        intensity::Variable::Extinction,
    );
    const DESCS: &[&str] = &[
        "flourishing",
        "recovering",
        "stabilizing",
        "struggling",
        "suffering",
        "plummeting",
    ];
    let idx = idx.min(DESCS.len() - 1).max(0);
    let text = format!("Biodiversity is {}.", DESCS[idx]);
    t!(&text)
}

fn describe_outlook(outlook: f32) -> String {
    let idx = intensity::scale(
        outlook,
        intensity::Variable::WorldOutlook,
    );
    const DESCS: &[&str] = &[
        "furious", "upset", "unhappy", "content", "happy",
        "ecstatic",
    ];
    let idx = idx.min(DESCS.len() - 1).max(0);
    let text = format!("People are {}.", DESCS[idx]);
    t!(&text)
}

#[component]
pub fn Interstitial() -> impl IntoView {
    let ui = expect_context::<RwSignal<UIState>>();
    let game = expect_context::<RwSignal<Game>>();

    let events = create_rw_signal(vec![]);

    game.update_untracked(|game| {
        game.update_production();
        ui.update_untracked(|ui| {
            ui.factors = rank_factors(&game.state);
        });
    });
    game.update_untracked(|game| {
        let evs = if game.won() {
            game.roll_events(EventPhase::InterstitialWin, None)
        } else {
            game.roll_events(
                EventPhase::InterstitialStart,
                None,
            )
        };
        events.set(evs);
    });

    let (ready, set_ready) = create_signal(false);

    let year = memo!(game.world.year);
    let pc = memo!(game.political_capital.max(0));
    let outlook = memo!(game.outlook());
    let emissions = memo!(game.state.emissions_gt());
    let extinction = memo!(game.world.extinction_rate);
    let temperature = memo!(game.world.temperature);
    let start_year = memo!(ui.start_year);
    let death_year = memo!(game.death_year);

    let number = move || {
        ((year.get() - start_year.get()) as f32 / 5. + 1.)
            .round() as usize
    };
    let title = move || {
        let n = number();
        let ext = match n {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        t!("The {n}{ext} Planning Session", n: n, ext: t!(ext))
    };
    let locale = move || {
        let idx = (number() - 1) % LOCALES.len();
        &LOCALES[idx]
    };
    let game_over = move || with!(|game| game.game_over());
    let game_win = move || with!(|game| game.won());
    let parliament = move || describe_parliament(pc.get());
    let world = move || {
        describe_warming(emissions.get(), temperature.get())
    };
    let biodiversity =
        move || describe_extinction(extinction.get());
    let contentedness = move || describe_outlook(outlook.get());
    let years_left = move || {
        let years_left = (death_year.get() - year.get()).max(0);
        t!(
            "You have {yearsLeft} years left in your tenure.",
            yearsLeft: years_left
        )
    };

    let ambience = locale().ambience;
    audio::play_atmosphere(&format!(
        "/assets/environments/ambience/{ambience}"
    ));

    let ui = expect_context::<RwSignal<UIState>>();
    let (_, set_phase) = slice!(ui.phase);
    let main_ref = create_node_ref::<html::Div>();
    let next_phase = move |_| {
        if let Some(elem) = main_ref.get() {
            let _ = elem.style(
                "animation",
                "1s fade-out ease-out forwards",
            );
            set_timeout(
                move || {
                    if game_over() {
                        set_phase.set(Phase::GameOver);
                    } else if game_win() {
                        set_phase.set(Phase::GameWin);
                    } else {
                        set_phase.set(Phase::Planning);
                    }
                },
                Duration::from_secs(1),
            );
        }
    };

    let background = move || {
        let locale = locale();
        format!(
            "url('/assets/environments/out/{}')",
            locale.background
        )
    };
    let name = move || {
        let locale = locale();
        t!(&locale.name)
    };
    let image_credit = move || {
        let locale = locale();
        locale.credit
    };

    view! {
        <div
            ref=main_ref
            class="interstitial"
            style:background-image=background
        >
            <div class="interstitial--inner">
                <header>
                    <h3>{year}</h3>
                    <br/>
                    <h1>{title}</h1>
                    <br/>
                    <h2>{name}</h2>
                </header>
                <div class="interstitial--summary">
                    <div>{contentedness}</div>
                    <div>{biodiversity}</div>
                    <div>{world}</div>
                    <div>{parliament}</div>
                    <div>{years_left}</div>
                </div>
            </div>
            <Events events on_done=move |_| {
                set_ready.set(true);
            } />
            <div class="interstitial--image-credit">
                {t!("Image:")}" "{image_credit}
            </div>
            <Show when=move || ready.get()>
                <div class="interstitial--next">
                    <button class="btn" on:click=next_phase>
                        {t!("Continue")}
                    </button>
                </div>
            </Show>
        </div>
    }
}
