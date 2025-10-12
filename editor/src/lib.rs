mod files;
mod inputs;
mod parts;
mod tabs;
mod validate;

use std::sync::{Arc, LazyLock};

use egui::mutex::Mutex;
use egui_notify::Toasts;
use files::FilePicker;
use hes_engine::{Collection, Event, NPC, Project, World};
use strum::{Display, EnumIter, IntoEnumIterator};
use tabs::*;

use crate::parts::Request;

pub(crate) static TOASTS: LazyLock<Arc<Mutex<Toasts>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Toasts::default())));

#[derive(Debug, Clone, Copy, Display, EnumIter, PartialEq)]
enum Tab {
    Planet,
    Industries,
    Processes,
    Projects,
    Events,
    Help,
}

pub struct WorldEditor {
    tab: Tab,
    world: World,
    npcs: Collection<NPC>,
    events: Collection<Event>,
    projects: Collection<Project>,
    file_picker: FilePicker,
}
impl WorldEditor {
    pub fn new() -> Self {
        let world = World::default();
        Self {
            tab: Tab::Planet,
            npcs: NPC::load(),
            events: world.events.clone(),
            projects: world.projects.clone(),
            world,
            file_picker: FilePicker::default(),
        }
    }
}
impl egui::Widget for &mut WorldEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::NONE
            .fill(egui::Color32::from_black_alpha(200))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    ui.horizontal(|ui| {
                        for tab in Tab::iter() {
                            ui.label(tab.to_string());
                        }
                    });

                    if let Err(err) =
                        self.file_picker.render(ui, &mut self.world)
                    {
                        TOASTS.lock().error(format!("Error: {err}"));
                    }

                    let mut request = None;
                    match self.tab {
                        Tab::Planet => world(ui, &mut self.world),
                        Tab::Industries => {
                            let resp = industries(ui, &mut self.world.industries);
                            request = resp.inner;
                        }
                        Tab::Processes => {
                            let resp = processes(
                                ui,
                                &mut self.world.processes,
                                &self.npcs,
                            );
                            request = resp.inner;
                        }
                        Tab::Projects => {
                            let resp = projects(
                                ui,
                                &mut self.world.projects,
                                &self.world.processes,
                                &self.projects,
                                &self.world.industries,
                                &self.world.events,
                                &self.npcs,
                            );
                            if resp.response.changed() {
                                self.projects =
                                    self.world.projects.clone();
                            }
                            request = resp.inner;
                        }
                        Tab::Events => {
                            let resp = events(
                                ui,
                                &mut self.world.events,
                                &self.world.processes,
                                &self.world.projects,
                                &self.world.industries,
                                &self.events,
                                &self.npcs,
                            );
                            if resp.response.changed() {
                                self.events = self.world.events.clone();
                            }
                            request = resp.inner;
                        }
                        Tab::Help => help(ui),
                    }

                    if let Some(request) = request {
                        match request {
                            Request::Delete(id) => {
                                let refs = validate::find_references(id, &self.world);
                                if refs.is_empty() {
                                    match self.tab {
                                        Tab::Industries => self.world.industries.remove(&id),
                                        Tab::Processes => self.world.processes.remove(&id),
                                        Tab::Projects => {
                                            self.projects.remove(&id);
                                            self.projects =
                                                self.world.projects.clone();
                                        },
                                        Tab::Events => {
                                            self.events.remove(&id);
                                            self.events = self.world.events.clone();
                                        },
                                        _ => {}
                                    }
                                } else {
                                    TOASTS.lock().error(format!("Can't delete, still referenced by:\n{}", refs.join(", ")));
                                }
                            },
                        }
                    }

                    TOASTS.lock().show(ui.ctx());
                });
            }).response
    }
}
