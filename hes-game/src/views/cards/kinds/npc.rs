use super::super::card::*;
use crate::{
    consts,
    icons,
    t,
    views::{tip, HasTip},
};
use hes_engine::npcs::NPC;
use leptos::*;

#[component]
pub fn NPCCard(
    #[prop(into)] npc: Signal<NPC>,
) -> impl IntoView {
    let hearts = move || {
        npc.with(|npc| {
            (0..consts::MAX_RELATIONSHIP)
                .map(|i| {
                    let icon = if i as f32 <= npc.relationship {
                        icons::RELATIONSHIP
                    } else {
                        icons::RELATIONSHIP_EMPTY
                    };
                    view! { <img src=icon/> }
                })
                .collect::<Vec<_>>()
        })
    };

    let rel_tip = move || {
        npc.with(|npc| {
            tip(icons::RELATIONSHIP, t!("Your relationship with {name}. Increase it by implementing projects they like. At 5 hearts or more they will join your coalition.", name: t!(&npc.name)))
        })
    };
    let portrait = move || {
        npc.with(|npc| {
            format!("/assets/characters/{}.webp", npc.name)
        })
    };
    let rel_icon = move || {
        npc.with(|npc| match npc.relationship_name() {
            "Ally" => icons::ALLY,
            "Friendly" => icons::FRIENDLY,
            "Nemesis" => icons::NEMESIS,
            "Neutral" => icons::NEUTRAL,
            _ => unreachable!(),
        })
    };
    let name = move || npc.with(|npc| t!(&npc.name));
    let rel_name =
        move || npc.with(|npc| t!(&npc.relationship_name()));
    let effects = move || {
        npc.with(|npc| {
            let effects = t!(&npc.flavor.effects);
            if npc.is_ally() {
                view! { <p class="npc-effect active" inner_html=effects></p> }.into_view()
            } else {
                let tip = tip(icons::RELATIONSHIP, t!("Improve your relationship with {name} to activate this ability.", name: t!(&npc.name)));
                view! {
                    <HasTip tip>
                        <p class="npc-effect inactive" inner_html=effects></p>
                    </HasTip>
                }.into_view()
            }
        })
    };
    let description =
        move || npc.with(|npc| t!(&npc.flavor.description));
    let likes = move || npc.with(|npc| t!(&npc.flavor.likes));
    let dislikes =
        move || npc.with(|npc| t!(&npc.flavor.dislikes));

    view! {
        <Card class="npc" background="#724680">
            <Header slot>
                <div>{t!("Parliament")}</div>
                <HasTip tip=rel_tip.into_signal()>
                    <div>{hearts}</div>
                </HasTip>
            </Header>
            <Figure slot>
                <img src=portrait/>
            </Figure>
            <Name slot>
                <div class="npc-tag">
                    <img src=rel_icon/>
                    {rel_name}
                </div>
                {name}
            </Name>
            <Body slot>{effects}</Body>
            <TopBack slot>
                <img src=portrait/>
                <p class="card-desc npc-desc">{description}</p>
            </TopBack>
            <BottomBack slot>
                <div class="likes-dislikes">
                    <div>
                        <h3>{t!("Likes")}</h3>
                        <p>{likes}</p>
                    </div>
                    <div>
                        <h3>{t!("Dislikes")}</h3>
                        <p>{dislikes}</p>
                    </div>
                </div>
            </BottomBack>
        </Card>
    }
}
