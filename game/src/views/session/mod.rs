mod govt;
mod plan;
mod regions;
mod stats;
mod treemap;

use std::{fmt::Display, sync::Arc};

use egui::{Color32, CornerRadius, Margin, Sense};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{EventPhase, Flag, State};
use rust_i18n::t;

use crate::{
    audio,
    debug::DEBUG,
    display::Icon,
    parts::{RaisedFrame, h_center, raised_frame},
    state::{GameState, StateExt, Tutorial, update_factors},
    views::events::{EventResult, Events},
};

use govt::Parliament;
use plan::{Plan, PlanAction};
use regions::Regions;
use stats::Stats;

pub struct Session {
    view: View,
    events: Events,
}
impl Session {
    pub fn new(state: &mut GameState) -> Self {
        let mut events: Vec<_> = StateExt::roll_events(
            &mut state.core,
            EventPhase::PlanningStart,
        )
        .into_iter()
        .chain(StateExt::roll_events(
            &mut state.core,
            EventPhase::PlanningPlan,
        ))
        .collect();

        update_factors(state);

        let points = state.collect_research_points();
        state.ui.points.research += points;

        audio::soundtrack(audio::Track::Planning);

        if DEBUG.view.is_some() {
            events.retain(|ev| {
                ev.name != "Planning Intro"
                    && ev.name != "Welcome Back"
            });
        }

        Self {
            view: View::Plan(Plan::new()),
            events: Events::new(events, state),
        }
    }

    pub(super) fn stats() -> Self {
        Self::from_view(View::Stats(Stats::new()))
    }

    pub(super) fn govt(state: &State) -> Self {
        Self::from_view(View::Govt(Parliament::new(state)))
    }

    pub(super) fn regions(
        context: &Arc<three_d::context::Context>,
    ) -> Self {
        Self::from_view(View::World(Regions::new(context)))
    }

    pub(super) fn plan() -> Self {
        Self::from_view(View::Plan(Plan::new()))
    }

    fn from_view(view: View) -> Self {
        Self {
            view,
            events: Events::empty(),
        }
    }

    fn set_tab(
        &mut self,
        tab: &Tab,
        state: &mut State,
        ctx: &Arc<three_d::context::Context>,
    ) {
        let phase = match tab {
            Tab::Plan => EventPhase::PlanningPlan,
            Tab::World => EventPhase::PlanningRegions,
            Tab::Stats => EventPhase::PlanningDashboard,
            Tab::Govt => EventPhase::PlanningParliament,
        };

        let events = StateExt::roll_events(state, phase);
        self.events.replace(events, state);
        self.view = match tab {
            Tab::Plan => View::Plan(Plan::new()),
            Tab::Govt => View::Govt(Parliament::new(state)),
            Tab::Stats => View::Stats(Stats::new()),
            Tab::World => View::World(Regions::new(ctx)),
        };
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut GameState,
        ctx: &Arc<three_d::context::Context>,
    ) -> bool {
        if self.view.show_tabs() {
            let tabs = &[
                TabItem {
                    tab: Tab::Plan,
                    selected: self.view.as_tab() == Tab::Plan,
                    tutorial: Some(Tutorial::Plan),
                    icon: None,
                    label: Tab::Plan.to_string(),
                    disabled: false,
                },
                TabItem {
                    tab: Tab::Govt,
                    selected: self.view.as_tab() == Tab::Govt,
                    tutorial: Some(Tutorial::Parliament),
                    icon: None,
                    label: Tab::Govt.to_string(),
                    disabled: false,
                },
                TabItem {
                    tab: Tab::Stats,
                    selected: self.view.as_tab() == Tab::Stats,
                    tutorial: Some(Tutorial::Dashboard),
                    icon: None,
                    label: Tab::Stats.to_string(),
                    disabled: false,
                },
                TabItem {
                    tab: Tab::World,
                    selected: self.view.as_tab() == Tab::World,
                    tutorial: Some(Tutorial::Regions),
                    icon: None,
                    label: Tab::World.to_string(),
                    disabled: false,
                },
            ];
            if let Some(tab) =
                render_tabs(ui, &state.ui.tutorial, tabs)
            {
                self.set_tab(tab, &mut state.core, ctx);
            }
        }

        if !self.events.is_finished {
            let result = self.events.render(ui, state);
            if result == Some(EventResult::JustFinished) {
                self.update_tutorial(
                    &mut state.core,
                    &mut state.ui.tutorial,
                );
            }
        }

        let mut go_to_world = false;
        match &mut self.view {
            View::Plan(plan) => {
                let action = plan.render(ui, state);
                if let Some(action) = action {
                    match action {
                        PlanAction::EnterWorld => {
                            go_to_world = true;
                        }
                        PlanAction::PlanChanged => {
                            self.events
                                .replace(StateExt::roll_events(
                                &mut state.core,
                                EventPhase::PlanningPlanChange,
                            ), state);
                        }
                        PlanAction::PageChanged(phase) => {
                            self.events.replace(
                                StateExt::roll_events(
                                    &mut state.core,
                                    phase,
                                ),
                                state,
                            );
                        }
                    }
                }
            }
            View::Govt(parliament) => {
                parliament.render(ui, state)
            }
            View::Stats(stats) => {
                stats.render(
                    ui,
                    &state.core,
                    &state.ui.process_mix_changes,
                );
            }
            View::World(regions) => regions.render(
                ui,
                &state.core,
                &state.ui.annual_region_events,
            ),
        }

        go_to_world
    }

