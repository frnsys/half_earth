use crate::{infinite_list, inputs::*};
use hes_engine::{industries::Industry, world::World};
use leptos::*;

#[component]
fn Industry(
    signal: (Signal<Industry>, SignalSetter<Industry>),
) -> impl IntoView {
    let (read, write) = signal;
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
            <div class="item-form">
                <ImageInput signal=slice!(industry.flavor.image) />
                <div class="input-groups">
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
             </div>
        </div>
    }
}

infinite_list!(Industries, Industry, industries);
