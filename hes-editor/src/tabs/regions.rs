use crate::inputs::*;
use hes_engine::{
    regions::{Income, Region},
    world::World,
};
use leptos::*;

#[component]
fn Region(
    region: (Signal<Region>, SignalSetter<Region>),
) -> impl IntoView {
    let (read, write) = region;
    let region = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(region.get());
    });

    let name = move || with!(|read| read.name.clone());

    view! {
        <div class="region">
            <h2>{name}</h2>
            <NumericInput
                label="Population"
                help="The region's starting population."
                signal=slice!(region.population) />
            <NumericInput
                label="Development"
                help="The region's starting progress to the next income level."
                signal=slice!(region.development) />
            <EnumInput
                label="Income Level"
                help="The region's starting income level."
                signal=slice!(region.income) />
        </div>
    }
}

#[component]
pub fn Regions(world: RwSignal<World>) -> impl IntoView {
    let n_regions = with!(|world| world.regions.len());
    view! {
        <div class="regions">
        {move || {
             (0..n_regions).map(|i| {
                 view! {
                     <Region
                        region=create_slice(world,
                            move |world| world.regions[i].clone(),
                            move |world, val| world.regions[i] = val
                        ) />
                 }
             }).collect::<Vec<_>>()
         }}
        </div>
    }
}
