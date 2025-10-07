use std::{collections::BTreeMap, sync::Arc};

use eframe::glow;
use egui::{Color32, Image, Margin, Sense, vec2};
use egui_taffy::TuiBuilderLogic;
use enum_map::EnumMap;
use hes_engine::{
    IconEvent,
    Id,
    KindMap,
    Output,
    Region,
    State,
};
use rust_i18n::t;
use strum::IntoEnumIterator;

use crate::{
    display::{
        self,
        AsText,
        HasIcon,
        icons,
        intensity::{self, intensity_bar},
    },
    image,
    parts::{
        RaisedFrame,
        flavor_image,
        h_center,
        raised_frame,
        set_full_bg_image,
    },
    tips::{Tip, add_tip, tip},
    views::globe::GlobeView,
};

pub struct Regions {
    selected_region: usize,
    globe_view: GlobeView,
}
impl Regions {
    pub fn new(glow_ctx: Arc<glow::Context>) -> Self {
        Regions {
            selected_region: 0,
            globe_view: GlobeView::new(glow_ctx, 280),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &State,
        region_events: &BTreeMap<Id, Vec<IconEvent>>,
    ) {
        set_full_bg_image(
            ui,
            image!("backgrounds/regions.png"),
            egui::vec2(1600., 1192.),
        );

        h_center(ui, "region-globe", |tui| {
            tui.ui(|ui| {
                inset_frame().show(ui, |ui| {
                    ui.add(&mut self.globe_view);
                });
            });
        });

        // TODO
        // let on_globe_click = move |region_idx| {
        //     set_selected_region.set(region_idx);
        // };
        //
        // let on_globe_ready = move |globe: GlobeRef| {
        //     globe.rotate(false);
        //     globe.set_zoom(0.15);
        //     globe.clouds(false);
        //     set_globe.set(Some(globe));
        //     center_on_region();
        // };

        let region =
            state.world.regions.by_idx(self.selected_region);
        let n_regions = state.world.regions.len();

        h_center(ui, "region-name", |tui| {
            tui.ui(|ui| {
                ui.horizontal_centered(|ui| {
                    if button_frame()
                        .show(ui, |ui| {
                            ui.add(icons::ARROW_LEFT.size(16.));
                        })
                        .interact(Sense::click())
                        .clicked()
                    {
                        if self.selected_region <= 0 {
                            self.selected_region =
                                n_regions - 1;
                        } else {
                            self.selected_region -= 1;
                        }
                        self.center_on_region();
                    }

                    inset_frame().show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&region.name)
                                .heading()
                                .size(18.)
                                .color(Color32::WHITE),
                        );
                    });

                    if button_frame()
                        .show(ui, |ui| {
                            ui.add(
                                icons::ARROW_RIGHT.size(16.),
                            );
                        })
                        .interact(Sense::click())
                        .clicked()
                    {
                        self.selected_region += 1;
                        if self.selected_region >= n_regions {
                            self.selected_region = 0;
                        }
                        self.center_on_region();
                    }
                });
            });
        });

        render_region_item(ui, region, state, region_events);
    }

    fn center_on_region(&mut self) {
        // TODO
        // if let Some(globe) = globe.get_untracked() {
        //     let name = region_name.get_untracked();
        //     globe.highlight_region(&name);
        // }
    }
}

fn temp_tip() -> Tip {
    tip(
        icons::TEMPERATURE,
        t!("This region's current temperature range."),
    )
}

fn precip_tip() -> Tip {
    tip(
        icons::PRECIPITATION,
        t!("This region's current precipitation range."),
    )
}

fn devel_tip() -> Tip {
    tip(
        icons::DEVELOPMENT,
        t!("This region's progress to the next income level."),
    )
}

fn cont_tip() -> Tip {
    tip(
        icons::CONTENTEDNESS,
        t!("This region's contentedness."),
    )
}

fn hab_tip() -> Tip {
    tip(
        icons::HABITABILITY,
        t!(
            "This region's habitability. Natural disasters and hotter temperatures lower habitability."
        ),
    )
}

fn inc_tip(income: &str) -> Tip {
    tip(
        icons::WEALTH,
        t!(
            "This region has %{incomeName} living standards. Higher living standards mean higher material footprints.",
            incomeName = income
        ),
    )
}

fn demand_tip(
    output: &Output,
    demand: f32,
    percent: String,
) -> Tip {
    let demand = if demand < 1. {
        "<1".to_string()
    } else {
        demand.to_string()
    };
    let icon = output.icon();
    let msg = t!(
        "This region's per-capita demand level for %{output}. The total regions's demand is %{demand}[i]%{icon}[/i]. This makes up %{percent}% of total demand for %{output}.",
        output = t!(output.lower()),
        icon = icon,
        demand = demand,
        percent = percent
    );
    tip(icon, msg)
}

