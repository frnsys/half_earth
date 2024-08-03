use super::super::region_item::RegionItem;
use crate::{
    icons,
    memo,
    t,
    util::{scale_text, to_ws_el},
    views::globe::{Globe, GlobeRef},
};
use hes_engine::Game;
use leptos::*;

#[component]
pub fn Regions() -> impl IntoView {
    let game = expect_context::<RwSignal<Game>>();

    let (globe, set_globe) =
        create_signal::<Option<GlobeRef>>(None);
    let regions = memo!(game.world.regions);
    let (selected_region, set_selected_region) =
        create_signal(0);
    let region_name = create_memo(move |_| {
        let name = with!(|regions, selected_region| regions
            .by_idx(*selected_region)
            .name
            .clone());
        t!(&name)
    });
    let region = move || {
        with!(|regions, selected_region| regions
            .by_idx(*selected_region)
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
            let name = region_name.get_untracked();
            globe.highlight_region(&name);
        }
    };

    let n_regions = memo!(game.world.regions.len());
    let next_region = move |_| {
        with!(|n_regions| {
            set_selected_region.update(|idx| {
                *idx += 1;
                if *idx >= *n_regions {
                    *idx = 0;
                }
            });
        });
        center_on_region();
    };
    let prev_region = move |_| {
        with!(|n_regions| {
            set_selected_region.update(|idx| {
                if *idx <= 0 {
                    *idx = *n_regions - 1;
                } else {
                    *idx -= 1;
                }
            });
        });
        center_on_region();
    };

    create_effect(move |_| {
        // Subscribe to selected region change.
        selected_region.track();
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
                    {region_name}
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
