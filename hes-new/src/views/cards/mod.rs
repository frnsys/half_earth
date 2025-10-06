use std::{collections::BTreeMap, ops::Deref};

use egui::{Color32, Sense};
use enum_map::EnumMap;
use hes_engine::{Id, Output, State};

use crate::state::{PlanChange, Points, Tutorial};

mod npc;
mod process;
mod project;

pub struct CardState<'a> {
    pub state: &'a State,
    pub viewed: &'a Vec<Id>,
    pub plan_changes: &'a BTreeMap<Id, PlanChange>,
    pub queued_upgrades: &'a BTreeMap<Id, bool>,
    pub process_mix_changes:
        &'a EnumMap<Output, BTreeMap<Id, isize>>,
    pub process_points: &'a isize,
}

pub struct CardStateMut<'a> {
    pub state: &'a mut State,
    pub viewed: &'a mut Vec<Id>,
    pub plan_changes: &'a mut BTreeMap<Id, PlanChange>,
    pub queued_upgrades: &'a mut BTreeMap<Id, bool>,
    pub process_mix_changes:
        &'a mut EnumMap<Output, BTreeMap<Id, isize>>,
    pub points: &'a mut Points,
    pub tutorial: &'a mut Tutorial,
    pub process_points: &'a mut isize,
}

pub trait AsCard {
    fn bg_color(&self) -> Color32;
    fn fg_color(&self) -> Color32;
    fn header(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn figure(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn name(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn body(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn top_back(&self, ui: &mut egui::Ui, ctx: &CardState);
    fn bottom_back(&self, ui: &mut egui::Ui, ctx: &CardState);
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
        ctx: &CardState,
        is_offscreen: bool,
    ) {
        ui.vertical(|ui| {
            ui.set_height(CARD_HEIGHT);
            ui.set_width(CARD_WIDTH);
            ui.style_mut().visuals.override_text_color =
                Some(self.data.fg_color());
            if !is_offscreen {
                if !self.flipped {
                    self.data.header(ui, ctx);
                    self.data.figure(ui, ctx);
                    self.data.name(ui, ctx);
                    self.data.body(ui, ctx);
                } else {
                    self.data.top_back(ui, ctx);
                    self.data.bottom_back(ui, ctx);
                }
            }
        });
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &CardState,
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
                                ctx,
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
                    self.render_contents(ui, ctx, is_offscreen)
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
