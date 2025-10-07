mod govt;
mod plan;
mod regions;
mod stats;
mod treemap;

use std::{collections::BTreeMap, fmt::Display, sync::Arc};

use egui::{Color32, CornerRadius, Margin, Sense};
use egui_taffy::TuiBuilderLogic;
use enum_map::EnumMap;
use hes_engine::{
    EventPhase,
    Flag,
    IconEvent,
    Id,
    Output,
    State,
};
use rust_i18n::t;

use crate::{
    display::DisplayEvent,
    parts::h_center,
    state::{
        PlanChange,
        Points,
        STATE,
        StateExt,
        Tutorial,
        UIState,
        update_factors,
    },
    views::events::{EventResult, Events},
};

use govt::Parliament;
pub(super) use plan::{Plan, PlanAction};
pub(super) use regions::Regions;
pub(super) use stats::Stats;

pub struct Session {
    pub(crate) view: View,
    pub(crate) events: Events,
    pub(crate) glow_ctx: Arc<eframe::glow::Context>,
}
impl Session {
    pub fn new(
        state: &mut State,
        ui: &mut UIState,
        glow_ctx: Arc<eframe::glow::Context>,
    ) -> Self {
        let events = state
            .roll_events(EventPhase::PlanningStart)
            .into_iter()
            .chain(state.roll_events(EventPhase::PlanningPlan))
            .map(|ev| DisplayEvent::new(ev, state))
            .collect();

        update_factors(state);

        let points = state.collect_research_points();
        ui.points.research += points;

        // TODO
        // if get_debug_opts().skip_to_planning {
        //     events.retain(|ev| {
        //         ev.name != "Planning Intro"
        //             && ev.name != "Welcome Back"
        //     });
        // }

        Self {
            view: View::Plan(Plan::new()),
            events: Events::new(events),
            glow_ctx,
        }
    }

    fn set_tab(&mut self, tab: Tab, state: &mut State) {
        let phase = match tab {
            Tab::Plan => EventPhase::PlanningPlan,
            Tab::World => EventPhase::PlanningRegions,
            Tab::Stats => EventPhase::PlanningDashboard,
            Tab::Govt => EventPhase::PlanningParliament,
        };

        let events = state
            .roll_events(phase)
            .into_iter()
            .map(|ev| DisplayEvent::new(ev, state))
            .collect();
        self.events.replace(events);
        self.view = match tab {
            Tab::Plan => View::Plan(Plan::new()),
            Tab::Govt => View::Govt(Parliament::new(state)),
            Tab::Stats => View::Stats(Stats::new()),
            Tab::World => {
                View::World(Regions::new(self.glow_ctx.clone()))
            }
        };
    }

