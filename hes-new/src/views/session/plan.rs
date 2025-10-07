use std::{borrow::Cow, collections::BTreeMap};

use egui::{Color32, Margin, Sense, Shadow, Stroke};
use egui_taffy::TuiBuilderLogic;
use enum_map::EnumMap;
use hes_engine::{
    EventPhase,
    Feedstock,
    Id,
    KindMap,
    Output,
    Process,
    Project,
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
        group_color,
        icons,
        resource,
        to_energy_units,
    },
    image,
    parts::{
        RaisedFrame,
        button,
        flavor_image,
        full_width_button,
        h_center,
        new_icon,
        raised_frame,
        set_full_bg_image,
    },
    state::{PlanChange, Points, StateExt, Tutorial},
    tips::{Tip, add_tip, tip},
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

        set_full_bg_image(
            ui,
            image!("backgrounds/plan.png"),
            egui::vec2(1600., 1192.),
        );

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

        // TODO
        // class:highlight=projects_highlighted
        let projects_highlighted =
            tutorial.eq(&Tutorial::Projects);

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

        let split_at = slots / 2;

        let items: Vec<_> = active_projects
            .into_iter()
            .take(n_projects)
            .map(|p| Some(p))
            .chain((0..placeholders).map(|_| None))
            .collect();
        let top = &items[0..split_at];
        let bot = &items[split_at..];

        ui.add_space(48.);

        h_center(ui, "plan-preview", |tui| {
            tui.ui(|ui| {
                ui.horizontal(|ui| {
                    let resp = ui
                        .add(add_cards_slot(any_new_projects))
                        .interact(Sense::click());
                    if resp.clicked() {
                        self.set_page(Page::Projects(
                            ProjectType::Research,
                        ));
                    }

                    for p in top {
                        match p {
                            Some(proj) => {
                                ui.add(project_card_slot(proj));
                            }
                            None => {
                                ui.add(empty_card_slot());
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    for p in bot {
                        match p {
                            Some(proj) => {
                                ui.add(project_card_slot(proj));
                            }
                            None => {
                                ui.add(empty_card_slot());
                            }
                        }
                    }

                    if n_active > slots {
                        let resp = ui
                            .add(view_all_slot())
                            .interact(Sense::click());
                        if resp.clicked() {
                            self.set_page(Page::All);
                        }
                    }
                });
            });
        });

        // next section (production)
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

        ui.add_space(48.);

        h_center(ui, "plan-processes", |tui| {
            tui.ui(|ui| {
                let resp = inset_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for process in max_processes {
                            let produced =
                                crate::display::output(
                                    state
                                        .produced
                                        .of(process.output),
                                    process.output,
                                );

                            let demand = crate::display::output(
                                output_demand[process.output],
                                process.output,
                            );

                            let icon = process.output.icon();

                            ui.vertical(|ui| {
                                ui.set_width(105.);

                                let has_shortage =
                                    produced / demand < 0.99;

                                let image = if has_shortage {
                                    icons::ALERT.size(14.)
                                } else {
                                    icons::CHECK.size(14.).tint(
                                        Color32::from_rgb(
                                            0x1B, 0xAC, 0x89,
                                        ),
                                    )
                                };

                                let text =
                                    center_text(format!(
                                        "{:.0}/{:.0}",
                                        produced, demand
                                    ))
                                    .image(image);

                                let resp = ui.add(text);

                                if has_shortage {
                                    add_tip(
                                        shortages_tip.clone(),
                                        resp,
                                    );
                                }

                                ui.vertical_centered(|ui| {
                                    ui.add(process_card_slot(
                                        process,
                                    ));

                                    ui.vertical_centered(
                                        |ui| {
                                            ui.label(
                                        egui::RichText::new(
                                            t!(&process
                                                .output
                                                .title()),
                                        ),
                                    );
                                        },
                                    );
                                });
                            });
                        }
                    });

                    ui.add_space(16.);

                    render_resource_status(
                        ui,
                        state,
                        if prod_shortages.is_some() {
                            Some(shortages_tip)
                        } else {
                            None
                        },
                    );

                    ui.add_space(16.);

                    let processes_disabled =
                        tutorial.lt(&Tutorial::Processes);
                    let processes_highlighted =
                        tutorial.eq(&Tutorial::Processes);
                    // TODO highlighting
                    if ui
                        .add(full_width_button(t!(
                            "Change Production"
                        )))
                        .clicked()
                    {
                        self.set_page(Page::Processes);
                    }
                });
                if any_new_processes {
                    ui.add(new_icon(resp.rect));
                }
            });
        });

        ui.add_space(48.);

        let ready_disabled = tutorial.lt(&Tutorial::Ready);
        let ready_highlighted = tutorial.eq(&Tutorial::Ready);
        // TODO highlighting
        let resp = ui
            .vertical_centered(|ui| {
                ui.set_width(320.);
                ui.add(full_width_button(t!("Ready")))
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
    ) {
        let projects = &state.world.projects;
        let active_projects = projects
            .iter()
            .filter(|p| p.is_online() || p.is_building());

        if ui.button(t!("Back")).clicked() {
            self.close_page(tutorial);
        }

        ui.horizontal(|ui| {
            let resp = ui
                .add(add_cards_slot(false))
                .interact(Sense::click());
            if resp.clicked() {
                self.set_page(Page::Projects(
                    ProjectType::Research,
                ));
            }
            for project in active_projects {
                ui.add(project_card_slot(project));
            }
        });
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

    h_center(ui, "resource-status", |tui| {
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
            tui.ui(|ui| {
                let resp = egui::Frame::NONE
                    .fill(Color32::from_rgb(0xE4, 0xC9, 0xC2))
                    .stroke(Stroke::new(
                        1.,
                        Color32::from_rgb(0xB8, 0xA2, 0x9C),
                    ))
                    .inner_margin(Margin::symmetric(3, 2))
                    .corner_radius(3)
                    .show(ui, |ui| {
                        ui.horizontal_centered(|ui| {
                            ui.add(k.icon().size(16.));
                            ui.label(format!(
                                "{:.0}/{:.0}",
                                demand, available
                            ));
                        });
                    })
                    .response;
                if let Some(shortages_tip) = &shortages_tip {
                    add_tip(shortages_tip.clone(), resp);
                }
            });
        }
    });
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

fn add_cards_slot(
    show_new: bool,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    move |ui| {
        let resp = inset_frame().margin(0).show(ui, |ui| {
            ui.set_height(155.);
            ui.set_width(105. - 1.); // account for inset shadow
            ui.vertical_centered(|ui| {
                ui.add_space(54.);
                ui.add(icons::ADD.size(32.));
                ui.label(t!("Add"));
            });
        });

        if show_new {
            ui.add(new_icon(resp.rect));
        }

        resp
    }
}

fn view_all_slot()
-> impl FnOnce(&mut egui::Ui) -> egui::Response {
    move |ui| {
        inset_frame().margin(0).show(ui, |ui| {
            ui.set_height(155.);
            ui.set_width(105. - 1.); // account for inset shadow
            ui.vertical_centered(|ui| {
                ui.add_space(68.);
                ui.label(t!("View All"));
            });
        })
    }
}

fn empty_card_slot()
-> impl FnOnce(&mut egui::Ui) -> egui::Response {
    |ui| {
        egui::Frame::NONE
            .stroke(Stroke::new(
                1.,
                Color32::from_black_alpha(48),
            ))
            .corner_radius(6)
            .show(ui, |ui| {
                ui.set_height(155.);
                ui.set_width(105.);
            })
            .response
    }
}

fn project_card_slot(
    project: &Project,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    let image = flavor_image(&project.flavor.image);
    let (color, _) = group_color(&project.group);
    let icon = project.kind.icon();
    let is_building = project.is_building();
    let is_finished = project.is_active();
    let progress = project.progress;

    move |ui| {
        egui::Frame::NONE
            .stroke(Stroke::new(5., color))
            .corner_radius(6)
            .shadow(Shadow {
                offset: [1, 1],
                blur: 5,
                spread: 2,
                color: Color32::from_black_alpha(48),
            })
            .show(ui, |ui| {
                // account for stroke
                ui.set_height(155. - 6.);
                ui.set_width(105. - 8.);
                let height = 155. - 6.;
                let width = 105. - 8.;

                if let Some(image_size) = image
                    .load_and_calc_size(ui, ui.available_size())
                {
                    let target_size = egui::vec2(width, height);

                    // Compute aspect ratios
                    let image_aspect =
                        image_size.x / image_size.y;
                    let target_aspect =
                        target_size.x / target_size.y;

                    let draw_size =
                        if image_aspect > target_aspect {
                            egui::Vec2::new(
                                target_size.y * image_aspect,
                                target_size.y,
                            )
                        } else {
                            egui::Vec2::new(
                                target_size.x,
                                target_size.x / image_aspect,
                            )
                        };

                    let center = ui.cursor().left_top()
                        + target_size / 2.;
                    let clip_rect = egui::Rect::from_min_size(
                        ui.cursor().left_top(),
                        target_size,
                    );
                    let draw_rect =
                        egui::Rect::from_center_size(
                            center, draw_size,
                        );
                    ui.scope(|ui| {
                        ui.shrink_clip_rect(clip_rect);
                        image.paint_at(ui, draw_rect);
                    });
                }

                // This is off-center for some reason and needs
                // manual adjustment.
                egui::Frame::NONE
                    .inner_margin(Margin {
                        left: -8,
                        ..Default::default()
                    })
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(42.);
                            ui.add(icon.size(48.));

                            ui.add_space(8.);

                            if is_finished {
                                ui.add(icons::CHECK.size(18.));
                            } else if is_building {
                                fill_bar(
                                    ui,
                                    (72., 8.),
                                    progress,
                                );
                            }
                        });
                    });
            })
            .response
    }
}

