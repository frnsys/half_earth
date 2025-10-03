use std::{collections::BTreeMap, sync::OnceLock};

use egui::{Color32, Pos2, Sense};
use hes_engine::{Flag, Id, NPC, State, flavor::Speaker};
use rust_i18n::t;

use crate::{
    consts,
    display::{icon_from_slug, icons, speaker_icon},
    image,
    views::{
        CardState,
        game::Card,
        parts::set_full_bg_image,
        tip,
        tips::add_tip,
    },
};

#[derive(Debug)]
struct Seat {
    name: String,
    color: String,
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
        let total_seats =
            consts::PARLIAMENT_SEATS.iter().sum::<usize>();

        let year = state.world.year;
        let npcs =
            state.npcs.unlocked().cloned().collect::<Vec<_>>();

        let seats =
            calculate_seats(year as u16, &npcs, total_seats);

        let mut coalition_seats = 0;
        for seats in &seats {
            if seats.is_ally {
                coalition_seats += seats.seats;
            }
        }

        let mut individual_seats =
            seats.into_iter().flat_map(|seats| {
                (0..seats.seats).map(move |_| Seat {
                    name: seats.name.clone(),
                    color: seats.color.clone(),
                    is_ally: seats.is_ally,
                })
            });
        let seats = consts::PARLIAMENT_SEATS
            .iter()
            .map(|n_seats| {
                individual_seats
                    .by_ref()
                    .take(*n_seats)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            seats,
            total_seats,
            coalition_seats,
            card: None,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
    ) {
        set_full_bg_image(
            ui,
            image!("backgrounds/parliament.png"),
            egui::vec2(1600., 1192.),
        );

        let suspended =
            state.flags.contains(&Flag::ParliamentSuspended);

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

        // TODO fade if suspended, etc
        if suspended {
            ui.label(t!("Parliament Suspended"));
        }

        ui.vertical_centered(|ui| {
            for row in &self.seats {
                ui.horizontal(|ui| {
                    for seat in row {
                        let speaker = as_speaker(&seat.name);
                        let image = speaker_icon(&speaker);
                        if seat.is_ally {
                            // TODO
                            ui.image(image);
                        } else {
                            ui.image(image);
                        }
                    }
                });
            }
        });

        add_tip(tip, ui.label(coalition_text));

        for npc in state.npcs.unlocked() {
            let resp = render_npc(ui, npc, self.total_seats);
            if resp.interact(Sense::click()).clicked() {
                self.card = Some(Card::new(npc.clone()));
            }
        }

        if let Some(card) = &mut self.card {
            // TODO
            let viewed = Default::default();
            let plan_changes = Default::default();
            let queued_upgrades = Default::default();
            let process_mix_changes = Default::default();
            let process_points = Default::default();
            let ctx = CardState {
                state: state,
                viewed: &viewed,
                plan_changes: &plan_changes,
                queued_upgrades: &queued_upgrades,
                process_mix_changes: &process_mix_changes,
                process_points: &process_points,
            };

            card.render(ui, &ctx, false);
        }
    }
}

struct Seats {
    id: Id,
    name: String,
    color: String,
    is_ally: bool,
    seats: usize,
}

fn calculate_seats(
    year: u16,
    npcs: &[NPC],
    total_seats: usize,
) -> Vec<Seats> {
    let mut used_seats = 0;
    let mut seats = npcs
        .into_iter()
        .map(|npc| {
            let color = npc.flavor.color.clone();
            let seats = (npc.seats * total_seats as f32).floor()
                as usize;
            used_seats += seats;
            Seats {
                id: npc.id,
                name: npc.name.clone(),
                color,
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
        let idx = (rng() * seats.len() as f64).floor().max(0.)
            as usize;
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
    let game_seed: &u16 =
        GAME_SEED.get_or_init(|| fastrand::u16(0..u16::MAX));

    // Combine the game seed with the provided seed.
    let mut state: u32 = seed as u32 * (*game_seed as u32);
    move || {
        state = state.wrapping_add(0x6D2B79F5);
        let mut t = state;
        t = t.wrapping_mul(t ^ (t >> 15));
        t = t.wrapping_mul(t | 1);
        t ^= t.wrapping_add(
            t.wrapping_mul(t ^ (t >> 7)).wrapping_mul(t | 61),
        );
        ((t ^ (t >> 14)) as f64) / 4294967296.0
    }
}

pub fn as_speaker(name: &str) -> Speaker {
    match name {
        "The Malthusian" => Speaker::TheMalthusian,
        "The Utopian" => Speaker::TheUtopian,
        "The Consumerist" => Speaker::TheConsumerist,
        "The Posadist" => Speaker::ThePosadist,
        "The Fanonist" => Speaker::TheFanonist,
        "The Ecofeminist" => Speaker::TheEcofeminist,
        "The Authoritarian" => Speaker::TheAuthoritarian,
        "The Accelerationist" => Speaker::TheAccelerationist,
        "The Environmentalist" => Speaker::TheEnvironmentalist,
        "The Animal Liberationist" => {
            Speaker::TheAnimalLiberationist
        }
        _ => Speaker::Gossy,
    }
}

fn render_npc(
    ui: &mut egui::Ui,
    npc: &NPC,
    total_seats: usize,
) -> egui::Response {
    // TODO scale text?

    let seats = (npc.seats * total_seats as f32).floor()
        as usize
        + npc.extra_seats;

    let speaker = as_speaker(&npc.name);
    let portrait = speaker_icon(&speaker);

    ui.vertical_centered(|ui| {
        ui.image(portrait);
        ui.label(&npc.name);

        let color = Color32::from_hex(&npc.flavor.color)
            .expect("is valid color");

        let side = 8.;
        let spacing = 1.;
        let width = (side * seats as f32)
            + (spacing * (seats - 1) as f32);
        let size = egui::vec2(width, side);
        let (rect, _) =
            ui.allocate_exact_size(size, Sense::empty());

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
    })
    .response
}
