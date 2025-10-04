mod credits;
mod menu;

use credits::Credits;
use egui_taffy::TuiBuilderLogic;
use menu::{Menu, MenuAction};

use crate::{
    AUDIO,
    audio,
    state::STATE,
    views::parts::center_center,
};

enum MenuView {
    Menu,
    Credits,
}

pub enum StartAction {
    Continue,
    NewGame,
}

pub struct Start {
    view: MenuView,
}
impl Default for Start {
    fn default() -> Self {
        Start {
            view: MenuView::Menu,
        }
    }
}
impl Start {
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Option<StartAction> {
        let mut start_action = None;
        match self.view {
            MenuView::Menu => {
                center_center(ui, "main-menu", |tui| {
                    tui.ui(|ui| {
                        if let Some(action) = Menu::render(ui) {
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
                                MenuAction::NewGame => {
                                    start_action = Some(
                                        StartAction::NewGame,
                                    );
                                }
                                MenuAction::ToggleSound => {
                                    let mut state =
                                        STATE.write();
                                    state.prefs.sound =
                                        !state.prefs.sound;

                                    if state.prefs.sound {
                                        let sound_data = audio!(
                                            "notification.mp3"
                                        );
                                        let _ = AUDIO
                                            .write()
                                            .play(sound_data);
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
