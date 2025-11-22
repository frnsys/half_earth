mod cards;
mod ending;
mod events;
mod factors;
mod globe;
mod hud;
mod interstitial;
mod intro;
mod menu;
mod report;
mod scanner;
mod session;
mod world;

use std::sync::Arc;

use crate::{
    debug::{DEBUG, DebugView},
    state::{GameState, Settings},
    tips::render_tip,
    views::{ending::End, report::Report, world::WorldEvents},
};
use hes_engine::State;
use hud::{HudAction, render_hud};
use interstitial::Interstitial;
use intro::Intro;
use menu::{MenuAction, render_menu};
use session::Session;

pub(crate) use cards::Card;
pub(crate) use events::render_event_card;
pub(crate) use factors::FactorsCard;
pub(crate) use session::Changes;

/// Phase of the game.
enum Phase {
    Intro(Intro),
    Interstitial(Interstitial),
    Planning(Session),
    Events(WorldEvents),
    Report(Report),
    Ending(End),
}

pub enum GameAction {
    Restart,
    ToggleSound,
    Save,
}

pub struct GameView {
    phase: Phase,
    show_menu: bool,
    ctx: Arc<three_d::context::Context>,
}
impl GameView {
    pub fn new(state: &mut GameState, context: &Arc<three_d::context::Context>) -> Self {
        if let Some(debug_view) = &DEBUG.view {
            let phase = match debug_view {
                DebugView::Plan => Phase::Planning(Session::plan()),
                DebugView::Regions => Phase::Planning(Session::regions(context)),
                DebugView::Parliament => Phase::Planning(Session::govt(&state.core)),
                DebugView::Stats => Phase::Planning(Session::stats()),
                DebugView::World => Phase::Events(WorldEvents::new(state, context)),
                DebugView::Report => Phase::Report(Report::new(state)),
                DebugView::GameOver => Phase::Ending(End::new(true, state)),
                DebugView::GameWin => Phase::Ending(End::new(false, state)),
            };
            GameView::with_phase(phase, context.clone())
        } else {
            Self::intro(&mut state.core, context.clone())
        }
    }

    pub fn from_save(state: &mut State, ctx: &Arc<three_d::context::Context>) -> Self {
        Self {
            phase: Phase::Interstitial(Interstitial::new(state)),
            show_menu: false,
            ctx: ctx.clone(),
        }
    }

    fn intro(state: &mut State, ctx: Arc<three_d::context::Context>) -> Self {
        Self {
            phase: Phase::Intro(Intro::new(state)),
            show_menu: false,
            ctx,
        }
    }

    fn with_phase(phase: Phase, ctx: Arc<three_d::context::Context>) -> Self {
        Self {
            phase,
            show_menu: false,
            ctx,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut GameState,
        prefs: &mut Settings,
    ) -> Option<GameAction> {
        let mut ret_action = None;
        match &mut self.phase {
            Phase::Intro(view) => {
                let next = view.render(ui, state);
                if next {
                    let view = Interstitial::new(state);
                    self.phase = Phase::Interstitial(view);
                }
            }
            Phase::Interstitial(view) => {
                let next = view.render(ui, &mut state.core, state.ui.start_year);
                if next {
                    if state.won() {
                        prefs.runs_played += 1;
                        self.phase = Phase::Ending(End::new(false, state));
                    } else if state.game_over {
                        prefs.runs_played += 1;
                        self.phase = Phase::Ending(End::new(true, state));
                    } else {
                        let session = Session::new(state);
                        self.phase = Phase::Planning(session);
                        ret_action = Some(GameAction::Save);
                    }
                }
            }
            Phase::Planning(session) => {
                if self.show_menu {
                    if let Some(action) =
                        render_menu(ui, &mut state.core, prefs, state.ui.start_year)
                    {
                        match action {
                            MenuAction::CloseMenu => self.show_menu = false,
                            MenuAction::RestartGame => {
                                ret_action = Some(GameAction::Restart);
                            }
                            MenuAction::ToggleSound => {
                                ret_action = Some(GameAction::ToggleSound);
                            }
                        }
                    }
                } else {
                    if let Some(action) = render_hud(ui, state) {
                        match action {
                            HudAction::OpenMenu => {
                                self.show_menu = true;
                            }
                        }
                    }

                    let go_to_world = session.render(ui, state, &self.ctx);
                    if go_to_world {
                        self.phase = Phase::Events(WorldEvents::new(state, &self.ctx));
                        ret_action = Some(GameAction::Save);
                    }
                }
            }
            Phase::Events(world) => {
                world.render(ui, state);
                if world.is_done() {
                    self.phase = Phase::Report(Report::new(state));
                }
            }
            Phase::Report(report) => {
                let done = report.render(ui, state);
                if done {
                    let view = Interstitial::new(state);
                    self.phase = Phase::Interstitial(view);
                }
            }
            Phase::Ending(end) => {
                let restart = end.render(ui, state);
                if restart {
                    ret_action = Some(GameAction::Restart);
                }
            }
        }

        render_tip(ui.ctx(), state);

        ret_action
    }
}
