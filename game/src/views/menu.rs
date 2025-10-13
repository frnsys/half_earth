use egui::{Align2, Color32, Margin, Sense};
use hes_engine::State;
use rust_i18n::t;

use crate::{
    display::icons,
    image,
    parts::{
        RaisedFrame,
        button,
        button_frame,
        raised_frame,
        set_full_bg_image,
    },
    state::Settings,
};

pub enum MenuAction {
    CloseMenu,
    RestartGame,
    ToggleSound,
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
    prefs: &Settings,
    start_year: usize,
) -> Option<MenuAction> {
    set_full_bg_image(
        ui,
        image!("backgrounds/menu.jpg"),
        egui::vec2(900., 1200.),
    );

    let year = state.world.year;

    let locale = {
        let elapsed = year - start_year;
        let idx = (elapsed as f32 / 5.).round() as usize
            % LOCALES.len();
        &LOCALES[idx]
    };
    let time_place = format!("{}, {}", locale, year);

    let mut action = None;
    egui::Area::new("menu-close".into())
        .anchor(Align2::RIGHT_TOP, egui::vec2(-8., 8.))
        .show(ui.ctx(), |ui| {
            let resp =
                button_frame().margin(6).show(ui, |ui| {
                    ui.add(icons::CLOSE.size(24.));
                });
            if resp.interact(Sense::click()).clicked() {
                action = Some(MenuAction::CloseMenu);
            }
        });

    ui.vertical_centered(|ui| {
        ui.set_max_width(480.);

        ui.add_space(64.);

        inset_frame().margin(6).show(ui, |ui| {
            ui.set_width(80.);
            let logo = image!("gosplant.svg");
            ui.add(egui::Image::new(logo).max_height(24.));
        });

        ui.add_space(8.);

        inset_frame().margin(Margin::symmetric(24, 12)).show(
            ui,
            |ui| {
                ui.label(
                    egui::RichText::new(time_place)
                        .heading()
                        .color(Color32::WHITE)
                        .size(18.),
                );
            },
        );

        ui.add_space(32.);
        let motto = image!("motto.png");
        ui.add(egui::Image::new(motto).max_height(80.));
        ui.add_space(32.);

        let sound = format!(
            "{}: {}",
            t!("Sound"),
            if prefs.sound { t!("On") } else { t!("Off") }
        );
        if ui.add(button(sound)).clicked() {
            action = Some(MenuAction::ToggleSound);
        }

        if ui.add(button(t!("Restart Game"))).clicked() {
            action = Some(MenuAction::RestartGame);
        }

        ui.add_space(36.);
    });

    action
}

fn inset_frame() -> RaisedFrame {
    raised_frame().colors(
        Color32::from_rgb(0x18, 0x15, 0x14),
        Color32::from_rgb(0x78, 0x75, 0x75),
        Color32::from_rgb(0x42, 0x3B, 0x3B),
    )
}