    fn update_tutorial(
        &mut self,
        state: &mut State,
        tutorial: &mut Tutorial,
    ) {
        if state.flags.contains(&Flag::SkipTutorial) {
            *tutorial = Tutorial::Ready;
        } else if state.flags.contains(&Flag::RepeatTutorial) {
            state
                .flags
                .retain(|flag| *flag != Flag::RepeatTutorial);
            *tutorial = Tutorial::Projects;

            let events = StateExt::roll_events(
                state,
                EventPhase::PlanningStart,
            );
            self.events.replace(events, state);
        }

        let should_advance = match self.view.as_tab() {
            Tab::Govt => *tutorial == Tutorial::Parliament,
            Tab::Stats => *tutorial == Tutorial::Dashboard,
            Tab::World => *tutorial == Tutorial::Regions,
            Tab::Plan => *tutorial == Tutorial::Plan,
        };
        if should_advance {
            tutorial.advance();
        }
    }
}

enum View {
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

    fn show_tabs(&self) -> bool {
        match self {
            View::Plan(plan) => !plan.page_is_open(),
            _ => true,
        }
    }
}

fn tab_frame(is_selected: bool) -> RaisedFrame {
    if is_selected {
        raised_frame().colors(
            Color32::from_rgb(0xe7, 0xff, 0xa6),
            Color32::from_rgb(0x8b, 0xb5, 0x19),
            Color32::from_rgb(0xB9, 0xF8, 0x0D),
        )
    } else {
        raised_frame().colors(
            Color32::WHITE,
            Color32::from_rgb(0xdc, 0xe0, 0xe6),
            Color32::from_rgb(0xfa, 0xfc, 0xff),
        )
    }
    .margin(Margin::symmetric(12, 8))
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

struct TabItem<T> {
    tab: T,
    selected: bool,
    tutorial: Option<Tutorial>,
    icon: Option<Icon>,
    label: String,
    disabled: bool,
}

fn render_tabs<'a, T>(
    ui: &mut egui::Ui,
    cur_tutorial: &Tutorial,
    tabs: &'a [TabItem<T>],
) -> Option<&'a T> {
    let mut clicked_tab = None;
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
                    ui.style_mut().visuals.override_text_color = Some(Color32::BLACK);

                    let n = tabs.len();
                    for (i, tab) in tabs.into_iter().enumerate() {
                        let mut highlight = false;
                        let mut disabled = tab.disabled;
                        if let Some(tutorial) = tab.tutorial {
                            highlight = *cur_tutorial == tutorial;
                            disabled = *cur_tutorial < tutorial || tab.disabled;
                        }

                        let radius = if i == 0 {
                            CornerRadius {
                                sw: 4,
                                ..Default::default()
                            }
                        } else if i == n - 1 {
                            CornerRadius {
                                se: 4,
                                ..Default::default()
                            }
                        } else {
                            CornerRadius::default()
                        };

                        let mut frame =
                            tab_frame(
                                tab.selected,
                            )
                            .radius(radius);

                        if highlight {
                            frame = frame.highlight();
                        }

                        frame.show(
                            ui,
                            |ui| {
                                let resp = egui::Frame::NONE
                                    .show(ui, |ui| {
                                        if disabled {
                                            ui.set_opacity(0.5);
                                        }

                                        if let Some(icon) = tab.icon {
                                            ui.add(icon.size(16.));
                                        }
                                        ui.label(
                                            egui::RichText::new(&tab.label)
                                            .heading()
                                            .size(15.),
                                        );
                                    })
                                .response;
                                let resp = resp.interact(Sense::all());
                                if !disabled && resp.clicked() {
                                    clicked_tab = Some(&tab.tab);
                                }
                            });
                    }
                });
            });
        })
    });
    clicked_tab
}
