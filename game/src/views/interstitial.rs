use std::borrow::Cow;

use egui::{Color32, Margin, TextWrapMode};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{EventPhase, State};
use rust_i18n::t;

use crate::{
    audio,
    display::intensity,
    parts::{button, center_center, set_full_bg_image},
    state::StateExt,
    views::events::Events,
};

pub struct Interstitial {
    events: Events,
}

impl Interstitial {
    pub fn new(state: &mut State) -> Self {
        let phase = if state.won() {
            EventPhase::InterstitialWin
        } else {
            EventPhase::InterstitialStart
        };

        let events = StateExt::roll_events(state, phase);

        audio::soundtrack(audio::Track::Interstitial);

        Self {
            events: Events::new(events, state),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        start_year: usize,
    ) -> bool {
        let year = state.world.year;
        let pc = state.political_capital.max(0);
        let outlook = state.outlook();
        let emissions = state.emissions.as_gtco2eq();
        let extinction = state.world.extinction_rate;
        let temperature = state.world.temperature;
        let death_year = state.death_year;

        let n = ((year - start_year) as f32 / 5. + 1.).round()
            as usize;
        let title = {
            let ext = match n {
                1 => t!("st"),
                2 => t!("nd"),
                3 => t!("rd"),
                _ => t!("th"),
            };
            t!(
                "The %{n}%{ext} Planning Session",
                n = n,
                ext = ext
            )
        };
        let locale = {
            let idx = (n - 1) % LOCALES.len();
            &LOCALES[idx]
        };

        let parliament = describe_parliament(pc);
        let world = describe_warming(emissions, temperature);
        let biodiversity = describe_extinction(extinction);
        let contentedness = describe_outlook(outlook);
        let years_left = {
            let years_left = death_year.saturating_sub(year);
            t!(
                "You have %{yearsLeft} years left in your tenure.",
                yearsLeft = years_left
            )
        };

        set_full_bg_image(
            ui,
            locale.background(),
            egui::Vec2::from(locale.background_size),
        );

        ui.painter().rect_filled(
            ui.ctx().screen_rect(),
            0.,
            Color32::from_rgba_premultiplied(0, 0, 0, 200),
        );

        let go_to_next = center_center(ui, "events", |tui| {
            tui.ui(|ui| {
                egui::Frame::NONE.show(ui, |ui| {
                    ui.set_width(360.);
                    ui.style_mut()
                        .visuals
                        .override_text_color =
                        Some(Color32::WHITE);
                    ui.label(
                        egui::RichText::new(title).heading(),
                    );
                    ui.label(format!(
                        "{}, {}",
                        locale.name, year
                    ));

                    ui.add_space(16.);

                    ui.label(contentedness);
                    ui.label(biodiversity);
                    ui.label(world);
                    ui.label(parliament);
                    ui.label(years_left);
                });

                if self.events.is_finished {
                    ui.add_space(18.);
                    ui.add(button(t!("Continue")).full_width())
                        .clicked()
                } else {
                    false
                }
            })
        });

        egui::Area::new(egui::Id::new("image-attrib"))
            .anchor(
                egui::Align2::LEFT_BOTTOM,
                egui::Vec2::new(10., -10.),
            )
            .show(ui.ctx(), |ui| {
                egui::Frame::NONE
                    .fill(Color32::from_black_alpha(128))
                    .inner_margin(Margin::symmetric(6, 3))
                    .show(ui, |ui| {
                        ui.style_mut()
                            .visuals
                            .override_text_color =
                            Some(Color32::WHITE);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(format!(
                                    "{} {}",
                                    t!("Image:"),
                                    locale.credit
                                ))
                                .size(11.),
                            )
                            .wrap_mode(TextWrapMode::Extend),
                        );
                    });
            });

        self.events.render(ui, state);

        go_to_next
    }
}

pub struct Locale {
    pub name: &'static str,
    background: &'static str,
    background_size: (f32, f32),
    credit: &'static str,
}
impl Locale {
    pub fn background<'a>(&self) -> egui::ImageSource<'a> {
        hes_images::locale_image(self.background)
    }
}

