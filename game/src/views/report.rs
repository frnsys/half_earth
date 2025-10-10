use egui::Color32;
use egui_extras::{Column, TableBuilder};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{
    EventPhase,
    IconEvent,
    Income,
    NPCRequest,
    Project,
};
use rust_i18n::t;

use crate::{
    consts,
    display::{
        self,
        AsText,
        DisplayValue,
        Icon,
        factors::factors_card,
        icons,
        intensity::{self, IntensityBar, intensity_bar},
    },
    image,
    parts::{
        button,
        center_center,
        center_text,
        raised_frame,
        set_full_bg_image,
    },
    state::{GameState, StateExt},
    tips::{Tip, add_tip, tip},
    vars::Var,
    views::events::Events,
};

const ROW_HEIGHT: f32 = 18.;

pub struct Report {
    events: Events,
    changes: Vec<ChangeRow>,
    projects_finished: Vec<Project>,
    requests_fulfilled: Vec<(String, isize)>,
    seat_changes: Vec<(String, f32, f32)>,
    world_events: Vec<(String, Tip)>,
    disasters: Vec<(String, Vec<IconEvent>)>,
    region_incomes: Vec<(String, Income)>,
    honeymoon_pc: isize,
    pc_change: isize,
}
impl Report {
    pub fn new(state: &mut GameState) -> Self {
        let events = StateExt::roll_events(
            &mut state.core,
            EventPhase::ReportStart,
        );

        state.ui.session_start_state = state.core.clone();

        let changes = vec![
            temp_row(state),
            cont_row(state),
            ext_row(state),
            ghg_row(state),
        ];

        let requests = requests_rows(state);
        let honeymoon_pc = honeymoon_pc(state);
        let pc_change = changes
            .iter()
            .map(|row| row.pc_change)
            .sum::<isize>()
            + requests
                .iter()
                .map(|(_, bounty)| bounty)
                .sum::<isize>()
            + (state
                .ui
                .cycle_start_state
                .completed_projects
                .len()
                * consts::PC_PER_COMPLETED_PROJECT)
                as isize
            + honeymoon_pc;

        Self {
            events: Events::new(events, state),
            changes,
            projects_finished: projects_rows(state),
            requests_fulfilled: requests,
            seat_changes: parliament_rows(state),
            world_events: event_rows(state),
            disasters: disaster_rows(state),
            region_incomes: region_rows(state),
            honeymoon_pc,
            pc_change,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut GameState,
    ) -> bool {
        let mut done = false;
        set_full_bg_image(
            ui,
            image!("backgrounds/report.png"),
            egui::vec2(1600., 1192.),
        );

        self.events.render(ui, &mut state.core);

        center_center(ui, "report", |tui| {
            tui.ui(|ui| {
                ui.style_mut().visuals.override_text_color =
                    Some(Color32::BLACK);
                ui.add(
                    center_text(t!("Report"))
                        .family(egui::FontFamily::Name(
                            "TimesTen".into(),
                        ))
                        .size(24.),
                );
                ui.add_space(8.);

                raised_frame()
                    .colors(
                        Color32::from_rgb(0xf7, 0xf4, 0xe6),
                        Color32::from_rgb(0xc2, 0xb8, 0x93),
                        Color32::from_rgb(0xFF, 0xF7, 0xD9),
                    )
                    .show(ui, |ui| {
                        ui.set_width(360.);

                        self.render_changes(ui, state);
                        self.render_projects(ui);
                        self.render_requests(ui);
                        self.render_total_pc_change(ui);

                        self.render_seat_changes(ui);
                        self.render_world_events(ui);
                        self.render_region_incomes(ui);
                        self.render_disasters(ui);

                        if ui
                            .add(
                                button(t!("Next")).full_width(),
                            )
                            .clicked()
                        {
                            state.change_political_capital(
                                self.pc_change,
                            );

                            // Reset session plan changes
                            state.ui.plan_changes.clear();
                            state
                                .ui
                                .points
                                .refundable_research = 0;

                            done = true;
                        }
                    });
            });
        });
        done
    }

    fn render_changes(
        &self,
        ui: &mut egui::Ui,
        state: &GameState,
    ) {
        let year = state.world.year;
        let start_year = state.ui.cycle_start_state.year;

        TableBuilder::new(ui)
            .id_salt("changes")
            .column(Column::remainder())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .header(ROW_HEIGHT, |mut header| {
                header.col(|ui| {
                    ui.label(
                        egui::RichText::new(t!("Changes"))
                            .size(12.)
                            .underline(),
                    );
                });
                header.col(|ui| {
                    ui.label(
                        egui::RichText::new(
                            start_year.to_string(),
                        )
                        .size(12.),
                    );
                });
                header.col(|ui| {
                    ui.add(icons::ARROW_RIGHT.size(12.));
                });
                header.col(|ui| {
                    ui.label(
                        egui::RichText::new(year.to_string())
                            .size(12.),
                    );
                });
                header.col(|ui| {
                    ui.add(icons::POLITICAL_CAPITAL.size(12.));
                });
            })
            .body(|mut body| {
                for change in &self.changes {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            add_tip(
                                change.tip.clone(),
                                ui.horizontal(|ui| {
                                    ui.add(
                                        change.icon.size(16.),
                                    );
                                    ui.label(&change.label);
                                })
                                .response,
                            );
                        });
                        row.col(|ui| {
                            match &change.from {
                                Value::Bar(intensity_bar) => {
                                    ui.add(intensity_bar);
                                }
                                Value::Val(val) => {
                                    ui.label(val);
                                }
                            };
                        });
                        row.col(|ui| {
                            ui.add(
                                icons::ARROW_RIGHT.size(16.),
                            );
                        });
                        row.col(|ui| {
                            match &change.to {
                                Value::Bar(intensity_bar) => {
                                    ui.add(intensity_bar);
                                }
                                Value::Val(val) => {
                                    ui.label(val);
                                }
                            };
                        });
                        row.col(|ui| {
                            let pc_change = format!(
                                "{:+}",
                                change.pc_change
                            );
                            ui.label(pc_change);
                        });
                    });
                }
            });
    }

    fn render_projects(&self, ui: &mut egui::Ui) {
        if !self.projects_finished.is_empty() {
            ui.add_space(12.);

            TableBuilder::new(ui)
                .id_salt("projects")
            .column(Column::remainder())
            .column(Column::auto())
            .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new(t!(
                                    "Completed Projects"
                                ))
                                .size(12.).underline(),
                            );
                        });
                    });

                    for p in &self.projects_finished {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                let tip = tip(
                                    icons::PROJECT,
                                    t!("This project was completed."),
                                )
                                    .card(p.clone());
                                add_tip(tip,
                                    ui.label(t!(&p.name)));
                            });
                            row.col(|ui| {
                                let pc_change = format!(
                                    "{:+}",
                                    consts::PC_PER_COMPLETED_PROJECT
                                );
                                ui.label(pc_change);
                            });
                        });
                    }
            });
        }
    }

    fn render_requests(&self, ui: &mut egui::Ui) {
        if !self.requests_fulfilled.is_empty() {
            TableBuilder::new(ui)
                .id_salt("requests")
                .column(Column::remainder())
                .column(Column::auto())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new(t!(
                                    "Completed Requests"
                                ))
                                .size(12.)
                                .underline(),
                            );
                        });
                    });

                    for (name, bounty) in
                        &self.requests_fulfilled
                    {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label(name);
                            });
                            row.col(|ui| {
                                let pc_change =
                                    format!("{:+}", bounty);
                                ui.label(pc_change);
                            });
                        });
                    }
                });
        }
    }

    fn render_total_pc_change(&self, ui: &mut egui::Ui) {
        ui.add_space(12.);
        TableBuilder::new(ui)
            .id_salt("pc-total")
            .column(Column::remainder())
            .column(Column::auto())
            .body(|mut body| {
                if self.honeymoon_pc > 0 {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(t!(
                                "Post-Revolution Optimism"
                            ));
                        });
                        row.col(|ui| {
                            let pc_change = format!(
                                "{:+}",
                                self.honeymoon_pc
                            );
                            ui.label(pc_change);
                        });
                    });
                }

                body.row(ROW_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                icons::POLITICAL_CAPITAL
                                    .size(16.),
                            );
                            ui.label(t!("Total Change"));
                        });
                    });
                    row.col(|ui| {
                        let pc_change =
                            format!("{:+}", self.pc_change);
                        ui.label(pc_change);
                    });
                });
            });
    }

    fn render_seat_changes(&self, ui: &mut egui::Ui) {
        if !self.seat_changes.is_empty() {
            ui.add_space(16.);
            TableBuilder::new(ui)
                .id_salt("parliament")
                .column(Column::remainder())
                .column(Column::auto())
                .column(Column::auto())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new(t!(
                                    "Parliament"
                                ))
                                .size(12.)
                                .underline(),
                            );
                        });
                    });

                    for (name, seats, change) in
                        &self.seat_changes
                    {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label(name);
                            });
                            row.col(|ui| {
                                let change =
                                    format!("{:+}", change);
                                ui.label(change);
                            });
                            row.col(|ui| {
                                ui.label(seats.to_string());
                            });
                        });
                    }
                });
        }
    }

    fn render_world_events(&self, ui: &mut egui::Ui) {
        if !self.world_events.is_empty() {
            ui.add_space(16.);
            TableBuilder::new(ui)
                .id_salt("events")
                .column(Column::remainder())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new(t!(
                                    "Events"
                                ))
                                .size(12.)
                                .underline(),
                            );
                        });
                    });

                    for (name, tip) in &self.world_events {
                        body.row(ROW_HEIGHT, |mut row| {
                            let (_, resp) = row.col(|ui| {
                                ui.label(name);
                            });
                            add_tip(tip.clone(), resp);
                        });
                    }
                });
        }
    }

    fn render_region_incomes(&self, ui: &mut egui::Ui) {
        if !self.region_incomes.is_empty() {
            ui.add_space(16.);
            TableBuilder::new(ui)
                .id_salt("regions")
                .column(Column::remainder())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new(t!(
                                        "Regions"
                                ))
                                .size(12.).underline(),
                            );
                        });
                    });

                    for (name, income) in &self.region_incomes {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label(
                                    t!(
                                        "%{region} is now %{income} income.", region=t!(name.as_str()),
                                        income=t!(income.lower())
                                    )
                                );
                            });
                        });
                    }
                });
        }
    }

    fn render_disasters(&self, ui: &mut egui::Ui) {
        if !self.disasters.is_empty() {
            ui.add_space(16.);
            TableBuilder::new(ui)
                .id_salt("disasters")
                .column(Column::auto().at_least(140.))
                .column(Column::auto())
                .body(|mut body| {
                    body.row(ROW_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new(t!(
                                        "Disasters"
                                ))
                                .size(12.).underline(),
                            );
                        });

                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                ui.add(
                                    icons::HABITABILITY
                                    .size(16.),
                                );
                                ui.label(
                                    egui::RichText::new(t!("Reduce the habitability of regions."))
                                    .size(12.),
                                );
                            });
                        });
                    });

                    for (name, events) in &self.disasters {
                        body.row(ROW_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label(t!(name.as_str()));
                            });

                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    for ev in events {
                                        ui.add(icons::disaster_icon(&ev.icon).size(16.));
                                    }
                                });
                            });
                        });
                    }
                });
        }
    }
}

