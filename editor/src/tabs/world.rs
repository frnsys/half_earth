use egui_extras::{Column, TableBuilder};
use hes_engine::{Output, World};
use strum::IntoEnumIterator;

use crate::{inputs, parts};

pub fn world(ui: &mut egui::Ui, world: &mut World) {
    ui.label("Initial Values");
    ui.add(
        inputs::edit(&mut world.base_outlook)
            .label("Contentedness")
            .help("The starting world contentedness."),
    );
    ui.add(
        inputs::edit(&mut world.extinction_rate)
            .label("Extinction Pressure")
            .help("The starting extinction pressure."),
    );
    ui.add(
        inputs::edit(&mut world.temperature)
            .label("Warming")
            .help("The starting temperature anomaly (C)."),
    );
    ui.add(
        inputs::edit(&mut world.sea_level_rise)
            .label("Sea Level Rise")
            .help("The starting sea level rise (meters)."),
    );

    ui.add(
        inputs::edit(&mut world.starting_resources)
            .label("Starting Resources")
            .help("The starting resource availability."),
    );

    ui.add(
        inputs::edit(&mut world.feedstock_reserves)
            .label("Feedstock Reserves")
            .help("The starting feedstock reserves."),
    );

    ui.label("Annual Population Growth Coefficients");
    ui.add(parts::help("The coefficients for cubic annual population growth model, with one for each income level."));

    const ROW_HEIGHT: f32 = 18.;
    TableBuilder::new(ui)
        .id_salt("pop-coefs")
        .columns(Column::auto(), 5)
        .header(ROW_HEIGHT, |mut header| {
            header.col(|_| {});
            header.col(|ui| {
                ui.label("β₀");
            });
            header.col(|ui| {
                ui.label("β₁");
            });
            header.col(|ui| {
                ui.label("β₂");
            });
            header.col(|ui| {
                ui.label("β₃");
            });
        })
        .body(|mut body| {
            for (i, label) in
                ["Low", "Lower-Middle", "Upper-Middle", "High"]
                    .iter()
                    .enumerate()
            {
                body.row(ROW_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.label(*label);
                    });
                    for coef in &mut world.income_pop_coefs[i] {
                        row.col(|ui| {
                            ui.add(inputs::edit(coef));
                        });
                    }
                });
            }
        });

    ui.label("Per-Capita Demand/Intensity By Income");
    ui.add(parts::help("The per-capita demand for outputs and resources for each income level."));
    TableBuilder::new(ui)
        .id_salt("pop-intensity")
        .columns(Column::auto(), 7)
        .header(ROW_HEIGHT, |mut header| {
            header.col(|_| {});
            for output in Output::iter() {
                let units = match output {
                    Output::Fuel | Output::Electricity => {
                        "kWh/month"
                    }
                    Output::PlantCalories
                    | Output::AnimalCalories => "kcals/year",
                };
                header.col(|ui| {
                    let label: &'static str = output.into();
                    ui.label(label).on_hover_text(format!(
                        "Per-capita demand for {} by income level, in {}.",
                        label, units
                    ));
                });
            }
            header.col(|ui| {
                ui.label("Water").on_hover_text("Per-capita municipal/household water demand by income level, in L/month.");
            });
            header.col(|ui| {
                ui.label("Material").on_hover_text("Per-capita material intensity by income level in metric tons/year, though the units are less important as these values are used for scaling.");
            });
        })
        .body(|mut body| {
            for (i, label) in
                ["Low", "Lower-Middle", "Upper-Middle", "High"]
                    .iter()
                    .enumerate()
            {
                body.row(ROW_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.label(*label);
                    });
                    for output in Output::iter() {
                        row.col(|ui| {
                            let demand = &mut world.per_capita_demand[i].base[output];
                            ui.add(inputs::edit(demand));
                        });
                    }
                    row.col(|ui| {
                        let demand = &mut world.water_by_income[i];
                        ui.add(inputs::edit(demand));
                    });
                    row.col(|ui| {
                        let demand = &mut world.materials_by_income[i];
                        ui.add(inputs::edit(demand));
                    });
                });
            }
        });

    ui.label("Regions");
    ui.add(parts::help("Parameters for the world's regions."));
    TableBuilder::new(ui)
        .id_salt("regions")
        .columns(Column::auto(), 4)
        .header(ROW_HEIGHT, |mut header| {
            header.col(|_| {});
            header.col(|ui| {
                ui.label("Population").on_hover_text("The region's starting population.");
            });
            header.col(|ui| {
                ui.label("Development").on_hover_text("The region's starting progress to the next income level, from 0.0 to 1.0.");
            });
            header.col(|ui| {
                ui.label("Income Level").on_hover_text("The region's starting income level.");
            });
        })
        .body(|mut body| {
            for region in world.regions.iter_mut()
            {
                body.row(ROW_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.label(&region.name);
                    });
                    row.col(|ui| {
                        ui.add(inputs::edit(&mut region.population));
                    });
                    row.col(|ui| {
                        ui.add(inputs::share(&mut region.development));
                    });
                    row.col(|ui| {
                        ui.add(inputs::edit(&mut region.income));
                    });
                });
            }
        });
}
