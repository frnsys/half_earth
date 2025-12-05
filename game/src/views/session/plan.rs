use std::{borrow::Cow, collections::BTreeMap};

use egui::{Align2, Color32, CornerRadius, Margin, Order, Rect, Sense, Shadow, Stroke};
use egui_taffy::TuiBuilderLogic;
use enum_map::EnumMap;
use hes_engine::{
    EventPhase, Feedstock, Id, KindMap, Output, Process, Project, ProjectType, Resource, State,
    Status,
};
use hes_images::flavor_image;
use rust_i18n::t;
use strum::IntoEnumIterator;

use crate::{
    consts,
    debug::DEBUG,
    display::{
        self, AsText, FloatExt, HasIcon, Icon, factors::factors_card, group_color, icons, resource,
        to_energy_units,
    },
    parts::{
        RaisedFrame, bg_cover_image, button, center_text, fill_bar, get_sizing, h_center, new_icon,
        raised_frame, set_full_bg_image,
    },
    state::{GameState, PlanChange, Points, StateExt, Tutorial},
    text::bbcode,
    tips::{Tip, add_card, add_tip, tip},
    vars::Var,
    views::{
        cards::draw_mix_cell,
        scanner::Cards,
        session::{TabItem, render_tabs},
    },
};

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
        let projects = &state.world.projects;
        let processes = &state.world.processes;
        let any_new_projects = projects.unlocked().any(|p| !viewed.contains(&p.id));
        let any_new_processes = processes.unlocked().any(|p| !viewed.contains(&p.id));

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
                        self.set_page(Page::Projects(Projects::new(state, plan_changes)));
                    }

                    for p in top {
                        match p {
                            Some(proj) => {
                                add_card((*proj).clone(), ui.add(project_card_slot(proj)));
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
                                add_card((*proj).clone(), ui.add(project_card_slot(proj)));
                            }
                            None => {
                                ui.add(empty_card_slot());
                            }
                        }
                    }

                    if n_active > slots {
                        let resp = ui.add(view_all_slot()).interact(Sense::click());
                        if resp.clicked() {
                            self.set_page(Page::All);
                        }
                    }
                });
            });
        });

        // next section (production)
        let prod_shortages = production_shortages(state);
        let inp_shortages = input_shortages(state);

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

        ui.add_space(48.);

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

                    render_resource_status(
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
                            self.set_page(Page::Processes(Processes::new(state)));
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

        let slots = calc_slots(ui);

        h_center(ui, "full-plan", |tui| {
            tui.ui(|ui| {
                for chunk in active_projects.chunks(slots) {
                    ui.horizontal(|ui| {
                        for project in chunk {
                            add_card((*project).clone(), ui.add(project_card_slot(project)));
                        }
                    });
                }
            });
        });

        // ui.horizontal(|ui| {
        //     let resp = ui
        //         .add(add_cards_slot(false, false))
        //         .interact(Sense::click());
        //     if resp.clicked() {
        //         self.set_page(Page::Projects(Projects::new(state, plan_changes)));
        //     }
        // });
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

fn input_shortages(state: &State) -> Option<String> {
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

fn production_shortages(state: &State) -> Option<String> {
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

fn render_resource_status(ui: &mut egui::Ui, state: &State, shortages_tip: Option<Tip>) {
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
                Resource::Electricity | Resource::Fuel => to_energy_units(resources.available[k]),
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

enum ProcessesAction {
    Changed,
    Back,
}

fn get_processes(state: &State, output: Output) -> Vec<Process> {
    let show_all = DEBUG.show_all_processes;
    let mut processes = state
        .world
        .processes
        .iter()
        .filter(|p| (!p.locked || show_all) && p.output == output)
        .cloned()
        .collect::<Vec<_>>();
    processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    processes
}

struct Processes {
    points: usize,
    output: Output,
    cards: Cards<Process>,
}
impl Processes {
    fn new(state: &State) -> Self {
        let output = Output::Electricity;
        Self {
            points: 0,
            output,
            cards: Cards::new(get_processes(state, output).into_iter()),
        }
    }

    fn set_output(&mut self, state: &State, output: Output) {
        self.output = output;
        self.cards = Cards::new(get_processes(state, output).into_iter());
    }

    fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> Option<ProcessesAction> {
        let mut ret_action = None;
        let allow_back = self.points == 0;

        let has_changes = state.ui.has_process_mix_changes(self.output);

        let changing_points = {
            let total = state.ui.process_mix_changes[self.output]
                .values()
                .map(|change| change.abs())
                .sum::<isize>() as f32;
            (total / 2.).ceil()
        };
        let changes_time = changing_points / consts::PROCESS_POINTS_PER_CYCLE as f32;

        let mut tabs: Vec<_> = Output::iter()
            .map(|output| TabItem {
                tab: Some(output),
                selected: self.output == output,
                tutorial: None,
                label: match output {
                    Output::Fuel => t!("Fuel"),
                    Output::Electricity => t!("Electricity"),
                    Output::PlantCalories => t!("Crops"),
                    Output::AnimalCalories => t!("Livestock"),
                }
                .to_string(),
                icon: Some(output.icon()),
                disabled: !allow_back,
            })
            .collect();
        tabs.push(TabItem {
            tab: None,
            selected: false,
            tutorial: Some(Tutorial::ProcessesBack),
            label: t!("Back").to_string(),
            icon: None,
            disabled: !allow_back,
        });
        if let Some(tab) = render_tabs(ui, &state.ui.tutorial, &tabs) {
            match tab {
                Some(output) => self.set_output(&state.core, *output),
                None => ret_action = Some(ProcessesAction::Back),
            }
        }

        if !allow_back {
            egui::Area::new("process-back-warning".into())
                .order(egui::Order::Foreground)
                .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 24. + 4.))
                .movable(false)
                .show(ui.ctx(), |ui| {
                    egui::Frame::NONE
                        .fill(Color32::from_black_alpha(224))
                        .corner_radius(4)
                        .inner_margin(Margin::symmetric(6, 6))
                        .show(ui, |ui| {
                            ui.style_mut().wrap_mode =
                                Some(egui::TextWrapMode::Extend);
                            ui.label(egui::RichText::new(t!("Drag a card up to assign leftover production")).size(12.));
                            let tip = tip(
                                icons::MIX_TOKEN,
                                t!(
                                    "One production point represents 5% of an entire production sector's productive capacity."
                                ),
                            );
                            add_tip(
                                tip,
                                ui.horizontal(|ui| {
                                    ui.style_mut().spacing.item_spacing.x = 2.;
                                    let h = ui.available_width() / 2.;
                                    ui.add_space(h - ((self.points as f32) * 10.)/2.);
                                    for _ in 0..self.points {
                                        draw_mix_cell(
                                            ui,
                                            Color32::from_rgb(0x1B, 0x97, 0xF3),
                                        );
                                    }
                                })
                                .response,
                            );
                        });
                });
        }

        let changed = self.cards.render(ui, state);
        self.points = state.ui.process_points.max(0) as usize;
        if changed && has_changes && self.points == 0 {
            ret_action = Some(ProcessesAction::Changed);
        }

        egui::Area::new("processes-info".into())
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0., -18.))
            .movable(false)
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    if has_changes {
                        let changes_time = changes_time.ceil() as usize;
                        let change_notice = {
                            let ext =
                                if changes_time > 1 { "s" } else { "" };
                            t!(
                                "These changes will take %{changesTime} planning cycle%{ext} to take effect.",
                                changesTime = changes_time,
                                ext = ext
                            )
                        };

                        let processes = &state.world.processes;

                        let estimated_changes = estimate_changes(
                            state,
                            &state.ui.process_mix_changes,
                            processes,
                        );

                        let tip = tip(
                            icons::PROJECT,
                            change_notice.to_string(),
                        ).card(estimated_changes.clone());
                        let resp = ui.horizontal(|ui| {

                            estimated_changes.render_compact(ui);

                            let sizing = get_sizing(ui);
                            egui::Frame::NONE
                                .fill(Color32::from_rgb(0xB9, 0xF8, 0x0D))
                                .corner_radius(3)
                                .inner_margin(6. * sizing.scale)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.style_mut().spacing.item_spacing.x = 4.;
                                        ui.add(icons::TIME.size(sizing.normal));
                                        ui.label(egui::RichText::new(changes_time.to_string()).size(sizing.normal + 1.).color(Color32::BLACK));
                                    });
                                });

                        }).response;
                        add_tip(tip, resp);
                    }

                    ui.horizontal(|ui| {
                        for (output, demand) in
                            display::outputs(&state.output_demand.total()).items()
                            {
                                let tip = tip(
                                    output.icon(),
                                    t!(
                                        "Global demand for %{output}.",
                                        output = output.lower()
                                    ),
                                )
                                    .card(factors_card(
                                            None,
                                            output.into(),
                                            state,
                                    ));
                                add_tip(tip, number_box(ui, demand.to_string(), output.icon()));
                            }

                        {
                            let emissions =
                                state.byproducts.total().gtco2eq().round_to(1);
                            let tip = tip(
                                icons::EMISSIONS,
                                t!("Current annual emissions, in gigatonnes of CO2 equivalent."),
                            )
                                .card(factors_card(None, Var::Emissions, state));
                            add_tip(tip, number_box(ui, emissions.to_string(), icons::EMISSIONS));
                        }
                    });
                });
            });
        ret_action
    }
}

