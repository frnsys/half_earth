use std::collections::BTreeMap;

use crate::{
    debug::DEBUG,
    display::{HasIcon, icons},
    state::{GameState, PlanChange, Points, StateExt, Tutorial},
    views::{
        Card, Cards,
        session::{TabItem, render_tabs},
    },
};

use super::grid::{CardGrid, CardGridAction};
use hes_engine::{Id, Project, ProjectType, State, Status};
use rust_i18n::t;
use strum::IntoEnumIterator;

pub enum ProjectsAction {
    ChangeTo(ProjectType),
    Changed,
    Back,
}

pub enum Projects {
    Scanner(ProjectsScannerView),
    Grid(CardGrid<Project>),
}
impl Projects {
    pub fn new(state: &State, plan_changes: &BTreeMap<Id, PlanChange>) -> Self {
        if DEBUG.use_card_grid {
            Self::Grid(CardGrid::new(
                get_projects(state, None, plan_changes)
                    .into_iter()
                    .map(|proj| Card::new(proj))
                    .collect(),
            ))
        } else {
            Self::Scanner(ProjectsScannerView::new(state, plan_changes))
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> Option<ProjectsAction> {
        match self {
            Self::Scanner(view) => view.render(ui, state),
            Self::Grid(view) => view.render(ui, state).map(|action| match action {
                CardGridAction::Back => ProjectsAction::Back,
                CardGridAction::Changed => ProjectsAction::Changed,
            }),
        }
    }
}

struct ProjectsScannerView {
    kind: ProjectType,
    cards: Cards<Project>,
}

impl ProjectsScannerView {
    fn new(state: &State, plan_changes: &BTreeMap<Id, PlanChange>) -> Self {
        let kind = ProjectType::Research;
        Self {
            kind,
            cards: Cards::new(get_projects(state, Some(&kind), plan_changes).into_iter()),
        }
    }

    fn set_kind(
        &mut self,
        state: &State,
        kind: ProjectType,
        plan_changes: &BTreeMap<Id, PlanChange>,
    ) {
        self.kind = kind;
        self.cards = Cards::new(get_projects(state, Some(&kind), plan_changes).into_iter());
    }

    fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> Option<ProjectsAction> {
        let mut action = None;

        let mut tabs: Vec<_> = ProjectType::iter()
            .map(|k| TabItem {
                tab: Some(k),
                selected: self.kind == k,
                tutorial: None,
                label: match k {
                    ProjectType::Policy => t!("Policies"),
                    ProjectType::Research => t!("Research"),
                    ProjectType::Initiative => {
                        t!("Infrastructure")
                    }
                }
                .to_string(),
                icon: Some(k.icon()),
                disabled: false,
            })
            .collect();
        tabs.push(TabItem {
            tab: None,
            selected: false,
            tutorial: Some(Tutorial::ProjectsBack),
            label: t!("Back").to_string(),
            icon: None,
            disabled: false,
        });
        if let Some(tab) = render_tabs(ui, &state.ui.tutorial, &tabs) {
            match tab {
                Some(kind) => {
                    self.set_kind(&state.core, *kind, &state.ui.plan_changes);
                    action = Some(ProjectsAction::ChangeTo(*kind));
                }
                None => action = Some(ProjectsAction::Back),
            }
        }

        let changed = self.cards.render(ui, state);
        if changed {
            action = Some(ProjectsAction::Changed);
        }

        egui::Area::new("project-points".into())
            .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0., -12.))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                egui::Frame::NONE
                    .fill(egui::Color32::from_black_alpha(200))
                    .corner_radius(4)
                    .inner_margin(4)
                    .show(ui, |ui| {
                        render_points(ui, state, &state.ui.points, self.kind);
                    });
            });

        action
    }
}

fn get_projects(
    state: &State,
    kind: Option<&ProjectType>,
    plan_changes: &BTreeMap<Id, PlanChange>,
) -> Vec<Project> {
    let projects = &state.world.projects;
    let project_lockers = &state.world.project_lockers;
    let show_all_projects = DEBUG.show_all_projects;
    let mut projects =
                projects
                .iter()
                .filter(|p| {
                    kind.is_none_or(|kind| *kind == p.kind) && (!p.locked || show_all_projects)

                // Filter out finished projects,
                // but show them if they have upgrades
                && (p.status != Status::Finished || !p.upgrades.is_empty())

                // Filter out finished policies
                // but only ones added before
                // this planning session
                && (p.status != Status::Active || plan_changes.contains_key(&p.id) || !p.upgrades.is_empty())

                // Filter out projects that are mutually exclusive
                // with active projects
                    && project_lockers.get(&p.id)
                    .map(|locker_id| {
                        // Is the locker satisfied?
                        !matches!(projects[locker_id].status, Status::Building | Status::Active | Status::Finished)
                    }).unwrap_or(true)
                })
                .cloned()
                .collect::<Vec<_>>();
    projects.sort_by_key(|a| a.name.to_lowercase());
    projects
}

fn render_points(ui: &mut egui::Ui, state: &State, points: &Points, kind: ProjectType) {
    let pc_points = state.political_capital;
    let available_points = match kind {
        ProjectType::Policy => state.political_capital,
        ProjectType::Initiative => points.initiative,
        ProjectType::Research => points.research,
    };
    let next_point_cost = state.next_point_cost(&kind);

    ui.horizontal_centered(|ui| {
        const ICON_SIZE: f32 = 16.;
        ui.label(pc_points.to_string());
        ui.add(icons::POLITICAL_CAPITAL.size(ICON_SIZE));

        if kind != ProjectType::Policy {
            if available_points > 0 {
                ui.label(available_points.to_string());
                ui.add(kind.icon().size(ICON_SIZE));
            } else {
                ui.label(next_point_cost.to_string());
                ui.add(icons::POLITICAL_CAPITAL.size(ICON_SIZE));
                ui.add(icons::ARROW_RIGHT_LIGHT.size(ICON_SIZE));
                ui.add(kind.icon().size(ICON_SIZE));
            }
        }
    });
}
