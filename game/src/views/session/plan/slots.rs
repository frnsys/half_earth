use std::collections::BTreeMap;

use egui::{Color32, CornerRadius, Margin, Rect, Sense, Shadow, Stroke};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{Id, Output, Process, Project, State};
use hes_images::flavor_image;
use rust_i18n::t;
use strum::IntoEnumIterator;

use crate::{
    display::{AsText, HasIcon, group_color, icons},
    parts::{
        RaisedFrame, bg_cover_image, button, center_text, fill_bar, get_sizing, h_center, new_icon,
        raised_frame,
    },
    state::{PlanChange, Tutorial},
    tips::{add_editable_project_card, add_tip, tip},
};

use super::{Page, Processes, Projects};

const SLOT_HEIGHT: f32 = 155.;
const SLOT_WIDTH: f32 = 105.;

pub fn projects_preview(
    ui: &mut egui::Ui,
    state: &State,
    tutorial: &Tutorial,
    viewed: &[Id],
    plan_changes: &BTreeMap<Id, PlanChange>,
) -> Option<Page> {
    let mut page = None;

    let projects = &state.world.projects;
    let any_new_projects = projects.unlocked().any(|p| !viewed.contains(&p.id));
    let projects_highlighted = tutorial.eq(&Tutorial::Projects);

    let slots = calc_slots(ui);

    let active_projects = projects.part_of_plan().collect::<Vec<_>>();
    let n_active = active_projects.len();
    let n_projects = {
        if n_active > slots {
            // Save one spot for "View All"
            slots - 1
        } else {
            n_active
        }
    };

    let placeholders = (slots as isize - active_projects.len() as isize).max(0) as usize;

    let split_at = slots / 2;

    let items: Vec<_> = active_projects
        .into_iter()
        .take(n_projects)
        .map(Some)
        .chain((0..placeholders).map(|_| None))
        .collect();
    let top = &items[0..split_at];
    let bot = &items[split_at..];

    ui.add_space(48.);

    h_center(ui, "plan-preview", |tui| {
        tui.ui(|ui| {
            ui.horizontal(|ui| {
                let resp = ui
                    .add(add_cards_slot(any_new_projects, projects_highlighted))
                    .interact(Sense::click());
                if resp.clicked() {
                    page = Some(Page::Projects(Projects::new(state, plan_changes)));
                }

                for p in top {
                    match p {
                        Some(proj) => {
                            add_editable_project_card(
                                (*proj).clone(),
                                ui.add(project_card_slot(proj)),
                            );
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
                            add_editable_project_card(
                                (*proj).clone(),
                                ui.add(project_card_slot(proj)),
                            );
                        }
                        None => {
                            ui.add(empty_card_slot());
                        }
                    }
                }

                if n_active > slots {
                    let resp = ui.add(view_all_slot()).interact(Sense::click());
                    if resp.clicked() {
                        page = Some(Page::All);
                    }
                }
            });
        });
    });

    page
}