struct ChangeRow {
    icon: Icon,
    label: String,
    from: Value,
    to: Value,
    tip: Tip,
    pc_change: isize,
}
enum Value {
    Bar(IntensityBar),
    Val(String),
}

fn ext_row(state: &GameState) -> ChangeRow {
    let tip_text = t!(
        r#"The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse. [g]Your goal is to get this to below 20.[/g]"#
    );
    let ext_tip = tip(icons::EXTINCTION_RATE, tip_text)
        .card(factors_card(None, Var::Biodiversity, state));

    let exr = state.world.extinction_rate;
    let start_exr = state.ui.cycle_start_state.extinction_rate;
    let start_exr_int = intensity::scale(
        start_exr,
        intensity::Variable::Extinction,
    );
    let end_exr_int =
        intensity::scale(exr, intensity::Variable::Extinction);

    let ext_pc_change = {
        let change = start_exr - exr;
        let end = end_exr_int;
        consts::EXTINCTION_PC.get(end).unwrap_or_else(|| {
            consts::EXTINCTION_PC.last().unwrap()
        }) + (change.round() as isize * consts::BIODIVERSITY_PC)
            .max(0)
    };

    ChangeRow {
        tip: ext_tip,
        icon: icons::EXTINCTION_RATE,
        label: t!("Extinction Rate").to_string(),
        from: Value::Bar(intensity_bar(start_exr_int)),
        to: Value::Bar(intensity_bar(end_exr_int)),
        pc_change: ext_pc_change,
    }
}

