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
            ui.add(inputs::edit(&mut event.name));
            ui.add(
                inputs::lock(&mut event.locked)
                    .label("Locked")
                    .tooltip(
                        "If this event is locked at the start.",
                    ),
            );
            ui.add(inputs::edit(&mut event.flavor.image));
            ui.add(
                inputs::edit(&mut event.flavor.arc)
                    .help("Optional story arc name"),
            );
            ui.add(inputs::edit(&mut event.phase).help(
                "What phase/screen the event can occur on.",
            ));

            ui.add(inputs::edit((
                &mut event.effects,
                processes,
                projects,
                industries,
                events,
                npcs,
            )));

            ui.add(inputs::edit((
                &mut event.probabilities,
                processes,
                projects,
                npcs,
            )));

            ui.add(
                inputs::textarea(&mut event.notes)
                    .label("Notes")
                    .help("Optional notes"),
            );
        })
        .response
}
