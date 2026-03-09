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

#[derive(Clone, Copy)]
enum ScanSide {
    Top,
    Bottom,
}
const LED_DURATION: u8 = 12;

fn scanner_height(ui: &egui::Ui) -> f32 {
    const SCANNER_HEIGHT: f32 = 48.;
    let screen_height = ui.ctx().content_rect().height();
    if screen_height < 600. {
        SCANNER_HEIGHT / 2.
    } else {
        SCANNER_HEIGHT
    }
}

#[derive(Clone)]
pub struct Cards<C: AsCard + Scannable> {
    cards: Vec<Card<C>>,
    scan_state: ScanState,
}
impl<C: AsCard + Scannable> Cards<C> {
    pub fn new(cards: impl Iterator<Item = C>) -> Self {
        Self {
            cards: cards.map(|card| Card::new(card)).collect(),
            scan_state: ScanState::default(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> bool {
        let mut changed = false;
        let mut action = None;
        let is_single_card = self.cards.len() == 1;

        let (top_scan_area, bot_scan_area) = self.render_scanners(ui, &mut action);

        let container_width = ui.available_width();
        let container_rect = ui.max_rect();
        let visual_center_x = container_rect.center().x;

        let mut closest_offset = f32::INFINITY;
        let mut selected_idx = None;
        let mut card_resps = vec![];

        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_height(ui.available_height());
            ui.add_space(ui.available_height() / 2. - CARD_HEIGHT / 2. - 12.);

            ui.horizontal(|ui| {
                if !is_single_card {
                    ui.add_space(container_width / 2. - CARD_WIDTH / 2.);
                }

                ui.style_mut().spacing.item_spacing.x = 18.;

                for (i, card) in self.cards.iter_mut().enumerate() {
                    let card_expected_rect =
                        Rect::from_min_size(ui.cursor().min, egui::vec2(CARD_WIDTH, CARD_HEIGHT));
                    let is_offscreen = !container_rect.intersects(card_expected_rect);

                    let resp = card.render(ui, state, is_offscreen);
                    let card_rect = resp.rect;

                    // Focused card
                    let cx = card_rect.center().x;
                    let offset = visual_center_x - cx;
                    if offset.abs() < closest_offset.abs() {
                        closest_offset = offset;
                        selected_idx = Some(i);

                        let id = card.id();
                        if !state.ui.viewed.contains(&id) {
                            state.ui.viewed.push(id);
                        }
                    }

                    card.draggable = offset.abs() <= 15. || is_single_card;

                    if card.draggable {
                        if card_rect.intersects(top_scan_area) {
                            handle_scan(
                                ui,
                                &mut self.scan_state,
                                state,
                                card,
                                ScanSide::Top,
                                &mut changed,
                            );
                        } else if card_rect.intersects(bot_scan_area) {
                            handle_scan(
                                ui,
                                &mut self.scan_state,
                                state,
                                card,
                                ScanSide::Bottom,
                                &mut changed,
                            );
                        } else {
                            self.scan_state.reset();
                        }
                    }
                    card_resps.push(resp);
                }

                if !is_single_card {
                    ui.add_space(container_width / 2. - CARD_WIDTH / 2.);
                }
            });

            self.handle_input(
                ui,
                state,
                &mut action,
                &mut changed,
                selected_idx,
                card_resps,
            );
        });
        changed
    }

    fn handle_input(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut GameState,
        action: &mut Option<Action>,
        changed: &mut bool,
        selected_idx: Option<usize>,
        card_resps: Vec<egui::Response>,
    ) {
        ui.input(|inp| {
            let delta = inp.events.iter().find_map(|e| match e {
                egui::Event::MouseWheel { unit: _, delta, .. } => Some(*delta),
                _ => None,
            });
            if let Some(delta) = delta {
                if delta.y > 0. {
                    *action = Some(Action::Prev);
                } else if delta.y < 0. {
                    *action = Some(Action::Next);
                }
            }

            if [Key::ArrowLeft, Key::A].iter().any(|k| inp.key_pressed(*k)) {
                *action = Some(Action::Prev);
            } else if [Key::ArrowRight, Key::D]
                .iter()
                .any(|k| inp.key_pressed(*k))
            {
                *action = Some(Action::Next);
            } else if [Key::ArrowUp, Key::W].iter().any(|k| inp.key_pressed(*k)) {
                *action = Some(Action::Up);
            } else if [Key::ArrowDown, Key::S].iter().any(|k| inp.key_pressed(*k)) {
                *action = Some(Action::Down);
            }
        });

        match action {
            Some(action) => match action {
                Action::Next => {
                    if let Some(resp) = selected_idx.and_then(|idx| {
                        let next_idx = idx.saturating_add(1);
                        card_resps.get(next_idx)
                    }) {
                        resp.scroll_to_me_animation(Some(Align::Center), ScrollAnimation::none());
                    }
                }
                Action::Prev => {
                    if let Some(resp) = selected_idx.and_then(|idx| {
                        let next_idx = idx.saturating_sub(1);
                        card_resps.get(next_idx)
                    }) {
                        resp.scroll_to_me_animation(Some(Align::Center), ScrollAnimation::none());
                    }
                }
                Action::Up => {
                    if let Some(card) = selected_idx.and_then(|idx| self.cards.get_mut(idx))
                        && card.is_add_allowed(state)
                    {
                        card.add_scan_done(state);
                        *changed = true;
                    }
                }
                Action::Down => {
                    if let Some(card) = selected_idx.and_then(|idx| self.cards.get_mut(idx))
                        && card.is_rem_allowed(state)
                    {
                        card.rem_scan_done(state);
                        *changed = true;
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
    }

    fn render_scanners(
        &mut self,
        ui: &mut egui::Ui,
        action: &mut Option<Action>,
    ) -> (egui::Rect, egui::Rect) {
        // Hacky: but assume that if we're only showing one card
        // then this is a tooltip.
        let is_single_card = self.cards.len() == 1;
        let order = if is_single_card {
            Order::Tooltip
        } else {
            Order::Middle
        };

        let scanner_height = scanner_height(ui);

        // A bit hacky, but assume the scanners will always be
        // screen-centered horizontally no matter the context.
        let screen_center_x = ui.ctx().content_rect().center().x;

        let cursor = ui.cursor();
        let mid_y = ui.available_height() / 2. - 24.;
        let top = egui::Area::new(ui.id().with("scan-up"))
            .order(order)
            .movable(false)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos((
                screen_center_x,
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

                        match &mut self.scan_state.result {
                            Some((ScanSide::Top, accepted, countdown)) => {
                                if lit_led(ui.painter(), c, *accepted, countdown) {
                                    self.scan_state.result = None;
                                }
                            }
                            _ => {
                                dim_led(ui.painter(), c);
                            }
                        }

                        let lt = ui.cursor().left_bottom() + egui::vec2(6., -7.);
                        let rect =
                            Rect::from_min_size(lt, egui::vec2(SCANNER_WIDTH - 12. - 16., 2.));

                        if matches!(self.scan_state.scanning, Some(ScanSide::Top)) {
                            let progress = self.scan_state.timer.progress();
                            scanning_bar(ui.painter(), rect, progress);
                        }
                    });
            })
            .response
            .rect;

        let bot = egui::Area::new(ui.id().with("scan-down"))
            .order(order)
            .movable(false)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos((
                screen_center_x,
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

                        match &mut self.scan_state.result {
                            Some((ScanSide::Bottom, accepted, countdown)) => {
                                if lit_led(ui.painter(), c, *accepted, countdown) {
                                    self.scan_state.result = None;
                                }
                            }
                            _ => {
                                dim_led(ui.painter(), c);
                            }
                        }

                        let lt = ui.cursor().left_top() + egui::vec2(6., 7.);
                        let rect =
                            Rect::from_min_size(lt, egui::vec2(SCANNER_WIDTH - 12. - 16., 2.));

                        if matches!(self.scan_state.scanning, Some(ScanSide::Bottom)) {
                            let progress = self.scan_state.timer.progress();
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

#[derive(Default, Clone)]
pub struct ScanState {
    scans: u8,
    scanning: Option<ScanSide>,
    timer: Timer,
    result: Option<(ScanSide, bool, u8)>,
}
impl ScanState {
    fn reset(&mut self) {
        self.timer.reset();
        self.scanning = None;
        self.scans = 0;
    }
}

fn handle_scan<C: Scannable>(
    ui: &mut egui::Ui,
    scan: &mut ScanState,
    state: &mut GameState,
    card: &mut Card<C>,
    side: ScanSide,
    changed: &mut bool,
) {
    ui.ctx().request_repaint();
    scan.scanning = Some(side);
    match side {
        ScanSide::Top => {
            if card.is_add_allowed(state)
                && scan.timer.has_elapsed(
                    // Speed up each subsequent scan
                    card.add_scan_time() / ((scan.scans + 1) as f32).sqrt(),
                )
            {
                scan.timer.reset();
                scan.scans = scan.scans.saturating_add(1);
                scan.result = match card.add_scan_done(state) {
                    ScanResult::SuccessContinue | ScanResult::SuccessStop => {
                        Some((ScanSide::Top, true, LED_DURATION))
                    }
                    ScanResult::Rejected => Some((ScanSide::Top, false, LED_DURATION)),
                };
                *changed = true;
            }
        }
        ScanSide::Bottom => {
            if card.is_rem_allowed(state)
                && scan.timer.has_elapsed(
                    // Speed up each subsequent scan
                    card.rem_scan_time() / ((scan.scans + 1) as f32).sqrt(),
                )
            {
                scan.timer.reset();
                scan.scans = scan.scans.saturating_add(1);
                scan.result = match card.rem_scan_done(state) {
                    ScanResult::SuccessContinue | ScanResult::SuccessStop => {
                        Some((ScanSide::Bottom, true, LED_DURATION))
                    }
                    ScanResult::Rejected => Some((ScanSide::Bottom, false, LED_DURATION)),
                };
                *changed = true;
            }
        }
    }
}

enum Action {
    Next,
    Prev,
    Up,
    Down,
}

#[derive(Default, Clone)]
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
    let screen_height = ui.ctx().content_rect().height();
    let is_short = screen_height < 600.;
    egui::Frame::NONE
        .inner_margin(Margin::symmetric(6, if is_short { 14 } else { 16 }))
        .show(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            h_center(ui, "controls-buttons", |tui| {
                tui.ui(|ui| {
                    let resp = padded_label(ui, "◄");
                    if resp.interact(Sense::click()).clicked() {
                        *action = Some(Action::Prev);
                    }
                });
                tui.ui(|ui| {
                    let resp = padded_label(ui, "►");
                    if resp.interact(Sense::click()).clicked() {
                        *action = Some(Action::Next);
                    }
                });
            });
        })
        .response
}

fn padded_label(ui: &mut egui::Ui, text: &str) -> egui::Response {
    egui::Frame::NONE
        .inner_margin(6)
        .show(ui, |ui| {
            let label = egui::RichText::new(text)
                .size(18.)
                .color(Color32::from_gray(230));
            ui.label(label);
        })
        .response
}
