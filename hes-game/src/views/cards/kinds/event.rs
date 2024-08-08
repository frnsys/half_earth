use crate::{
    icons,
    state::Settings,
    t,
    util::ImageExt,
    views::{tip, DisplayEvent, Effects, HasTip, Help},
};
use leptos::*;

#[component]
pub fn EventCard(
    #[prop(into)] event: Signal<DisplayEvent>,
) -> impl IntoView {
    let factor_tip = "The factors behind this event.↓";
    let (_, set_settings) = Settings::rw();
    on_cleanup(move || {
        set_settings.update(|settings| {
            settings.read_help.push(factor_tip.to_string());
        });
    });

    let image_info = with!(|event| {
        event.flavor.image.as_ref().map(move |image| {
            (image.src(), image.attribution.clone())
        })
    });
    image_info.map(|(image, attrib)| {
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

        let attribution = move || {
            if attrib.trim().is_empty() {
                "".into()
            } else {
                format!("{} {attrib}", t!("Image:"))
            }
        };

        view! {
            <div
                class="event--body"
                style:background-image={background}
            >
                <Help text={t!(factor_tip)} x=0.55 y=-18.0 center=false/>
                <div class="arc">{arc}</div>
                <div class="event--factors">{factors_list}</div>
                <div class="image-attribution">
                    {attribution}
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
