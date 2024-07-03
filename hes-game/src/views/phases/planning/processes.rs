use crate::display::factors::factors_card;
use crate::display::text::AsText;
use crate::display::Var;
use crate::views::phases::cards::{CardScanProps, Scannable, ScannerCards, ScannerControls};
use crate::views::{parts::Help, scanner::*};
use crate::{
    consts,
    display::format,
    icons::{self, HasIcon},
    state,
    state::{GameExt, Tutorial},
    state_with, t,
    util::to_ws_el,
    views::{cards::CardFocusArea, tip, HasTip},
    write_state,
};
use hes_engine::kinds::Output;
use hes_engine::{
    production::Process,
    projects::{Project, Status, Type},
};
use leptos::*;

impl Scannable for Process {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn as_card(&self) -> View {
        todo!() // TODO
    }
}

#[component]
pub fn Processes(
    #[prop(into)] on_change: Callback<()>,
    #[prop(into)] close: Callback<()>,
) -> impl IntoView {
    let back_disabled = state!(|state, ui| { ui.tutorial < Tutorial::ProcessesBack });
    let back_highlighted = state!(|state, ui| { ui.tutorial == Tutorial::ProcessesBack });

    let (process, set_process) = create_signal::<Option<Process>>(None);
    let (output, set_output) = create_signal(Output::Electricity);
    let (points, set_points) = create_signal(0);
    let allow_back = move || points.get() == 0;

    let processes = state!(move |state, ui| {
        let output = output.get();
        let mut processes = state
            .world
            .processes
            .iter()
            .filter(|p| !p.locked && p.output == output)
            .cloned()
            .collect::<Vec<_>>();
        processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        processes
    });

    let has_changes = state_with!(|state, ui, output| { ui.has_process_mix_changes(*output) });
    let changing_points = state_with!(|state, ui, output| {
        let total = ui.process_mix_changes[*output]
            .values()
            .map(|change| change.abs())
            .sum::<isize>() as f32;
        (total / 2.).ceil()
    });
    let changes_time = move || changing_points() / consts::PROCESS_POINTS_PER_CYCLE as f32;
    let add_point = write_state!(move |state, ui| {
        if let Some(process) = process.get() {
            let max_share = state.process_max_share(&process);
            set_points.update(|points| {
                ui.add_point(points, &process, max_share);

                // Consider the process mix 'changed'
                // when all points have been assigned
                if *points == 0 {
                    on_change.call(());
                }
            });
        }
    });
    let remove_point = write_state!(move |state, ui| {
        if let Some(process) = process.get() {
            set_points.update(|points| {
                ui.remove_point(points, &process);
            });
        }
    });
    let estimated_changes = state!(move |state, ui| {
        #[derive(Default)]
        struct Changes {
            emissions: f32,
            energy_use: f32,
            land_use: f32,
            water_use: f32,
            extinction_rate: f32,
        }

        // Total demand for each of these
        let before = Changes {
            emissions: state.emissions(),
            energy_use: state.output_demand.energy(),
            land_use: state.resources_demand.land,
            water_use: state.resources_demand.water,
            extinction_rate: state.world.extinction_rate,
        };

        // Demand for each of these just from the current set of processes
        let mut current = Changes::default();
        let starting_land = state.world.starting_resources.land;
        let processes = processes();
        for process in &processes {
            let mix_share = process.mix_share as f32;
            let total = mix_share / 20. * state.output_demand[process.output];
            current.land_use += process.resources.land * total;
            current.water_use += process.resources.water * total;
            current.energy_use += process.resources.energy() * total;
            current.emissions += process.byproducts.co2eq() * total;
            current.extinction_rate += process.extinction_rate(starting_land) * total;
        }

        // Changed demand for each of these, just for the current set of processes
        let mut changed = Changes::default();
        for process in &processes {
            let mix_share = process.mix_share as f32
                + (*ui.process_mix_changes[process.output]
                    .get(&process.id)
                    .unwrap_or(&0)) as f32;
            let total = mix_share / 20. * state.output_demand[process.output];
            current.land_use += process.resources.land * total;
            current.water_use += process.resources.water * total;
            current.energy_use += process.resources.energy() * total;
            current.emissions += process.byproducts.co2eq() * total;
            current.extinction_rate += process.extinction_rate(starting_land) * total;
        }

        // Changed overall/total/global demand for each of these
        // Subtract out previous process demand, then add in changed process demand
        let mut after = Changes::default();
        after.land_use = before.land_use - current.land_use + changed.land_use;
        after.water_use = before.water_use - current.water_use + changed.water_use;
        after.energy_use = before.energy_use - current.energy_use + changed.energy_use;
        after.emissions = before.emissions - current.emissions + changed.emissions;
        after.extinction_rate =
            before.extinction_rate - current.extinction_rate + changed.extinction_rate;

        let descs = [
            calc_change("land use", before.land_use, after.land_use),
            calc_change("water use", before.water_use, after.water_use),
            calc_change("energy use", before.energy_use, after.energy_use),
            calc_change("emissions", before.emissions, after.emissions),
            calc_change(
                "the extinction rate",
                before.extinction_rate,
                after.extinction_rate,
            ),
        ]
        .into_iter()
        .filter_map(|c| c)
        .collect::<Vec<_>>();

        if descs.is_empty() {
            view! {
              {t!("They won't have much effect.")}
            }
            .into_view()
        } else {
            view! {
                {t!("This output's production will")}: {descs}
            }
            .into_view()
        }
    });

    let addable = state!(move |state, ui| {
        if let Some(process) = process.get() {
            let max_share = state.process_max_share(&process);
            let change = ui.process_mix_changes[process.output]
                .get(&process.id)
                .unwrap_or(&0);
            points.get() != 0 && (*change + 1) < max_share as isize
        } else {
            false
        }
    });

    let finish_scan_add = move |controls: ScannerControls| {
        if addable() {
            let state = expect_context::<RwSignal<crate::state::GameState>>();
            state.try_update(|state| {
                let ui = &mut state.ui;
                let state = &mut state.game;
                if ui.tutorial == Tutorial::Processes {
                    ui.tutorial.advance();
                }
                add_point();
                pulse_card();
            });
            true
        } else {
            (controls.reject_scan)();
            shake_progress(to_ws_el(controls.progress_elem));
            false
        }
    };

    let add_props = CardScanProps {
        should_show: addable.into_signal(),
        scan_allowed: addable.into_signal(),
        scan_time: consts::PROCESS_CARD_SCAN_TIME,
        on_finish_scan: finish_scan_add.into(),
    };

    let subtractable = state!(move |state, ui| {
        if let Some(process) = process.get() {
            let change = ui.process_mix_changes[process.output]
                .get(&process.id)
                .unwrap_or(&0);
            process.mix_share as isize + *change != 0
        } else {
            false
        }
    });

    let finish_scan_rem = move |controls: ScannerControls| {
        remove_point();
        // If still subtractable, continue scanning
        subtractable()
    };

    let rem_props = CardScanProps {
        should_show: subtractable.into_signal(),
        scan_allowed: subtractable.into_signal(),
        scan_time: consts::PROCESS_CARD_WITHDRAW_TIME,
        on_finish_scan: finish_scan_rem.into(),
    };

    let tab = move |name: &str, icon: &'static str, kind: Output| {
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
                <div>{t!(name)}</div>
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

    let output_demands = state!(|state, ui| {
        format::outputs(&state.output_demand)
            .items()
            .map(|(output, demand)| {
                let tip = tip(
                    output.icon(),
                    t!("Global demand for {output}.", output: output.lower()),
                )
                .card(factors_card(None, output.into(), state));
                view! {
                    <HasTip tip>
                        <div class="demand-unit">
                        <span>{demand}</span><img class="demand-icon" src=output.icon()/>
                        </div>
                    </HasTip>
                }
            })
            .to_vec()
    });
    let emissions = state!(|state, ui| {
        let emissions = state.byproducts.gtco2eq();
        let tip = tip(
            icons::EMISSIONS,
            t!("Current annual emissions, in gigatonnes of CO2 equivalent."),
        )
        .card(factors_card(None, Var::Emissions, state));
        view! {
            <HasTip tip>
                <div class="demand-unit">
                <span>{emissions}</span><img class="demand-icon" src=icons::EMISSIONS/>
                </div>
                </HasTip>
        }
    });

    view! {
        <div class="plan-change-select planning--page">
            <div class="planning--page-tabs">
                <Show when=move || !allow_back()>
                    <div class="unspent-warning">
                        {t!("Drag a card up to assign leftover production")}
                    </div>
                </Show>
                {move || tab(
                    "Electricity",
                    icons::ELECTRICITY,
                    Output::Electricity,
                )}

                {move || tab("Fuel", icons::FUEL, Output::Fuel)}
                {move || tab(
                    "Crops",
                    icons::PLANT_CALORIES,
                    Output::PlantCalories,
                )}

                {move || tab(
                    "Livestock",
                    icons::ANIMAL_CALORIES,
                    Output::AnimalCalories,
                )}

                <div
                    class:disabled=move || !allow_back() || back_disabled()
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
                remove_label=move || t!("Remove points")
                add_props
                remove_props=rem_props
            />

            <div>
                <Show when=has_changes>
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

fn calc_change(key: &str, before: f32, after: f32) -> Option<View> {
    let mut change = 0.;
    if before == 0. {
        if after > 0. {
            change = 1.;
        } else if after < 0. {
            change = -1.;
        } else {
            change = 0.;
        }
    } else {
        change = (after - before) / before;
    }
    if before < 0. {
        change *= -1.;
    }

    if change > 0.0 {
        let s = t!("increase {k} by {warn}{change}%", k: t!(key), warn: if change > 100. {
            "⚠️"
        } else { "" }, change: format::percent(change, true));
        Some(
            view! {
                <span class="change-increase">
                    <strong>{s}</strong>
                </span>
            }
            .into_view(),
        )
    } else if change < 0.0 {
        let s = t!("decrease {k} by {change}%", k: t!(key), change: format::percent(change.abs(), true));
        Some(
            view! {
                <span class="change-decrease">
                    <strong>{s}</strong>
                </span>
            }
            .into_view(),
        )
    } else {
        None
    }
}
