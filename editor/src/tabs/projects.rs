use crate::{
    inputs::{self, CostKind},
    parts,
};
use hes_engine::*;

pub fn projects(
    ui: &mut egui::Ui,
    items: &mut Vec<Project>,
    processes: &Collection<Process>,
    projects: &Collection<Project>,
    industries: &Collection<Industry>,
    events: &Collection<Event>,
    npcs: &Collection<NPC>,
) -> parts::ListResponse {
    parts::editable_list(ui, items, |ui, item| {
        project_view(
            ui, item, processes, projects, industries, events,
            npcs,
        )
    })
}

fn project_view(
    ui: &mut egui::Ui,
    project: &mut Project,
    processes: &Collection<Process>,
    projects: &Collection<Project>,
    industries: &Collection<Industry>,
    events: &Collection<Event>,
    npcs: &Collection<NPC>,
) -> egui::Response {
    ui.vertical(|ui| {
        ui.add(inputs::heading(&mut project.name));

        parts::space(ui);

        parts::two_columns(ui, |ui| {
            ui.add(inputs::edit(&mut project.flavor.image));

            parts::space(ui);

            ui.add(
                inputs::lock(&mut project.locked)
                .label("Locked")
                .help(
                    "If this project is locked at the start.",
                ).inline(),
            );
        }, |ui| {
            ui.add(inputs::toggle_enum(&mut project.kind).label("Type").help("The type of project.").inline());

            parts::space(ui);

            ui.add(inputs::edit(&mut project.group).label("Category").help("The project's category.").inline());

            parts::space(ui);

            ui.add(inputs::toggle(&mut project.ongoing, "Ongoing", "One-Shot").label("Upkeep").help("Is this a one-and-done project, or does it need continued maintenance?").inline());

            parts::space(ui);

            if project.kind == ProjectType::Initiative {
                ui.add(inputs::toggle(&mut project.gradual, "Gradual", "On Completion").label("Activation").help("Does this project have to be 100% finished before the effects occur, or do they develop as the project is developed?").inline());

                parts::space(ui);
            }

            match &mut project.base_cost {
                Cost::Fixed(cost) => {
                    let label = match project.kind {
                        ProjectType::Policy => "Political Capital",
                        ProjectType::Research
                            | ProjectType::Initiative => "Build Years",
                    };
                    ui.add(inputs::edit(cost).label(label).help("A fixed project cost.").inline());
                },
                Cost::Dynamic(mult, factor) => {
                    ui.add(inputs::edit(factor).label("Factor").help("The factor to use for computing the cost.").inline());
                    ui.add(inputs::edit(mult).label("Factor Multiplier").help("The project's cost equals this value multiplied by the factor's value.").inline());
                },
            }

            parts::space(ui);

            let mut cost_kind = match &project.base_cost {
                Cost::Fixed(_) => CostKind::Fixed,
                Cost::Dynamic(_, _) => CostKind::Dynamic,
            };
            let prev = cost_kind.clone();
            ui.add(inputs::toggle_enum(&mut cost_kind).label("Cost Kind").help("If this project uses a dynamically-calculated cost.").inline());
            if cost_kind != prev {
                ui.memory_mut(|mem| {
                    let existing: &mut Cost = mem.data.get_temp_mut_or_insert_with(project.id.to_string().into(), || {
                        match cost_kind {
                            CostKind::Fixed => Cost::Fixed(10),
                            CostKind::Dynamic => Cost::Dynamic(0.01, Factor::Income),
                        }
                    });
                    std::mem::swap(existing, &mut project.base_cost);
                });
            }
        });

        parts::space(ui);

        parts::two_columns(ui, |ui| {
            ui.add(
                inputs::edit((&mut project.supporters, npcs))
                .label("Supporters")
                .help("NPCs that support this project."),
            );
        }, |ui| {
            ui.add(
                inputs::edit((&mut project.opposers, npcs))
                .label("Opposers")
                .help("NPCs that oppose this project."),
            );
        });

        parts::space(ui);
        parts::space(ui);

        ui.add(inputs::edit((
                    &mut project.effects,
                    processes,
                    projects,
                    industries,
                    events,
                    npcs,
        )));

        parts::space(ui);
        parts::space(ui);

        ui.add(inputs::edit((
                    &mut project.upgrades,
                    processes,
                    projects,
                    industries,
                    events,
                    npcs,
        )));

        parts::space(ui);
        parts::space(ui);

        ui.add(inputs::edit((
                    &mut project.outcomes,
                    processes,
                    projects,
                    industries,
                    events,
                    npcs,
        )));

        parts::space(ui);
        parts::space(ui);

        ui.add(
            inputs::textarea(&mut project.flavor.description)
            .label("Description")
            .help("Describe the project."),
        );

        parts::space(ui);

        ui.add(
            inputs::textarea(&mut project.notes)
            .label("Notes")
            .help("Optional notes"),
        );
    }).response
}
