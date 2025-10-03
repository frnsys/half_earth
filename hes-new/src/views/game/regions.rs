use std::collections::BTreeMap;

use egui::{Color32, Image, Sense, vec2};
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
        icon_from_slug,
        icons,
        intensity::{self, render_intensity_bar_with_pips},
    },
    views::{Tip, tip, tips::add_tip},
};

pub struct Regions {
    selected_region: usize,
}
impl Regions {
    pub fn new() -> Self {
        Regions { selected_region: 0 }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &State,
        region_events: &BTreeMap<Id, Vec<IconEvent>>,
    ) {
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

        ui.horizontal_centered(|ui| {
            if ui
                .button(Image::new(icon_from_slug(
                    icons::ARROW_LEFT,
                )))
                .clicked()
            {
                if self.selected_region <= 0 {
                    self.selected_region = n_regions - 1;
                } else {
                    self.selected_region -= 1;
                }
                self.center_on_region();
            }

            ui.label(&region.name);

            if ui
                .button(Image::new(icon_from_slug(
                    icons::ARROW_RIGHT,
                )))
                .clicked()
            {
                self.selected_region += 1;
                if self.selected_region >= n_regions {
                    self.selected_region = 0;
                }
                self.center_on_region();
            }
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
        "This region's per-capita demand level for %{output}. The total regions's demand is %{demand}[i]%{icon}[/i]. This makes up %{percent} of total demand for %{output}.",
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

    let image = &region.flavor.image;
    // ui.image(); // TODO

    if region.seceded {
        ui.label(t!("Seceded"));
    }

    add_tip(
        temp_tip(),
        ui.horizontal_centered(|ui| {
            ui.image(icon_from_slug(icons::TEMPERATURE));
            ui.label(temp_range);
        })
        .response,
    );

    add_tip(
        precip_tip(),
        ui.horizontal_centered(|ui| {
            ui.image(icon_from_slug(icons::PRECIPITATION));
            ui.label(precip_range);
        })
        .response,
    );

    let is_max_level = region.is_max_income();
    let development = region.development;
    add_tip(
        devel_tip(),
        ui.horizontal_centered(|ui| {
            ui.label(format!(
                "{}: ",
                t!("Development Progress")
            ));
            render_devel_bar(ui, is_max_level, development);
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

    ui.vertical(|ui| {
        add_tip(
            hab_tip(),
            ui.vertical_centered(|ui| {
                ui.image(icon_from_slug(icons::HABITABILITY));
                render_intensity_bar_with_pips(
                    ui,
                    habitability,
                    true,
                    4,
                );
            })
            .response,
        );
    });

    ui.vertical(|ui| {
        add_tip(
            cont_tip(),
            ui.vertical_centered(|ui| {
                ui.image(icon_from_slug(icons::CONTENTEDNESS));
                render_intensity_bar_with_pips(
                    ui,
                    contentedness,
                    true,
                    4,
                );
            })
            .response,
        );
    });

    ui.vertical(|ui| {
        add_tip(
            income_tip,
            ui.vertical_centered(|ui| {
                ui.image(icon_from_slug(icons::WEALTH));
                render_intensity_bar_with_pips(
                    ui,
                    income_level,
                    true,
                    4,
                );
            })
            .response,
        );
    });

    let output_demand = &state.world.per_capita_demand;
    let demand_for_outputs: EnumMap<Output, f32> =
        Output::iter()
            .map(|output| {
                (output, state.output_demand.of(output))
            })
            .collect();

    for (k, demand) in region.demand(&output_demand).items() {
        let per_capita_demand = demand / region.population;
        let int =
            intensity::output_intensity(per_capita_demand, k);
        let per = display::demand_percent(
            demand,
            demand_for_outputs[k],
            true,
        );
        let amount = display::output(demand, k);

        let tip = demand_tip(&k, amount, per);
        add_tip(
            tip,
            ui.vertical_centered(|ui| {
                // ui.image(k.icon()); // TODO
                render_intensity_bar_with_pips(
                    ui, int, false, 4,
                );
            })
            .response,
        );
    }
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
    painter.rect_filled(rect, 0, Color32::BLUE);

    let mut inner = rect.shrink(1.);
    inner.set_width(inner.width() * filled);
    painter.rect_filled(inner, 0, Color32::PURPLE);
}