fn number_box(ui: &mut egui::Ui, label: String, icon: Icon) -> egui::Response {
    let sizing = get_sizing(ui);
    egui::Frame::NONE
        .fill(Color32::from_gray(20))
        .corner_radius(3)
        .inner_margin(6. * sizing.scale)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing.x = 2.;
                ui.label(egui::RichText::new(label).size(sizing.normal));
                ui.add(icon.size(sizing.normal + 1.));
            });
        })
        .response
}

enum ProjectsAction {
    ChangeTo(ProjectType),
    Changed,
    Back,
}

struct Projects {
    kind: ProjectType,
    cards: Cards<Project>,
}
impl Projects {
    fn new(state: &State, plan_changes: &BTreeMap<Id, PlanChange>) -> Self {
        let kind = ProjectType::Research;
        Self {
            kind,
            cards: Cards::new(get_projects(state, &kind, plan_changes).into_iter()),
        }
    }

    fn set_kind(
        &mut self,
        state: &State,
        kind: ProjectType,
        plan_changes: &BTreeMap<Id, PlanChange>,
    ) {
        self.kind = kind;
        self.cards = Cards::new(get_projects(state, &kind, plan_changes).into_iter());
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
            .anchor(Align2::CENTER_BOTTOM, egui::vec2(0., -12.))
            .order(Order::Foreground)
            .show(ui.ctx(), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                egui::Frame::NONE
                    .fill(Color32::from_black_alpha(200))
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
    kind: &ProjectType,
    plan_changes: &BTreeMap<Id, PlanChange>,
) -> Vec<Project> {
    let projects = &state.world.projects;
    let project_lockers = &state.world.project_lockers;
    let show_all_projects = DEBUG.show_all_projects;
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
                        !matches!(projects[locker_id].status, Status::Building | Status::Active | Status::Finished)
                    }).unwrap_or(true)
                })
                .cloned()
                .collect::<Vec<_>>();
    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    projects
}

