mod dialogue;
mod update;

use egui::{Color32, Sense};
use egui_taffy::TuiBuilderLogic;
use hes_engine::State;
use hes_images::flavor_image;
use rust_i18n::t;

use crate::{
    display::{DisplayEffect, DisplayEvent, render_effects},
    parts::{
        bg_cover_image,
        button,
        center_text,
        h_center,
        overlay,
    },
    tips::{add_hover_tip, tip},
};

use dialogue::{Dialogue, DialogueResult};

pub type Updates = Events<hes_engine::Update>;

#[derive(Debug, PartialEq)]
pub enum EventResult {
    Advanced,
    JustFinished,
    AlreadyFinished,
}

pub struct EventDetails<'a> {
    title: &'a str,
    name: &'a str,
    image: Option<egui::Image<'a>>,
    attrib: Option<&'a str>,
    effects: Option<&'a Vec<DisplayEffect>>,
}

pub trait AsEventView {
    fn details<'a>(
        &'a self,
        state: &'a State,
    ) -> EventDetails<'a>;
    fn dialogue(&self, state: &State) -> Option<Dialogue>;
    fn show_card(&self) -> bool;
    fn render_extras(&self, ui: &mut egui::Ui, state: &State);
}

impl AsEventView for DisplayEvent {
    fn dialogue(&self, _state: &State) -> Option<Dialogue> {
        Some(Dialogue::from(self))
    }

    fn show_card(&self) -> bool {
        self.show_as_card()
    }

    fn render_extras(&self, ui: &mut egui::Ui, _state: &State) {
        let factors_list = self
            .factors
            .iter()
            .cloned()
            .map(|(icon, factor)| {
                (icon, tip(icon, factor.to_string()))
            })
            .collect::<Vec<_>>();
        ui.horizontal(|ui| {
            for (icon, tip) in factors_list {
                add_hover_tip(tip, ui.add(icon.size(16.)));
            }
        });
    }

    fn details<'a>(
        &'a self,
        _state: &'a State,
    ) -> EventDetails<'a> {
        let image =
            self.flavor.image.as_ref().map(flavor_image);
        let attrib = self
            .flavor
            .image
            .as_ref()
            .map(|image| image.attribution.as_str());

        let show_effects = self.has_visible_effects();
        let effects = if show_effects {
            Some(&self.effects)
        } else {
            None
        };
        EventDetails {
            title: &self.flavor.arc,
            name: &self.name,
            image: image,
            attrib: attrib,
            effects: effects,
        }
    }
}

pub struct Events<E: AsEventView = DisplayEvent> {
    idx: usize,
    events: Vec<E>,
    dialogue: Option<Dialogue>,
    pub is_finished: bool,
}
impl<E: AsEventView> Events<E> {
    pub fn new(events: Vec<E>, state: &State) -> Self {
        if events.is_empty() {
            Self {
                idx: 0,
                dialogue: None,
                events,
                is_finished: true,
            }
        } else {
            Self {
                idx: 0,
                dialogue: events[0].dialogue(state),
                events,
                is_finished: false,
            }
        }
    }

    pub fn empty() -> Self {
        Events {
            idx: 0,
            events: vec![],
            dialogue: None,
            is_finished: true,
        }
    }

