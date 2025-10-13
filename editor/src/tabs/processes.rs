use crate::{inputs, parts};
use hes_engine::{
    Collection,
    Feedstock,
    NPC,
    Process,
    ProcessFeature,
};

impl inputs::Describe for ProcessFeature {
    fn describe(&self) -> &'static str {
        match self {
            Self::UsesPesticides => {
                "For agriculture; does the process use a significant amount of pesticides"
            }
            Self::UsesSynFertilizer => {
                "For agriculture; does the process use a significant amount of synthetic fertilizer"
            }
            Self::UsesLivestock => {
                "For agriculture; does the process use a significant amount of livestock"
            }
            Self::IsIntermittent => {
                "For electricity sources; if the supply is intermittent"
            }
            Self::MakesNuclearWaste => {
                "For electricity sources, if the supply produces nuclear waste"
            }
            Self::CanMeltdown => {
                "For electricity sources, if the supply has a meltdown risk"
            }
            Self::IsSolar => {
                "If the process depends on sunlight"
            }
            Self::IsCCS => {
                "Whether this process produces CO2 that is then stored/transported/used"
            }
            Self::IsCombustion => {
                "If this process depends on combustion"
            }
            Self::IsFossil => {
                "If this process uses fossil fuels"
            }
            Self::UsesOil => "If this process uses oil",
            Self::IsLaborIntensive => {
                "If this process is especially labor intensive"
            }
        }
    }
}

fn units(feedstock: &Feedstock) -> &'static str {
    match feedstock {
        Feedstock::Oil | Feedstock::NaturalGas => "liters (L)",
        Feedstock::Thorium
        | Feedstock::Uranium
        | Feedstock::Lithium
        | Feedstock::Coal => "grams (g)",
        Feedstock::Soil | Feedstock::Other => "(n/a)",
    }
}

pub fn processes(
    ui: &mut egui::Ui,
    items: &mut Vec<Process>,
    npcs: &Collection<NPC>,
) -> parts::ListResponse {
    parts::editable_list(ui, items, |ui, item| {
        process_view(ui, item, npcs)
    })
}

fn process_view(
    ui: &mut egui::Ui,
    process: &mut Process,
    npcs: &Collection<NPC>,
) -> egui::Response {
    ui.vertical(|ui| {
        ui.add(inputs::heading(&mut process.name));

        parts::space(ui);

        parts::two_columns(ui, |ui| {
            ui.add(inputs::edit(&mut process.flavor.image));

            parts::space(ui);

            ui.add(inputs::lock(&mut process.locked).label("Locked").help("If this process is locked at the start.").inline());
        }, |ui| {
            ui.add(
                inputs::edit(&mut process.output)
                .label("Output Type")
                .help("What this process produces.").inline(),
            );

            parts::space(ui);

            ui.add(
                inputs::edit(&mut process.feedstock.0)
                .label("Feedstock Type")
                .help(r#"What this feedstock this process requires. If no particular feedstock, just set to "Other". Note that "Soil" is ignored."#).inline(),
            );

            if process.feedstock.0 != Feedstock::Other {
                ui.add(inputs::nonneg_float(&mut process.feedstock.1).label("Feedstock").help(
                        format!("Feedstock required per unit output, in {} of {}.", units(&process.feedstock.0), process.feedstock.0.to_string())
                ));
            }

            parts::space(ui);

            ui.add(inputs::edit(&mut process.byproducts).label("Byproducts").help("Byproducts produced, per unit output."));

            parts::space(ui);

            ui.add(inputs::edit(&mut process.resources).label("Resources").help("Resources used, per unit output."));
        });

        parts::space(ui);

        parts::two_columns(ui, |ui| {
            ui.add(inputs::edit(&mut process.mix_share).label("Mix Share").help("What percent of total output production this process represents at the start. Note that 1 mix share = 5% of total output.").inline());
        }, |ui| {
            ui.add(inputs::edit(&mut process.limit).label("Output Limit").help("(Optional) This process can never produce more than this much output, effectively setting a limit on its mix share. This may be because, for example, of a finite availability, e.g. with geothermal.").inline());
        });

        parts::space(ui);

        ui.add(
            inputs::edit(&mut process.features)
            .label("Features")
            .help("Special properties associated with this process."),
        );

        parts::space(ui);

        parts::two_columns(ui, |ui| {
            ui.add(
                inputs::edit((&mut process.supporters, npcs))
                .label("Supporters")
                .help("NPCs that support this process."),
            );
        }, |ui| {
            ui.add(
                inputs::edit((&mut process.opposers, npcs))
                .label("Opposers")
                .help("NPCs that oppose this process."),
            );
        });

        parts::space(ui);

        ui.add(
            inputs::textarea(&mut process.flavor.description)
            .label("Description")
            .help("Describe the process."),
        );

        parts::space(ui);

        ui.add(
            inputs::textarea(&mut process.notes)
            .label("Notes")
            .help("Optional notes"),
        );
    }).response
}
