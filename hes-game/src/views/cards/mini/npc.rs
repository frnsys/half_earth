use crate::{
    consts,
    icons,
    t,
    util::{scale_text, to_ws_el},
};

use super::{
    super::{kinds::NPCCard, *},
    MiniCard,
};
use hes_engine::NPC;
use leptos::*;

#[component]
pub fn MiniNPC(
    #[prop(into)] npc: Signal<NPC>,
) -> impl IntoView {
    let total_seats =
        consts::PARLIAMENT_SEATS.iter().sum::<usize>();

    let name_ref = create_node_ref::<html::Div>();
    create_effect(move |_| {
        if let Some(name_ref) = name_ref.get() {
            scale_text(to_ws_el(name_ref), 11);
        }
    });

    let portrait = move || {
        npc.with(|npc| {
            format!("/assets/characters/{}.webp", npc.name)
        })
    };
    let name = move || npc.with(|npc| t!(&npc.name));
    let faction_seats = move || {
        npc.with(|npc| {
            let seats = (npc.seats * total_seats as f32).floor() as usize + npc.extra_seats;
            (0..seats)
                .map(|_| {
                    view! { <div style:background=&npc.flavor.color></div> }
                })
                .collect::<Vec<_>>()
        })
    };
    let is_ally = move || npc.with(|npc| npc.is_ally());

    view! {
        <MiniCard>
            <Body slot>
                <div class="mini-character">
                    <img src=portrait/>
                </div>
                <div class="mini-npc-name">{name}</div>
                <div class="mini-npc-seats">{faction_seats}</div>
                <Show when=is_ally>
                    <div class="mini-npc-tag npc-tag">
                        <img src=icons::ALLY/>
                        {t!("Ally")}
                    </div>
                </Show>
            </Body>
            <Expanded slot>
                <NPCCard npc/>
            </Expanded>
        </MiniCard>
    }
}