fn cont_row(state: &GameState) -> ChangeRow {
    let tip_text = t!(
        r#"How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions. [w]If this goes below 0 you will be removed from power.[/w]"#
    );
    let cont_tip = tip(icons::CONTENTEDNESS, tip_text)
        .card(factors_card(None, Var::Contentedness, state));

    let outlook = state.outlook();
    let start_outlook =
        state.ui.cycle_start_state.contentedness;
    let start_cont_int = intensity::scale(
        start_outlook,
        intensity::Variable::WorldOutlook,
    );
    let end_cont_int = intensity::scale(
        outlook,
        intensity::Variable::WorldOutlook,
    );

    let cont_pc_change = {
        let end = end_cont_int;
        consts::CONTENTEDNESS_PC.get(end).unwrap_or_else(|| {
            consts::CONTENTEDNESS_PC.last().unwrap()
        })
    };

    ChangeRow {
        tip: cont_tip,
        icon: icons::CONTENTEDNESS,
        label: t!("Contentedness").to_string(),
        from: Value::Bar(
            intensity_bar(start_cont_int).invert(),
        ),
        to: Value::Bar(intensity_bar(end_cont_int).invert()),
        pc_change: *cont_pc_change,
    }
}

fn temp_row(state: &GameState) -> ChangeRow {
    let temp = state.world.temperature;
    let start_temp = state.ui.cycle_start_state.temperature;
    let temp_change = temp - start_temp;
    let temp_pc_change = {
        // Double temp change score for every degree above 1C
        let temp_change_multiplier =
            ((temp.round() - 1.).max(0.) * 2.).max(1.);

        // Temp scored for every 0.1C change
        let change = (temp_change * 10.).round()
            * -(consts::TEMPERATURE_PC as f32)
            * temp_change_multiplier;
        change as isize
    };

    let start = display::temp(start_temp);
    let end = display::temp(temp);

    let warming_tip = tip(
        icons::WARMING,
        t!(
            r#"The current global temperature anomaly. [b]Increased warming[/b] will damage your political capital. [g]Your goal is to get this below 1°C.[/g]"#
        ),
    );

    ChangeRow {
        tip: warming_tip,
        icon: icons::WARMING,
        label: t!("Temperature").to_string(),
        from: Value::Val(start),
        to: Value::Val(end),
        pc_change: temp_pc_change,
    }
}