// TODO dedupe
fn fill_bar(
    ui: &mut egui::Ui,
    (width, height): (f32, f32),
    filled: f32,
) {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(width, height),
        Sense::empty(),
    );
    let painter = ui.painter();
    painter.rect_filled(rect, 2, Color32::WHITE);

    let mut inner = rect.shrink(1.);
    inner.set_width(inner.width() * filled);
    painter.rect_filled(inner, 2, Color32::PURPLE);
}

fn inset_frame() -> RaisedFrame {
    raised_frame().colors(
        Color32::from_rgb(0x91, 0x7e, 0x7e),
        Color32::from_rgb(0xF5, 0xE8, 0xD7),
        Color32::from_rgb(0xF0, 0xD4, 0xCC),
    )
}

fn process_card_slot(
    process: &Process,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    let image = flavor_image(&process.flavor.image);
    let icon = process.output.icon();

    move |ui| {
        egui::Frame::NONE
            .corner_radius(6)
            .show(ui, |ui| {
                ui.set_height(155.);
                ui.set_width(105.);
                let height = 155.;
                let width = 105.;

                if let Some(image_size) = image
                    .load_and_calc_size(ui, ui.available_size())
                {
                    let target_size = egui::vec2(width, height);

                    // Compute aspect ratios
                    let image_aspect =
                        image_size.x / image_size.y;
                    let target_aspect =
                        target_size.x / target_size.y;

                    let draw_size =
                        if image_aspect > target_aspect {
                            egui::Vec2::new(
                                target_size.y * image_aspect,
                                target_size.y,
                            )
                        } else {
                            egui::Vec2::new(
                                target_size.x,
                                target_size.x / image_aspect,
                            )
                        };

                    let center = ui.cursor().left_top()
                        + target_size / 2.;
                    let clip_rect = egui::Rect::from_min_size(
                        ui.cursor().left_top(),
                        target_size,
                    );
                    let draw_rect =
                        egui::Rect::from_center_size(
                            center, draw_size,
                        );
                    ui.scope(|ui| {
                        ui.shrink_clip_rect(clip_rect);
                        image.paint_at(ui, draw_rect);
                    });
                }

                egui::Frame::NONE.show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(48.);
                        ui.add(icon.size(48.));
                    });
                });
            })
            .response
    }
}