fn render_region_item(
    ui: &mut egui::Ui,
    region: &Region,
    state: &State,
    region_events: &BTreeMap<Id, Vec<IconEvent>>,
) {
    let events = region_events.get(&region.id);

    let contentedness = intensity::scale(
        region.outlook,
        intensity::Variable::Outlook,
    );
    let habitability = intensity::scale(
        region.habitability(),
        intensity::Variable::Habitability,
    );
    let income_tip = {
        let name = t!(region.income.lower());
        inc_tip(&name)
    };

    let income_level = region.income.level() + 1;
    let temp_range = region.temp_range();
    let precip_range = region.precip_range();

    h_center(ui, "region-details", |tui| {
        tui.ui(|ui| {
            ui.horizontal(|ui| {
                inset_frame().show(ui, |ui| {
                    ui.vertical(|ui| {
                        let image =
                            flavor_image(&region.flavor.image);
                        ui.add(image.fit_to_exact_size(
                            egui::vec2(320., 200.),
                        ));

                        ui.style_mut()
                            .visuals
                            .override_text_color =
                            Some(Color32::WHITE);

                        if region.seceded {
                            ui.label(t!("Seceded"));
                        }

                        ui.horizontal(|ui| {
                            add_tip(
                                temp_tip(),
                                ui.horizontal_centered(|ui| {
                                    ui.add(
                                        icons::TEMPERATURE
                                            .size(12.),
                                    );
                                    ui.label(temp_range);
                                })
                                .response,
                            );

                            add_tip(
                                precip_tip(),
                                ui.horizontal_centered(|ui| {
                                    ui.add(
                                        icons::PRECIPITATION
                                            .size(12.),
                                    );
                                    ui.label(precip_range);
                                })
                                .response,
                            );
                        });

                        let is_max_level =
                            region.is_max_income();
                        let development = region.development;
                        add_tip(
                            devel_tip(),
                            ui.horizontal_centered(|ui| {
                                ui.label(format!(
                                    "{}: ",
                                    t!("Development Progress")
                                ));
                                render_devel_bar(
                                    ui,
                                    is_max_level,
                                    development,
                                );
                            })
                            .response,
                        );

                        ui.label(t!("Recent Disasters"));
                        ui.horizontal_centered(|ui| {
                            if let Some(events) = events {
                                for ev in events {
                                    let icon = ev.icon.clone(); // TODO
                                    // ui.image(); // TODO
                                }
                            }
                        });
                    });
                });

                inset_frame().show(ui, |ui| {
                    ui.vertical(|ui| {
                        add_tip(
                            hab_tip(),
                            ui.horizontal(|ui| {
                                ui.add(
                                    icons::HABITABILITY
                                        .size(18.),
                                );
                                ui.add(
                                    intensity_bar(habitability)
                                        .invert()
                                        .pips(4),
                                );
                            })
                            .response,
                        );

                        add_tip(
                            cont_tip(),
                            ui.horizontal(|ui| {
                                ui.add(
                                    icons::CONTENTEDNESS
                                        .size(18.),
                                );
                                ui.add(
                                    intensity_bar(
                                        contentedness,
                                    )
                                    .invert()
                                    .pips(4),
                                );
                            })
                            .response,
                        );

                        add_tip(
                            income_tip,
                            ui.horizontal(|ui| {
                                ui.add(icons::WEALTH.size(18.));
                                ui.add(
                                    intensity_bar(income_level)
                                        .invert()
                                        .pips(4),
                                );
                            })
                            .response,
                        );

                        let output_demand =
                            &state.world.per_capita_demand;
                        let demand_for_outputs: EnumMap<
                            Output,
                            f32,
                        > = Output::iter()
                            .map(|output| {
                                (
                                    output,
                                    state
                                        .output_demand
                                        .of(output),
                                )
                            })
                            .collect();

                        for (k, demand) in region
                            .demand(&output_demand)
                            .items()
                        {
                            let per_capita_demand =
                                demand / region.population;
                            let int =
                                intensity::output_intensity(
                                    per_capita_demand,
                                    k,
                                );
                            let per = display::demand_percent(
                                demand,
                                demand_for_outputs[k],
                                true,
                            );
                            let amount =
                                display::output(demand, k);

                            let tip =
                                demand_tip(&k, amount, per);
                            add_tip(
                                tip,
                                ui.horizontal(|ui| {
                                    ui.add(k.icon().size(18.));
                                    ui.add(
                                        intensity_bar(int)
                                            .pips(4),
                                    );
                                })
                                .response,
                            );
                        }
                    });
                });
            });
        });
    });
}

fn render_devel_bar(
    ui: &mut egui::Ui,
    is_max_level: bool,
    development: f32,
) {
    // <div class:max-level=is_max_level> // TODO
    if is_max_level {
        ui.label(t!("Max Level"));
    } else {
        fill_bar(ui, (80., 9.), development);
    }
}

fn fill_bar(
    ui: &mut egui::Ui,
    (width, height): (f32, f32),
    filled: f32,
) {
    let (rect, _) = ui.allocate_exact_size(
        vec2(width, height),
        Sense::empty(),
    );
    let painter = ui.painter();
    painter.rect_filled(rect, 2, Color32::WHITE);

    let mut inner = rect.shrink(1.);
    inner.set_width(inner.width() * filled);
    painter.rect_filled(
        inner,
        2,
        Color32::from_rgb(0x2F, 0xE8, 0x63),
    );
}

fn inset_frame() -> RaisedFrame {
    raised_frame().colors(
        Color32::from_rgb(0x1a, 0x29, 0x1e),
        Color32::from_rgb(0x99, 0xc2, 0xa4),
        Color32::from_rgb(0x30, 0x44, 0x36),
    )
}

fn button_frame() -> RaisedFrame {
    raised_frame()
        .colors(
            Color32::from_rgb(0xC4, 0xD4, 0xC9),
            Color32::from_rgb(0x59, 0x69, 0x5E),
            Color32::from_rgb(0xB3, 0xD2, 0xBC),
        )
        .hover(Color32::from_rgb(0x97, 0xd1, 0xa7))
        .margin(Margin::symmetric(12, 4))
}