fn ghg_row(state: &GameState) -> ChangeRow {
    let emissions_gt = state.emissions.display();
    let tip_text = t!(
        r#"Current annual emissions are %{emissions}. [g]Your goal is to get this to below 0.[/g]"#,
        emissions = emissions_gt
    );
    let emissions_tip = tip(icons::EMISSIONS, tip_text)
        .card(factors_card(None, Var::Emissions, state));

    let emissions = state.emissions.as_gtco2eq();
    let start_emissions = state.ui.cycle_start_state.emissions;
    let ghg_pc_change = {
        let emissions_change = emissions - start_emissions;
        (emissions_change * 2.).round() as isize
            * -consts::EMISSIONS_PC
    };

    let start = format!("{:+.1}", start_emissions);
    let end = format!("{:+.1}", emissions);

    ChangeRow {
        tip: emissions_tip,
        icon: icons::EMISSIONS,
        label: t!("Emissions").to_string(),
        from: Value::Val(start),
        to: Value::Val(end),
        pc_change: ghg_pc_change,
    }
}

fn honeymoon_pc(state: &GameState) -> isize {
    let year = state.world.year;
    let start_year = state.ui.cycle_start_state.year;
    if year < start_year + consts::HONEYMOON_YEARS {
        consts::HONEYMOON_PC as isize
    } else {
        0
    }
}

