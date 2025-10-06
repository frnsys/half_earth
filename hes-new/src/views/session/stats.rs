use std::{borrow::Cow, collections::BTreeMap};

use egui::{Color32, Margin};
use egui_taffy::TuiBuilderLogic;
use enum_map::EnumMap;
use hes_engine::{Id, Output, Process, Resource, State};
use numfmt::{Formatter, Precision, Scales};
use rust_i18n::t;
use strum::IntoEnumIterator;

use crate::{
    display::{
        self,
        AsText,
        DisplayValue,
        HasIcon,
        Icon,
        factors::factors_card,
        icons,
        intensity,
    },
    image,
    parts::{h_center, raised_frame, set_full_bg_image},
    state::{FACTORS, StateExt},
    tips::{Tip, add_tip, tip},
    vars::Var,
    views::factors::render_factors_list,
};

use super::treemap::{TreeItem, treemap};

impl Var {
    pub fn color(&self) -> [u32; 2] {
        match self {
            Var::Land => [0xB7FF7A, 0x0E681F],
            Var::Water => [0x7DE1EF, 0x4560FF],
            Var::Energy => [0xFDCE4C, 0xE81224],
            Var::Emissions => [0xF2F7E2, 0x6CB30B],
            Var::Biodiversity => [0xEA8BCF, 0x6865F8],
            Var::Electricity => [0xFFFF1A, 0xFF8C1A],
            Var::Fuel => [0xF7F6C7, 0xD3753F],
            Var::AnimalCalories => [0xF8AD72, 0xCA5704],
            Var::PlantCalories => [0xB1EF8F, 0x06CA9B],
            Var::Contentedness => [0x000000, 0xFFFFFF],
        }
    }
}

