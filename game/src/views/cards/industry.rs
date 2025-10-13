use crate::{
    display::{self, AsText, HasIcon, icons},
    parts::{center_text, flex_spaced},
    state::GameState,
    tips::{add_tip, tip},
    vars::Impact,
};

use super::AsCard;
use egui::{Color32, Stroke};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{Industry, KindMap};
use rust_i18n::t;

impl AsCard for Industry {
    fn id(&self) -> hes_engine::Id {
        self.id
    }

    fn bg_color(&self) -> egui::Color32 {
        egui::Color32::from_rgb(0x84, 0x7b, 0x9e)
    }

    fn fg_color(&self) -> Color32 {
        egui::Color32::WHITE
    }

    fn header(&self, ui: &mut egui::Ui, _state: &GameState) {
        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(6, 6))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(
                        t!("Sector").to_uppercase(),
                    )
                    .monospace(),
                );
            });
    }

    fn figure(&self, ui: &mut egui::Ui, _state: &GameState) {
        super::render_flavor_image(ui, &self.flavor.image);
    }

    fn name(&self, ui: &mut egui::Ui, _state: &GameState) {
        super::card_title(ui, &self.name);
    }

    fn body(&self, ui: &mut egui::Ui, state: &GameState) {
        let lic_pop = state.world.lic_population();
        let demand = self.demand(lic_pop);
        let total_resources = self.adj_resources() * demand;
        let empty = total_resources.sum() == 0.;
        let emissions =
            (self.adj_byproducts() * demand).co2eq();
        let resources_demand = state.resource_demand.total();
        let available_resources = state.resources.available;

        egui::Frame::NONE
            .outer_margin(egui::Margin {
                left: 6,
                right: 6,
                top: 0,
                bottom: 6,
            })
            .inner_margin(egui::Margin::symmetric(4, 4))
            .corner_radius(4)
            .stroke(Stroke::new(
                1.,
                Color32::from_white_alpha(24),
            ))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());

                ui.add_space(42.);

                if empty {
                    ui.add(center_text(t!("This industry is not yet significant.")));
                } else {
                    let id = format!("{}-intensities", self.id);
                    flex_spaced(ui, &id, |tui| {
                        for (key, val) in total_resources.items() {
                            let formatted = display::format_resource(val, key, available_resources);
                            let percent = display::demand_percent(
                                val,
                                resources_demand[key],
                                false,
                            );
                            let tip = tip(
                                key.icon(),
                                t!(
                                    "This industry's demand for %{output}. This makes up %{percent}% of total demand for %{output}.",
                                    output=key.lower(), percent=percent,
                                ),
                            );

                            tui.ui(|ui| {
                                add_tip(tip, ui.vertical_centered(|ui| {
                                    ui.add(key.icon().size(18.));
                                    ui.label(formatted);
                                }).response);
                            });
                        }

                        if emissions != 0. {
                            let tip = tip(icons::EMISSIONS, t!("This industry's non-energy CO2eq emissions."));
                            let formatted =
                                if emissions < 1. {
                                    "<1".to_string()
                                } else {
                                    display::format_impact(Impact::Emissions, emissions, available_resources)
                                };
                            tui.ui(|ui| {
                                add_tip(tip, ui.vertical_centered(|ui| {
                                    ui.add(icons::EMISSIONS.size(18.));
                                    ui.label(formatted);
                                }).response);
                            });
                        }
                    });
                }
            });
    }

    fn top_back(&self, ui: &mut egui::Ui, _state: &GameState) {
        super::card_desc(ui, &self.flavor.description);
    }

    fn bottom_back(
        &self,
        ui: &mut egui::Ui,
        _state: &GameState,
    ) {
        let image_attrib = &self.flavor.image.attribution;
        ui.label(format!("{} {image_attrib}", t!("Image:")));
    }
}
