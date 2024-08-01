use super::super::region_item::RegionItem;
use crate::{
    icons,
    state,
    t,
    util::{scale_text, to_ws_el},
    views::globe::{Globe, GlobeRef},
};
use leptos::*;

#[component]
pub fn Regions() -> impl IntoView {
    let (globe, set_globe) =
        create_signal::<Option<GlobeRef>>(None);
    let regions = state!(world.regions.clone());
    let (selected_region, set_selected_region) =
        create_signal(0);
    let region_name = move || {
        with!(|regions| regions
            .by_idx(selected_region.get())
            .name
            .clone())
    };
    let region = move || {
        with!(|regions| regions
            .by_idx(selected_region.get())
            .clone())
    };

    let region_name_ref = create_node_ref::<html::Div>();
    let fit_region_name = move || {
        if let Some(region_name_ref) = region_name_ref.get() {
            scale_text(to_ws_el(region_name_ref), 18);
        }
    };

    let center_on_region = move || {
        if let Some(globe) = globe.get_untracked() {
            let name = region_name();
            globe.highlight_region(&name);
        }
    };

    let next_region = move |_| {
        set_selected_region.update(|idx| {
            let regions = regions.get();
            *idx += 1;
            if *idx >= regions.len() {
                *idx = 0;
            }
        });
        center_on_region();
    };
    let prev_region = move |_| {
        set_selected_region.update(|idx| {
            let regions = regions.get();
            if *idx <= 0 {
                *idx = regions.len() - 1;
            } else {
                *idx -= 1;
            }
        });
        center_on_region();
    };

    create_effect(move |_| {
        // Subscribe to selected region change.
        let _ = selected_region.get();
        fit_region_name();
        center_on_region();
    });

    let on_globe_click = move |region_idx| {
        set_selected_region.set(region_idx);
    };

    let on_globe_ready = move |globe: GlobeRef| {
        globe.clear();
        globe.rotate(false);
        globe.set_zoom(0.15);
        globe.clouds(false);
        set_globe.set(Some(globe));
        center_on_region();
    };

    view! {
        <div class="planning--page planning--page--regions">
            <Globe
                id="regions-globe"
                class="cell"
                on_ready=on_globe_ready
                on_click=on_globe_click
            />
            <div class="regions-browse">
                <div class="region-change btn" on:click=prev_region>
                    <img src=icons::ARROW_LEFT/>
                </div>
                <div class="region-name cell" ref=region_name_ref>
                    {move || t!(& region_name())}
                </div>
                <div class="region-change btn" on:click=next_region>
                    <img src=icons::ARROW_RIGHT/>
                </div>
            </div>
            <div class="regions-region">
                <RegionItem region/>
            </div>
        </div>
    }
}
