use std::{collections::HashMap, ops::Deref};

use super::{Dialogue, DisplayEvent};
use crate::{
    t,
    views::{tip, Effects, Events, HasTip, Help},
};
use leptos::*;

#[component]
pub fn Event(
    #[prop(into)] event: Signal<DisplayEvent>,
    #[prop(into)] on_done: Callback<()>,
    #[prop(into)] on_advance: Callback<()>,
) -> impl IntoView {
    on_cleanup(|| {
        // TODO
        // settings.hide_help[factor_tip] = true
    });

    let event_card = move || {
        let image_info = with!(|event| {
            event.flavor.image.as_ref().map(move |image| {
                let image_url = format!(
                    "url(/assets/content/images/{})",
                    image.fname
                );
                (image_url, image.attribution.clone())
            })
        });
        image_info.map(|(url, attrib)| {
            let factor_tip = t!("The factors behind this event.↓");
            let (arc, name, factors_list) = with!(|event| {
                let arc = t!(&event.flavor.arc);
                let name = t!(&event.name);
                let factors_list = event.factors
                    .iter()
                    .cloned()
                    .map(|(icon, factor)| {
                        let tip = tip(icon, factor.to_string());
                        view! {
                            <HasTip tip>
                                <img class="event--factor" src=icon/>
                                </HasTip>
                        }
                    })
                .collect::<Vec<_>>();
                (arc, name, factors_list)
            });
            let show_effects = move || with!(|event| event.has_visible_effects());
            let effects = move || with!(|event| event.effects.clone());
            view! {
                <div
                    class="event--body"
                    style:background-image=url
                >
                    <Help text=factor_tip x=0.55 y=-18.0 center=false/>
                    <div class="arc">{arc}</div>
                    <div class="event--factors">{factors_list}</div>
                    <div class="image-attribution">
                        {t!("Image:")}" "{attrib}
                    </div>
                    <div class="event--name">{name}</div>
                    <Show when=show_effects>
                        <div class="event--effects">
                            <Effects effects/>
                        </div>
                    </Show>
                </div>
            }
        })
    };

    let ctx = move || {
        with!(|event| {
            let mut ctx = HashMap::default();
            if let Some((_, name)) = &event.region {
                ctx.insert(
                    "region".to_string(),
                    name.to_string(),
                );
            }
            ctx
        })
    };

    let dialogue =
        move || with!(|event| event.flavor.dialogue.clone());

    view! {
        <div class="event">
            {event_card}
            <Dialogue
                dialogue=dialogue
                context=ctx
                on_advance=on_advance
                on_done=on_done
            />
        </div>
    }
}
