use hes_engine::*;

use crate::{inputs, parts};

pub fn events(
    ui: &mut egui::Ui,
    items: &mut Vec<Event>,
    processes: &Collection<Process>,
    projects: &Collection<Project>,
    industries: &Collection<Industry>,
    events: &Collection<Event>,
    npcs: &Collection<NPC>,
) -> parts::ListResponse {
    parts::editable_list(ui, items, |ui, item| {
        event_view(
            ui, item, processes, projects, industries, events,
            npcs,
        )
    })
}

fn event_view(
    ui: &mut egui::Ui,
    event: &mut Event,
    processes: &Collection<Process>,
    projects: &Collection<Project>,
    industries: &Collection<Industry>,
    events: &Collection<Event>,
    npcs: &Collection<NPC>,
) -> egui::Response {
    egui::Frame::NONE
        .show(ui, |ui| {
            ui.add(inputs::heading(&mut event.name));

            parts::space(ui);

            parts::two_columns(
                ui,
                |ui| {
                    ui.add(inputs::edit(
                        &mut event.flavor.image,
                    ));

                    parts::space(ui);

                    ui.add(
                    inputs::lock(&mut event.locked)
                    .label("Locked")
                    .help(
                        "If this event is locked at the start.",
                    )
                    .inline(),
                    );
                },
                |ui| {
                    ui.add(
                        inputs::edit(&mut event.flavor.arc)
                        .label("Arc")
                        .help("Optional story arc name").inline(),
                    );

                    parts::space(ui);

                    ui.add(inputs::edit(&mut event.phase).label("Phase").help(
                            "What phase/screen the event can occur on.",
                    ).inline());
                },
                );

            parts::space(ui);

            ui.add(inputs::edit((
                &mut event.effects,
                processes,
                projects,
                industries,
                events,
                npcs,
            )));

            parts::space(ui);

            ui.add(inputs::edit((
                &mut event.probabilities,
                processes,
                projects,
                npcs,
            )));

            parts::space(ui);

            ui.add(
                inputs::textarea(&mut event.notes)
                    .label("Notes")
                    .help("Optional notes"),
            );
        })
        .response
}
