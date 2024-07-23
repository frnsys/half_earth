use crate::{infinite_list, inputs::*, subsignal};
use hes_engine::{industries::Industry, world::World};
use leptos::*;

#[component]
fn Industry(
    signal: (Signal<Industry>, SignalSetter<Industry>),
) -> impl IntoView {
    let industry = signal;

    view! {
        <div class="industry" id={move || signal.0.with(|signal| signal.id.to_string())}>
            <div class="name">
                <TextInput signal=subsignal!(industry.name) />
            </div>
            <div class="item-form">
                <ImageInput signal=subsignal!(industry.flavor.image) />
                <div class="input-groups">
                    <ByproductMapInput
                        label="Byproducts"
                        help="Byproducts produced, per low-income-capita (LIC) per year."
                        signal=subsignal!(industry.byproducts) />
                    <ResourceMapInput
                        label="Resources"
                        help="Resources used, per low-income-capita (LIC) per year."
                        signal=subsignal!(industry.resources)
                     />
                 </div>
             </div>

             <div class="item-form notes-form">
                 <TextArea label="Notes" help="Optional notes" signal=subsignal!(industry.notes) />
             </div>
        </div>
    }
}

infinite_list!(Industries, Industry, industries);
