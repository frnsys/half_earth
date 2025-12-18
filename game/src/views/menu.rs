use std::time::{Duration, Instant};

use egui::{Align2, Color32, Margin, Sense};
use hes_engine::State;
use rust_i18n::t;

use crate::{
    debug::serialize_state,
    display::icons,
    image,
    parts::{RaisedFrame, button, button_frame, raised_frame, set_full_bg_image},
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
        hes_images::background_image("menu.jpg"),
        egui::vec2(900., 1200.),
    );

    let year = state.world.year;

    let locale = {
        let elapsed = year - start_year;
        let idx = (elapsed as f32 / 5.).round() as usize % LOCALES.len();
        &LOCALES[idx]
    };
    let time_place = format!("{}, {}", locale, year);

    let mut action = None;
    egui::Area::new("menu-close".into())
        .anchor(Align2::RIGHT_TOP, egui::vec2(-8., 8.))
        .show(ui.ctx(), |ui| {
            let resp = button_frame().margin(6).show(ui, |ui| {
                ui.add(icons::CLOSE.size(24.));
            });
            if resp.interact(Sense::click()).clicked() {
                action = Some(MenuAction::CloseMenu);
            }
        });

    egui::Frame::NONE
        .inner_margin(Margin::symmetric(6, 6))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                let width = (ui.ctx().content_rect().width() - 12.).min(480.);
                ui.set_max_width(width);

                ui.add_space(64.);

                inset_frame().margin(6).show(ui, |ui| {
                    ui.set_width(80.);
                    let logo = image!("gosplant.svg");
                    ui.add(egui::Image::new(logo).max_height(24.));
                });

                ui.add_space(8.);

                inset_frame()
                    .margin(Margin::symmetric(24, 12))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(time_place)
                                .heading()
                                .color(Color32::WHITE)
                                .size(18.),
                        );
                    });

                ui.add_space(32.);
                let motto = image!("motto.png");
                ui.add(egui::Image::new(motto).max_height(80.));
                ui.add_space(32.);

                copy_session_button(ui, state);

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
        });

    action
}

fn copy_session_button(ui: &mut egui::Ui, state: &State) {
    let id = egui::Id::new("session-button");
    let now = Instant::now();
    let dur = Duration::from_secs(2);
    let active_until = ui.ctx().data(|d| d.get_temp::<Instant>(id)).and_then(|t0| {
        if now.duration_since(t0) < dur {
            Some(t0 + dur)
        } else {
            None
        }
    });
    let label = if active_until.is_some() {
        t!("Copied to Clipboard")
    } else {
        t!("Copy Session")
    };
    if ui.add(button(label)).clicked() {
        ui.ctx().data_mut(|d| d.insert_temp(id, now));
        if let Some(session) = serialize_state(state) {
            ui.ctx().copy_text(session);
        }
        ui.ctx().request_repaint_after(dur);
    } else if let Some(until) = active_until {
        let remaining = until.saturating_duration_since(now);
        ui.ctx()
            .request_repaint_after(remaining.min(Duration::from_millis(16)));
    }
}

fn inset_frame() -> RaisedFrame {
    raised_frame().colors(
        Color32::from_rgb(0x18, 0x15, 0x14),
        Color32::from_rgb(0x78, 0x75, 0x75),
        Color32::from_rgb(0x42, 0x3B, 0x3B),
    )
}