pub fn processes_preview(
    ui: &mut egui::Ui,
    state: &State,
    tutorial: &Tutorial,
    viewed: &[Id],
    plan_changes: &BTreeMap<Id, PlanChange>,
) -> Option<Page> {
    let mut page = None;

    let processes = &state.world.processes;
    let any_new_processes = processes.unlocked().any(|p| !viewed.contains(&p.id));

    let prod_shortages = prod_status::production_shortages(state);
    let inp_shortages = prod_status::input_shortages(state);

    let shortages_tip = tip(
        icons::ALERT,
        format!(
            "{}. {}",
            prod_shortages.clone().unwrap_or_default(),
            inp_shortages.unwrap_or_default()
        ),
    );

    let max_processes = Output::iter().map(|output| {
        processes
            .iter()
            .filter(|p| p.output == output)
            .max_by_key(|p| p.mix_share)
            .unwrap()
    });
    let output_demand = state.output_demand.total();

    h_center(ui, "plan-processes", |tui| {
        tui.ui(|ui| {
                let resp = inset_frame().show(ui, |ui| {
                    ui.style_mut().visuals.override_text_color = Some(Color32::BLACK);
                    ui.horizontal(|ui| {
                        for process in max_processes {
                            let produced = crate::display::output(
                                state.produced.of(process.output),
                                process.output,
                            );

                            let demand = crate::display::output(
                                output_demand[process.output],
                                process.output,
                            );

                            ui.vertical(|ui| {
                                let sizing = get_sizing(ui);

                                ui.set_width(105. * sizing.scale);

                                let has_shortage = produced / demand < 0.99;

                                let image = if has_shortage {
                                    icons::ALERT.size(sizing.normal)
                                } else {
                                    icons::CHECK
                                        .size(sizing.normal)
                                        .tint(Color32::from_rgb(0x1B, 0xAC, 0x89))
                                };

                                let text = center_text(format!("{:.0}/{:.0}", produced, demand))
                                    .size(sizing.normal)
                                    .image(image);

                                let resp = ui.add(text);

                                if has_shortage {
                                    add_tip(shortages_tip.clone(), resp);
                                }

                                ui.vertical_centered(|ui| {
                                    ui.add(process_card_slot(process));

                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            egui::RichText::new(t!(process.output.title()))
                                                .size(sizing.normal),
                                        );
                                    });
                                });
                            });
                        }
                    });

                    ui.add_space(16.);

                    prod_status::render_resource_status(
                        ui,
                        state,
                        if prod_shortages.is_some() {
                            Some(shortages_tip.clone())
                        } else {
                            None
                        },
                    );

                    ui.add_space(16.);
                    ui.scope(|ui| {
                        let processes_disabled = tutorial.lt(&Tutorial::Processes);
                        let processes_highlighted = tutorial.eq(&Tutorial::Processes);
                        if processes_disabled {
                            ui.disable();
                        }
                        let b = button(t!("Change Production"))
                            .full_width()
                            .maybe_highlight(processes_highlighted);
                        if ui.add(b).clicked() {
                            page = Some(Page::Processes(Processes::new(state)));
                        }
                    });
                });
                if any_new_processes {
                    ui.add(new_icon(resp.rect));
                }

                let size = egui::vec2(48., 24.);
                let rect = egui::Rect::from_min_size(resp.rect.right_top() - egui::vec2(80., 24.), size);
                ui.place(
                    rect,
                    |ui: &mut egui::Ui| {
                        ui.horizontal(|ui| {
                            let processes_over_limit = state
                                .world
                                .processes
                                .over_limit(state.output_demand.total(), state.feedstocks.available)
                                .map(|p| t!(&p.name))
                                .collect::<Vec<_>>();
                            if !processes_over_limit.is_empty() {
                                let tip = tip(
                                    icons::ALERT,
                                    t!(
                                        "The following processes can't produce as much as they need to: %{processesOverLimit}",
                                        processesOverLimit = processes_over_limit.join(", ")
                                    ),
                                );
                                add_tip(tip, egui::Frame::NONE
                                    .fill(Color32::from_rgb(0xEF, 0x38, 0x38))
                                    .corner_radius(CornerRadius {
                                        nw: 3,
                                        ne: 3,
                                        ..Default::default()
                                    })
                                    .inner_margin(Margin::symmetric(6, 3))
                                    .show(ui, |ui| ui.add(icons::ALERT.size(18.))).response);
                            }


                            if prod_shortages.is_some() {
                                add_tip(shortages_tip, egui::Frame::NONE
                                    .fill(Color32::from_rgb(0xEF, 0x38, 0x38))
                                    .corner_radius(CornerRadius {
                                        nw: 3,
                                        ne: 3,
                                        ..Default::default()
                                    })
                                    .inner_margin(Margin::symmetric(6, 3))
                                    .show(ui, |ui| ui.add(icons::ALERT.size(18.))).response);
                            }
                        }).response
                    });
            });
    });

    page
}

