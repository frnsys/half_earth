use std::{borrow::Cow, collections::BTreeMap};

use egui::Sense;
use enum_map::EnumMap;
use hes_engine::{
    EventPhase,
    Feedstock,
    Id,
    KindMap,
    Output,
    ProjectType,
    Resource,
    State,
    Status,
};
use rust_i18n::t;
use strum::IntoEnumIterator;

use crate::{
    display::{
        AsText,
        HasIcon,
        icon_from_slug,
        icons,
        resource,
        to_energy_units,
    },
    image,
    state::{PlanChange, Points, StateExt, Tutorial},
    views::{Tip, tip, tips::add_tip},
};

pub enum PlanAction {
    EnterWorld,
    PageChanged(EventPhase),
}

// TODO
// Save when starting the planning session.
// game.with_untracked(move |game| {
//     ui.with_untracked(move |ui| {
//         crate::state::save(game, ui);
//     });
// });

// TODO
// let enter_world = move || {
//     game.with_untracked(|game| {
//         crate::state::save(game, &ui.get_untracked());
//     });
//     set_phase.set(Phase::Events);
// };

pub struct Plan {
    page: Page,
}
impl Plan {
    pub fn new() -> Self {
        Self {
            page: Page::Overview,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &State,
        tutorial: &mut Tutorial,
        viewed: &Vec<Id>,
        points: &Points,
        plan_changes: &BTreeMap<Id, PlanChange>,
    ) -> Option<PlanAction> {
        let cur_page = self.page;
        let mut ret_action = None;
        match &mut self.page {
            Page::Overview => {
                ret_action = self.render_overview(
                    ui, state, tutorial, viewed,
                );
            }
            Page::Processes => todo!(),
            Page::Projects(kind) => {
                let action = render_projects(
                    ui,
                    state,
                    kind,
                    tutorial,
                    points,
                    plan_changes,
                );

                if let Some(action) = action {
                    match action {
                        ProjectsAction::ChangeTo(next_kind) => {
                            let phase = match next_kind {
                                ProjectType::Policy => {
                                    EventPhase::PlanningPolicies
                                }
                                ProjectType::Research => {
                                    EventPhase::PlanningResearch
                                }
                                ProjectType::Initiative => {
                                    EventPhase::PlanningInitiatives
                                }
                            };
                            ret_action = Some(
                                PlanAction::PageChanged(phase),
                            );
                            *kind = next_kind;
                        }
                        ProjectsAction::Back => {
                            self.close_page(tutorial)
                        }
                    }
                }
            }
            Page::All => {
                self.render_full_plan(ui, state, tutorial);
            }
        }

        if self.page != cur_page {
            let phase = match self.page {
                Page::Overview => EventPhase::PlanningPlan,
                Page::Projects(_) => EventPhase::PlanningAdd,
                Page::Processes => {
                    EventPhase::PlanningProcesses
                }
                Page::All => EventPhase::PlanningPlan,
            };
            ret_action = Some(PlanAction::PageChanged(phase));
        }

        ret_action
    }

    fn render_overview(
        &mut self,
        ui: &mut egui::Ui,
        state: &State,
        tutorial: &Tutorial,
        viewed: &Vec<Id>,
    ) -> Option<PlanAction> {
        let projects = &state.world.projects;
        let processes = &state.world.processes;
        let any_new_projects = projects
            .unlocked()
            .any(|p| !viewed.contains(&p.id));
        let any_new_processes = processes
            .unlocked()
            .any(|p| !viewed.contains(&p.id));

        // TODO
        // <Help
        //     text=t!("Add some cards to get started")
        //     x=0.5
        //     y=220.0
        //     center=true
        // />

        let new_icon = image!("new.svg");
        if any_new_projects {
            ui.image(new_icon.clone());
        }

        // TODO
        // class:highlight=projects_highlighted
        let projects_highlighted =
            tutorial.eq(&Tutorial::Projects);
        let button =
            egui::Button::image_and_text(icons::ADD, t!("Add"));
        if ui.add(button).clicked() {
            self.set_page(Page::Projects(
                ProjectType::Research,
            ));
        }

        let slots = calc_slots(ui);

        let active_projects =
            projects.part_of_plan().collect::<Vec<_>>();
        let n_active = active_projects.len();
        let n_projects = {
            if n_active > slots {
                // Save one spot for "View All"
                slots - 1
            } else {
                n_active
            }
        };

        let placeholders = (slots as isize
            - active_projects.len() as isize)
            .max(0) as usize;

        for p in active_projects.into_iter().take(n_projects) {
            // TODO
            // <MiniProject project/>
        }
        for _ in 0..placeholders {
            // TODO empty card slot
        }
        if n_active > slots {
            let resp = ui.button(t!("View All"));
            if resp.clicked() {
                self.set_page(Page::All);
            }
        }

        // next section (production)

        if any_new_processes {
            ui.image(new_icon);
        }

        let processes_over_limit = state
            .world
            .processes
            .over_limit(
                state.output_demand.total(),
                state.feedstocks.available,
            )
            .map(|p| t!(&p.name))
            .collect::<Vec<_>>();
        if !processes_over_limit.is_empty() {
            let tip = tip(
                icons::ALERT,
                t!(
                    "The following processes can't produce as much as they need to: %{processesOverLimit}",
                    processesOverLimit =
                        processes_over_limit.join(", ")
                ),
            );
            add_tip(tip, ui.image(icons::ALERT));
        }

        let prod_shortages = production_shortages(state);
        let inp_shortages = input_shortages(state);

        let shortages_tip = tip(
            icons::ALERT,
            format!(
                "{}. {}",
                prod_shortages.clone().unwrap_or(String::new()),
                inp_shortages.unwrap_or(String::new())
            ),
        );

        if prod_shortages.is_some() {
            add_tip(
                shortages_tip.clone(),
                ui.image(icons::ALERT),
            );
        }

        let max_processes = Output::iter().map(|output| {
            processes
                .iter()
                .filter(|p| p.output == output)
                .max_by_key(|p| p.mix_share)
                .unwrap()
        });
        let output_demand = state.output_demand.total();
        for process in max_processes {
            let produced = crate::display::output(
                state.produced.of(process.output),
                process.output,
            );

            let demand = crate::display::output(
                output_demand[process.output],
                process.output,
            );

            let icon = process.output.icon();

            // TODO
            // <MiniProcess process/>
            if produced / demand < 0.99 {
                add_tip(
                    shortages_tip.clone(),
                    ui.image(icons::ALERT),
                );
            } else {
                ui.image(icons::CHECK);
            }
            ui.label(format!("{:.0}/{:.0}", produced, demand));
        }

        render_resource_status(
            ui,
            state,
            if prod_shortages.is_some() {
                Some(shortages_tip)
            } else {
                None
            },
        );

        let processes_disabled =
            tutorial.lt(&Tutorial::Processes);
        let processes_highlighted =
            tutorial.eq(&Tutorial::Processes);
        // TODO highlighting
        if ui.button(t!("Change Production")).clicked() {
            self.set_page(Page::Processes);
        }

        let ready_disabled = tutorial.lt(&Tutorial::Ready);
        let ready_highlighted = tutorial.eq(&Tutorial::Ready);
        // TODO highlighting
        if ui.button(t!("Ready")).clicked() {
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
    ) {
        let projects = &state.world.projects;
        let active_projects = projects
            .iter()
            .filter(|p| p.is_online() || p.is_building());

        if ui.button(t!("Back")).clicked() {
            self.close_page(tutorial);
        }

        let button =
            egui::Button::image_and_text(icons::ADD, t!("Add"));
        if ui.add(button).clicked() {
            self.set_page(Page::Projects(
                ProjectType::Research,
            ));
        }

        for project in active_projects {
            // TODO
            // <MiniProject project/>
            ui.label(t!(&project.name));
        }
    }

    fn close_page(&mut self, tutorial: &mut Tutorial) {
        if matches!(self.page, Page::Projects(_))
            && *tutorial == Tutorial::ProjectsBack
        {
            tutorial.advance();
        } else if self.page == Page::Processes
            && *tutorial == Tutorial::ProcessesBack
        {
            tutorial.advance();
        }
        self.set_page(Page::Overview);
    }

    fn set_page(&mut self, page: Page) {
        self.page = page;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Overview,
    Processes,
    Projects(ProjectType),
    All,
}

fn calc_slots(ui: &mut egui::Ui) -> usize {
    let width = ui.ctx().screen_rect().width();
    if width > 680. {
        9
    } else if width > 560. {
        7
    } else {
        5
    }
}

fn input_shortages(state: &State) -> Option<String> {
    let resources = &state.resources;
    let feedstocks = &state.feedstocks;
    let resources: Vec<_> = Resource::iter()
        .filter(|res| resources.has_shortage(*res))
        .map(|r| t!(r.title()))
        .collect();

    let feedstock: Vec<_> = Feedstock::iter()
        .filter(|res| {
            feedstocks.has_shortage(*res)
                && *res != Feedstock::Other
                && *res != Feedstock::Soil
        })
        .map(|r| t!(r.title()))
        .collect();

    let shortages = [resources, feedstock].concat();
    if shortages.is_empty() {
        None
    } else {
        Some(t!(
            "There is not enough %{resources}. You should change your production mixes to use less of these or reduce demand elsewhere.",
            resources = shortages.join(", ")
        ).to_string())
    }
}

fn production_shortages(state: &State) -> Option<String> {
    let produced = &state.produced;
    let output_demand = state.output_demand.total();

    let problems = {
        let mut problems: EnumMap<Output, f32> =
            EnumMap::from_array([1.; 4]);
        for output in Output::iter() {
            tracing::debug!(
                "{output:?}: produced={}, demand={}",
                crate::display::output(
                    produced.of(output),
                    output
                ),
                crate::display::output(
                    output_demand[output],
                    output
                )
            );
            let met =
                produced.of(output) / output_demand[output];
            if met >= 0.99 {
                continue;
            } else {
                if met < problems[output] {
                    problems[output] = met;
                }
            }
        }
        problems
    };

    let problems: Vec<_> = problems
        .into_iter()
        .filter(|(_, met)| *met < 1.)
        .map(|(output, met)| {
            (
                output,
                if met >= 0.85 {
                    Severity::Mild
                } else if met >= 0.75 {
                    Severity::Alarming
                } else if met >= 0.5 {
                    Severity::Severe
                } else {
                    Severity::Critical
                },
            )
        })
        .collect();

    if problems.is_empty() {
        None
    } else {
        if problems.len() == 1 {
            let (output, severity) = &problems[0];
            let desc = severity.desc();
            let details = t!(&output.title());
            Some(format!("{desc}: {details}"))
        } else {
            let list = problems
                .into_iter()
                .map(|(output, severity)| {
                    let title = t!(&output.title());
                    let label = severity.label();
                    format!("{title} ({label})")
                })
                .collect::<Vec<_>>()
                .join("\n");
            let desc =
                t!("There are multiple production shortages:");
            Some(format!("{desc} {list}"))
        }
    }
}

fn render_resource_status(
    ui: &mut egui::Ui,
    state: &State,
    shortages_tip: Option<Tip>,
) {
    let resources = &state.resources;
    let protected_land = state.protected_land;
    let resource_demand = &state.resource_demand;
    let starting_resources = state.world.starting_resources;

    for (k, demand) in resource_demand.total().items() {
        let demand = match k {
            Resource::Electricity | Resource::Fuel => {
                to_energy_units(demand)
            }
            Resource::Water => {
                resource(demand, k, resources.available)
            }
            Resource::Land => {
                // For land we add in protected land as well.
                let protected = protected_land * 100.;
                resource(demand, k, starting_resources)
                    + protected
            }
        };
        let available = match k {
            Resource::Electricity | Resource::Fuel => {
                to_energy_units(resources.available[k])
            }
            Resource::Land | Resource::Water => 100.,
        };
        // TODO
        // <div class="resources-info-pill" class:not-enough={demand > available}>
        let resp = ui
            .horizontal_centered(|ui| {
                ui.image(k.icon());
                ui.label(format!(
                    "{:.0}/{:.0}",
                    demand, available
                ));
            })
            .response;
        if let Some(shortages_tip) = &shortages_tip {
            add_tip(shortages_tip.clone(), resp);
        }
    }
}

enum Severity {
    Mild,
    Alarming,
    Severe,
    Critical,
}
impl Severity {
    fn desc(&self) -> Cow<'static, str> {
        match self {
            Severity::Mild => {
                t!("There is a mild production shortage")
            }
            Severity::Alarming => {
                t!("There is a alarming production shortage")
            }
            Severity::Severe => {
                t!("There is a severe production shortage")
            }
            Severity::Critical => {
                t!("There is a critical production shortage")
            }
        }
    }

    fn label(&self) -> Cow<'static, str> {
        match self {
            Severity::Mild => t!("mild"),
            Severity::Alarming => t!("alarming"),
            Severity::Severe => t!("severe"),
            Severity::Critical => t!("critical"),
        }
    }
}

fn render_points(
    ui: &mut egui::Ui,
    state: &State,
    points: &Points,
    kind: ProjectType,
) {
    let pc_points = state.political_capital;
    let available_points = match kind {
        ProjectType::Policy => state.political_capital,
        ProjectType::Initiative => points.initiative,
        ProjectType::Research => points.research,
    };
    let next_point_cost = state.next_point_cost(&kind);

    ui.horizontal_centered(|ui| {
        ui.label(pc_points.to_string());
        ui.image(icons::POLITICAL_CAPITAL);

        if kind != ProjectType::Policy {
            if available_points > 0 {
                ui.label(available_points.to_string());
                ui.image(kind.icon());
            } else {
                ui.label(next_point_cost.to_string());
                ui.image(icons::POLITICAL_CAPITAL);
                ui.image(icons::ARROW_RIGHT);
                ui.image(kind.icon());
            }
        }
    });
}

enum ProjectsAction {
    ChangeTo(ProjectType),
    Back,
}

fn render_projects(
    ui: &mut egui::Ui,
    state: &State,
    kind: &ProjectType,
    tutorial: &Tutorial,
    points: &Points,
    plan_changes: &BTreeMap<Id, PlanChange>,
) -> Option<ProjectsAction> {
    // TODO
    let scan_tip = t!(
        "↑ Swipe this card up and hold to add it to your plan ↑"
    );
    let scroll_tip = format!(
        "⟵ {}⟶ ",
        t!("Swipe sideways to see other projects")
    );
    // <Help text=scan_tip x=0.5 y=150. center=true/>
    // <Help text=scroll_tip x=0.5 y=250. center=true/>

    let back_disabled = tutorial.lt(&Tutorial::ProjectsBack);
    let back_highlighted = tutorial.eq(&Tutorial::ProjectsBack);

    let mut action = None;
    ui.horizontal_centered(|ui| {
        for (label, kind, icon) in [
            (
                t!("Research"),
                ProjectType::Research,
                icons::RESEARCH,
            ),
            (
                t!("Infrastructure"),
                ProjectType::Initiative,
                icons::INITIATIVE,
            ),
            (
                t!("Policies"),
                ProjectType::Policy,
                icons::POLICY,
            ),
        ] {
            // TODO if selected
            let resp = ui
                .vertical_centered(|ui| {
                    ui.image(icon);
                    ui.label(label);
                })
                .response;
            let resp = resp.interact(Sense::all());
            if resp.clicked() {
                action = Some(ProjectsAction::ChangeTo(kind));
            }
        }

        // TODO
        // class:disabled=back_disabled
        // class:highlight=back_highlighted
        let resp = ui
            .vertical_centered(|ui| {
                ui.label(t!("Back"));
            })
            .response;
        if resp.clicked() {
            action = Some(ProjectsAction::Back);
        }
    });

    // let debug = get_debug_opts(); // TODO
    let show_all_projects = true;

    let projects = &state.world.projects;
    let project_lockers = &state.world.project_lockers;
    let projects = {
        let mut projects =
                projects
                .iter()
                .filter(|p| {
                    p.kind == *kind && (!p.locked || show_all_projects)

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
                        match projects[locker_id].status {
                            Status::Building | Status::Active | Status::Finished => false,
                            _=> true
                        }
                    }).unwrap_or(true)
                })
                .cloned()
                .collect::<Vec<_>>();
        projects.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });
        projects
    };

    // TODO scanner
    // let scanner = ProjectScanner::new(Some(on_plan_change));

    render_points(ui, state, points, *kind);

    action
}