    fn render_tab(
        &mut self,
        ui: &mut egui::Ui,
        tab: Tab,
        tutorial: Tutorial,
        cur_tutorial: &Tutorial,
        state: &mut State,
    ) {
        // TODO use these
        let is_active = tab == self.view.as_tab();
        let highlight = *cur_tutorial == tutorial;
        let disabled = *cur_tutorial < tutorial;

        let resp = egui::Frame::NONE
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(tab.to_string())
                        .heading()
                        .size(15.),
                );
            })
            .response;
        let resp = resp.interact(Sense::all());
        if resp.clicked() {
            self.set_tab(tab, state);
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        tutorial: &mut Tutorial,
        region_events: &BTreeMap<Id, Vec<IconEvent>>,
        process_mix_changes: &EnumMap<
            Output,
            BTreeMap<Id, isize>,
        >,
        viewed: &Vec<Id>,
        points: &Points,
        plan_changes: &BTreeMap<Id, PlanChange>,
    ) {
        // TODO
        // audio::play_phase_music(
        //     "/assets/music/planning.mp3",
        //     true,
        // );

        h_center(ui, "session-tabs", |tui| {
            tui.ui(|ui| {
                egui::Frame::NONE
                    .inner_margin(Margin {
                        top: -3,
                        ..Default::default()
                    })
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.style_mut()
                                .spacing
                                .item_spacing
                                .x = 1.;
                            tab_frame(
                                self.view.as_tab() == Tab::Plan,
                            )
                            .corner_radius(CornerRadius {
                                sw: 4,
                                ..Default::default()
                            })
                            .show(
                                ui,
                                |ui| {
                                    self.render_tab(
                                        ui,
                                        Tab::Plan,
                                        Tutorial::Plan,
                                        tutorial,
                                        state,
                                    );
                                },
                            );
                            tab_frame(
                                self.view.as_tab() == Tab::Govt,
                            )
                            .show(
                                ui,
                                |ui| {
                                    self.render_tab(
                                        ui,
                                        Tab::Govt,
                                        Tutorial::Parliament,
                                        tutorial,
                                        state,
                                    );
                                },
                            );
                            tab_frame(
                                self.view.as_tab()
                                    == Tab::Stats,
                            )
                            .show(
                                ui,
                                |ui| {
                                    self.render_tab(
                                        ui,
                                        Tab::Stats,
                                        Tutorial::Dashboard,
                                        tutorial,
                                        state,
                                    );
                                },
                            );
                            tab_frame(
                                self.view.as_tab()
                                    == Tab::World,
                            )
                            .corner_radius(CornerRadius {
                                se: 4,
                                ..Default::default()
                            })
                            .show(
                                ui,
                                |ui| {
                                    self.render_tab(
                                        ui,
                                        Tab::World,
                                        Tutorial::Regions,
                                        tutorial,
                                        state,
                                    );
                                },
                            );
                        });
                    });
            })
        });

        if !self.events.is_finished {
            let result = self.events.render(ui, state);
            if result == Some(EventResult::JustFinished) {
                self.update_tutorial(state, tutorial);
            }
        }

        match &mut self.view {
            View::Plan(plan) => {
                let action = plan.render(
                    ui,
                    state,
                    tutorial,
                    viewed,
                    points,
                    plan_changes,
                );
                if let Some(action) = action {
                    match action {
                        PlanAction::EnterWorld => todo!(),
                        PlanAction::PageChanged(phase) => {
                            self.events.replace(
                                StateExt::roll_events(
                                    state, phase,
                                ),
                            );
                        }
                    }
                }
                // TODO on plan change
                // events.set(StateExt::roll_events(
                //         game,
                //         EventPhase::PlanningPlanChange,
                // ));
            }
            View::Govt(parliament) => {
                parliament.render(ui, state)
            }
            View::Stats(stats) => {
                stats.render(ui, state, process_mix_changes);
            }
            View::World(regions) => {
                regions.render(ui, state, region_events)
            }
        }
    }

    fn update_tutorial(
        &mut self,
        state: &mut State,
        tutorial: &mut Tutorial,
    ) {
        if state.flags.contains(&Flag::SkipTutorial) {
            *tutorial = Tutorial::Ready;
            let mut state = STATE.write();
            state.prefs.tutorial = Tutorial::Ready;
            if state.prefs.runs_played == 0 {
                state.prefs.runs_played = 1;
            }
        } else if state.flags.contains(&Flag::RepeatTutorial) {
            state
                .flags
                .retain(|flag| *flag != Flag::RepeatTutorial);
            *tutorial = Tutorial::Projects;

            let events = state
                .roll_events(EventPhase::PlanningStart)
                .into_iter()
                .map(|ev| DisplayEvent::new(ev, state))
                .collect();
            self.events.replace(events);
        }

        let should_advance = match self.view.as_tab() {
            Tab::Govt => *tutorial == Tutorial::Parliament,
            Tab::Stats => *tutorial == Tutorial::Dashboard,
            Tab::World => *tutorial == Tutorial::Regions,
            Tab::Plan => *tutorial == Tutorial::Plan,
        };
        if should_advance {
            tutorial.advance();
            if *tutorial == Tutorial::Ready {
                let mut state = STATE.write();
                state.prefs.tutorial = Tutorial::Ready;
                if state.prefs.runs_played == 0 {
                    state.prefs.runs_played = 1;
                }
            }
        }
    }
}

pub(crate) enum View {
    Plan(Plan),
    Govt(Parliament),
    Stats(Stats),
    World(Regions),
}
impl View {
    fn as_tab(&self) -> Tab {
        match self {
            View::Plan(_) => Tab::Plan,
            View::Govt(_) => Tab::Govt,
            View::Stats(_) => Tab::Stats,
            View::World(_) => Tab::World,
        }
    }
}

fn tab_frame(is_selected: bool) -> egui::Frame {
    egui::Frame::NONE
        .fill(if is_selected {
            Color32::from_rgb(0xB9, 0xF8, 0x0D)
        } else {
            Color32::WHITE
        })
        .inner_margin(Margin::symmetric(12, 8))
}

#[derive(Debug, PartialEq)]
enum Tab {
    Plan,
    Govt,
    Stats,
    World,
}
impl Display for Tab {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tab::Plan => t!("Plan"),
                Tab::Govt => t!("Govt"),
                Tab::Stats => t!("Stats"),
                Tab::World => t!("World"),
            }
        )
    }
}