    pub fn replace(&mut self, events: Vec<E>, state: &State) {
        *self = Events::new(events, state);
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
    ) -> Option<EventResult> {
        let mut result = None;
        if !self.events.is_empty() && !self.is_finished {
            let mut dialogue_result = None;
            overlay(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    dialogue_result =
                        self.render_event(ui, state);
                })
                .response
            });
            if let Some(dialogue_result) = dialogue_result {
                match dialogue_result {
                    DialogueResult::Advanced => {
                        result = Some(EventResult::Advanced);
                    }
                    DialogueResult::Finished => {
                        if self.idx < self.events.len() - 1 {
                            self.idx += 1;
                            let event = &self.events[self.idx];
                            self.dialogue =
                                event.dialogue(state);
                            result =
                                Some(EventResult::Advanced);
                        } else {
                            self.is_finished = true;
                            result =
                                Some(EventResult::JustFinished);
                        }
                    }
                }
            }
        } else {
            result = Some(EventResult::AlreadyFinished);
        }
        result
    }

    fn render_event(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
    ) -> Option<DialogueResult> {
        let event = &self.events[self.idx];
        if let Some(dialogue) = &mut self.dialogue {
            ui.set_width(420.);
            let go_to_next = ui
                .vertical(|ui| {
                    if event.show_card() {
                        render_event_card(ui, state, event);
                    }
                    dialogue.render(ui, state)
                })
                .inner;
            go_to_next
        } else if event.show_card() {
            let go_to_next = ui
                .vertical(|ui| {
                    ui.set_width(420.);
                    render_event_card(ui, state, event);
                    ui.add(button(t!("Continue"))).clicked()
                })
                .inner;
            if go_to_next {
                Some(DialogueResult::Finished)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl From<&DisplayEvent> for Dialogue {
    fn from(event: &DisplayEvent) -> Self {
        let region_name = event
            .region
            .as_ref()
            .map(|(_, name)| name.to_string());

        // Only show effects in the dialogue if there's
        // no event card being shown.
        let effects = if event.show_as_card() {
            None
        } else {
            Some(event.effects.clone())
        };

        Dialogue::new(
            event.flavor.dialogue.clone(),
            effects,
            Some(event.id),
            region_name,
        )
    }
}

pub fn render_event_card<E: AsEventView>(
    ui: &mut egui::Ui,
    state: &State,
    event: &E,
) -> egui::Response {
    let details = event.details(state);

    let attribution = details.attrib.and_then(|attrib| {
        if attrib.trim().is_empty() {
            None
        } else {
            Some(format!("{} {attrib}", t!("Image:")))
        }
    });

    ui.vertical(|ui| {
        if let Some(image) = details.image {
            let target_rect = egui::Rect::from_min_size(
                ui.cursor().left_top(),
                egui::vec2(ui.available_width(), 240.),
            );
            ui.allocate_rect(target_rect, Sense::empty());
            bg_cover_image(ui, image, target_rect);

            if let Some(attribution) = attribution {
                let pos = target_rect.left_bottom();
                let rect = egui::Rect::from_min_size(
                    pos - egui::vec2(0., 24.),
                    egui::vec2(ui.available_width(), 16.),
                );
                ui.place(
                    rect,
                    title_label("attrib", &attribution, 11.),
                );
            }

            let pos = target_rect.left_top();
            let rect = egui::Rect::from_min_size(
                pos + egui::vec2(0., 4.),
                egui::vec2(ui.available_width(), 16.),
            );
            ui.place(
                rect,
                title_label("title", &t!(details.title), 11.),
            );

            let pos = target_rect.left_top();
            let rect = egui::Rect::from_min_size(
                pos + egui::vec2(0., 24.),
                egui::vec2(ui.available_width(), 16.),
            );
            ui.place(
                rect,
                title_label("name", &t!(details.name), 14.),
            );
        } else {
            ui.add(center_text(t!(details.title)));
            ui.add(center_text(t!(details.name)).size(18.));
        }

        if let Some(effects) = details.effects {
            ui.add_space(8.);
            render_effects(ui, state, effects);
        }

        ui.add_space(8.);
        event.render_extras(ui, state);
    })
    .response
}

fn title_label(
    id: &str,
    text: &str,
    size: f32,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    move |ui| {
        h_center(ui, id, |tui| {
            tui.ui(|ui| {
                ui.style_mut().wrap_mode =
                    Some(egui::TextWrapMode::Extend);
                egui::Frame::NONE
                    .fill(Color32::from_gray(20))
                    .corner_radius(3)
                    .inner_margin(egui::Margin::symmetric(3, 1))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(text)
                                .color(Color32::WHITE)
                                .size(size),
                        );
                    })
                    .response
            })
        })
    }
}
