mod cards;
mod ending;
mod events;
mod factors;
mod hud;
mod interstitial;
mod intro;
mod menu;
mod session;

use crate::state::UIState;
use hes_engine::State;
use hud::{HudAction, render_hud};
use interstitial::Interstitial;
use intro::Intro;
use menu::{MenuAction, render_menu};
use session::Session;

pub(crate) use cards::Card;
pub(crate) use factors::FactorsCard;

/// Phase of the game.
#[derive(Default)]
pub(crate) enum Phase {
    Intro(Intro),
    Interstitial(Interstitial),
    Planning(Session),
    Events,
    Report,
    GameOver,
    GameWin,

    #[default] // TODO
    Foo,
}

pub struct GameView {
    phase: Phase,
    show_menu: bool,
}
impl GameView {
    pub fn new(state: &mut State) -> Self {
        Self {
            phase: Phase::Intro(Intro::new(state)),
            show_menu: false,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        ui_state: &mut UIState,
    ) {
        match &mut self.phase {
            Phase::Intro(view) => {
                let next = view.render(ui, state);
                if next {
                    let view = Interstitial::new(state);
                    self.phase = Phase::Interstitial(view);
                }
            }
            Phase::Interstitial(view) => {
                let next = view.render(ui, state);
                if next {
                    if state.won() {
                        self.phase = Phase::GameWin;
                    } else if state.game_over {
                        self.phase = Phase::GameOver;
                    } else {
                        let session =
                            Session::new(state, ui_state);
                        self.phase = Phase::Planning(session);
                    }
                }
            }
            Phase::Planning(session) => {
                if let Some(action) = render_hud(ui, state) {
                    match action {
                        HudAction::OpenMenu => {
                            self.show_menu = true;
                        }
                    }
                }
                if self.show_menu {
                    if let Some(action) = render_menu(ui, state)
                    {
                        match action {
                            MenuAction::CloseMenu => {
                                self.show_menu = false
                            }
                            MenuAction::RestartGame => todo!(),
                            MenuAction::ShowCredits => todo!(),
                            MenuAction::ToggleSound => todo!(),
                            MenuAction::HideHelp => todo!(),
                        }
                    }
                } else {
                    session.render(
                        ui,
                        state,
                        &mut ui_state.tutorial,
                        &ui_state.annual_region_events,
                        &ui_state.process_mix_changes,
                        &ui_state.viewed,
                        &ui_state.points,
                        &ui_state.plan_changes,
                    );
                }
            }
            Phase::Events => todo!(),
            Phase::Report => todo!(),
            Phase::GameOver => todo!(),
            Phase::GameWin => todo!(),
            Phase::Foo => unreachable!(),
        }
    }
}
