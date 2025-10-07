mod cards;
mod ending;
mod events;
mod factors;
mod globe;
mod hud;
mod interstitial;
mod intro;
mod menu;
mod session;

use std::sync::Arc;

use crate::state::UIState;
use hes_engine::State;
use hud::{HudAction, render_hud};
use interstitial::Interstitial;
use intro::Intro;
use menu::{MenuAction, render_menu};
use session::Session;

pub(crate) use cards::Card;
pub(crate) use factors::FactorsCard;

pub(crate) fn debug_view(
    glow_ctx: Arc<eframe::glow::Context>,
) -> GameView {
    GameView::with_phase(
        Phase::Planning(Session {
            glow_ctx: glow_ctx.clone(),
            // view: session::View::Stats(session::Stats::new()),
            // view: session::View::Plan(session::Plan::new()),
            // view: View::Govt(Parliament::new(
            //     &state.game,
            // )),
            view: session::View::World(session::Regions::new(
                glow_ctx.clone(),
            )),
            events: events::Events::new(vec![]),
        }),
        glow_ctx.clone(),
    )
}

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
    glow_ctx: Arc<eframe::glow::Context>,
}
impl GameView {
    pub fn new(
        state: &mut State,
        glow_ctx: Arc<eframe::glow::Context>,
    ) -> Self {
        Self {
            phase: Phase::Intro(Intro::new(state)),
            show_menu: false,
            glow_ctx,
        }
    }

    fn with_phase(
        phase: Phase,
        glow_ctx: Arc<eframe::glow::Context>,
    ) -> Self {
        Self {
            phase,
            show_menu: false,
            glow_ctx,
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
                        let session = Session::new(
                            state,
                            ui_state,
                            self.glow_ctx.clone(),
                        );
                        self.phase = Phase::Planning(session);
                    }
                }
            }
            Phase::Planning(session) => {
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
                    if let Some(action) = render_hud(ui, state)
                    {
                        match action {
                            HudAction::OpenMenu => {
                                self.show_menu = true;
                            }
                        }
                    }

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