pub fn all_projects(ui: &mut egui::Ui, active_projects: &[&Project]) {
    let slots = calc_slots(ui);

    h_center(ui, "full-plan", |tui| {
        tui.ui(|ui| {
            for chunk in active_projects.chunks(slots) {
                ui.horizontal(|ui| {
                    for project in chunk {
                        add_editable_project_card(
                            (*project).clone(),
                            ui.add(project_card_slot(project)),
                        );
                    }
                });
            }
        });
    });
}

fn calc_slots(ui: &mut egui::Ui) -> usize {
    let width = ui.ctx().content_rect().width();
    if width > 680. {
        9
    } else if width > 560. {
        7
    } else {
        5
    }
}

fn add_cards_slot(show_new: bool, highlight: bool) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    move |ui| {
        let resp = inset_frame()
            .margin(0)
            .maybe_highlight(highlight)
            .show(ui, |ui| {
                ui.set_height(SLOT_HEIGHT);
                ui.set_width(SLOT_WIDTH - 1.); // account for inset shadow
                ui.vertical_centered(|ui| {
                    ui.add_space(54.);
                    ui.add(icons::ADD.size(32.));
                    ui.colored_label(Color32::BLACK, t!("Add"));
                });
            });

        if show_new {
            ui.add(new_icon(resp.rect));
        }

        resp
    }
}

fn view_all_slot() -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    move |ui| {
        inset_frame().margin(0).show(ui, |ui| {
            ui.set_height(SLOT_HEIGHT);
            ui.set_width(SLOT_WIDTH - 1.); // account for inset shadow
            ui.vertical_centered(|ui| {
                ui.add_space(68.);
                ui.colored_label(Color32::BLACK, t!("View All"));
            });
        })
    }
}

fn empty_card_slot() -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    |ui| {
        egui::Frame::NONE
            .stroke(Stroke::new(1., Color32::from_black_alpha(48)))
            .corner_radius(6)
            .show(ui, |ui| {
                ui.set_height(SLOT_HEIGHT);
                ui.set_width(SLOT_WIDTH);
            })
            .response
    }
}

fn project_card_slot(project: &Project) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
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
                ui.set_height(SLOT_HEIGHT - 6.);
                ui.set_width(SLOT_WIDTH - 8.);
                let height = SLOT_HEIGHT - 6.;
                let width = SLOT_WIDTH - 8.;

                let target_size = egui::vec2(width, height);
                let center = ui.cursor().left_top() + target_size / 2.;
                let target_rect = Rect::from_center_size(center, target_size);
                bg_cover_image(ui, image, target_rect);

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
                                ui.add(fill_bar((72., 8.), progress));
                            }
                        });
                    });
            })
            .response
    }
}

fn inset_frame() -> RaisedFrame {
    raised_frame().colors(
        Color32::from_rgb(0x91, 0x7e, 0x7e),
        Color32::from_rgb(0xF5, 0xE8, 0xD7),
        Color32::from_rgb(0xF0, 0xD4, 0xCC),
    )
}

fn process_card_slot(process: &Process) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    let image = flavor_image(&process.flavor.image);
    let icon = process.output.icon();

    move |ui| {
        let sizing = get_sizing(ui);

        egui::Frame::NONE
            .corner_radius(6)
            .show(ui, |ui| {
                ui.set_height(SLOT_HEIGHT * sizing.scale);
                ui.set_width(SLOT_WIDTH * sizing.scale);
                let height = SLOT_HEIGHT * sizing.scale;
                let width = SLOT_WIDTH * sizing.scale;

                let target_size = egui::vec2(width, height);
                let center = ui.cursor().left_top() + target_size / 2.;
                let target_rect = Rect::from_center_size(center, target_size);
                bg_cover_image(ui, image, target_rect);

                egui::Frame::NONE.show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(48. * sizing.scale);
                        ui.add(icon.size(48. * sizing.scale));
                    });
                });
            })
            .response
    }
}

mod prod_status {
    use std::borrow::Cow;

