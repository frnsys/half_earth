use crate::{
    consts,
    icons,
    state,
    t,
    views::{cards::MiniNPC, tip, HasTip},
    with_state,
};
use hes_engine::events::Flag;
use leptos::*;
use std::{collections::HashMap, sync::OnceLock};

#[component]
pub fn Parliament() -> impl IntoView {
    let total_seats =
        consts::PARLIAMENT_SEATS.iter().sum::<usize>();
    let suspended =
        state!(flags.contains(&Flag::ParliamentSuspended));
    let npcs = with_state!(|state, _ui| {
        state
            .npcs
            .iter()
            .filter(|npc| !npc.locked)
            .cloned()
            .map(create_rw_signal)
            .collect::<Vec<_>>()
    });

    let (extra_seats, set_extra_seats) =
        create_signal::<HashMap<usize, usize>>(
            HashMap::default(),
        );
    let (coalition_seats, set_coalition_seats) =
        create_signal(0);

    struct Seat {
        name: String,
        color: String,
        is_ally: bool,
    }
    let seats = with_state!(|state, _ui| {
        struct Seats {
            id: usize,
            name: String,
            color: String,
            is_ally: bool,
            seats: usize,
        }

        let mut used_seats = 0;
        let mut seats = npcs()
            .into_iter()
            .map(|npc| {
                let npc = npc.get();
                let seats = (npc.seats * total_seats as f32)
                    .floor()
                    as usize;
                used_seats += seats;
                Seats {
                    id: npc.id,
                    name: npc.name.clone(),
                    color: "#000000".into(), // TODO get from NPCFlavor
                    is_ally: npc.is_ally(),
                    seats,
                }
            })
            .collect::<Vec<_>>();

        // TODO do this at the start of each planning cycle
        // and store it on the NPC

        // Assign extra seats randomly
        // We generate the assignment based on the current year
        // so that it's consistent
        let mut extra_seats = total_seats - used_seats;
        let mut rng = mulberry32(state.world.year as u16);
        set_extra_seats.update(|extras| {
            extras.clear();
            while extra_seats > 0 {
                let idx = (rng() * seats.len() as f64).floor()
                    as usize;
                let s = &mut seats[idx];
                s.seats += 1;
                let e = extras.entry(s.id).or_default();
                *e += 1;
                extra_seats -= 1;
            }
        });

        let mut coalition_seats = 0;
        for seats in &seats {
            if seats.is_ally {
                coalition_seats += seats.seats;
            }
        }
        set_coalition_seats.set(coalition_seats);

        let mut individual_seats =
            seats.into_iter().flat_map(|seats| {
                (0..seats.seats).map(move |_| Seat {
                    name: seats.name.clone(),
                    color: seats.color.clone(),
                    is_ally: seats.is_ally,
                })
            });
        consts::PARLIAMENT_SEATS
            .iter()
            .map(|n_seats| {
                individual_seats
                    .by_ref()
                    .take(*n_seats)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });

    let tip = tip(icons::POLITICAL_CAPITAL, t!("How many seats your coalition has. Draw parties to your coalition by implementing projects they support."));

    let seat_elems = move || {
        seats()
            .into_iter()
            .map(|col| {
                let seats: Vec<_> = col
                    .into_iter()
                    .map(|seat| {
                        let img = format!("/assets/characters/{}.png", seat.name);
                        view! {
                            <div class:coalitionSeat=seat.is_ally>
                                <img src=img/>
                            </div>
                        }
                    })
                    .collect();

                view! { <div>{seats}</div> }
            })
            .collect::<Vec<_>>()
    };
    let coalition_text = move || {
        t!("Your coalition has {coalitionSeats}/{totalSeats} seats.",
            coalitionSeats: coalition_seats.get(),
            totalSeats: total_seats)
    };

    view! {
        <div class="planning--page parliament">
            <Show when=move || suspended.get()>
                <div class="parliament-suspended">
                    {t!("Parliament Suspended")}
                </div>
            </Show>
            <div
                class="parliament-seats"
                class:parliament-suspended-fade=suspended
            >
                {seat_elems}
            </div>
            <HasTip tip>
                <div
                    class="coalition-seats"
                    class:parliament-suspended-fade=suspended
                >
                    {coalition_text}
                </div>
            </HasTip>

            <div class="minicard-grid">
                <For
                    each=move || npcs()
                    key=|npc| npc.get().id
                    children=move |npc| {
                        view! {
                            <div class="minicard-grid-item">
                                <MiniNPC npc/>
                            </div>
                        }
                    }
                />

            </div>
        </div>
    }
}

// Seedable RNG.
// https://stackoverflow.com/a/47593316/1097920
fn mulberry32(seed: u16) -> impl FnMut() -> f64 {
    // Different seed for each game.
    static GAME_SEED: OnceLock<u16> = OnceLock::new();
    let game_seed: &u16 = GAME_SEED.get_or_init(|| {
        (js_sys::Math::random() * u16::MAX as f64).floor()
            as u16
    });

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
