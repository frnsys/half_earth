use crate::{
    parts::{get_sizing, h_center, raised_frame},
    state::GameState,
    views::{
        cards::{AsCard, CARD_HEIGHT, CARD_WIDTH, Card},
        scanner::{ScanResult, Scannable},
    },
};

use egui::{Align, Align2, Color32, Key, Margin, Order, Rect, Sense, style::ScrollAnimation};
use egui_taffy::TuiBuilderLogic;
use web_time::Instant;

const GAP: f32 = 24.;
const SCANNER_WIDTH: f32 = 300.;

enum ScanSide {
    Top,
    Bottom,
}
const LED_DURATION: u8 = 12;

fn scanner_height(ui: &egui::Ui) -> f32 {
    const SCANNER_HEIGHT: f32 = 48.;
    let screen_height = ui.ctx().screen_rect().height();
    if screen_height < 600. {
        SCANNER_HEIGHT / 2.
    } else {
        SCANNER_HEIGHT
    }
}

pub struct Cards<C: AsCard + Scannable> {
    cards: Vec<Card<C>>,
    scans: u8,
    scanning: Option<ScanSide>,
    scan_timer: Timer,
    scan_result: Option<(ScanSide, bool, u8)>,
}
impl<C: AsCard + Scannable> Cards<C> {
    pub fn new(cards: impl Iterator<Item = C>) -> Self {
        Self {
            cards: cards.map(|card| Card::new(card)).collect(),
            scan_timer: Timer::default(),
            scan_result: None,
            scanning: None,
            scans: 0,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> bool {
        let mut changed = false;

        let mut action = None;
        let (top_scan_area, bot_scan_area) = self.render_scanners(ui, &mut action);

        let h_center = ui.cursor().left() + ui.available_width() / 2.;
        let mut closest_offset = f32::INFINITY;
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_height(ui.available_height());

            let width = ui.available_width();
            ui.set_max_width(width);
            let half_width = width / 2.;

            ui.add_space(ui.available_height() / 2. - CARD_HEIGHT / 2. - 12.);

            let mut selected_idx = None;
            let mut card_resps = vec![];

            ui.horizontal(|ui| {
                ui.add_space(half_width);

                ui.style_mut().spacing.item_spacing.x = 18.;
                for (i, card) in self.cards.iter_mut().enumerate() {
                    let left_pos = ui.cursor().left();
                    let is_offscreen = (left_pos + CARD_WIDTH) < 0. || left_pos > width;

                    let resp = card.render(ui, state, is_offscreen);
                    let card_rect = resp.rect;
                    let cx = resp.rect.center().x;
                    let offset = h_center - cx;
                    if offset.abs() < closest_offset.abs() {
                        closest_offset = offset;
                        selected_idx = Some(i);

                        let id = card.id();
                        if !state.ui.viewed.contains(&id) {
                            state.ui.viewed.push(id);
                        }
                    }

                    card.draggable = offset.abs() <= 15.;

                    if card.draggable {
                        if card_rect.intersects(top_scan_area) {
                            ui.ctx().request_repaint();
                            self.scanning = Some(ScanSide::Top);
                            if card.is_add_allowed(state)
                                && self.scan_timer.has_elapsed(
                                    // Speed up each subsequent scan
                                    card.add_scan_time() / ((self.scans + 1) as f32).sqrt(),
                                )
                            {
                                self.scan_timer.reset();
                                self.scans = self.scans.saturating_add(1);
                                self.scan_result = match card.add_scan_done(state) {
                                    ScanResult::SuccessContinue | ScanResult::SuccessStop => {
                                        Some((ScanSide::Top, true, LED_DURATION))
                                    }
                                    ScanResult::Rejected => {
                                        Some((ScanSide::Top, false, LED_DURATION))
                                    }
                                };
                                changed = true;
                            }
                        } else if card_rect.intersects(bot_scan_area) {
                            ui.ctx().request_repaint();
                            self.scanning = Some(ScanSide::Bottom);
                            if card.is_rem_allowed(state)
                                && self.scan_timer.has_elapsed(
                                    // Speed up each subsequent scan
                                    card.rem_scan_time() / ((self.scans + 1) as f32).sqrt(),
                                )
                            {
                                self.scan_timer.reset();
                                self.scans = self.scans.saturating_add(1);
                                self.scan_result = match card.rem_scan_done(state) {
                                    ScanResult::SuccessContinue | ScanResult::SuccessStop => {
                                        Some((ScanSide::Bottom, true, LED_DURATION))
                                    }
                                    ScanResult::Rejected => {
                                        Some((ScanSide::Bottom, false, LED_DURATION))
                                    }
                                };
                                changed = true;
                            }
                        } else {
                            self.scan_timer.reset();
                            self.scanning = None;
                            self.scans = 0;
                        }
                    }

                    card_resps.push(resp);
                }

                ui.add_space(half_width);
            });

            ui.input(|inp| {
                if [Key::ArrowLeft, Key::A].iter().any(|k| inp.key_pressed(*k)) {
                    action = Some(Action::Prev);
                } else if [Key::ArrowRight, Key::D]
                    .iter()
                    .any(|k| inp.key_pressed(*k))
                {
                    action = Some(Action::Next);
                } else if [Key::ArrowUp, Key::W].iter().any(|k| inp.key_pressed(*k)) {
                    action = Some(Action::Up);
                } else if [Key::ArrowDown, Key::S].iter().any(|k| inp.key_pressed(*k)) {
                    action = Some(Action::Down);
                }
            });

            match action {
                Some(action) => match action {
                    Action::Next => {
                        if let Some(resp) = selected_idx.and_then(|idx| {
                            let next_idx = idx.saturating_add(1);
                            card_resps.get(next_idx)
                        }) {
                            resp.scroll_to_me_animation(
                                Some(Align::Center),
                                ScrollAnimation::none(),
                            );
                        }
                    }
                    Action::Prev => {
                        if let Some(resp) = selected_idx.and_then(|idx| {
                            let next_idx = idx.saturating_sub(1);
                            card_resps.get(next_idx)
                        }) {
                            resp.scroll_to_me_animation(
                                Some(Align::Center),
                                ScrollAnimation::none(),
                            );
                        }
                    }
                    Action::Up => {
                        if let Some(card) = selected_idx.and_then(|idx| self.cards.get_mut(idx))
                            && card.is_add_allowed(state)
                        {
                            card.add_scan_done(state);
                            changed = true;
                        }
                    }
                    Action::Down => {
                        if let Some(card) = selected_idx.and_then(|idx| self.cards.get_mut(idx))
                            && card.is_rem_allowed(state)
                        {
                            card.rem_scan_done(state);
                            changed = true;
                        }
                    }
                },
                None => {
                    if let Some(resp) = selected_idx.map(|idx| &card_resps[idx]) {
                        resp.scroll_to_me_animation(
                            Some(Align::Center),
                            ScrollAnimation::duration(0.05),
                        );
                    }
                }
            }
        });
        changed
    }

