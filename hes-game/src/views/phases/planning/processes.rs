use std::collections::BTreeMap;

use crate::{
    consts,
    debug::get_debug_opts,
    display::{self, AsText, FloatExt},
    icons::{self, HasIcon},
    memo,
    state::{Tutorial, UIState},
    t,
    vars::Var,
    views::{factors::factors_card, scanner::*, tip, HasTip},
};
use enum_map::EnumMap;
use hes_engine::{
    Id,
    KindMap,
    Output,
    Process,
    Resource,
    State,
};
use leptos::*;

#[component]
pub fn Processes(
    #[prop(into)] on_change: Callback<()>,
    #[prop(into)] close: Callback<()>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

    let back_disabled =
        memo!(ui.tutorial.lt(&Tutorial::ProcessesBack));
    let back_highlighted =
        memo!(ui.tutorial.eq(&Tutorial::ProcessesBack));

    let (output, set_output) =
        create_signal(Output::Electricity);
    let points = create_rw_signal(0);
    let allow_back = move || points.get() == 0;

    let debug = get_debug_opts();
    let processes = memo!(game.world.processes);
    let processes = move || {
        with!(|processes| {
            let output = output.get();
            let mut processes = processes
                .iter()
                .filter(|p| {
                    (!p.locked || debug.show_all_processes)
                        && p.output == output
                })
                .cloned()
                .collect::<Vec<_>>();
            processes.sort_by(|a, b| {
                a.name
                    .to_lowercase()
                    .cmp(&b.name.to_lowercase())
            });
            processes
        })
    };

    let has_changes =
        memo!(ui.has_process_mix_changes(output.get()));
    let mix_changes = memo!(ui.process_mix_changes);
    let changing_points = move || {
        let total = mix_changes.get()[output.get()]
            .values()
            .map(|change| change.abs())
            .sum::<isize>() as f32;
        (total / 2.).ceil()
    };
    let changes_time = move || {
        changing_points()
            / consts::PROCESS_POINTS_PER_CYCLE as f32
    };

    let tab = move |name: String,
                    icon: &'static str,
                    kind: Output| {
        let selected = output.get() == kind;
        let disabled = !allow_back();
        view! {
            <div
                class="planning-sub-tab"
                class:selected=selected
                class:disabled=disabled
                on:click=move |_| {
                    if allow_back() {
                        set_output.set(kind);
                    }
                }
            >

                <img src=icon/>
                <div>{name}</div>
            </div>
        }
    };

    let mix_tokens = move || {
        (0..points.get()).map(|_| {
            let tip = tip(icons::MIX_TOKEN, t!("One production point represents 5% of an entire production sector's productive capacity."));
            view! {
                <HasTip tip>
                    <div class="mix-token"></div>
                </HasTip>
            }
        }).collect::<Vec<_>>()
    };

    let change_notice = move || {
        let changes_time = changes_time().ceil() as usize;
        let ext = if changes_time > 1 { "s" } else { "" };
        t!("These changes will take {changesTime} planning cycle{ext} to take effect.", changesTime: changes_time, ext: ext)
    };

    let output_demands = move || {
        with!(|game| {
            display::outputs(&game.output_demand.total())
            .items()
            .map(|(output, demand)| {
                let tip = tip(
                    output.icon(),
                    t!("Global demand for {output}.", output: output.lower()),
                )
                .card(factors_card(None, output.into(), game));
                view! {
                    <HasTip tip>
                        <div class="demand-unit">
                        <span>{demand}</span><img class="demand-icon" src=output.icon()/>
                        </div>
                    </HasTip>
                }
            })
            .to_vec()
        })
    };

    let emissions = move || {
        with!(|game| {
            let emissions =
                game.byproducts.total().gtco2eq().round_to(1);
            let tip = tip(
            icons::EMISSIONS,
            t!("Current annual emissions, in gigatonnes of CO2 equivalent."),
        )
        .card(factors_card(None, Var::Emissions, game));
            view! {
                <HasTip tip>
                    <div class="demand-unit">
                    <span>{emissions}</span><img class="demand-icon" src=icons::EMISSIONS/>
                    </div>
                    </HasTip>
            }
        })
    };

    let scanner = ProcessScanner {
        points,
        on_change,
        mix_changes,
    };

    let estimated_changes = move || {
        with!(|game, mix_changes| {
            display_changes(game, mix_changes, &processes())
        })
    };
    view! {
        <div class="plan-change-select planning--page">
            <div class="planning--page-tabs">
                <Show when=move || !allow_back()>
                    <div class="unspent-warning">
                        {t!("Drag a card up to assign leftover production")}
                    </div>
                </Show>
                {move || tab(
                    t!("Electricity"),
                    icons::ELECTRICITY,
                    Output::Electricity,
                )}

                {move || tab(t!("Fuel"), icons::FUEL, Output::Fuel)}
                {move || tab(
                    t!("Crops"),
                    icons::PLANT_CALORIES,
                    Output::PlantCalories,
                )}

                {move || tab(
                    t!("Livestock"),
                    icons::ANIMAL_CALORIES,
                    Output::AnimalCalories,
                )}

                <div
                    class:disabled=move || !allow_back() || back_disabled.get()
                    class:highlight=back_highlighted
                    on:click=move |_| {
                        if allow_back() {
                            close.call(());
                        }
                    }
                >

                    {t!("Back")}
                </div>
            </div>
            <div class="available-mix-tokens">{mix_tokens}</div>

            <ScannerCards
                items=processes
                spec=scanner
            />

            <div>
                <Show when=move || has_changes.get()>
                    <div class="process-mix-change-notice-wrapper">
                        <div class="process-mix-change-notice">
                            <div>{change_notice}</div>
                            <div>{estimated_changes}</div>
                        </div>
                    </div>
                </Show>
                <div class="production--demand planning--demand">
                    {output_demands} {emissions}
                </div>
            </div>
        </div>
    }
}

