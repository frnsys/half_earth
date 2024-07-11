use super::super::region_item::RegionItem;
use crate::{
    icons, state, t,
    util::{scale_text, to_ws_el},
    views::globe::Globe,
};
use leptos::*;

#[component]
pub fn Regions() -> impl IntoView {
    // TODO
    // import regionsToTiles from 'assets/surface/regions_to_tiles.json';
    // import tilesToRegions from 'assets/surface/tiles_to_regions.json';

    let regions = state!(world.regions.clone());
    let (selected_region, set_selected_region) = create_signal(0);
    let region_name = move || regions.get()[selected_region.get()].name.clone();
    let region = move || regions.get()[selected_region.get()].clone();

    let region_name_ref = create_node_ref::<html::Div>();
    let fit_region_name = move || {
        if let Some(region_name_ref) = region_name_ref.get() {
            scale_text(to_ws_el(region_name_ref), 18);
        }
    };
    create_effect(move |_| {
        // Subscribe to selected region change.
        let _ = selected_region.get();
        fit_region_name();
    });

    let center_on_region = move || {
        let idx = selected_region.get();

        // Reset highlights
        // Object.keys(tilesToRegions).forEach((idx) => {
        //   this.globe.hexsphere.unhighlightIdx(idx);
        // });
        //
        // let tiles = this.regionTiles(regionId);
        // this.globe.hexsphere.centerOnIndex(tiles[0]);
        //
        // // Highlight region
        // tiles.forEach((idx) => {
        //   this.globe.hexsphere.highlightIdx(idx);
        // });
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

    on_cleanup(|| {
        // TODO this could probably be moved into the globe view?
        // just unhighlight all?
        // Object.keys(tilesToRegions).forEach((idx) => {
        //   this.globe.hexsphere.unhighlightIdx(idx);
        // });
    });

    // TODO
    // let region_tiles = move |region_id| {
    //     // TODO
    //     // let name = state.gameState.world.regions[regionId].name;
    //     // let tiles = regionsToTiles[name];
    //     // return tiles['inland'].concat(tiles['coasts']);
    // };

    let on_globe_click = move |region_idx| {
        // TODO
        // let obj = intersects[0].object;
        // let regionId = tilesToRegions[obj.userData.idx];
        // if (regionId !== undefined) {
        //   this.selectedRegion = regionId;
        //   this.centerOnRegion(regionId);
        // }
    };

    let on_globe_ready = move |globe| {
        // globe.clear();
        // globe.rotate = false;
        // globe.scene.camera.zoom = 0.15;
        // globe.scene.camera.updateProjectionMatrix();
        // globe.clouds.visible = false;
        // this.globe = globe;
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