pub struct Stats {
    breakdown_factor: Var,
    show_breakdown_menu: bool,
}
impl Stats {
    pub fn new() -> Self {
        Self {
            breakdown_factor: Var::Land,
            show_breakdown_menu: false,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &State,
        process_mix_changes: &EnumMap<
            Output,
            BTreeMap<Id, isize>,
        >,
    ) {
        set_full_bg_image(
            ui,
            image!("backgrounds/dashboard.png"),
            egui::vec2(1600., 1192.),
        );

        let demand_for_outputs: EnumMap<Output, f32> =
            Output::iter()
                .map(|output| {
                    (output, state.output_demand.of(output))
                })
                .collect();

        let process_changes = state
            .world
            .processes
            .iter()
            .filter(|p| !p.locked)
            .filter_map(move |p| {
                let mix_change = (*process_mix_changes
                    [p.output]
                    .get(&p.id)
                    .unwrap_or(&0))
                    as f32
                    * 0.05;
                if mix_change != 0. {
                    let change = mix_change
                        * demand_for_outputs[p.output];
                    Some((p.clone(), change))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        ui.vertical_centered(|ui| {
            ui.add_space(32.);
            ui.set_max_width(720.);

            h_center(ui, "stats-top".into(), |tui| {
                tui.ui(|ui| {
                    stat(
                        ui,
                        |ui| render_temp(ui, state),
                        t!("Temp. Anomaly"),
                    );
                });
                tui.ui(|ui| {
                    render_emissions(
                        ui,
                        state,
                        &process_changes,
                    );
                });
                tui.ui(|ui| {
                    render_land(ui, state, &process_changes);
                });
                tui.ui(|ui| {
                    render_energy(ui, state, &process_changes);
                });
                tui.ui(|ui| {
                    render_water(ui, state, &process_changes);
                });
            });

            ui.add_space(32.);

            h_center(ui, "stats-bottom".into(), |tui| {
                tui.ui(|ui| {
                    render_biodiversity(
                        ui,
                        state,
                        &process_changes,
                    );
                });
                tui.ui(|ui| {
                    stat(
                        ui,
                        |ui| render_sea_level_rise(ui, state),
                        t!("Sea Level Rise"),
                    );
                });
                tui.ui(|ui| {
                    stat(
                        ui,
                        |ui| render_population(ui, state),
                        t!("Population"),
                    );
                });
                tui.ui(|ui| {
                    stat(
                        ui,
                        |ui| render_income(ui, state),
                        t!("Avg. Living Standards"),
                    );
                });
                tui.ui(|ui| {
                    stat(
                        ui,
                        |ui| render_habitability(ui, state),
                        t!("Avg. Habitability"),
                    );
                });
            });

            ui.add_space(32.);

            render_breakdown(ui, state, self.breakdown_factor);

            ui.add_space(64.);
        });

        if self.show_breakdown_menu {
            if let Some(factor) = render_breakdown_menu(ui) {
                self.breakdown_factor = factor;
                self.show_breakdown_menu = false;
            }
        }
    }
}

fn stat(
    ui: &mut egui::Ui,
    inner: impl FnOnce(&mut egui::Ui),
    label: Cow<'static, str>,
) -> egui::Response {
    ui.vertical_centered(|ui| {
        ui.set_width(120.);
        raised_frame()
            .colors(
                Color32::from_rgb(0x96, 0x8a, 0x68),
                Color32::from_rgb(0xEF, 0xE5, 0xD2),
                Color32::from_rgb(0xEB, 0xDE, 0xC6),
            )
            .margin(Margin {
                left: 8,
                right: 8,
                top: -12,
                bottom: 18,
            })
            .show(ui, |ui| {
                ui.set_width(80.);
                inner(ui);
            });
        ui.label(label);
    })
    .response
}

fn render_temp(ui: &mut egui::Ui, state: &State) {
    let temp_anomaly = state.temp_anomaly();
    ui.vertical_centered(|ui| {
        ui.add(icons::WARMING.size(24.));
        ui.add_space(8.);
        ui.label(
            egui::RichText::new(temp_anomaly)
                .monospace()
                .size(24.),
        );
    });
}

fn render_emissions(
    ui: &mut egui::Ui,
    state: &State,
    process_changes: &[(Process, f32)],
) {
    let emissions_change = process_changes
        .into_iter()
        .map(|(p, mult)| p.adj_byproducts().gtco2eq() * mult)
        .sum::<f32>()
        .round();

    let tip = {
        let tip_text = t!(
            "Current annual emissions, in gigatonnes of CO2 equivalent."
        );
        tip(icons::EMISSIONS, tip_text).card(factors_card(
            None,
            Var::Emissions,
            state,
        ))
    };
    let emissions = state.emissions.as_gtco2eq();
    let emissions_display = state.emissions.display();
    let emissions_changed =
        display::emissions(emissions_change + emissions);

    render_dashboard_item(
        ui,
        &t!("Emissions"),
        None,
        &emissions_display,
        &emissions_changed,
        tip,
        emissions_change,
        icons::EMISSIONS,
    );
}

fn render_land(
    ui: &mut egui::Ui,
    state: &State,
    process_changes: &[(Process, f32)],
) {
    let available_land = state.world.starting_resources.land;
    let tip = tip(icons::LAND, t!("Current land use."))
        .card(factors_card(None, Var::Land, state));
    let land_use = state.land_use_percent();
    let land_demand = state.resource_demand.of(Resource::Land);

    let land_change = process_changes
        .into_iter()
        .map(|(p, mult)| p.adj_resources().land * mult)
        .sum::<f32>()
        .round();
    let land_changed = format!(
        "{:.0}%",
        display::land_use_percent(
            land_change + land_demand,
            available_land
        )
    );

    render_dashboard_item(
        ui,
        &t!("Land Use"),
        None,
        &land_use,
        &land_changed,
        tip,
        land_change,
        icons::LAND,
    );
}

fn render_energy(
    ui: &mut egui::Ui,
    state: &State,
    process_changes: &[(Process, f32)],
) {
    let energy_change = process_changes
        .into_iter()
        .map(|(p, mult)| {
            let energy = p.adj_resources().energy();
            energy * mult
        })
        .sum::<f32>()
        .round();

    let tip = tip(icons::ENERGY, t!("Current energy use."))
        .card(factors_card(None, Var::Energy, state));
    let energy_use = state.energy_pwh();
    let energy_demand = state.output_demand.total().energy();
    let energy_changed = format!(
        "{}PWh",
        (display::pwh(energy_change + energy_demand)).round()
    );

    render_dashboard_item(
        ui,
        &t!("Energy Use"),
        None,
        &energy_use,
        &energy_changed,
        tip,
        energy_change,
        icons::ENERGY,
    );
}

fn render_water(
    ui: &mut egui::Ui,
    state: &State,
    process_changes: &[(Process, f32)],
) {
    let available_water = state.resources.available.water;
    let water_change = process_changes
        .into_iter()
        .map(|(p, mult)| p.adj_resources().water * mult)
        .sum::<f32>()
        .round();

    let water_demand =
        state.resource_demand.of(Resource::Water);
    let current_water_stress = state.water_use_percent();
    let after_water_stress = format!(
        "{:.0}%",
        display::water_use_percent(
            water_change + water_demand,
            available_water
        )
    );

    let tip = tip(icons::WATER, t!("Current water demand."))
        .card(factors_card(None, Var::Water, state));

    render_dashboard_item(
        ui,
        &t!("Water Stress"),
        None,
        &current_water_stress,
        &after_water_stress,
        tip,
        water_change,
        icons::WATER,
    );
}

fn render_biodiversity(
    ui: &mut egui::Ui,
    state: &State,
    process_changes: &[(Process, f32)],
) {
    let extinction = |amount: f32| {
        let int = intensity::scale(
            amount,
            intensity::Variable::Extinction,
        );
        MiniCardData {
            label: intensity::describe(int),
            color: intensity::color(int, false),
        }
    };

    let available_land = state.world.starting_resources.land;
    let extinction_change = process_changes
        .into_iter()
        .map(|(p, mult)| {
            p.extinction_rate(available_land) * mult
        })
        .sum::<f32>()
        .round();
    let extinction_rate = state.world.extinction_rate;
    let current = extinction(extinction_rate);
    let after_extinction =
        extinction(extinction_rate + extinction_change).label;

    let tip_text = t!(
        "The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse."
    );
    let tip = tip(icons::EXTINCTION_RATE, tip_text)
        .card(factors_card(None, Var::Biodiversity, state));
    render_dashboard_item(
        ui,
        &t!("Extinction Rate"),
        Some(current.color),
        &current.label,
        &after_extinction,
        tip,
        extinction_change,
        icons::EXTINCTION_RATE,
    );
}

fn render_sea_level_rise(ui: &mut egui::Ui, state: &State) {
    let sea_level_rise = state.world.sea_level_rise;
    let sea_level_rise_rate = state.world.sea_level_rise_rate();
    let rise = format!("{:.2}", sea_level_rise);
    let tip_text = t!(
        "Average sea levels have risen by %{rise}m and are rising at a rate of %{rate}mm per year.",
        rise = rise,
        rate = format!("{:.1}", sea_level_rise_rate * 1000.)
    );
    let tip: Tip = tip(icons::SEA_LEVEL_RISE, tip_text);

    add_tip(
        tip,
        ui.vertical_centered(|ui| {
            ui.add(icons::SEA_LEVEL_RISE.size(24.));
            ui.add_space(8.);
            ui.label(format!("{rise} m"));
        })
        .response,
    );
}

fn render_population(ui: &mut egui::Ui, state: &State) {
    let population = state.world.regions.population();
    let mut f = Formatter::default()
        .scales(Scales::short())
        .precision(Precision::Decimals(1));
    let pop_fmted = f.fmt2(population as f64).to_string();

    ui.vertical_centered(|ui| {
        ui.add(icons::POPULATION.size(24.));
        ui.add_space(8.);
        ui.label(pop_fmted);
    });
}

fn render_income(ui: &mut egui::Ui, state: &State) {
    let income = state.avg_income_level();
    let income = MiniCardData {
        label: intensity::describe(income - 1),
        color: intensity::color(income, true),
    };

    ui.vertical_centered(|ui| {
        ui.add(icons::WEALTH.size(24.));
        // <span style:color=income.color>{&income.label}</span> TODO
        ui.add_space(8.);
        ui.label(income.label);
    });
}

fn render_habitability(ui: &mut egui::Ui, state: &State) {
    let habitability =
        state.world.regions.habitability().max(0.);
    let habitability = {
        let int = intensity::scale(
            habitability,
            intensity::Variable::Habitability,
        );
        MiniCardData {
            label: intensity::describe(habitability as usize),
            color: intensity::color(int, true),
        }
    };

    ui.vertical_centered(|ui| {
        ui.add(icons::HABITABILITY.size(24.));
        // <span style:color=habitability
        //     .color>{&habitability.label}</span> TODO
        ui.add_space(8.);
        ui.label(habitability.label);
    });
}

fn render_breakdown_menu(ui: &mut egui::Ui) -> Option<Var> {
    let mut selected = None;
    for var in Var::iter() {
        let button = egui::Button::image_and_text(
            var.icon(),
            t!(var.title()),
        );
        let resp = ui.add(button);
        if resp.clicked() {
            selected = Some(var);
        }
    }
    selected
}

fn render_breakdown(
    ui: &mut egui::Ui,
    state: &State,
    factor: Var,
) {
    let available_land = state.world.starting_resources.land;

    let dataset = {
        let mut total = 0.;
        let mut data: BTreeMap<String, f32> =
            BTreeMap::default();
        let factors = FACTORS.read();
        for fac in &factors[factor] {
            let name = t!(&fac.name());
            data.insert(name.to_string(), fac.amount());
            total += fac.amount();
        }
        if factor == Var::Land {
            let name = t!("Unused");
            let unused = available_land - total;
            data.insert(name.to_string(), unused);
        }
        data
    };

    let table_data = factors_card(None, factor, state);
    let button = egui::Button::image_and_text(
        factor.icon(),
        format!("{}▼", t!(factor.title())),
    );
    ui.add(button);

    render_chart(ui, &dataset, factor.color());
    render_factors_list(ui, table_data);

    ui.label(t!("Only direct impacts are shown."));
}

struct MiniCardData {
    label: Cow<'static, str>,
    color: Color32,
}

fn render_dashboard_item(
    ui: &mut egui::Ui,
    label: &str,
    color: Option<Color32>,
    display_value: &str,
    display_changed_value: &str,
    item_tip: Tip,
    change: f32,
    icon: Icon,
) {
    add_tip(
        item_tip,
        stat(
            ui,
            |ui| {
                ui.add(icon.size(24.));
                ui.add_space(8.);
                let text = egui::RichText::new(display_value)
                    .monospace()
                    .size(24.);
                if let Some(color) = color {
                    ui.colored_label(color, text);
                } else {
                    ui.label(text);
                }
                if change != 0. {
                    let change_tip = tip(
                        icon,
                        t!(
                            "The estimated value after production changes have finished."
                        ),
                    );
                    add_tip(
                        change_tip,
                        ui.horizontal_centered(|ui| {
                            ui.image(icons::DOWN_ARROW_SMALL);
                            ui.label(display_changed_value);
                        })
                        .response,
                    );
                }
            },
            Cow::Owned(label.to_string()),
        ),
    );
}

// TODO
fn render_chart(
    ui: &mut egui::Ui,
    dataset: &BTreeMap<String, f32>,
    colors: [u32; 2],
) {
    let n = dataset.len() as f32;
    let items: Vec<_> = dataset
        .iter()
        .enumerate()
        .map(|(i, (k, v))| {
            let (r, g, b) =
                lerp_color(colors[0], colors[1], i as f32 / n);
            TreeItem {
                label: k,
                value: *v,
                color: Color32::from_rgb(r, g, b),
            }
        })
        .collect();

    treemap(ui, "breakdown-chart", (320., 200.), items);
}

fn lerp_color(from: u32, to: u32, ratio: f32) -> (u8, u8, u8) {
    let ar = ((from & 0xFF0000) >> 16) as f32;
    let ag = ((from & 0x00FF00) >> 8) as f32;
    let ab = (from & 0x0000FF) as f32;

    let br = ((to & 0xFF0000) >> 16) as f32;
    let bg = ((to & 0x00FF00) >> 8) as f32;
    let bb = (to & 0x0000FF) as f32;

    let rr = ar + ratio * (br - ar);
    let rg = ag + ratio * (bg - ag);
    let rb = ab + ratio * (bb - ab);

    (rr.round() as u8, rg.round() as u8, rb.round() as u8)
}
