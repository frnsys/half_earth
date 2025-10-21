use std::ops::Deref;

use egui::{Color32, Sense};
use hes_images::flavor_image;
use rust_i18n::t;

use crate::{parts::bg_cover_image, state::GameState, text::scale_text_ui};

mod industry;
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

pub const CARD_HEIGHT: f32 = 360.;
pub const CARD_WIDTH: f32 = 280.;

#[derive(Clone)]
pub struct Card<C: AsCard + Clone> {
    data: C,
    y_offset: f32,
    flipped: bool,
    pub draggable: bool,
}

impl<C: AsCard + Clone> Deref for Card<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<C: AsCard + Clone> Card<C> {
    pub fn new(data: C) -> Self {
        Self {
            data,
            y_offset: 0.,
            flipped: false,
            draggable: false,
        }
    }

    fn render_contents(&mut self, ui: &mut egui::Ui, state: &GameState, is_offscreen: bool) {
        ui.vertical(|ui| {
            ui.set_height(CARD_HEIGHT);
            ui.set_width(CARD_WIDTH);
            ui.style_mut().visuals.override_text_color = Some(self.data.fg_color());
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

            let (mut rect, _) =
                ui.allocate_exact_size(egui::vec2(CARD_WIDTH, CARD_HEIGHT), Sense::empty());

            rect.set_top(rect.top() + top_margin);

            // Detect drag "under" the card contents or else
            // it will capture all clicks.
            let drag_id = ui.id().with("card-drag");

            let is_already_dragging = ui.ctx().is_being_dragged(drag_id);
            let y_delta = ui.input(|inp| {
                let is_dragging = inp.pointer.is_decidedly_dragging();
                if is_dragging && let Some(pos) = inp.pointer.interact_pos() {
                    let contains = rect.contains(pos);
                    let delta = inp.pointer.delta();

                    // Starting a new drag or moving an existing drag.
                    // Here we check if the movement is vertical.
                    if contains && delta.y.abs() > delta.x.abs() {
                        Some(delta.y)

                    // No movement in an existing drag.
                    // If we've already detected a vertical drag
                    // then we don't care about horizontal movement.
                    } else if is_already_dragging {
                        Some(0.)
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

            let mut resp = if y_delta.is_some() && ui.input(|inp| inp.pointer.primary_down()) {
                ui.ctx().set_dragged_id(drag_id);
                ui.interact(rect, drag_id, Sense::empty())
            } else {
                if is_already_dragging {
                    ui.ctx().stop_dragging();
                }
                ui.allocate_rect(rect, Sense::empty())
            };

            ui.place(rect, |ui: &mut egui::Ui| -> egui::Response {
                let resp = egui::Frame::NONE
                    .corner_radius(4.)
                    .fill(self.data.bg_color())
                    .show(ui, |ui| {
                        self.render_contents(ui, state, is_offscreen);
                    })
                    .response;

                corner(
                    ui,
                    resp.rect,
                    "card-flip".into(),
                    self.bg_color().intensity() < 0.5,
                    &mut self.flipped,
                );
                resp
            });

            if let Some(y) = y_delta {
                // if resp.is_pointer_button_down_on() {
                // self.y_offset += resp.drag_delta().y;
                self.y_offset += y;

                // const MAX: f32 = GAP * 2.;
                const MAX: f32 = 20.;
                self.y_offset = self.y_offset.clamp(-MAX, MAX);
            } else {
                self.y_offset *= 0.8;
                if self.y_offset.abs() < 0.1 {
                    self.y_offset = 0.;
                } else {
                    ui.ctx().request_repaint();
                }
            }

            // This is a bit hacky, but offsetting the rect
            // above does not actually preserve the correct height;
            // it shrinks to keep the bottom rect position consistent.
            // The problem is that then this means the incorrect bottom
            // rect position is reported. Forcing the correct height fixes this.
            resp.rect.set_height(CARD_HEIGHT);

            resp
        } else {
            let resp = egui::Frame::NONE
                .corner_radius(4.)
                .fill(self.data.bg_color())
                .show(ui, |ui| self.render_contents(ui, state, is_offscreen))
                .response;
            corner(
                ui,
                resp.rect,
                resp.id,
                self.bg_color().intensity() < 0.5,
                &mut self.flipped,
            );

            resp
        }
    }
}

fn corner(ui: &mut egui::Ui, rect: egui::Rect, id: egui::Id, on_dark: bool, flipped: &mut bool) {
    let pad = 4.0;
    let size = 14.0;

    let tri_rect = egui::Rect::from_min_max(
        rect.max - egui::Vec2::new(size + pad, size + pad),
        rect.max - egui::Vec2::new(pad, pad),
    );

    let tri_id = id.with("corner");
    let resp = ui.interact(tri_rect, tri_id, Sense::click());

    let base = if on_dark {
        Color32::from_white_alpha(32)
    } else {
        Color32::from_black_alpha(32)
    };
    let hover = if on_dark {
        Color32::from_white_alpha(64)
    } else {
        Color32::from_black_alpha(64)
    };
    let press = Color32::from_white_alpha(64);
    let fill = if resp.is_pointer_button_down_on() {
        press
    } else if resp.hovered() {
        hover
    } else {
        base
    };

    // Triangle
    let p0 = egui::Pos2::new(tri_rect.min.x, tri_rect.max.y); // bottom-left
    let p1 = tri_rect.max; // bottom-right
    let p2 = egui::Pos2::new(tri_rect.max.x, tri_rect.min.y); // top-right

    let painter = ui.painter_at(rect);
    painter.add(egui::Shape::convex_polygon(
        vec![p0, p1, p2],
        fill,
        egui::Stroke::NONE,
    ));

    if resp.clicked() {
        *flipped = !*flipped;
    }
}

fn card_title(ui: &mut egui::Ui, name: &str) {
    scaled_text(ui, name, 32.);
}

fn card_desc(ui: &mut egui::Ui, desc: &str) {
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(8, 0))
        .show(ui, |ui| {
            scaled_text(ui, desc, 128.);
        });
}

fn scaled_text(ui: &mut egui::Ui, text: &str, height: f32) {
    ui.vertical_centered(|ui| {
        ui.set_height(height);
        let max_size = egui::vec2(ui.available_width(), height);
        let text = t!(text).to_string();
        scale_text_ui(ui, max_size, move |ui| {
            ui.label(egui::RichText::new(&text).heading());
        });
    });
}

fn render_flavor_image(ui: &mut egui::Ui, image: &hes_engine::flavor::Image) -> egui::Rect {
    let rect = egui::Rect::from_min_size(ui.cursor().left_top(), egui::vec2(CARD_WIDTH, 160.));
    ui.allocate_rect(rect, egui::Sense::empty());
    let image = flavor_image(image);
    bg_cover_image(ui, image, rect);
    rect
}