const SLOT_HEIGHT: f32 = 155.;
const SLOT_WIDTH: f32 = 105.;

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

fn calc_change(
    key: &str,
    icon: Icon,
    before: f32,
    after: f32,
    short: bool,
) -> Option<Box<dyn FnOnce(&mut egui::Ui) -> egui::Response>> {
    let mut change = if before == 0. {
        if after > 0. {
            1.
        } else if after < 0. {
            -1.
        } else {
            0.
        }
    } else {
        (after - before) / before
    };
    if before < 0. {
        change *= -1.;
    }

    if change > 0.0 {
        if short {
            let change = display::percent(change, true);
            Some(Box::new(move |ui: &mut egui::Ui| {
                ui.style_mut().visuals.override_text_color =
                    Some(Color32::from_rgb(0xEF, 0x38, 0x38));
                let label = format!("↑{change}%");
                number_box(ui, label, icon)
            }))
        } else {
            let s = t!(
                "increase [i]%{icon}[/i] %{k} by %{warn}%{change}%",
                k = key,
                warn = if change > 100. { "⚠️" } else { "" },
                change = display::percent(change, true),
                icon = icon,
            );
            Some(Box::new(move |ui: &mut egui::Ui| {
                ui.style_mut().visuals.override_text_color =
                    Some(Color32::from_rgb(0xEF, 0x38, 0x38));
                ui.add(bbcode(&s))
            }))
        }
    } else if change < 0.0 {
        if short {
            let change = display::percent(change.abs(), true);
            Some(Box::new(move |ui: &mut egui::Ui| {
                ui.style_mut().visuals.override_text_color =
                    Some(Color32::from_rgb(0x2F, 0xE8, 0x63));
                let label = format!("↓{change}%");
                number_box(ui, label, icon)
            }))
        } else {
            let s = t!(
                "decrease [i]%{icon}[/i] %{k} by %{change}%",
                k = key,
                change = display::percent(change.abs(), true),
                icon = icon,
            );
            Some(Box::new(move |ui: &mut egui::Ui| {
                ui.style_mut().visuals.override_text_color =
                    Some(Color32::from_rgb(0x2F, 0xE8, 0x63));
                ui.add(bbcode(&s))
            }))
        }
    } else {
        None
    }
}

