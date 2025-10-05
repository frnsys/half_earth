use egui::{Button, Color32, Margin};
use hes_engine::State;
use rust_i18n::t;

use crate::{
    display::{self, icon_from_slug, icons, intensity},
    image,
    state::STATE,
    views::parts::{
        button,
        raised_frame_no_shadow_impl,
        set_full_bg_image,
    },
};

pub enum MenuAction {
    CloseMenu,
    RestartGame,
    ShowCredits,
    ToggleSound,
    HideHelp,
}

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

pub fn render_menu(
    ui: &mut egui::Ui,
    state: &mut State,
) -> Option<MenuAction> {
    set_full_bg_image(
        ui,
        image!("backgrounds/menu.jpg"),
        egui::vec2(900., 1200.),
    );

    let year = state.world.year;
    let pc = state.political_capital.max(0);
    let outlook = state.outlook();
    let emissions = state.emissions.as_gtco2eq();
    let extinction = state.world.extinction_rate;
    let temperature = state.world.temperature;

    // let start_year = ui.start_year; // TODO
    let start_year = 2022;

    let temp = display::temp(temperature);
    let emissions = display::emissions(emissions);
    let contentedness = intensity::scale(
        outlook,
        intensity::Variable::WorldOutlook,
    );
    let extinction = intensity::scale(
        extinction,
        intensity::Variable::Extinction,
    );
    let locale = {
        let elapsed = year - start_year;
        let idx = (elapsed as f32 / 5.).round() as usize
            % LOCALES.len();
        &LOCALES[idx]
    };
    let time_place = format!("{}, {}", locale, year);

    let close = ui.add(Button::image(icons::CLOSE));
    if close.clicked() {
        return Some(MenuAction::CloseMenu);
    }

    raised_frame_no_shadow_impl(
        ui,
        Color32::from_rgb(0x18, 0x15, 0x14),
        Color32::from_rgb(0x78, 0x75, 0x75),
        |ui| {
            egui::Frame::NONE
                .fill(Color32::from_rgb(0x42, 0x3B, 0x3B))
                .inner_margin(Margin::symmetric(6, 6))
                .corner_radius(4)
                .show(ui, |ui| {
                    let logo = image!("gosplant.svg");
                    ui.add(
                        egui::Image::new(logo).max_height(24.),
                    );
                })
                .response
        },
    );

    ui.horizontal(|ui| {
        ui.label("CLOCK"); // TODO
        //
        raised_frame_no_shadow_impl(
            ui,
            Color32::from_rgb(0x18, 0x15, 0x14),
            Color32::from_rgb(0x78, 0x75, 0x75),
            |ui| {
                egui::Frame::NONE
                    .fill(Color32::from_rgb(0x42, 0x3B, 0x3B))
                    .inner_margin(Margin::symmetric(24, 12))
                    .corner_radius(4)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(time_place)
                                .heading()
                                .color(Color32::WHITE)
                                .size(18.),
                        );
                    })
                    .response
            },
        )
    });

    ui.vertical_centered(|ui| {
        ui.image(icons::POLITICAL_CAPITAL);
        ui.label(pc.to_string());
        ui.label(t!("Political Capital"));
    });

    ui.vertical_centered(|ui| {
        ui.image(icons::EMISSIONS);
        ui.label(emissions);
        ui.label(t!("CO2 Emissions/Yr"));
    });

    ui.vertical_centered(|ui| {
        ui.image(icons::WARMING);
        ui.label(temp);
        ui.label(t!("Temp. Anomaly"));
    });

    ui.vertical_centered(|ui| {
        ui.image(icons::EXTINCTION_RATE);
        // <IntensityBar intensity=extinction.into_signal()/> // TODO
        ui.label(t!("Extinction Rate"));
    });

    ui.vertical_centered(|ui| {
        ui.image(icons::CONTENTEDNESS);
        // <IntensityBar
        //     intensity=contentedness.into_signal()
        //     invert=true
        //     /> // TODO
        ui.label(t!("Contentedness"));
    });

    let motto = image!("motto.png");
    ui.image(motto);

    let sound = format!(
        "{}: {}",
        t!("Sound"),
        if STATE.read().prefs.sound {
            t!("On")
        } else {
            t!("Off")
        }
    );
    if ui.add(button(sound.into())).clicked() {
        return Some(MenuAction::ToggleSound);
    }

    let tips = format!(
        "{}: {}",
        t!("Tips"),
        if STATE.read().prefs.hide_help {
            t!("On")
        } else {
            t!("Off")
        }
    );
    if ui.add(button(tips.into())).clicked() {
        return Some(MenuAction::HideHelp);
    }

    if ui.add(button(t!("Restart Game"))).clicked() {
        return Some(MenuAction::RestartGame);
    }

    if ui.add(button(t!("Credits"))).clicked() {
        return Some(MenuAction::ShowCredits);
    }

    None
}
