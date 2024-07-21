use crate::inputs::*;
use hes_engine::{industries::Industry, world::World};
use leptos::*;

#[component]
fn Industry(
    industry: (Signal<Industry>, SignalSetter<Industry>),
) -> impl IntoView {
    let (read, write) = industry;
    let industry = create_rw_signal(read.get_untracked());

    // Hacky way to keep the data synchronized.
    create_effect(move |_| {
        write.set(industry.get());
    });

    view! {
        <div class="industry">
            <div class="name">
                <TextInput signal=slice!(industry.name) />
            </div>
            <ImageInput signal=slice!(industry.flavor.image) />
            <ByproductMapInput
                label="Byproducts"
                help="Byproducts produced, per low-income-capita (LIC) per year."
                signal=slice!(industry.byproducts) />
            <ResourceMapInput
                label="Resources"
                help="Resources used, per low-income-capita (LIC) per year."
                signal=slice!(industry.resources)
             />
        </div>
    }
}

#[component]
pub fn Industries(world: RwSignal<World>) -> impl IntoView {
    let n_industries = with!(|world| world.industries.len());
    view! {
        <div class="industries scroll-list">
        {move || {
             (0..n_industries).map(|i| {
                 view! {
                     <Industry
                        industry=create_slice(world,
                            move |world| world.industries.by_idx(i).clone(),
                            move |world, val| *world.industries.by_idx_mut(i) = val
                        ) />
                 }
             }).collect::<Vec<_>>()
         }}
        </div>
    }
}
