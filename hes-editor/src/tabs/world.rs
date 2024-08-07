use crate::inputs::*;
use hes_engine::{Income, Output, World};
use leptos::*;
use strum::IntoEnumIterator;

#[component]
pub fn World(world: RwSignal<World>) -> impl IntoView {
    view! {
        <div class="world scroll-list">
            <div class="map-group-block">
                <div class="map-group">
                    <h2>Initial Values</h2>
                    <div class="map-inputs">
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

                <div class="map-group-block">
                    <div class="map-group table-group">
                        <h2 class="tooltip-parent">
                            Annual Population Growth Coefficients
                            <div class="tooltip">The coefficients for cubic annual population growth model, with one for each income level.</div>
                        </h2>
                        <div class="map-table">
                            <div class="label-column input-column">
                                <div>-</div>
                                <label>Low</label>
                                <label>Lower-Middle</label>
                                <label>Upper-Middle</label>
                                <label>High</label>
                            </div>
                            {move || {
                                 ["β₀", "β₁", "β₂", "β₃"].iter().enumerate().map(|(i, label)| {
                                     let label = *label;
                                     view! {
                                         <div class="input-column">
                                             <label>{label}</label>
                                             {move || {
                                                  Income::iter().enumerate().map(|(j, income)| {
                                                      view! {
                                                          <NumericInput
                                                              label=format!("{}-{}", income, label)
                                                              help=format!("{} parameter for {} income annual growth equation.", label, income)
                                                              signal=create_slice(world,
                                                                  move |world| world.income_pop_coefs[j][i],
                                                                  move |world, val| world.income_pop_coefs[j][i] = val
                                                              ) />
                                                      }
                                                  }).collect::<Vec<_>>()
                                             }}
                                         </div>
                                     }
                                 }).collect::<Vec<_>>()
                             }}
                        </div>
                    </div>
                </div>

                <div class="map-group table-group">
                    <h2 class="tooltip-parent">
                        Per-Capita Demand/Intensity By Income
                        <div class="tooltip">The per-capita demand for outputs and resources for each income level.</div>
                    </h2>
                    <div class="map-table">
                        <div class="label-column input-column">
                            <div>-</div>
                            <label>Low</label>
                            <label>Lower-Middle</label>
                            <label>Upper-Middle</label>
                            <label>High</label>
                        </div>
                        {move || {
                             Output::iter().map(|output| {
                                 let label: &'static str = output.into();
                                 let units = match output {
                                     Output::Fuel | Output::Electricity => "kWh/month",
                                     Output::PlantCalories | Output::AnimalCalories => "kcals/year",
                                 };
                                 view! {
                                     <div class="input-column">
                                         <label>{label}</label>
                                         {move || {
                                              Income::iter().enumerate().map(|(j, income)| {
                                                  view! {
                                                      <NumericInput
                                                          label=format!("{}-{}", income, label)
                                                          help=format!("{} income per-capita demand for {}, in {}.", income, label, units)
                                                          signal=create_slice(world,
                                                              move |world| world.output_demand[j][output],
                                                              move |world, val| world.output_demand[j][output] = val
                                                          ) />
                                                  }
                                              }).collect::<Vec<_>>()
                                         }}
                                     </div>
                                 }
                             }).collect::<Vec<_>>()
                         }}
                         <div class="input-column">
                             <label>"Water"</label>
                             {move || {
                                  Income::iter().enumerate().map(|(j, income)| {
                                      view! {
                                          <NumericInput
                                              label=format!("{}-Water Demand", income)
                                              help="Per-capita municipal/household water demand by income level, in L/month."
                                              signal=create_slice(world,
                                                  move |world| world.materials_by_income[j],
                                                  move |world, val| world.materials_by_income[j] = val
                                              ) />
                                      }
                                  }).collect::<Vec<_>>()
                             }}
                         </div>

                         <div class="input-column">
                             <label>"Material"</label>
                             {move || {
                                  Income::iter().enumerate().map(|(j, income)| {
                                      view! {
                                          <NumericInput
                                              label=format!("{}-Material Intensity", income)
                                              help="Per-capita material intensity by income level in metric tons/year, though the units are less important as these values are used for scaling."
                                              signal=create_slice(world,
                                                  move |world| world.water_by_income[j],
                                                  move |world, val| world.water_by_income[j] = val
                                              ) />
                                      }
                                  }).collect::<Vec<_>>()
                             }}
                         </div>
                    </div>
                </div>

                <Regions world />
            </div>
        </div>
    }
}

#[component]
fn Regions(world: RwSignal<World>) -> impl IntoView {
    let n_regions =
        world.with_untracked(|world| world.regions.len());
    let names = move || {
        with!(|world| {
            world
                .regions
                .iter()
                .map(|r| r.name.to_string())
                .collect::<Vec<_>>()
        })
    };
    view! {
        <div class="regions map-group-block">
            <div class="map-group table-group">
                <h2 class="tooltip-parent">
                    Regions
                    <div class="tooltip">"Parameters for the world's regions."</div>
                </h2>
                <div class="map-table">
                    <div class="label-column input-column">
                        <div>-</div>
                        {move || {
                             names().into_iter().map(|name| {
                                 view! {
                                     <label>{name}</label>
                                 }
                             }).collect::<Vec<_>>()
                        }}
                    </div>
                     <div class="input-column">
                         <label>"Population"</label>
                        {move || {
                             (0..n_regions).map(|i| {
                                 view! {
                                  <NumericInput
                                      label="Population"
                                      help="The region's starting population."
                                      signal=create_slice(world,
                                          move |world| world.regions.by_idx(i).population,
                                          move |world, val| world.regions.by_idx_mut(i).population = val
                                      ) />
                                 }
                             }).collect::<Vec<_>>()
                         }}
                     </div>
                     <div class="input-column">
                         <label>"Development"</label>
                        {move || {
                             (0..n_regions).map(|i| {
                                 view! {
                                  <NumericInput
                                      label="Development"
                                      help="The region's starting progress to the next income level, from 0.0 to 1.0."
                                      signal=create_slice(world,
                                          move |world| world.regions.by_idx(i).development,
                                          move |world, val| world.regions.by_idx_mut(i).development = val
                                      ) />
                                 }
                             }).collect::<Vec<_>>()
                         }}
                     </div>
                     <div class="input-column">
                         <label>"Income Level"</label>
                        {move || {
                             (0..n_regions).map(|i| {
                                 view! {
                                    <EnumInput
                                        tooltip=true
                                        label="Income Level"
                                        help="The region's starting income level."
                                        signal=create_slice(world,
                                            move |world| world.regions.by_idx(i).income,
                                            move |world, val| world.regions.by_idx_mut(i).income = val
                                        ) />
                                 }
                             }).collect::<Vec<_>>()
                         }}
                     </div>
                </div>
            </div>
        </div>
    }
}
