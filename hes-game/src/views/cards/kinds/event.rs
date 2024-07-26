use super::super::card::*;
use crate::{
    display::{self, AsText},
    i18n,
    icons::{self, HasIcon},
    t,
    util::ImageExt,
    views::{
        intensity::{self, IntensityIcon, Variable},
        tip,
        DisplayEvent,
        Effects,
        HasTip,
        Help,
    },
    with_state,
};
use hes_engine::events::Event;
use leptos::*;

#[component]
pub fn EventCard(
    #[prop(into)] event: Signal<DisplayEvent>,
) -> impl IntoView {
    on_cleanup(|| {
        // TODO
        // settings.hide_help[factor_tip] = true
    });

    let image_info = with!(|event| {
        event.flavor.image.as_ref().map(move |image| {
            (image.src(), image.attribution.clone())
        })
    });
    image_info.map(|(image, attrib)| {
        let factor_tip = t!("The factors behind this event.↓");
        let (arc, name, factors_list) = with!(|event| {
            let arc = t!(&event.flavor.arc);
            let name = t!(&event.name);
            let factors_list = event.factors
                .iter()
                .cloned()
                .map(|(icon, factor)| {
                    let tip = tip(icons::to_static(&icon).unwrap(), factor.to_string());
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
        let background = move || {
            format!("url('{image}')")
        };

        view! {
            <div
                class="event--body"
                style:background-image={background}
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
}
