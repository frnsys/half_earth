use std::collections::HashMap;

use gloo_utils::format::JsValueSerdeExt;
use leptos::*;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;

use crate::{
    display::{factors::factors_card, format, intensity, text::AsText, Var},
    icons::{self, HasIcon},
    state,
    state::GameExt,
    state_with, t,
    util::to_ws_el,
    views::{parts::FactorsList, tip, HasTip, Tip},
};

#[wasm_bindgen(module = "/public/js/pie.js")]
extern "C" {
    type PieChart;

    #[wasm_bindgen(constructor)]
    fn new(el: &web_sys::HtmlElement) -> PieChart;

    #[wasm_bindgen(method)]
    fn render(this: &PieChart, dataset: JsValue, colors: JsValue);
}

impl Var {
    pub fn color(&self) -> [u32; 2] {
        match self {
            Var::Land => [0xB7FF7A, 0x0E681F],
            Var::Water => [0x7DE1EF, 0x4560FF],
            Var::Energy => [0xFDCE4C, 0xE81224],
            Var::Emissions => [0xF2F7E2, 0x6CB30B],
            Var::Biodiversity => [0xEA8BCF, 0x6865F8],
            Var::Electricity => [0xFFFF1A, 0xFF8C1A],
            Var::Fuel => [0xF7F6C7, 0xD3753F],
            Var::AnimalCalories => [0xF8AD72, 0xCA5704],
            Var::PlantCalories => [0xB1EF8F, 0x06CA9B],
            Var::Contentedness => [0x000000, 0xFFFFFF],
        }
    }
}

struct MiniCardData {
    label: String,
    color: &'static str,
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let (breakdown_factor, set_breakdown_factor) = create_signal(Var::Land);
    let (show_breakdown_menu, set_show_breakdown_menu) = create_signal(false);
    let color = move || breakdown_factor.get().color();
    let dataset = state_with!(|state, ui, breakdown_factor| {
        let mut total = 0.;
        let mut data: HashMap<String, f32> = HashMap::default();
        for fac in &ui.factors[*breakdown_factor] {
            let name = t!(&fac.name());
            data.insert(name, fac.amount());
            total += fac.amount();
        }
        if *breakdown_factor == Var::Land {
            let name = t!("Unused");
            let unused = state.world.starting_resources.land - total;
            data.insert(name, unused);
        }
        data
    });
    let choose_breakdown = move |choice: Var| {
        set_show_breakdown_menu.set(false);
        set_breakdown_factor.set(choice);
    };
    let avg_income_level = state!(|state, ui| {
        let avg = state.avg_income_level();
        MiniCardData {
            label: intensity::describe(avg - 1).to_string(),
            color: intensity::color(avg, true),
        }
    });
    let avg_habitability = state!(|state, ui| {
        let avg = state.avg_habitability();
        let int = intensity::scale(avg, intensity::Variable::Habitability);
        MiniCardData {
            label: intensity::describe(avg as usize).to_string(),
            color: intensity::color(int, true),
        }
    });
    let water_stress = |demand: f32| {
        let percent_use = format::water_use_percent(demand);
        MiniCardData {
            label: format::percent(percent_use, true),
            color: intensity::color(percent_use.round() as usize * 4, false),
        }
    };
    let extinction = |amount: f32| {
        let int = intensity::scale(amount, intensity::Variable::Extinction);
        MiniCardData {
            label: intensity::describe(int).to_string(),
            color: intensity::color(int, false),
        }
    };

