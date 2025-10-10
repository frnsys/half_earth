use std::ops::Deref;

use egui::{Color32, Sense};

use crate::state::GameState;

mod npc;
mod process;
mod project;

pub use process::draw_mix_cell;

pub trait AsCard {
    fn id(&self) -> hes_engine::Id;
    fn bg_color(&self) -> Color32;
    fn fg_color(&self) -> Color32;
    fn header(&self, ui: &mut egui::Ui, state: &GameState);
    fn figure(&self, ui: &mut egui::Ui, state: &GameState);
    fn name(&self, ui: &mut egui::Ui, state: &GameState);
    fn body(&self, ui: &mut egui::Ui, state: &GameState);
    fn top_back(&self, ui: &mut egui::Ui, state: &GameState);
    fn bottom_back(&self, ui: &mut egui::Ui, state: &GameState);
}

pub const CARD_HEIGHT: f32 = 380.;
pub const CARD_WIDTH: f32 = 280.;

pub struct Card<C: AsCard> {
    data: C,
    y_offset: f32,
    flipped: bool,
    pub draggable: bool,
}

impl<C: AsCard> Deref for Card<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<C: AsCard> Card<C> {
    pub fn new(data: C) -> Self {
        Self {
            data,
            y_offset: 0.,
            flipped: false,
            draggable: false,
        }
    }

    fn render_contents(
        &mut self,
        ui: &mut egui::Ui,
        state: &GameState,
        is_offscreen: bool,
    ) {
        ui.vertical(|ui| {
            ui.set_height(CARD_HEIGHT);
            ui.set_width(CARD_WIDTH);
            ui.style_mut().visuals.override_text_color =
                Some(self.data.fg_color());
            if !is_offscreen {
                if !self.flipped {
                    self.data.header(ui, state);
                    self.data.figure(ui, state);
                    self.data.name(ui, state);
                    self.data.body(ui, state);
                } else {
                    self.data.top_back(ui, state);
                    self.data.bottom_back(ui, state);
                }
            }
        });
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &GameState,
        is_offscreen: bool,
    ) -> egui::Response {
        if self.draggable {
            let top_margin = self.y_offset;

            let (mut rect, _) = ui.allocate_exact_size(
                egui::vec2(CARD_WIDTH, CARD_HEIGHT),
                Sense::empty(),
            );

            rect.set_top(rect.top() + top_margin);

            let resp = ui.place(
                rect,
                |ui: &mut egui::Ui| -> egui::Response {
                    egui::Frame::NONE
                        .corner_radius(4.)
                        .fill(self.data.bg_color())
                        .show(ui, |ui| {
                            self.render_contents(
                                ui,
                                state,
                                is_offscreen,
                            )
                        })
                        .response
                },
            );

            let resp = resp.interact(Sense::click_and_drag());
            if resp.clicked() {
                self.flipped = !self.flipped;
            }

            if resp.is_pointer_button_down_on() {
                self.y_offset += resp.drag_delta().y;

                // const MAX: f32 = GAP * 2.;
                const MAX: f32 = 24. * 2.;
                self.y_offset = self.y_offset.clamp(-MAX, MAX);
            } else {
                self.y_offset *= 0.8;
                if self.y_offset.abs() < 0.1 {
                    self.y_offset = 0.;
                } else {
                    ui.ctx().request_repaint();
                }
            }

            resp
        } else {
            let resp = egui::Frame::NONE
                .corner_radius(4.)
                .fill(self.data.bg_color())
                .show(ui, |ui| {
                    self.render_contents(
                        ui,
                        state,
                        is_offscreen,
                    )
                })
                .response;
            let resp = resp.interact(Sense::click());
            if resp.clicked() {
                self.flipped = !self.flipped;
            }
            resp
        }
    }
}
