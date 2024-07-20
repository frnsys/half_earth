use crate::inputs::*;
use hes_engine::{regions::Income, world::World};
use leptos::*;
use strum::IntoEnumIterator;

#[component]
pub fn World(world: RwSignal<World>) -> impl IntoView {
    view! {
        <div class="world">
            <div class="world-inputs">
                <div class="world-inputs-global">
                    <NumericInput
                        label="Contentedness"
                        help="The starting world contentedness."
                        signal=slice!(world.base_outlook) />
                    <NumericInput
                        label="Extinction Pressure"
                        help="The starting extinction pressure."
                        signal=slice!(world.extinction_rate) />
                    <NumericInput
                        label="Warming"
                        help="The starting temperature anomaly (C)."
                        signal=slice!(world.temperature) />
                    <NumericInput
                        label="Sea Level Rise"
                        help="The starting sea level rise (meters)."
                        signal=slice!(world.sea_level_rise) />
                </div>
                <ResourceMapInput
                    label="Starting Resources"
                    help="The starting resource availability."
                    signal=slice!(world.starting_resources)
                />
                <FeedstockMapInput
                    label="Feedstocks Reserves"
                    help="The starting feedstock reserves."
                    signal=slice!(world.feedstock_reserves)
                />

                <div class="pop-growth-inputs">
                    {move || {
                         Income::iter().enumerate().map(|(i, income)| {
                             view! {
                                <MultiNumericInput
                                    label=&format!("{} Income Population Growth Coefficients", income)
                                    help="Parameters used for the cubic regression model."
                                    sublabels=[
                                        "β₀ = ",
                                        "β₁ = ",
                                        "β₂ = ",
                                        "β₃ = ",
                                    ]
                                    signal=create_slice(world,
                                        move |world| world.income_pop_coefs[i],
                                        move |world, val| world.income_pop_coefs[i] = val
                                    )
                                />
                             }
                         }).collect::<Vec<_>>()
                     }}
                </div>

                <MultiNumericInput
                    label="Material Intensity By Income"
                    help="Per-capita material intensity by income level in metric tons/year, though the units are less important as these values are used for scaling."
                    sublabels=[
                        "Low Income",
                        "Lower-Middle Income",
                        "Upper-Middle Income",
                        "High Income",
                    ]
                    signal=slice!(world.materials_by_income)
                />

                <MultiNumericInput
                    label="Water Demand By Income"
                    help="Per-capita municipal/household water demand by income level, in L/month."
                    sublabels=[
                        "Low Income",
                        "Lower-Middle Income",
                        "Upper-Middle Income",
                        "High Income",
                    ]
                    signal=slice!(world.water_by_income)
                />

                <div class="output-demand-inputs">
                    {move || {
                         Income::iter().enumerate().map(|(i, income)| {
                             view! {
                                <OutputMapInput
                                    label=&format!("{} Per-Capita Output Demand", income)
                                    help="Fuel and Electricity are in kWh/month; Plant and Animal Calories are in kcals/year."
                                    signal=create_slice(world,
                                        move |world| world.output_demand[i],
                                        move |world, val| world.output_demand[i] = val
                                    )
                                />
                             }
                         }).collect::<Vec<_>>()
                     }}
                </div>

            </div>
        </div>
    }
}