struct CenteredText<'a> {
    text: String,
    image: Option<egui::Image<'a>>,
    font_size: f32,
    font_family: egui::FontFamily,
}
fn center_text<'a>(
    text: impl Into<String>,
) -> CenteredText<'a> {
    CenteredText {
        text: text.into(),
        image: None,
        font_size: 14.,
        font_family: egui::FontFamily::Proportional,
    }
}
impl<'a> CenteredText<'a> {
    pub fn image(mut self, image: egui::Image<'a>) -> Self {
        self.image = Some(image);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn family(mut self, family: egui::FontFamily) -> Self {
        self.font_family = family;
        self
    }
}
impl egui::Widget for CenteredText<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let font_id =
            egui::FontId::new(self.font_size, self.font_family);

        let galley = ui.fonts(|f| {
            f.layout_delayed_color(
                self.text,
                font_id,
                f32::INFINITY,
            )
        });
        let mut content_width = galley.size().x;
        let width = ui.available_width();

        if let Some(image) = &self.image {
            let spacing = ui.style().spacing.item_spacing.x;
            let image_width =
                image.calc_size(ui.available_size(), None).x;
            content_width += image_width + spacing;
        }

        let offset = width / 2. - content_width / 2.;
        ui.horizontal(|ui| {
            ui.add_space(offset);
            if let Some(image) = self.image {
                ui.add(image);
            }
            ui.label(galley);
        })
        .response
    }
}