    fn render_scanners(
        &mut self,
        ui: &mut egui::Ui,
        action: &mut Option<Action>,
    ) -> (egui::Rect, egui::Rect) {
        let scanner_height = scanner_height(ui);

        let cursor = ui.cursor();
        let mid_y = ui.available_height() / 2. - 24.;
        let top = egui::Area::new("scan-up".into())
            .order(Order::Middle)
            .movable(false)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos((
                cursor.left() + ui.available_width() / 2.,
                cursor.top() + mid_y - CARD_HEIGHT / 2. - GAP + 24.,
            ))
            .show(ui.ctx(), |ui| {
                raised_frame()
                    .colors(
                        Color32::WHITE,
                        Color32::from_rgb(0xdc, 0xe0, 0xe6),
                        Color32::from_rgb(0xfa, 0xfc, 0xff),
                    )
                    .show(ui, |ui| {
                        ui.set_width(SCANNER_WIDTH);
                        ui.set_height(scanner_height);

                        let c = ui.cursor().right_bottom() - egui::vec2(8., 6.);

                        match &mut self.scan_result {
                            Some((ScanSide::Top, accepted, countdown)) => {
                                if lit_led(ui.painter(), c, *accepted, countdown) {
                                    self.scan_result = None;
                                }
                            }
                            _ => {
                                dim_led(ui.painter(), c);
                            }
                        }

                        let lt = ui.cursor().left_bottom() + egui::vec2(6., -7.);
                        let rect =
                            Rect::from_min_size(lt, egui::vec2(SCANNER_WIDTH - 12. - 16., 2.));

                        if matches!(self.scanning, Some(ScanSide::Top)) {
                            let progress = self.scan_timer.progress();
                            scanning_bar(ui.painter(), rect, progress);
                        }
                    });
            })
            .response
            .rect;

        let bot = egui::Area::new("scan-down".into())
            .order(Order::Middle)
            .movable(false)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos((
                cursor.left() + ui.available_width() / 2.,
                cursor.top() + mid_y + CARD_HEIGHT / 2. + GAP + GAP + scanner_height,
            ))
            .show(ui.ctx(), |ui| {
                let rect = raised_frame()
                    .colors(
                        Color32::WHITE,
                        Color32::from_rgb(0xdc, 0xe0, 0xe6),
                        Color32::from_rgb(0xfa, 0xfc, 0xff),
                    )
                    .show(ui, |ui| {
                        ui.set_width(SCANNER_WIDTH);
                        ui.set_height(scanner_height);

                        let c = ui.cursor().right_top() - egui::vec2(8., -6.);

                        match &mut self.scan_result {
                            Some((ScanSide::Bottom, accepted, countdown)) => {
                                if lit_led(ui.painter(), c, *accepted, countdown) {
                                    self.scan_result = None;
                                }
                            }
                            _ => {
                                dim_led(ui.painter(), c);
                            }
                        }

                        let lt = ui.cursor().left_top() + egui::vec2(6., 7.);
                        let rect =
                            Rect::from_min_size(lt, egui::vec2(SCANNER_WIDTH - 12. - 16., 2.));

                        if matches!(self.scanning, Some(ScanSide::Bottom)) {
                            let progress = self.scan_timer.progress();
                            scanning_bar(ui.painter(), rect, progress);
                        }
                    })
                    .rect;

                let sizing = get_sizing(ui);
                if sizing.is_small {
                    ui.place(rect, |ui: &mut egui::Ui| touch_controls(ui, action));
                }
            })
            .response
            .rect;

        (top, bot)
    }
}

