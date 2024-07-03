use crate::{
    consts, icons, t,
    util::{scale_text, to_ws_el},
};

use super::super::region::RegionCard;
use super::*;
use hes_engine::regions::Region;
use leptos::*;

#[component]
pub fn MiniRegion(#[prop(into)] region: Signal<Region>) -> impl IntoView {
    let image = move || {
        region.with(|region| format!("url(/public/assets/content/{})", region.flavor.image.fname))
    };
    let seceded = move || region.with(|region| region.seceded);

    view! {
        <MiniCard>
            <Body slot>
                <Show when=seceded>
                    <div class="seceded-label">{t!("Seceded")}</div>
                </Show>
                <div
                    class="minicard-background"
                    style:background-image=image
                    class:seceded=seceded
                ></div>
            </Body>
            <Expanded slot>
                <RegionCard region/>
            </Expanded>
        </MiniCard>
    }
}