// List from Troy:
// Bandung, Hanoi, Mexico City, Budapest, Thiruvananthapuram, Luanda, Ayn Issa, Ferrara, Vienna, Beijing, Aden, Caracas, Algiers, Belgrade, Moscow, Managua, Buenos Aires, Trier, Prague, Porto Alegre, Seattle/Burlington/Bronx, Dar es Salaam
pub const LOCALES: &[Locale] = &[
    Locale {
        name: "Havana",
        background: "pexels-matthias-oben-3687869.webp",
        credit: "Matthias Oben",
        background_size: (1200., 800.),
    },
    Locale {
        name: "Ouagadougou",
        background: "2560px-Ouagadougou_BCEAO_day.webp",
        credit: "Wegmann, CC BY-SA 3.0, via Wikimedia Commons",
        background_size: (1200., 803.),
    },
    Locale {
        name: "Port-au-Prince",
        background: "robin-canfield-CkCV7vTmmz4-unsplash.webp",
        credit: "Robin Canfield",
        background_size: (800., 1200.),
    },
    Locale {
        name: "San Cristóbal de las Casas",
        background: "1536px-Street_Scene_with_Church_Cupola_-_San_Cristobal_de_las_Casas_-_Chiapas_-_Mexico.webp",
        credit: "Adam Jones, CC BY 2.0, via Wikimedia Commons",
        background_size: (900., 1200.),
    },
    Locale {
        name: "Paris",
        background: "pexels-pierre-blache-3073666.webp",
        credit: "Pierre Blaché",
        background_size: (960., 1200.),
    },
    Locale {
        name: "Bandung",
        background: "Street_Braga,_Bandung_City,_West_Java,_Indonesia.webp",
        credit: "PACARNYAKEYES, CC BY-SA 4.0, via Wikimedia Commons",
        background_size: (430., 534.),
    },
    Locale {
        name: "Seattle",
        background: "2560px-Seattle_4.webp",
        credit: "Daniel Schwen, CC BY-SA 4.0, via Wikimedia Commons",
        background_size: (1200., 674.),
    },
    Locale {
        name: "Hanoi",
        background: "2560px-Vietnam,_Hanoi,_Streets_of_central_Hanoi_2.webp",
        credit: "© Vyacheslav Argenberg / http://www.vascoplanet.com/, CC BY 4.0, via Wikimedia Commons",
        background_size: (1200., 800.),
    },
    Locale {
        name: "Dar es Salaam",
        background: "Dar_es_Salaam_before_dusk.webp",
        credit: "Muhammad Mahdi Karim, GFDL 1.2, via Wikimedia Commons",
        background_size: (1200., 643.),
    },
    Locale {
        name: "Ayn Issa",
        background: "2560px-Another_Year_Without_Daesh.webp",
        credit: "Combined Joint Task Force - Operation Inherent Resolve/Sgt. Raymond Boyington, Public domain, via Wikimedia Commons",
        background_size: (1200., 800.),
    },
    Locale {
        name: "Algiers",
        background: "2560px-Martyrs_Memorial,_A_cloudy_day_in_Algiers.webp",
        credit: "EL Hacene Boulkroune, CC BY-SA 4.0, via Wikimedia Commons",
        background_size: (1200., 900.),
    },
    Locale {
        name: "Managua",
        background: "Old_Managua_Cathedral_from_Highway_2.webp",
        credit: "Byralaal, CC BY-SA 4.0, via Wikimedia Commons",
        background_size: (1200., 720.),
    },
    Locale {
        name: "Prague",
        background: "2560px-Vltava_river_in_Prague.webp",
        credit: "Dmitry A. Mottl, CC BY-SA 4.0, via Wikimedia Commons",
        background_size: (1200., 800.),
    },
    Locale {
        name: "Havana",
        background: "pexels-matthias-oben-3687869.webp",
        credit: "Matthias Oben",
        background_size: (1200., 800.),
    },
];

fn describe_parliament(pc: isize) -> Cow<'static, str> {
    if pc <= 20 {
        t!("Parliament is conspiring against you.")
    } else if pc <= 200 {
        t!("Parliament is ready to work with you.")
    } else {
        t!("Parliament trusts you completely.")
    }
}

fn describe_warming(
    emissions: f32,
    temp: f32,
) -> Cow<'static, str> {
    if emissions > 0. {
        if temp > 3. {
            t!("The world is becoming hostile to life.")
        } else if temp >= 2. {
            t!("The world is becoming unbearable.")
        } else {
            t!("The world is still warming.")
        }
    } else {
        t!("The world is recovering.")
    }
}

fn describe_extinction(
    extinction_rate: f32,
) -> Cow<'static, str> {
    let idx = intensity::scale(
        extinction_rate,
        intensity::Variable::Extinction,
    );
    match idx {
        0 => t!("Biodiversity is flourishing."),
        1 => t!("Biodiversity is recovering."),
        2 => t!("Biodiversity is stabilizing."),
        3 => t!("Biodiversity is struggling."),
        4 => t!("Biodiversity is suffering."),
        _ => t!("Biodiversity is plummeting."),
    }
}

fn describe_outlook(outlook: f32) -> Cow<'static, str> {
    let idx = intensity::scale(
        outlook,
        intensity::Variable::WorldOutlook,
    );
    match idx {
        0 => t!("People are furious."),
        1 => t!("People are upset."),
        2 => t!("People are unhappy."),
        3 => t!("People are content."),
        4 => t!("People are happy."),
        _ => t!("People are ecstatic."),
    }
}