    use egui::{Color32, Margin, Stroke};
    use egui_taffy::TuiBuilderLogic;
    use enum_map::EnumMap;
    use hes_engine::{Feedstock, KindMap, Output, Resource, State};
    use rust_i18n::t;
    use strum::IntoEnumIterator;

    use crate::{
        display::{AsText, HasIcon, resource, to_energy_units},
        parts::{get_sizing, h_center},
        tips::{Tip, add_tip},
    };

    pub fn input_shortages(state: &State) -> Option<String> {
        let resources = &state.resources;
        let feedstocks = &state.feedstocks;
        let resources: Vec<_> = Resource::iter()
            .filter(|res| resources.has_shortage(*res))
            .map(|r| t!(r.title()))
            .collect();

        let feedstock: Vec<_> = Feedstock::iter()
            .filter(|res| {
                feedstocks.has_shortage(*res) && *res != Feedstock::Other && *res != Feedstock::Soil
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

    pub fn production_shortages(state: &State) -> Option<String> {
        let produced = &state.produced;
        let output_demand = state.output_demand.total();

        let problems = {
            let mut problems: EnumMap<Output, f32> = EnumMap::from_array([1.; 4]);
            for output in Output::iter() {
                tracing::debug!(
                    "{output:?}: produced={}, demand={}",
                    crate::display::output(produced.of(output), output),
                    crate::display::output(output_demand[output], output)
                );
                let met = produced.of(output) / output_demand[output];
                if met >= 0.99 {
                    continue;
                } else if met < problems[output] {
                    problems[output] = met;
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
        } else if problems.len() == 1 {
            let (output, severity) = &problems[0];
            let desc = severity.desc();
            let details = t!(output.title());
            Some(format!("{desc}: {details}"))
        } else {
            let list = problems
                .into_iter()
                .map(|(output, severity)| {
                    let title = t!(output.title());
                    let label = severity.label();
                    format!("{title} ({label})")
                })
                .collect::<Vec<_>>()
                .join("\n");
            let desc = t!("There are multiple production shortages:");
            Some(format!("{desc} {list}"))
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

    pub fn render_resource_status(ui: &mut egui::Ui, state: &State, shortages_tip: Option<Tip>) {
        let resources = &state.resources;
        let protected_land = state.protected_land;
        let resource_demand = &state.resource_demand;
        let starting_resources = state.world.starting_resources;

        h_center(ui, "resource-status", |tui| {
            for (k, demand) in resource_demand.total().items() {
                let demand = match k {
                    Resource::Electricity | Resource::Fuel => to_energy_units(demand),
                    Resource::Water => resource(demand, k, resources.available),
                    Resource::Land => {
                        // For land we add in protected land as well.
                        let protected = protected_land * 100.;
                        resource(demand, k, starting_resources) + protected
                    }
                };
                let available = match k {
                    Resource::Electricity | Resource::Fuel => {
                        to_energy_units(resources.available[k])
                    }
                    Resource::Land | Resource::Water => 100.,
                };

                let not_enough = demand > available;
                tui.ui(|ui| {
                    let sizing = get_sizing(ui);
                    let resp = egui::Frame::NONE
                        .fill(if not_enough {
                            Color32::from_rgb(0xFF, 0x00, 0x00)
                        } else {
                            Color32::from_rgb(0xE4, 0xC9, 0xC2)
                        })
                        .stroke(Stroke::new(1., Color32::from_rgb(0xB8, 0xA2, 0x9C)))
                        .inner_margin(Margin::symmetric(3, 2))
                        .corner_radius(3)
                        .show(ui, |ui| {
                            ui.horizontal_centered(|ui| {
                                ui.style_mut().spacing.item_spacing.x = 2.;
                                ui.add(k.icon().size(sizing.normal));

                                let t = format!("{:.0}/{:.0}", demand, available);
                                let text = egui::RichText::new(t).size(sizing.normal);
                                if not_enough {
                                    ui.colored_label(Color32::WHITE, text);
                                } else {
                                    ui.label(text);
                                }
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
}
