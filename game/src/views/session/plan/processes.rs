use crate::{
    debug::DEBUG,
    display::{self, AsText, FloatExt, HasIcon, Icon, factors::factors_card, icons},
    parts::get_sizing,
    state::{GameState, Tutorial},
    tips::{add_tip, tip},
    vars::Var,
    views::{
        Cards,
        cards::draw_mix_cell,
        session::{TabItem, render_tabs},
    },
};

use super::grid::{CardGrid, CardGridAction};
use egui::Color32;
use hes_engine::{KindMap, Output, Process, State};
use rust_i18n::t;
use strum::IntoEnumIterator;

pub enum ProcessesAction {
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
    processes.sort_by_key(|a| a.name.to_lowercase());
    processes
}

pub struct Processes {
    points: usize,
    output: Output,
    cards: Cards<Process>,
}
impl Processes {
    pub fn new(state: &State) -> Self {
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

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> Option<ProcessesAction> {
        let mut ret_action = None;
        let allow_back = self.points == 0;

        let has_changes = state.ui.has_any_process_mix_changes();
        let changes_time = state.ui.all_process_mix_change_time();

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
                        .fill(egui::Color32::from_black_alpha(224))
                        .corner_radius(4)
                        .inner_margin(egui::Margin::symmetric(6, 6))
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
                                            egui::Color32::from_rgb(0x1B, 0x97, 0xF3),
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

                        let estimated_changes = changes::estimate_changes(
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

mod changes {
    use std::collections::BTreeMap;

    use egui::Color32;
    use enum_map::EnumMap;
    use hes_engine::{Id, Output, Process, Resource, State};
    use rust_i18n::t;

    use super::number_box;
    use crate::{
        display::{self, Icon, icons},
        text::bbcode,
    };

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

    pub fn estimate_changes(
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
            current.land_use += process.adj_resources().land * total;
            current.water_use += process.adj_resources().water * total;
            current.energy_use += process.adj_resources().energy() * total;
            current.emissions += process.adj_byproducts().gtco2eq() * total;
            current.extinction_rate += process.extinction_rate(available_land) * total;
        }

        // Changed demand for each of these, just for the current set of processes
        let mut changed = Usage::default();
        for process in processes {
            let mix_share = process.mix_share as f32
                + (*mix_changes[process.output].get(&process.id).unwrap_or(&0)) as f32;
            let total = mix_share / 20. * state.output_demand.of(process.output);
            changed.land_use += process.adj_resources().land * total;
            changed.water_use += process.adj_resources().water * total;
            changed.energy_use += process.adj_resources().energy() * total;
            changed.emissions += process.adj_byproducts().gtco2eq() * total;
            changed.extinction_rate += process.extinction_rate(available_land) * total;
        }

        // Changed overall/total/global demand for each of these
        // Subtract out previous process demand, then add in changed process demand
        let after = Usage {
            land_use: before.land_use - current.land_use + changed.land_use,
            water_use: before.water_use - current.water_use + changed.water_use,
            energy_use: before.energy_use - current.energy_use + changed.energy_use,
            emissions: before.emissions - current.emissions + changed.emissions,
            extinction_rate: before.extinction_rate - current.extinction_rate
                + changed.extinction_rate,
        };

        Changes { before, after }
    }

    #[derive(Clone)]
    pub struct Changes {
        before: Usage,
        after: Usage,
    }
    impl Changes {
        pub fn render_compact(&self, ui: &mut egui::Ui) {
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
}
pub use changes::Changes;

fn number_box(ui: &mut egui::Ui, label: String, icon: Icon) -> egui::Response {
    let sizing = get_sizing(ui);
    egui::Frame::NONE
        .fill(egui::Color32::from_gray(20))
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
