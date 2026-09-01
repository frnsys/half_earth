mod grid;
mod processes;
mod projects;
mod slots;

use std::collections::BTreeMap;

use egui::Color32;
use hes_engine::{EventPhase, Id, ProjectType, State};
use rust_i18n::t;

use crate::{
    parts::{button, set_full_bg_image},
    state::{GameState, PlanChange, Tutorial},
    views::session::{TabItem, render_tabs},
};

pub use processes::Changes;
use processes::{Processes, ProcessesAction};
use projects::{Projects, ProjectsAction};

pub enum PlanAction {
    EnterWorld,
    PageChanged(&'static [EventPhase]),
    PlanChanged,
}

pub struct Plan {
    page: Page,
}
impl Plan {
    pub fn new() -> Self {
        Self {
            page: Page::Overview,
        }
    }

    pub fn page_is_open(&self) -> bool {
        !matches!(self.page, Page::Overview)
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> Option<PlanAction> {
        let cur_page = self.page.as_uint();
        let mut ret_action = None;

        set_full_bg_image(
            ui,
            hes_images::background_image("plan.png"),
            egui::vec2(1600., 1192.),
        );

        match &mut self.page {
            Page::Overview => {
                ret_action = self.render_overview(
                    ui,
                    &state.core,
                    &state.ui.tutorial,
                    &state.ui.viewed,
                    &state.ui.plan_changes,
                );
            }
            Page::Processes(processes) => {
                if let Some(action) = processes.render(ui, state) {
                    match action {
                        ProcessesAction::Changed => {
                            ret_action = Some(PlanAction::PlanChanged);
                        }
                        ProcessesAction::Back => self.close_page(&mut state.ui.tutorial),
                    }
                }
            }
            Page::Projects(projects) => {
                let action = projects.render(ui, state);
                if let Some(action) = action {
                    match action {
                        ProjectsAction::ChangeTo(next_kind) => {
                            let phases = match next_kind {
                                ProjectType::Policy => &[EventPhase::PlanningPolicies],
                                ProjectType::Research => &[EventPhase::PlanningResearch],
                                ProjectType::Initiative => &[EventPhase::PlanningInitiatives],
                            };
                            ret_action = Some(PlanAction::PageChanged(phases));
                        }
                        ProjectsAction::Changed => {
                            ret_action = Some(PlanAction::PlanChanged);
                        }
                        ProjectsAction::Back => self.close_page(&mut state.ui.tutorial),
                    }
                }
            }
            Page::All => {
                self.render_full_plan(
                    ui,
                    &state.core,
                    &mut state.ui.tutorial,
                    &state.ui.plan_changes,
                );
            }
        }

        if self.page.as_uint() != cur_page {
            let phases: &[EventPhase] = match self.page {
                Page::Overview => &[EventPhase::PlanningPlan],
                Page::Projects(_) => &[EventPhase::PlanningAdd, EventPhase::PlanningResearch],
                Page::Processes(_) => &[EventPhase::PlanningProcesses],
                Page::All => &[EventPhase::PlanningPlan],
            };
            ret_action = Some(PlanAction::PageChanged(phases));
        }

        ret_action
    }

    fn render_overview(
        &mut self,
        ui: &mut egui::Ui,
        state: &State,
        tutorial: &Tutorial,
        viewed: &[Id],
        plan_changes: &BTreeMap<Id, PlanChange>,
    ) -> Option<PlanAction> {
        if let Some(page) = slots::projects_preview(ui, state, tutorial, viewed, plan_changes) {
            self.set_page(page);
        }

        ui.add_space(48.);

        if let Some(page) = slots::processes_preview(ui, state, tutorial, viewed, plan_changes) {
            self.set_page(page);
        }

        ui.add_space(48.);

        let resp = ui
            .vertical_centered(|ui| {
                ui.set_width(320.);

                let ready_disabled = tutorial.lt(&Tutorial::Ready);
                let ready_highlighted = tutorial.eq(&Tutorial::Ready);

                let mut b = button(t!("Ready")).full_width();

                if ready_disabled {
                    ui.disable();
                } else {
                    b = b.colors(
                        Color32::from_rgb(0xf7, 0x5c, 0x52),
                        Color32::from_rgb(0x82, 0x14, 0x0c),
                        Color32::from_rgb(0xfa, 0x23, 0x14),
                        Color32::from_rgb(0xeb, 0x40, 0x34),
                    );
                }
                let b = b.maybe_highlight(ready_highlighted);
                ui.add(b)
            })
            .inner;

        ui.add_space(48.);

        if resp.clicked() {
            Some(PlanAction::EnterWorld)
        } else {
            None
        }
    }

    fn render_full_plan(
        &mut self,
        ui: &mut egui::Ui,
        state: &State,
        tutorial: &mut Tutorial,
        plan_changes: &BTreeMap<Id, PlanChange>,
    ) {
        let projects = &state.world.projects;
        let active_projects: Vec<_> = projects
            .iter()
            .filter(|p| p.is_online() || p.is_building())
            .collect();

        let tabs = vec![TabItem::<()> {
            tab: (),
            selected: false,
            tutorial: None,
            label: t!("Back").to_string(),
            icon: None,
            disabled: false,
        }];
        if render_tabs(ui, tutorial, &tabs).is_some() {
            self.close_page(tutorial);
        }

        ui.add_space(32.);

        slots::all_projects(ui, &active_projects);
    }

    fn close_page(&mut self, tutorial: &mut Tutorial) {
        if (matches!(self.page, Page::Projects(_)) && *tutorial == Tutorial::ProjectsBack)
            || (matches!(self.page, Page::Processes(_)) && *tutorial == Tutorial::ProcessesBack)
        {
            tutorial.advance();
        }
        self.set_page(Page::Overview);
    }

    fn set_page(&mut self, page: Page) {
        self.page = page;
    }
}

enum Page {
    Overview,
    Processes(Processes),
    Projects(Projects),
    All,
}
impl Page {
    fn as_uint(&self) -> u8 {
        match self {
            Page::Overview => 0,
            Page::Processes(_) => 1,
            Page::Projects(_) => 2,
            Page::All => 3,
        }
    }
}
