use crate::{inputs, parts};
use hes_engine::Industry;

pub fn industries(
    ui: &mut egui::Ui,
    items: &mut Vec<Industry>,
) -> parts::ListResponse {
    parts::editable_list(ui, items, |ui, item| {
        industry_view(ui, item)
    })
}

fn industry_view(
    ui: &mut egui::Ui,
    industry: &mut Industry,
) -> egui::Response {
    ui.vertical(|ui| {
        ui.add(inputs::heading(&mut industry.name));

        parts::space(ui);

        parts::two_columns(ui, |ui| {
            ui.add(inputs::edit(&mut industry.flavor.image));
        }, |ui| {
            ui.add(inputs::edit(&mut industry.byproducts)
                .label("Byproducts")
                .help("Byproducts produced, per low-income-capita (LIC) per year."));
            parts::space(ui);
            ui.add(inputs::edit(&mut industry.resources)
                .label("Resources")
                .help("Resources used, per low-income-capita (LIC) per year."));
                ui.add(parts::help("Note that an industry's *direct* emissions (including due to land use) should be represented as byproducts, but for many industries their principle byproducts are due to energy use, which should be represented as fuel/electricity resource use."));
        });

        parts::space(ui);

        ui.add(
            inputs::textarea(&mut industry.flavor.description)
            .label("Description")
            .help("Describe the industry."),
        );

        parts::space(ui);

        ui.add(
            inputs::textarea(&mut industry.notes)
            .label("Notes")
            .help("Optional notes"),
        );
    }).response
}
