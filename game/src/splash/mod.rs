mod credits;
mod menu;

use credits::Credits;
use egui::Align2;
use egui_taffy::TuiBuilderLogic;
use hes_engine::World;
use menu::{Menu, MenuAction};

use crate::{audio, parts::center_center, state::Settings};

enum MenuView {
    Menu,
    Credits,
}

pub enum StartAction {
    Continue,
    NewGame(World),
}

pub struct Start {
    view: MenuView,
    menu: Menu,
}
impl Default for Start {
    fn default() -> Self {
        Start {
            view: MenuView::Menu,
            menu: Menu::new(),
        }
    }
}
impl Start {
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        prefs: &mut Settings,
        has_save: bool,
    ) -> Option<StartAction> {
        lang_picker(ui);
        let mut start_action = None;
        match self.view {
            MenuView::Menu => {
                center_center(ui, "main-menu", |tui| {
                    tui.ui(|ui| {
                        if let Some(action) = self
                            .menu
                            .render(ui, prefs, has_save)
                        {
                            match action {
                                MenuAction::Credits => {
                                    self.view =
                                        MenuView::Credits;
                                }
                                MenuAction::Continue => {
                                    start_action = Some(
                                        StartAction::Continue,
                                    );
                                }
                                MenuAction::NewGame(world) => {
                                    start_action = Some(
                                        StartAction::NewGame(
                                            world,
                                        ),
                                    );
                                }
                                MenuAction::ToggleSound => {
                                    prefs.sound = !prefs.sound;

                                    if prefs.sound {
                                        audio::unmute();
                                        audio::ping();
                                    } else {
                                        audio::mute();
                                    }
                                }
                            }
                        }
                    });
                });
            }
            MenuView::Credits => {
                if Credits::render(ui) {
                    self.view = MenuView::Menu;
                }
            }
        }
        start_action
    }
}

fn lang_picker(ui: &mut egui::Ui) {
    egui::Area::new("lang-picker-area".into())
        .anchor(Align2::RIGHT_TOP, egui::vec2(-8., 8.))
        .show(ui.ctx(), |ui| {
            let mut lang = rust_i18n::locale().to_string();
            egui::ComboBox::new("lang-picker", "")
                .width(0.)
                .selected_text(
                    egui::RichText::new(&lang)
                        .color(egui::Color32::WHITE),
                )
                .show_ui(ui, |ui| {
                    ui.style_mut()
                        .visuals
                        .override_text_color =
                        Some(egui::Color32::WHITE);
                    let locales =
                        rust_i18n::available_locales!();
                    for locale in locales {
                        ui.selectable_value(
                            &mut lang,
                            locale.to_string(),
                            locale,
                        );
                    }
                });
            if *rust_i18n::locale() != lang {
                rust_i18n::set_locale(&lang);
            }
        });
}
