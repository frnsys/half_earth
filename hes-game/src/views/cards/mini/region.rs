use crate::{t, util::ImageExt};

use super::{
    super::{kinds::RegionCard, *},
    MiniCard,
};
use hes_engine::regions::Region;
use leptos::*;

#[component]
pub fn MiniRegion(
    #[prop(into)] region: Signal<Region>,
) -> impl IntoView {
    let image =
        move || with!(|region| region.flavor.image.src());
    let seceded = move || with!(|region| region.seceded);

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
