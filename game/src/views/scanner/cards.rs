use std::time::Instant;

use crate::{
    parts::raised_frame,
    state::GameState,
    views::{
        cards::{AsCard, CARD_HEIGHT, CARD_WIDTH, Card},
        scanner::Scannable,
    },
};

use egui::{
    Align,
    Align2,
    Color32,
    Key,
    Order,
    style::ScrollAnimation,
};

pub struct Cards<C: AsCard + Scannable> {
    cards: Vec<Card<C>>,
    scan_timer: Timer,
}
impl<C: AsCard + Scannable> Cards<C> {
    pub fn new(cards: impl Iterator<Item = C>) -> Self {
        Self {
            cards: cards.map(|card| Card::new(card)).collect(),
            scan_timer: Timer::default(),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut GameState,
    ) -> bool {
        let mut changed = false;

        let mid_y = ui.available_height() / 2.;

        let cursor = ui.cursor();
        const GAP: f32 = 24.;
        let top_scan_area = egui::Area::new("scan-up".into())
            .order(Order::Middle)
            .movable(false)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos((
                cursor.left() + ui.available_width() / 2.,
                cursor.top() + mid_y - CARD_HEIGHT / 2. - GAP,
            ))
            .show(ui.ctx(), |ui| {
                raised_frame()
                    .colors(
                        Color32::WHITE,
                        Color32::from_rgb(0xdc, 0xe0, 0xe6),
                        Color32::from_rgb(0xfa, 0xfc, 0xff),
                    )
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
            .order(Order::Middle)
            .movable(false)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos((
                cursor.left() + ui.available_width() / 2.,
                cursor.top()
                    + mid_y
                    + CARD_HEIGHT / 2.
                    + GAP
                    + scanner_height,
            ))
            .show(ui.ctx(), |ui| {
                raised_frame()
                    .colors(
                        Color32::WHITE,
                        Color32::from_rgb(0xdc, 0xe0, 0xe6),
                        Color32::from_rgb(0xfa, 0xfc, 0xff),
                    )
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
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_height(ui.available_height());

            let width = ui.available_width();
            ui.set_max_width(width);
            let half_width = width / 2.;

            ui.add_space(
                ui.available_height() / 2. - CARD_HEIGHT / 2.,
            );

            let mut selected_idx = None;
            let mut card_resps = vec![];

            ui.horizontal(|ui| {
                ui.add_space(half_width);

                ui.style_mut().spacing.item_spacing.x = 18.;
                for (i, card) in
                    self.cards.iter_mut().enumerate()
                {
                    let left_pos = ui.cursor().left();
                    let is_offscreen = (left_pos + CARD_WIDTH)
                        < 0.
                        || left_pos > width;

                    let resp =
                        card.render(ui, state, is_offscreen);
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
                    if offset.abs() <= 5. {
                        card.draggable = true;
                    } else {
                        card.draggable = false;
                    }

                    if card.draggable {
                        if card_rect.intersects(top_scan_area) {
                            if card.is_add_allowed(state)
                                && self.scan_timer.has_elapsed(
                                    card.add_scan_time(),
                                )
                            {
                                self.scan_timer.reset();
                                card.add_scan_done(state);
                                changed = true;
                            }
                        } else if card_rect
                            .intersects(bot_scan_area)
                        {
                            if card.is_rem_allowed(state)
                                && self.scan_timer.has_elapsed(
                                    card.rem_scan_time(),
                                )
                            {
                                self.scan_timer.reset();
                                card.rem_scan_done(state);
                                changed = true;
                            }
                        } else {
                            self.scan_timer.reset();
                        }
                    }

                    card_resps.push(resp);
                }

                ui.add_space(half_width);
            });

            let action = ui.input(|inp| {
                if [Key::ArrowLeft, Key::A]
                    .iter()
                    .any(|k| inp.key_pressed(*k))
                {
                    Some(Action::Prev)
                } else if [Key::ArrowRight, Key::D]
                    .iter()
                    .any(|k| inp.key_pressed(*k))
                {
                    Some(Action::Next)
                } else if [Key::ArrowUp, Key::W]
                    .iter()
                    .any(|k| inp.key_pressed(*k))
                {
                    Some(Action::Up)
                } else if [Key::ArrowDown, Key::S]
                    .iter()
                    .any(|k| inp.key_pressed(*k))
                {
                    Some(Action::Down)
                } else {
                    None
                }
            });

            match action {
                Some(action) => match action {
                    Action::Next => {
                        if let Some(resp) = selected_idx
                            .and_then(|idx| {
                                let next_idx =
                                    idx.saturating_add(1);
                                card_resps.get(next_idx)
                            })
                        {
                            resp.scroll_to_me_animation(
                                Some(Align::Center),
                                ScrollAnimation::none(),
                            );
                        }
                    }
                    Action::Prev => {
                        if let Some(resp) = selected_idx
                            .and_then(|idx| {
                                let next_idx =
                                    idx.saturating_sub(1);
                                card_resps.get(next_idx)
                            })
                        {
                            resp.scroll_to_me_animation(
                                Some(Align::Center),
                                ScrollAnimation::none(),
                            );
                        }
                    }
                    Action::Up => {
                        if let Some(card) = selected_idx
                            .and_then(|idx| {
                                self.cards.get_mut(idx)
                            })
                        {
                            if card.is_add_allowed(state) {
                                card.add_scan_done(state);
                                changed = true;
                            }
                        }
                    }
                    Action::Down => {
                        if let Some(card) = selected_idx
                            .and_then(|idx| {
                                self.cards.get_mut(idx)
                            })
                        {
                            if card.is_rem_allowed(state) {
                                card.rem_scan_done(state);
                                changed = true;
                            }
                        }
                    }
                },
                None => {
                    if let Some(resp) =
                        selected_idx.map(|idx| &card_resps[idx])
                    {
                        resp.scroll_to_me(Some(Align::Center));
                    }
                }
            }
        });
        changed
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
