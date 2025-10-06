use std::time::Instant;

use crate::views::{
    cards::{AsCard, CardState},
    game::card::{CARD_HEIGHT, CARD_WIDTH},
    scanner::Scannable,
};

use super::card::Card;
use egui::{Align, Align2, Color32, Order};

pub struct Cards<'a, C: AsCard + Scannable> {
    cards: &'a mut Vec<Card<C>>,
    scan_timer: Timer,
}
impl<'a, C: AsCard + Scannable> Cards<'a, C> {
    pub fn new(cards: &'a mut Vec<Card<C>>) -> Self {
        Self {
            cards,
            scan_timer: Timer::default(),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &CardState,
    ) {
        let mid_y = ui.available_height() / 2.;

        let cursor = ui.cursor();
        const GAP: f32 = 24.;
        let top_scan_area = egui::Area::new("scan-up".into())
            .order(Order::Foreground)
            .movable(false)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos((
                cursor.left() + ui.available_width() / 2.,
                mid_y - CARD_HEIGHT / 2. - GAP,
            ))
            .show(ui.ctx(), |ui| {
                egui::Frame::NONE
                    .corner_radius(4.)
                    .fill(Color32::WHITE)
                    .show(ui, |ui| {
                        ui.set_width(300.);
                        ui.set_height(80.);
                        ui.label("above");
                    });
            })
            .response
            .rect;

        let scanner_height = 80.;
        let bot_scan_area = egui::Area::new("scan-down".into())
            .order(Order::Foreground)
            .movable(false)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos((
                cursor.left() + ui.available_width() / 2.,
                mid_y + CARD_HEIGHT / 2. + GAP + scanner_height,
            ))
            .show(ui.ctx(), |ui| {
                egui::Frame::NONE
                    .corner_radius(4.)
                    .fill(Color32::WHITE)
                    .show(ui, |ui| {
                        ui.set_width(300.);
                        ui.set_height(scanner_height);
                    });
            })
            .response
            .rect;

        let h_center =
            ui.cursor().left() + ui.available_width() / 2.;
        let mut closest_offset = f32::INFINITY;
        let mut closest_card = None;
        let area =
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.set_height(ui.available_height());

                let width = ui.available_width();
                ui.set_max_width(width);
                let half_width = width / 2.;

                ui.add_space(
                    ui.available_height() / 2.
                        - CARD_HEIGHT / 2.,
                );

                ui.horizontal(|ui| {
                    ui.add_space(half_width);

                    ui.style_mut().spacing.item_spacing.x = 18.;
                    for card in self.cards.iter_mut() {
                        let left_pos = ui.cursor().left();
                        let is_offscreen =
                            (left_pos + CARD_WIDTH) < 0.
                                || left_pos > width;

                        let resp =
                            card.render(ui, ctx, is_offscreen);
                        let card_rect = resp.rect;
                        let cx = resp.rect.center().x;
                        let offset = h_center - cx;
                        if offset.abs() < closest_offset.abs() {
                            closest_offset = offset;
                            closest_card = Some(resp);
                        }
                        if offset.abs() <= 5. {
                            card.draggable = true;
                        } else {
                            card.draggable = false;
                        }

                        if card.draggable {
                            if card_rect
                                .intersects(top_scan_area)
                            {
                                if card.is_add_allowed(ctx)
                                    && self
                                        .scan_timer
                                        .has_elapsed(
                                            card.add_scan_time(
                                            ),
                                        )
                                {
                                    // card.add_scan_done(ctx) // TODO
                                    println!("SCANNING TOP");
                                } else {
                                    println!("NOT ALLOWED");
                                }
                            } else if card_rect
                                .intersects(bot_scan_area)
                            {
                                if card.is_add_allowed(ctx)
                                    && self
                                        .scan_timer
                                        .has_elapsed(
                                            card.add_scan_time(
                                            ),
                                        )
                                {
                                    // card.rem_scan_done(ctx) // TODO
                                    println!("SCANNING BOTTOM");
                                } else {
                                    println!("NOT ALLOWED");
                                }
                            } else {
                                self.scan_timer.reset();
                            }
                        }
                    }

                    ui.add_space(half_width);
                });
                if let Some(resp) = closest_card {
                    resp.scroll_to_me(Some(Align::Center));
                }
            });
        ui.label("below");
    }
}

#[derive(Default)]
struct Timer {
    start: Option<Instant>,
}
impl Timer {
    fn has_elapsed(&mut self, ms: f32) -> bool {
        match self.start {
            Some(start) => {
                let duration = start.elapsed();
                duration.as_millis() as f32 >= ms
            }
            None => {
                self.start = Some(Instant::now());
                false
            }
        }
    }

    fn reset(&mut self) {
        self.start = None;
    }
}