enum Action {
    Next,
    Prev,
    Up,
    Down,
}

#[derive(Default)]
struct Timer {
    start: Option<(Instant, f32)>,
}
impl Timer {
    fn has_elapsed(&mut self, target_ms: f32) -> bool {
        match self.start {
            Some((start, ms)) if ms == target_ms => {
                let duration = start.elapsed();
                duration.as_millis() as f32 >= ms
            }
            _ => {
                self.start = Some((Instant::now(), target_ms));
                false
            }
        }
    }

    fn progress(&self) -> f32 {
        match self.start {
            Some((start, ms)) => {
                let duration = start.elapsed();
                (duration.as_millis() as f32 / ms).min(1.)
            }
            None => 0.,
        }
    }

    fn reset(&mut self) {
        self.start = None;
    }
}

fn led(painter: &egui::Painter, center: egui::Pos2, color: Color32, glow: bool) {
    const RADIUS: f32 = 3.;
    painter.circle_filled(center, RADIUS + 1., color);

    if glow {
        for i in 0..=12 {
            let i = i as f32;
            let alpha = (1. - (i * 0.1)).max(0.).powi(2);
            painter.circle_stroke(
                center,
                RADIUS + i,
                egui::Stroke::new(1., color.gamma_multiply(alpha)),
            );
        }
    }
}

fn lit_led(
    painter: &egui::Painter,
    center: egui::Pos2,
    accepted: bool,
    countdown: &mut u8,
) -> bool {
    let color = if accepted {
        Color32::from_rgb(0x8a, 0xff, 0x8c)
    } else {
        Color32::from_rgb(0xff, 0x8a, 0x8a)
    };
    led(painter, center, color, true);
    *countdown -= 1;
    *countdown == 0
}

fn dim_led(painter: &egui::Painter, center: egui::Pos2) {
    led(painter, center, Color32::from_gray(220), false);
}

fn scanning_bar(painter: &egui::Painter, rect: Rect, progress: f32) {
    if progress > 0. {
        progress_bar(
            painter,
            rect,
            Color32::from_rgb(0xb5, 0x8a, 0xff),
            egui_animation::easing::quad_in_out(progress),
        );
    }
}

fn progress_bar(painter: &egui::Painter, mut rect: Rect, color: Color32, percent: f32) {
    rect.set_width(rect.width() * percent);
    painter.rect_filled(rect, 2, color);

    for i in 0..=12 {
        let i = i as f32;
        let alpha = (0.6 - (i * 0.05)).max(0.).powi(2) / 1.5;
        painter.rect_stroke(
            rect,
            2 + i as u8,
            egui::Stroke::new(i * 2., color.gamma_multiply(alpha)),
            egui::StrokeKind::Middle,
        );
    }
}

fn touch_controls(ui: &mut egui::Ui, action: &mut Option<Action>) -> egui::Response {
    let screen_height = ui.ctx().screen_rect().height();
    let is_short = screen_height < 600.;
    egui::Frame::NONE
        .inner_margin(Margin::symmetric(6, if is_short { 18 } else { 20 }))
        .show(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            h_center(ui, "controls-buttons", |tui| {
                tui.ui(|ui| {
                    let label = egui::RichText::new("◄")
                        .size(18.)
                        .color(Color32::from_gray(230));
                    if ui.label(label).interact(Sense::click()).clicked() {
                        *action = Some(Action::Prev);
                    }
                });
                tui.ui(|ui| {
                    let label = egui::RichText::new("►")
                        .size(18.)
                        .color(Color32::from_gray(230));
                    if ui.label(label).interact(Sense::click()).clicked() {
                        *action = Some(Action::Next);
                    }
                });
            });
        })
        .response
}
