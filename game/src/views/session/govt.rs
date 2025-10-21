use std::{collections::BTreeMap, sync::OnceLock};

use egui::{Color32, CornerRadius, Margin, Pos2, Sense, Shadow, Stroke};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{Flag, Id, NPC, State};
use rust_i18n::t;

use crate::{
    consts,
    display::{as_speaker, icons, speaker_icon},
    parts::{center_center, get_sizing, h_center, overlay, raised_frame, set_full_bg_image},
    state::GameState,
    tips::{add_tip, tip},
    views::cards::Card,
};

#[derive(Debug)]
struct Seat {
    name: String,
    is_ally: bool,
}

pub struct Parliament {
    seats: Vec<Vec<Seat>>,
    total_seats: usize,
    coalition_seats: usize,
    card: Option<Card<NPC>>,
}
impl Parliament {
    pub fn new(state: &State) -> Self {
        let total_seats = consts::PARLIAMENT_SEATS.iter().sum::<usize>();

        let year = state.world.year;
        let npcs = state.npcs.unlocked().cloned().collect::<Vec<_>>();

        let seats = calculate_seats(year as u16, &npcs, total_seats);

        let mut coalition_seats = 0;
        for seats in &seats {
            if seats.is_ally {
                coalition_seats += seats.seats;
            }
        }

        let mut individual_seats = seats.into_iter().flat_map(|seats| {
            (0..seats.seats).map(move |_| Seat {
                name: seats.name.clone(),
                is_ally: seats.is_ally,
            })
        });
        let seats = consts::PARLIAMENT_SEATS
            .iter()
            .map(|n_seats| individual_seats.by_ref().take(*n_seats).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            seats,
            total_seats,
            coalition_seats,
            card: None,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &GameState) {
        set_full_bg_image(
            ui,
            hes_images::background_image("parliament.webp"),
            egui::vec2(1600., 1192.),
        );

        // Note: this needs to go early so that
        // clicking to show the card doesn't close
        // the overlay in the same frame.
        if let Some(card) = &mut self.card {
            let should_close = overlay(ui.ctx(), |ui| card.render(ui, state, false));
            if should_close {
                self.card = None;
            }
        }

        let suspended = state.flags.contains(&Flag::ParliamentSuspended);

        let tip = tip(
            icons::POLITICAL_CAPITAL,
            t!(
                "How many seats your coalition has. Draw parties to your coalition by implementing projects they support."
            ),
        );

        let coalition_text = t!(
            "Your coalition has %{coalitionSeats}/%{totalSeats} seats.",
            coalitionSeats = self.coalition_seats,
            totalSeats = self.total_seats
        );

        ui.vertical_centered(|ui| {
            if suspended {
                ui.set_opacity(0.5);
            }

            let resp = egui::Frame::NONE
                .fill(Color32::from_rgb(0x96, 0x5F, 0xA9))
                .inner_margin(Margin::symmetric(8, 12))
                .outer_margin(Margin::symmetric(0, 16))
                .stroke(Stroke::new(1., Color32::from_rgb(0x76, 0x44, 0x87)))
                .shadow(Shadow {
                    offset: [2, 2],
                    blur: 8,
                    spread: 2,
                    color: Color32::from_black_alpha(32),
                })
                .corner_radius(CornerRadius {
                    nw: 5,
                    ne: 5,
                    sw: 255,
                    se: 255,
                })
                .show(ui, |ui| {
                    ui.set_width(320.);
                    ui.set_height(140.);
                    for (i, row) in self.seats.iter().enumerate() {
                        h_center(ui, &format!("govt-row-{i}"), |tui| {
                            tui.ui(|ui| {
                                ui.horizontal(|ui| {
                                    for seat in row {
                                        let speaker = as_speaker(&seat.name);
                                        let image = speaker_icon(&speaker);
                                        let fill = if seat.is_ally {
                                            Color32::from_rgb(0xfc, 0xe2, 0x97)
                                        } else {
                                            Color32::TRANSPARENT
                                        };
                                        let stroke = if seat.is_ally {
                                            Color32::from_rgb(0xED, 0xB1, 0x40)
                                        } else {
                                            Color32::TRANSPARENT
                                        };
                                        egui::Frame::NONE
                                            .fill(fill)
                                            .stroke(Stroke::new(1., stroke))
                                            .corner_radius(3)
                                            .show(ui, |ui| {
                                                ui.add(
                                                    image.fit_to_exact_size(egui::Vec2::splat(28.)),
                                                );
                                            });
                                    }
                                });
                            });
                        });
                    }
                })
                .response;

            if suspended {
                ui.place(resp.rect, |ui: &mut egui::Ui| {
                    ui.set_opacity(1.);
                    center_center(ui, "suspended", |tui| {
                        tui.ui(|ui| {
                            egui::Frame::NONE
                                .fill(Color32::RED)
                                .inner_margin(3)
                                .show(ui, |ui| {
                                    ui.set_width(resp.rect.width());
                                    ui.label(
                                        egui::RichText::new(
                                            t!("Parliament Suspended").to_uppercase(),
                                        )
                                        .heading(),
                                    );
                                })
                                .response
                        })
                    })
                });
            }
        });

        ui.vertical_centered(|ui| {
            if !suspended {
                add_tip(
                    tip,
                    ui.label(
                        egui::RichText::new(coalition_text)
                            .heading()
                            .color(Color32::WHITE),
                    ),
                );
            }

            ui.add_space(32.);

            let npcs: Vec<_> = state.npcs.unlocked().collect();
            let sizing = get_sizing(ui);
            let chunks = if sizing.is_small { 2 } else { 3 };
            for (i, row) in npcs.chunks(chunks).enumerate() {
                ui.add_space(32.);
                h_center(ui, &format!("npc-row-{i}"), |tui| {
                    tui.ui(|ui| {
                        ui.horizontal(|ui| {
                            for npc in row {
                                let resp = render_npc(ui, npc, self.total_seats);
                                if resp.interact(Sense::click()).clicked() {
                                    self.card = Some(Card::new((*npc).clone()));
                                }
                            }
                        });
                    });
                });
            }

            ui.add_space(32.);
        });
    }
}

struct Seats {
    id: Id,
    name: String,
    is_ally: bool,
    seats: usize,
}

fn calculate_seats(year: u16, npcs: &[NPC], total_seats: usize) -> Vec<Seats> {
    let mut used_seats = 0;
    let mut seats = npcs
        .into_iter()
        .map(|npc| {
            let seats = (npc.seats * total_seats as f32).floor() as usize;
            used_seats += seats;
            Seats {
                id: npc.id,
                name: npc.name.clone(),
                is_ally: npc.is_ally(),
                seats,
            }
        })
        .collect::<Vec<_>>();

    // Assign extra seats randomly
    // We generate the assignment based on the current year
    // so that it's consistent
    let mut extra_seats = total_seats - used_seats;
    let mut rng = mulberry32(year);
    let mut extras: BTreeMap<Id, usize> = BTreeMap::default();
    while extra_seats > 0 {
        let idx = (rng() * seats.len() as f64).floor().max(0.) as usize;
        let s = &mut seats[idx];
        s.seats += 1;
        let e = extras.entry(s.id).or_default();
        *e += 1;
        extra_seats -= 1;
    }

    seats
}

// Seedable RNG.
// https://stackoverflow.com/a/47593316/1097920
fn mulberry32(seed: u16) -> impl FnMut() -> f64 {
    // Different seed for each game.
    static GAME_SEED: OnceLock<u16> = OnceLock::new();
    let game_seed: &u16 = GAME_SEED.get_or_init(|| fastrand::u16(0..u16::MAX));

    // Combine the game seed with the provided seed.
    let mut state: u32 = seed as u32 * (*game_seed as u32);
    move || {
        state = state.wrapping_add(0x6D2B79F5);
        let mut t = state;
        t = t.wrapping_mul(t ^ (t >> 15));
        t = t.wrapping_mul(t | 1);
        t ^= t.wrapping_add(t.wrapping_mul(t ^ (t >> 7)).wrapping_mul(t | 61));
        ((t ^ (t >> 14)) as f64) / 4294967296.0
    }
}

fn render_npc(ui: &mut egui::Ui, npc: &NPC, total_seats: usize) -> egui::Response {
    let seats = (npc.seats * total_seats as f32).floor() as usize + npc.extra_seats;

    let speaker = as_speaker(&npc.name);
    let portrait = speaker_icon(&speaker);

    raised_frame()
        .colors(
            Color32::from_rgb(0xB0, 0x93, 0xBA),
            Color32::from_rgb(0x4e, 0x2c, 0x59),
            Color32::from_rgb(0x96, 0x5F, 0xA9),
        )
        .margin(Margin {
            left: 8,
            right: 8,
            top: -32,
            bottom: 24,
        })
        .show(ui, |ui| {
            ui.set_width(150.);
            ui.vertical_centered(|ui| {
                ui.add(portrait.fit_to_exact_size(egui::Vec2::splat(64.)));
                ui.label(&npc.name);

                let color = Color32::from_hex(&npc.flavor.color).expect("is valid color");

                let side = 8.;
                let spacing = 1.;
                let width = (side * seats as f32) + (spacing * (seats - 1) as f32);
                let size = egui::vec2(width, side);
                let (rect, _) = ui.allocate_exact_size(size, Sense::empty());

                let painter = ui.painter();
                let mut x = rect.left();
                for _ in 0..seats {
                    let pip = egui::Rect::from_min_max(
                        Pos2::new(x, rect.top()),
                        Pos2::new(x + side, rect.bottom()),
                    );
                    painter.rect_filled(pip, 0, color);
                    x += side + spacing;
                }

                if npc.is_ally() {
                    ui.image(icons::ALLY);
                    ui.label(t!("Ally"));
                }
            });
        })
}
