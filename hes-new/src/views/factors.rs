use egui::{Align, Color32, Layout, Stroke};
use egui_taffy::TuiBuilderLogic;
use rust_i18n::t;

use crate::{
    consts,
    display::{
        self,
        HasIcon,
        Icon,
        factors::Factor,
        icons,
        intensity::intensity_bar,
    },
    parts::{flex_justified, raised_frame},
    state::FACTORS,
    vars::Var,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FactorsCard {
    pub icon: Icon,
    pub kind: Var,
    pub total: f32,
    pub current: Option<String>,
}
impl FactorsCard {
    pub fn total_formatted(&self) -> String {
        match self.kind {
            Var::Emissions => display::emissions(self.total),
            Var::Biodiversity => format!("{:.0}", self.total),
            Var::Land => {
                format!(
                    "{}%",
                    display::percent(self.total / 100., true)
                )
            }
            Var::Water => {
                format!(
                    "{}%",
                    display::percent(self.total / 100., true)
                )
            }
            Var::Energy => format!("{}", self.total.round()),
            Var::Electricity => {
                format!("{}", self.total.round())
            }
            Var::Fuel => format!("{}", self.total.round()),
            Var::PlantCalories => {
                format!("{}", self.total.round())
            }
            Var::AnimalCalories => {
                format!("{}", self.total.round())
            }
            Var::Contentedness => {
                format!("{}", self.total)
            }
        }
    }
}

pub fn render_factors_list(
    ui: &mut egui::Ui,
    factors: FactorsCard,
) {
    egui::Frame::NONE.inner_margin(8).show(ui, |ui| {
        ui.vertical(|ui| {
            ui.style_mut().wrap_mode =
                Some(egui::TextWrapMode::Extend);

            let relation = {
                let relation = match factors.kind {
                    Var::Emissions => "makes",
                    Var::Biodiversity => "causes",
                    _ => "uses",
                };
                t!(relation)
            };

            let cur_name = factors.current.as_ref();

            egui::Frame::NONE
                .inner_margin(4)
                .fill(Color32::from_rgb(0xBB, 0xB4, 0xA7))
                .stroke(Stroke::new(
                    1.,
                    Color32::from_rgb(0x81, 0x78, 0x74),
                ))
                .corner_radius(4)
                .show(ui, |ui| {
                    flex_justified(
                        ui,
                        "factors-header",
                        |tui| {
                            tui.label(format!(
                                "{} :",
                                t!("Total")
                            ));
                            tui.ui(|ui| {
                        let max_value = match factors.kind {
                            Var::Biodiversity => {
                                Some(consts::MAX_BIODIVERSITY)
                            }
                            Var::Contentedness => {
                                Some(consts::MAX_CONTENTEDNESS)
                            }
                            _ => None,
                        };
                        let total = factors.total_formatted();

                        ui.horizontal(|ui| {
                            if let Some(max_value) = max_value {
                                ui.label(format!(
                                    "{total}/{max_value}"
                                ));
                            } else {
                                ui.label(total);
                            }
                            ui.add(factors.icon.size(18.));
                        });
                    });
                        },
                    );
                });

            let ranked = FACTORS.read();
            let ranked =
                ranked[factors.kind].iter().filter(|user| {
                    match user {
                        Factor::Industry {
                            produced, ..
                        }
                        | Factor::Process {
                            produced, ..
                        } => *produced != 0.,
                        _ => true,
                    }
                });

            ui.style_mut().spacing.item_spacing.y = 0.;
            for user in ranked {
                let highlight = cur_name
                    .is_some_and(|name| name == user.name()); // TODO use to highlight user
                let name = user.name();
                egui::Frame::NONE
                    .inner_margin(4)
                    .corner_radius(4)
                    .fill(if highlight {
                        Color32::from_rgb(0xf5, 0xf9, 0xc7)
                    } else {
                        Color32::TRANSPARENT
                    })
                    .show(ui, |ui| {
                        ui.label(t!(name));
                        render_factor_line(
                            ui,
                            name,
                            user,
                            &relation,
                            factors.icon,
                        );
                    });
            }
        });
    });
}

fn render_factor_line(
    ui: &mut egui::Ui,
    name: &str,
    factor: &Factor,
    relation: &str,
    icon: Icon,
) {
    match factor {
        Factor::Region {
            intensity, display, ..
        } => {
            ui.horizontal(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.set_width(ui.available_width() / 2.);
                    ui.add(icons::WEALTH.size(18.));
                    ui.add(intensity_bar(*intensity).pips(4));
                });

                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.style_mut().spacing.item_spacing.x =
                            4.;
                        ui.set_width(ui.available_width() / 2.);
                        ui.add(icon.size(18.));
                        ui.label(display);
                    },
                );
            });
        }
        Factor::Project { display, .. } => {
            ui.horizontal(|ui| {
                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.style_mut().spacing.item_spacing.x =
                            4.;
                        ui.add(icon.size(18.));
                        ui.label(display);
                    },
                );
            });
        }
        Factor::Event {
            display, amount, ..
        } => {
            let display = display
                .clone()
                .unwrap_or_else(|| amount.to_string());
            ui.horizontal(|ui| {
                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.style_mut().spacing.item_spacing.x =
                            4.;
                        ui.add(icon.size(18.));
                        ui.label(display);
                    },
                );
            });
        }
        Factor::Process {
            intensity,
            display_produced,
            display,
            output,
            ..
        } => {
            flex_justified(ui, name, |tui| {
                tui.ui(|ui| {
                    ui.horizontal_centered(|ui| {
                        ui.add(icon.size(18.));
                        ui.add(
                            intensity_bar(*intensity).pips(4),
                        );
                    });
                });
                tui.ui(|ui| {
                    ui.style_mut().spacing.item_spacing.x = 4.;
                    ui.horizontal_centered(|ui| {
                        ui.label(display_produced);
                        ui.add(output.icon().size(18.));
                        ui.label(relation);
                        ui.label(display);
                        ui.add(icon.size(18.));
                    });
                });
            });
        }
        Factor::Industry {
            intensity, display, ..
        } => {
            ui.horizontal(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.set_width(ui.available_width() / 2.);
                    ui.add(icon.size(18.));
                    ui.add(intensity_bar(*intensity).pips(4));
                });

                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.style_mut().spacing.item_spacing.x =
                            4.;
                        ui.set_width(ui.available_width() / 2.);
                        ui.add(icon.size(18.));
                        ui.label(display);
                    },
                );
            });
        }
    }
}
