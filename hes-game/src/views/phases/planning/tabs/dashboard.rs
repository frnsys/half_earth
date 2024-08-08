use std::collections::BTreeMap;

use enum_map::EnumMap;
use gloo_utils::format::JsValueSerdeExt;
use hes_engine::{Output, Resource, State};
use leptos::*;
use numfmt::{Formatter, Precision, Scales};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;

use crate::{
    display::{self, AsText, DisplayValue},
    icons::{self, HasIcon},
    memo,
    state::{StateExt, UIState},
    t,
    util::to_ws_el,
    vars::Var,
    views::{
        factors::{factors_card, FactorsList},
        intensity,
        HasTip,
        Tip,
    },
};

#[wasm_bindgen(module = "/public/js/pie.js")]
extern "C" {
    type PieChart;

    #[wasm_bindgen(constructor)]
    fn new(el: &web_sys::HtmlElement) -> PieChart;

    #[wasm_bindgen(method)]
    fn render(
        this: &PieChart,
        dataset: JsValue,
        colors: JsValue,
    );
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
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

    let (breakdown_factor, set_breakdown_factor) =
        create_signal(Var::Land);
    let (show_breakdown_menu, set_show_breakdown_menu) =
        create_signal(false);

    let factors = memo!(ui.factors);
    let available_land = memo!(game.resources.available.land);
    let dataset = move || {
        let mut total = 0.;
        let mut data: BTreeMap<String, f32> =
            BTreeMap::default();
        let breakdown_factor = breakdown_factor.get();
        for fac in &factors.get()[breakdown_factor] {
            let name = t!(&fac.name());
            data.insert(name, fac.amount());
            total += fac.amount();
        }
        if breakdown_factor == Var::Land {
            let name = t!("Unused");
            let unused = available_land.get() - total;
            data.insert(name, unused);
        }
        data
    };

    let income = memo!(game.avg_income_level());
    let avg_income_level = move || {
        let avg = income.get();
        MiniCardData {
            label: intensity::describe(avg - 1),
            color: intensity::color(avg, true),
        }
    };

    let habitability = memo!(game.world.regions.habitability());
    let avg_habitability = move || {
        let avg = habitability.get();
        let int = intensity::scale(
            avg,
            intensity::Variable::Habitability,
        );
        MiniCardData {
            label: intensity::describe(avg as usize),
            color: intensity::color(int, true),
        }
    };

    let available_water = memo!(game.resources.available.water);
    let water_stress = move |demand: f32| {
        let percent_use = display::water_use_percent(
            demand,
            available_water.get(),
        );
        MiniCardData {
            label: display::percent(percent_use / 100., true),
            color: intensity::color(
                percent_use.round() as usize * 4,
                false,
            ),
        }
    };
    let extinction = |amount: f32| {
        let int = intensity::scale(
            amount,
            intensity::Variable::Extinction,
        );
        MiniCardData {
            label: intensity::describe(int),
            color: intensity::color(int, false),
        }
    };

    let processes = memo!(game.world.processes);
    let process_mix_changes = memo!(ui.process_mix_changes);
    let demand_for_outputs = create_memo(move |_| {
        let demands: EnumMap<Output, f32> =
            with!(|game| Output::iter()
                .map(|output| (
                    output,
                    game.output_demand.of(output)
                ))
                .collect());
        demands
    });
    let process_multipliers = move || {
        with!(|processes,
               process_mix_changes,
               demand_for_outputs| {
            processes
                .iter()
                .filter(|p| !p.locked)
                .filter_map(move |p| {
                    let mix_change = (*process_mix_changes
                        [p.output]
                        .get(&p.id)
                        .unwrap_or(&0))
                        as f32
                        * 0.05;
                    if mix_change != 0. {
                        let multiplier = mix_change
                            * demand_for_outputs[p.output];
                        Some((p.clone(), multiplier))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    let extinction_change = move || {
        process_multipliers()
            .into_iter()
            .map(|(p, mult)| {
                p.extinction_rate(available_land.get()) * mult
            })
            .sum::<f32>()
            .round()
    };
    let extinction_rate = memo!(game.world.extinction_rate);
    let current_extinction =
        move || extinction(extinction_rate.get());
    let after_extinction = move || {
        extinction(extinction_rate.get() + extinction_change())
            .label
    };

    let land_change = move || {
        process_multipliers()
            .into_iter()
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
            .map(|(p, mult)| {
                p.adj_byproducts().gtco2eq() * mult
            })
            .sum::<f32>()
            .round()
    };

    let water_demand =
        memo!(game.resource_demand.of(Resource::Water));
    let current_water_stress =
        move || water_stress(water_demand.get());
    let after_water_stress = move || {
        water_stress(water_change() + water_demand.get()).label
    };

    let temp_anomaly = memo!(game.temp_anomaly());
    let temp_view = move || {
        view! {
            <div class="dashboard--item">
                <div class="minicard">
                    <span>{temp_anomaly}</span>
                </div>
                <img src=icons::WARMING/>
                <div class="dashboard--item-name">
                    {t!("Temp. Anomaly")}
                </div>
            </div>
        }
    };

    let emissions_tip = move || {
        with!(|game| {
            let tip_text = t!("Current annual emissions, in gigatonnes of CO2 equivalent.");
            crate::views::tip(icons::EMISSIONS, tip_text)
                .card(factors_card(None, Var::Emissions, game))
        })
    };
    let emissions = memo!(game.emissions.as_gtco2eq());
    let emissions_display = memo!(game.emissions.display());
    let emissions_changed = move || {
        display::emissions(emissions_change() + emissions.get())
    };
    let emissions_view = move || {
        view! {
            <DashboardItem
                tip=emissions_tip.into_signal()
                label=t!("Emissions")
                display_value=emissions_display
                display_changed_value=emissions_changed
                change=emissions_change
                icon=icons::EMISSIONS
            />
        }
    };

    let land_tip = move || {
        with!(|game| {
            crate::views::tip(
                icons::LAND,
                t!("Current land use."),
            )
            .card(factors_card(
                None,
                Var::Land,
                game,
            ))
        })
    };
    let land_use = memo!(game.land_use_percent());
    let land_demand =
        memo!(game.resource_demand.of(Resource::Land));
    let land_changed = move || {
        format!(
            "{:.0}%",
            display::land_use_percent(
                land_change() + land_demand.get(),
                available_land.get()
            )
        )
    };
    let land_view = move || {
        view! {
            <DashboardItem
                tip=land_tip.into_signal()
                label=t!("Land Use")
                display_value=land_use
                display_changed_value=land_changed
                change=land_change
                icon=icons::LAND
            />
        }
    };

    let energy_tip = move || {
        with!(|game| {
            crate::views::tip(
                icons::ENERGY,
                t!("Current energy use."),
            )
            .card(factors_card(
                None,
                Var::Energy,
                game,
            ))
        })
    };
    let energy_use = memo!(game.energy_pwh());
    let energy_demand =
        memo!(game.output_demand.total().energy());
    let energy_changed = move || {
        format!(
            "{}TWh",
            (display::twh(
                energy_change() + energy_demand.get()
            ))
            .round()
        )
    };
    let energy_view = move || {
        view! {
            <DashboardItem
                tip=energy_tip.into_signal()
                label=t!("Energy Use")
                display_value=energy_use
                display_changed_value=energy_changed
                change=energy_change
                icon=icons::ENERGY
            />
        }
    };

    let water_tip = move || {
        with!(|game| {
            crate::views::tip(
                icons::WATER,
                t!("Current water demand."),
            )
            .card(factors_card(
                None,
                Var::Water,
                game,
            ))
        })
    };
    let water_view = move || {
        let current = current_water_stress();

        view! {
            <DashboardItem
                tip=water_tip.into_signal()
                label=t!("Water Stress")
                color=current.color
                display_value=current.label
                display_changed_value=after_water_stress
                change=water_change
                icon=icons::WATER
            />
        }
    };

    let biodiversity_tip = move || {
        with!(|game| {
            let tip_text = t!("The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse.");
            crate::views::tip(icons::EXTINCTION_RATE, tip_text)
                .card(factors_card(
                    None,
                    Var::Biodiversity,
                    game,
                ))
        })
    };
    let biodiversity_view = move || {
        let current = current_extinction();
        view! {
            <DashboardItem
                tip=biodiversity_tip.into_signal()
                label=t!("Extinction Rate")
                color=current.color
                display_value=current.label
                display_changed_value=after_extinction
                change=extinction_change
                icon=icons::EXTINCTION_RATE
            />
        }
    };

    let sea_level_rise = memo!(game.world.sea_level_rise);
    let sea_level_rise_rate =
        memo!(game.world.sea_level_rise_rate());
    let sea_level_rise_view = move || {
        let rise = format!("{:.2}", sea_level_rise.get());
        let tip_text = t!("Average sea levels have risen by {rise}m and are rising at a rate of {rate}mm per year.",
            rise: rise,
            rate: format!("{:.1}", sea_level_rise_rate.get() * 1000.));
        let tip: Tip =
            crate::views::tip(icons::SEA_LEVEL_RISE, tip_text);
        view! {
            <HasTip tip>
                <div class="dashboard--item">
                    <div class="minicard">
                        <span>{rise} m</span>
                    </div>
                    <img src=icons::SEA_LEVEL_RISE/>
                    <div class="dashboard--item-name">
                        {t!("Sea Level Rise")}
                    </div>

                </div>
            </HasTip>
        }
    };

    let population = memo!(game.world.regions.population());
    let pop_fmted = move || {
        let mut f = Formatter::default()
            .scales(Scales::short())
            .precision(Precision::Decimals(1));
        f.fmt2(population.get() as f64).to_string()
    };
    let population_view = move || {
        view! {
            <div class="dashboard--item">
                <div class="minicard">
                    <span>{pop_fmted}</span>
                </div>
                <img src=icons::POPULATION/>
                <div class="dashboard--item-name">{t!("Population")}</div>
            </div>
        }
    };

    let income_view = move || {
        let income = avg_income_level();
        view! {
            <div class="dashboard--item">
                <div class="minicard">
                    <span style:color=income.color>{&income.label}</span>
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
                        .color>{&habitability.label}</span>
                </div>
                <img src=icons::HABITABILITY/>
                <div class="dashboard--item-name">
                    {t!("Avg. Habitability")}
                </div>
            </div>
        }
    };

    let table_data = move || {
        with!(|game| {
            factors_card(None, breakdown_factor.get(), game)
        })
    };
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
                                        <div on:click=move |_| {
                                            set_breakdown_factor.set(var);
                                            set_show_breakdown_menu.set(false);
                                        }>
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
            {menu}
            <div class="planning--dashboard">
                {temp_view} {emissions_view} {land_view} {energy_view}
                {water_view} {biodiversity_view} {sea_level_rise_view}
                {population_view} {income_view} {habitability_view}
            </div> <div class="dashboard-breakdown">
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
    #[prop(into)] dataset: Signal<BTreeMap<String, f32>>,
    #[prop(into)] colors: Signal<[u32; 2]>,
) -> impl IntoView {
    let stage_ref = create_node_ref::<html::Div>();
    let (_, set_chart) = create_signal(None);

    create_effect(move |_| {
        set_chart.update(|chart| {
            if chart.is_none() {
                let stage = stage_ref.get().unwrap();
                *chart = Some(PieChart::new(&to_ws_el(stage)));
            }
            if let Some(chart) = chart {
                let dataset =
                    JsValue::from_serde(&dataset.get())
                        .unwrap();
                let colors =
                    JsValue::from_serde(&colors.get()).unwrap();
                chart.render(dataset, colors);
            }
        });
    });

    view! { <div class="pie-chart" ref=stage_ref></div> }
}