    let process_multipliers = state!(|state, ui| {
        // TODO just iterate over process_mix_changes
        state
            .world
            .processes
            .iter()
            .filter(|p| !p.locked)
            .filter_map(move |p| {
                let mix_change =
                    (*ui.process_mix_changes[p.output].get(&p.id).unwrap_or(&0)) as f32 * 0.05;
                if mix_change != 0. {
                    let multiplier = mix_change * state.output_demand[p.output];
                    // TODO avoid cloning?
                    Some((p.clone(), multiplier))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>() // TODO ideally don't have to collect
    });

    let extinction_change = state!(move |state, ui| {
        let starting_land = state.world.starting_resources.land;
        process_multipliers()
            .into_iter()
            .map(|(p, mult)| p.extinction_rate(starting_land) * mult)
            .sum::<f32>()
            .round()
    });
    let current_extinction = state!(move |state, ui| { extinction(state.world.extinction_rate) });
    let after_extinction = state!(move |state, ui| {
        extinction(state.world.extinction_rate + extinction_change()).label
    });

    let land_change = move || {
        process_multipliers()
            .into_iter()
            // TODO check we're using adj_resources where needed
            .map(|(p, mult)| p.adj_resources().land * mult)
            .sum::<f32>()
            .round()
    };
    let water_change = move || {
        process_multipliers()
            .into_iter()
            .map(|(p, mult)| p.adj_resources().water * mult)
            .sum::<f32>()
            .round()
    };
    let energy_change = move || {
        process_multipliers()
            .into_iter()
            .map(|(p, mult)| {
                let energy = p.adj_resources().energy();
                energy * mult
            })
            .sum::<f32>()
            .round()
    };
    let emissions_change = move || {
        process_multipliers()
            .into_iter()
            .map(|(p, mult)| p.adj_byproducts().gtco2eq() * mult)
            .sum::<f32>()
            .round()
    };

    let current_water_stress =
        state!(move |state, ui| { water_stress(state.resources_demand.water) });
    let after_water_stress = state!(move |state, ui| {
        water_stress(water_change() + state.resources_demand.water).label
    });

    let temp_view = state!(|state, ui| {
        let temp = state.temp_anomaly();
        view! {
            <div class="dashboard--item">
                <div class="minicard">
                <span>{temp}</span>
                </div>
                <img src=icons::WARMING />
                <div class="dashboard--item-name">{t!("Temp. Anomaly")}</div>
                </div>
        }
    });
    let emissions_view = state!(move |state, ui| {
        // let tip = factors.tips
        let tip_text = t!("Current annual emissions, in gigatonnes of CO2 equivalent.");
        let tip: Tip = crate::views::tip(icons::EMISSIONS, tip_text).card(factors_card(
            None,
            Var::Emissions,
            state,
        ));
        let value = state.emissions_gt();
        let changed_value = state!(move |state, ui| {
            format::emissions(emissions_change() + state.state.emissions_gt())
        });
        view! {
            <DashboardItem
                tip
                label=t!("Emissions")
                display_value=value
                display_changed_value=changed_value
                change=emissions_change
                icon=icons::EMISSIONS
                />
        }
    });
    let land_view = state!(move |state, ui| {
        // let tip = factors.tips
        let tip_text = t!("Current land use.");
        let tip: Tip =
            crate::views::tip(icons::LAND, tip_text).card(factors_card(None, Var::Land, state));
        let value = state.land_use_percent();
        let changed_value = state!(move |state, ui| {
            format!(
                "{:.0}%",
                format::land_use_percent(land_change() + state.resources_demand.land)
            )
        });
        view! {
            <DashboardItem
                tip
                label=t!("Land Use")
                display_value=value
                display_changed_value=changed_value
                change=land_change
                icon=icons::LAND
                />
        }
    });

    let energy_view = state!(move |state, ui| {
        // let tip = factors.tips
        let tip_text = t!("Current energy use.");
        let tip: Tip =
            crate::views::tip(icons::ENERGY, tip_text).card(factors_card(None, Var::Energy, state));
        let value = state.energy_pwh();
        let changed_value = state!(move |state, ui| {
            format!(
                "{}TWh",
                (format::twh(energy_change() + state.output_demand.energy())).round()
            )
        });
        view! {
            <DashboardItem
                tip
                label=t!("Energy Use")
                display_value=value
                display_changed_value=changed_value
                change=energy_change
                icon=icons::ENERGY
                />
        }
    });

    let water_view = state!(move |state, ui| {
        let tip_text = t!("Current water demand.");
        let tip: Tip =
            crate::views::tip(icons::WATER, tip_text).card(factors_card(None, Var::Water, state));
        let current = current_water_stress();

        view! {
            <DashboardItem
                tip
                label=t!("Water Stress")
                color=current.color
                display_value=current.label
                display_changed_value=after_water_stress
                change=water_change
                icon=icons::WATER
                />
        }
    });

    let biodiversity_view = state!(move |state, ui| {
        let tip_text = t!("The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse.");
        let tip: Tip = crate::views::tip(icons::EXTINCTION_RATE, tip_text).card(factors_card(
            None,
            Var::Biodiversity,
            state,
        ));
        let current = current_extinction();

        view! {
            <DashboardItem
                tip
                label=t!("Extinction Rate")
                color=current.color
                display_value=current.label
                display_changed_value=after_extinction
                change=extinction_change
                icon=icons::EXTINCTION_RATE
                />
        }
    });

    let sea_level_rise_view = state!(|state, ui| {
        let rise = format!("{:.2}", state.world.sea_level_rise);
        let tip_text = t!("Average sea levels have risen by {rise}m and are rising at a rate of {rate}mm per year.",
            rise: rise,
            rate: format!("{:.1}", state.sea_level_rise_rate() * 1000.));
        let tip: Tip = crate::views::tip(icons::SEA_LEVEL_RISE, tip_text);
        view! {
            <HasTip tip>
                <div class="dashboard--item">
                <div class="minicard">
                <span>{rise}m</span>
                </div>
                <img src=icons::SEA_LEVEL_RISE />
                <div class="dashboard--item-name">{t!("Sea Level Rise")}</div>

                </div>
            </HasTip>
        }
    });

    let population_view = state!(|state, ui| {
        let population = state.world.population();
        view! {
         <div class="dashboard--item">
              <div class="minicard">
                <span>{population}</span>
              </div>
              <img src=icons::POPULATION />
              <div class="dashboard--item-name">{t!("Population")}</div>
            </div>
        }
    });

    let income_view = move || {
        let income = avg_income_level();
        view! {
            <div class="dashboard--item">
                <div class="minicard">
                    <span style:color=income.color>{t!(& income.label)}</span>
                </div>
                <img src=icons::WEALTH/>
                <div class="dashboard--item-name">
                    {t!("Avg. Living Standards")}
                </div>
            </div>
        }
    };

    let habitability_view = move || {
        let habitability = avg_habitability();
        view! {
            <div class="dashboard--item">
                <div class="minicard">
                    <span style:color=habitability
                        .color>{t!(& habitability.label)}</span>
                </div>
                <img src=icons::HABITABILITY/>
                <div class="dashboard--item-name">
                    {t!("Avg. Habitability")}
                </div>
            </div>
        }
    };

    let table_data = state!(move |state, ui| { factors_card(None, breakdown_factor.get(), state) });
    let icon = move || breakdown_factor.get().icon();
    let name = move || t!(breakdown_factor.get().title());

    let menu = move || {
        view! {
            <Show when=move || show_breakdown_menu.get()>
                <div class="dashboard-breakdown-menu-overlay">
                    <div class="dashboard-breakdown-menu">
                        {move || {
                            Var::iter()
                                .map(|var| {
                                    view! {
                                        <div on:click=move |_| set_breakdown_factor.set(var)>
                                            <img class="pip-icon" src=var.icon()/>
                                            {t!(var.title())}
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}

                    </div>
                </div>
            </Show>
        }
    };

    view! {
        <div class="planning--page planning--page--dashboard">
            <div class="planning--dashboard">
                {temp_view} {emissions_view} {land_view} {energy_view}
                {water_view} {biodiversity_view} {sea_level_rise_view}
                {population_view} {income_view} {habitability_view}
            </div>

            <div class="dashboard-breakdown">
                <div
                    class="dashboard-breakdown-select btn"
                    on:click=move |_| set_show_breakdown_menu.set(true)
                >
                    <img class="pip-icon" src=icon/>
                    {name}
                    "▼"
                </div>
                <PieChart
                    dataset=dataset
                    colors=move || breakdown_factor.get().color()
                />
                <div class="dashboard--factors">
                    <FactorsList factors=table_data/>
                </div>
                <div class="dashboard-breakdown-note">
                    {t!("Only direct impacts are shown.")}
                </div>
            </div>
        </div>
    }
}

#[component]
fn DashboardItem(
    #[prop(into)] label: MaybeSignal<String>,
    #[prop(into)] display_value: MaybeSignal<String>,
    #[prop(into)] display_changed_value: Signal<String>,
    #[prop(into)] tip: MaybeSignal<Tip>,
    #[prop(into)] change: Signal<f32>,
    #[prop(into)] icon: MaybeSignal<&'static str>,
    #[prop(into, optional)] color: Option<String>,
) -> impl IntoView {
    let change_tip = move || {
        crate::views::tip(
            icon.get(),
            t!("The estimated value after production changes have finished."),
        )
    };
    view! {
        <HasTip tip>
            <div class="dashboard--item">
                <div class="minicard">
                    <span style:color=color>{display_value}</span>
                    <Show when=move || change.get() != 0.>
                        <HasTip tip=change_tip.into_signal()>
                            <div class="dashboard--change">
                                <img src=icons::DOWN_ARROW_SMALL/>
                                <span class="dashboard--change-value">
                                    {display_changed_value}
                                </span>

                            </div>
                        </HasTip>
                    </Show>
                </div>
                <img src=icon/>
                <div class="dashboard--item-name">{label}</div>
            </div>
        </HasTip>
    }
}

#[component]
fn PieChart(
    #[prop(into)] dataset: Signal<HashMap<String, f32>>,
    #[prop(into)] colors: Signal<[u32; 2]>,
) -> impl IntoView {
    let stage_ref = create_node_ref::<html::Div>();
    let (chart, set_chart) = create_signal(None);

    create_effect(move |_| {
        let stage = stage_ref.get().unwrap();
        let chart = PieChart::new(&to_ws_el(stage));
        let dataset = JsValue::from_serde(&dataset).unwrap();
        let colors = JsValue::from_serde(&colors).unwrap();
        chart.render(dataset, colors);
        set_chart.set(Some(chart));
    });

    view! { <div class="pie-chart" ref=stage_ref></div> }
}