#[derive(Default, Clone)]
struct Usage {
    emissions: f32,
    energy_use: f32,
    land_use: f32,
    water_use: f32,
    extinction_rate: f32,
}

fn estimate_changes(
    state: &State,
    mix_changes: &EnumMap<Output, BTreeMap<Id, isize>>,
    processes: &[Process],
) -> Changes {
    // Total demand for each of these
    let before = Usage {
        emissions: state.emissions.as_gtco2eq(),
        energy_use: state.output_demand.total().energy(),
        land_use: state.resource_demand.of(Resource::Land),
        water_use: state.resource_demand.of(Resource::Water),
        extinction_rate: state.world.extinction_rate,
    };

    // Demand for each of these just from the current set of processes
    let mut current = Usage::default();
    let available_land = state.world.starting_resources.land;
    for process in processes {
        let mix_share = process.mix_share as f32;
        let total = mix_share / 20. * state.output_demand.of(process.output);
        current.land_use += process.resources.land * total;
        current.water_use += process.resources.water * total;
        current.energy_use += process.resources.energy() * total;
        current.emissions += process.byproducts.gtco2eq() * total;
        current.extinction_rate += process.extinction_rate(available_land) * total;
    }

    // Changed demand for each of these, just for the current set of processes
    let mut changed = Usage::default();
    for process in processes {
        let mix_share = process.mix_share as f32
            + (*mix_changes[process.output].get(&process.id).unwrap_or(&0)) as f32;
        let total = mix_share / 20. * state.output_demand.of(process.output);
        changed.land_use += process.resources.land * total;
        changed.water_use += process.resources.water * total;
        changed.energy_use += process.resources.energy() * total;
        changed.emissions += process.byproducts.gtco2eq() * total;
        changed.extinction_rate += process.extinction_rate(available_land) * total;
    }

    // Changed overall/total/global demand for each of these
    // Subtract out previous process demand, then add in changed process demand
    let after = Usage {
        land_use: before.land_use - current.land_use + changed.land_use,
        water_use: before.water_use - current.water_use + changed.water_use,
        energy_use: before.energy_use - current.energy_use + changed.energy_use,
        emissions: before.emissions - current.emissions + changed.emissions,
        extinction_rate: before.extinction_rate - current.extinction_rate + changed.extinction_rate,
    };

    Changes { before, after }
}

#[derive(Clone)]
pub struct Changes {
    before: Usage,
    after: Usage,
}
impl Changes {
    fn render_compact(&self, ui: &mut egui::Ui) {
        let descs = [
            calc_change(
                &t!("land use"),
                icons::LAND,
                self.before.land_use,
                self.after.land_use,
                true,
            ),
            calc_change(
                &t!("water use"),
                icons::WATER,
                self.before.water_use,
                self.after.water_use,
                true,
            ),
            calc_change(
                &t!("energy use"),
                icons::ENERGY,
                self.before.energy_use,
                self.after.energy_use,
                true,
            ),
            calc_change(
                &t!("emissions"),
                icons::EMISSIONS,
                self.before.emissions,
                self.after.emissions,
                true,
            ),
            calc_change(
                &t!("the extinction rate"),
                icons::EXTINCTION_RATE,
                self.before.extinction_rate,
                self.after.extinction_rate,
                true,
            ),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        if !descs.is_empty() {
            for desc in descs {
                ui.add(desc);
            }
        }
    }
}
impl egui::Widget for &Changes {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let descs = [
            calc_change(
                &t!("land use"),
                icons::LAND,
                self.before.land_use,
                self.after.land_use,
                false,
            ),
            calc_change(
                &t!("water use"),
                icons::WATER,
                self.before.water_use,
                self.after.water_use,
                false,
            ),
            calc_change(
                &t!("energy use"),
                icons::ENERGY,
                self.before.energy_use,
                self.after.energy_use,
                false,
            ),
            calc_change(
                &t!("emissions"),
                icons::EMISSIONS,
                self.before.emissions,
                self.after.emissions,
                false,
            ),
            calc_change(
                &t!("the extinction rate"),
                icons::EXTINCTION_RATE,
                self.before.extinction_rate,
                self.after.extinction_rate,
                false,
            ),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        if descs.is_empty() {
            ui.label(t!("They won't have much effect."))
        } else {
            ui.vertical(|ui| {
                ui.set_width(320.);
                ui.label(t!("This output's production will"));
                for desc in descs {
                    ui.add(desc);
                }
            })
            .response
        }
    }
}