fn projects_rows(state: &GameState) -> Vec<Project> {
    let recent_completed_projects =
        &state.ui.cycle_start_state.completed_projects;
    recent_completed_projects
        .iter()
        .map(|project_id| {
            state.world.projects[project_id].clone()
        })
        .collect::<Vec<_>>()
}

fn requests_rows(
    state: &mut GameState,
) -> Vec<(String, isize)> {
    let finished_requests = state.check_requests();
    let projects = &state.world.projects;
    let processes = &state.world.processes;
    finished_requests.into_iter().map(|(kind, id, active, bounty)| {
            match kind {
                NPCRequest::Project => {
                    let project = &projects[&id];
                    (
                        if active {
                            t!("Completed Request: Implement %{name}", name=t!(&project.name))
                        } else {
                            t!("Completed Request: Stop %{name}", name=t!(&project.name))
                        }.to_string(),
                        bounty as isize,
                    )
                }
                NPCRequest::Process => {
                    let process = &processes[&id];
                    (
                        if active {
                            t!("Completed Request: Unban %{name}", name=t!(&process.name))
                        } else {
                            t!("Completed Request: Ban %{name}", name=t!(&process.name))
                        }.to_string(),
                        bounty as isize,
                    )
                }
            }
        }).collect::<Vec<_>>()
}

fn parliament_rows(
    state: &GameState,
) -> Vec<(String, f32, f32)> {
    let start_parliament =
        &state.ui.cycle_start_state.parliament;
    start_parliament
        .iter()
        .enumerate()
        .map(|(i, start_seats)| {
            let npc = &state.npcs.by_idx(i);
            let change = (npc.seats - start_seats).round();
            (npc.name.clone(), npc.seats, change)
        })
        .filter(|(_, _, change)| *change != 0.)
        .collect::<Vec<_>>()
}

fn event_rows(state: &GameState) -> Vec<(String, Tip)> {
    let recent_world_events = &state.ui.world_events;
    recent_world_events.iter().map(|ev| {
        (
            ev.name.clone(),
            tip(
                icons::CHANCE,
                t!("This event occurred during this planning cycle.")
            ).card(ev.clone())
        )
    }).collect::<Vec<_>>()
}

fn disaster_rows(
    state: &GameState,
) -> Vec<(String, Vec<IconEvent>)> {
    let regions = &state.world.regions;
    let region_events = &state.ui.annual_region_events;
    region_events
        .iter()
        .map(|(idx, events)| {
            let reg = regions[idx].name.clone();
            (reg, events.clone())
        })
        .collect::<Vec<_>>()
}

fn region_rows(state: &GameState) -> Vec<(String, Income)> {
    let regions = &state.world.regions;
    let start_region_incomes =
        &state.ui.cycle_start_state.region_incomes;
    regions
        .iter()
        .zip(start_region_incomes.iter())
        .filter(|(reg, inc)| reg.income != **inc)
        .map(|(reg, _)| (reg.name.clone(), reg.income))
        .collect::<Vec<_>>()
}