fn calc_change(
    key: &str,
    before: f32,
    after: f32,
) -> Option<View> {
    let mut change = if before == 0. {
        if after > 0. {
            1.
        } else if after < 0. {
            -1.
        } else {
            0.
        }
    } else {
        (after - before) / before
    };
    if before < 0. {
        change *= -1.;
    }

    if change > 0.0 {
        let s = t!("increase {k} by {warn}{change}%", k: key, warn: if change > 100. {
            "⚠️"
        } else { "" }, change: display::percent(change, true));
        Some(
            view! {
                <span class="change change-increase">
                    <strong>{s}</strong>
                </span>
            }
            .into_view(),
        )
    } else if change < 0.0 {
        let s = t!("decrease {k} by {change}%", k: key, change: display::percent(change.abs(), true));
        Some(
            view! {
                <span class="change change-decrease">
                    <strong>{s}</strong>
                </span>
            }
            .into_view(),
        )
    } else {
        None
    }
}

#[derive(Default)]
struct Usage {
    emissions: f32,
    energy_use: f32,
    land_use: f32,
    water_use: f32,
    extinction_rate: f32,
}

fn estimate_changes(
    state: &State,
    mix_changes: &EnumMap<Output, BTreeMap<Id, isize>>,
    processes: &[Process],
) -> (Usage, Usage) {
    // Total demand for each of these
    let before = Usage {
        emissions: state.emissions.as_gtco2eq(),
        energy_use: state.output_demand.total().energy(),
        land_use: state.resource_demand.of(Resource::Land),
        water_use: state.resource_demand.of(Resource::Water),
        extinction_rate: state.world.extinction_rate,
    };

    // Demand for each of these just from the current set of processes
    let mut current = Usage::default();
    let available_land = state.world.starting_resources.land;
    for process in processes {
        let mix_share = process.mix_share as f32;
        let total = mix_share / 20.
            * state.output_demand.of(process.output);
        current.land_use += process.resources.land * total;
        current.water_use += process.resources.water * total;
        current.energy_use +=
            process.resources.energy() * total;
        current.emissions +=
            process.byproducts.gtco2eq() * total;
        current.extinction_rate +=
            process.extinction_rate(available_land) * total;
    }

    // Changed demand for each of these, just for the current set of processes
    let mut changed = Usage::default();
    for process in processes {
        let mix_share = process.mix_share as f32
            + (*mix_changes[process.output]
                .get(&process.id)
                .unwrap_or(&0)) as f32;
        let total = mix_share / 20.
            * state.output_demand.of(process.output);
        changed.land_use += process.resources.land * total;
        changed.water_use += process.resources.water * total;
        changed.energy_use +=
            process.resources.energy() * total;
        changed.emissions +=
            process.byproducts.gtco2eq() * total;
        changed.extinction_rate +=
            process.extinction_rate(available_land) * total;
    }

    // Changed overall/total/global demand for each of these
    // Subtract out previous process demand, then add in changed process demand
    let mut after = Usage::default();
    after.land_use =
        before.land_use - current.land_use + changed.land_use;
    after.water_use = before.water_use - current.water_use
        + changed.water_use;
    after.energy_use = before.energy_use - current.energy_use
        + changed.energy_use;
    after.emissions = before.emissions - current.emissions
        + changed.emissions;
    after.extinction_rate = before.extinction_rate
        - current.extinction_rate
        + changed.extinction_rate;

    (before, after)
}

fn display_changes(
    state: &State,
    mix_changes: &EnumMap<Output, BTreeMap<Id, isize>>,
    processes: &[Process],
) -> impl IntoView {
    let (before, after) =
        estimate_changes(state, mix_changes, processes);
    let descs = [
        calc_change(
            &t!("land use"),
            before.land_use,
            after.land_use,
        ),
        calc_change(
            &t!("water use"),
            before.water_use,
            after.water_use,
        ),
        calc_change(
            &t!("energy use"),
            before.energy_use,
            after.energy_use,
        ),
        calc_change(
            &t!("emissions"),
            before.emissions,
            after.emissions,
        ),
        calc_change(
            &t!("the extinction rate"),
            before.extinction_rate,
            after.extinction_rate,
        ),
    ]
    .into_iter()
    .filter_map(|c| c)
    .collect::<Vec<_>>();

    if descs.is_empty() {
        t!("They won't have much effect.").into_view()
    } else {
        view! {
            {t!("This output's production will")}:" "{descs}"."
        }
        .into_view()
    }
}
