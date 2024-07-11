use crate::{
    consts,
    display::*,
    icons,
    state,
    state::Phase,
    t,
    ui,
    views::{
        events::Events,
        hud::Hud,
        intensity::{self, IntensityBar},
        tip,
        HasTip,
        Tip,
    },
    with_state,
    write_state,
};
use hes_engine::{events::Phase as EventPhase, game::Update};
use leptos::*;

pub struct Request {
    text: String,
    bounty: isize,
}

#[component]
pub fn Report() -> impl IntoView {
    let (events, set_events) = create_signal(vec![]);
    create_effect(move |_| {
        let state = expect_context::<
            RwSignal<crate::state::GameState>,
        >();
        state.update(|state| {
            let events = state.game.roll_events_for_phase(
                EventPhase::ReportStart,
                None,
            );
            set_events.set(events);
        });
    });

    let year = state!(world.year);
    let start_year = ui!(cycle_start_state.year);

    let outlook = state!(outlook());
    let start_outlook = ui!(cycle_start_state.contentedness);
    let start_cont_int = move || {
        intensity::scale(
            start_outlook.get(),
            intensity::Variable::WorldOutlook,
        )
    };
    let end_cont_int = move || {
        intensity::scale(
            outlook.get(),
            intensity::Variable::WorldOutlook,
        )
    };

    let exr = state!(world.extinction_rate);
    let start_exr = ui!(cycle_start_state.extinction_rate);
    let start_exr_int = move || {
        intensity::scale(
            start_exr.get(),
            intensity::Variable::Extinction,
        )
    };
    let end_exr_int = move || {
        intensity::scale(
            exr.get(),
            intensity::Variable::Extinction,
        )
    };

    let region_income_changes = with_state!(|game, ui| {
        game.world
            .regions
            .iter()
            .zip(ui.cycle_start_state.region_incomes.iter())
            .filter(|(reg, inc)| reg.income != **inc)
            .map(|(reg, _)| (reg.name.clone(), reg.income))
            .collect::<Vec<_>>()
    });
    let region_disasters = with_state!(|game, ui| {
        ui.annual_region_events
            .iter()
            .map(|(idx, events)| {
                let reg = game.world.regions[*idx].name.clone();
                (reg, events.clone())
            })
            .collect::<Vec<_>>()
    });
    let world_events = with_state!(|game, ui| {
        // TODO
        // import EVENTS from 'content/events.json';
        // ui.world_events
        // return state.worldEvents.map((ev_id) => {
        //     let ev = EVENTS[ev_id];
        //     return {
        //         name: ev.name,
        //         tip: {
        //             icon: 'chance',
        //             text: t('This event occurred during this planning cycle.'),
        //             card: {
        //                 type: 'Event',
        //                 data: ev
        //             }
        //         }
        //     }
        // });
        Vec::<(String, Tip)>::new()
    });
    let seat_changes = with_state!(|game, ui| {
        ui.cycle_start_state
            .parliament
            .iter()
            .enumerate()
            .map(|(i, start_seats)| {
                let npc = &game.npcs[i];
                let change = (npc.seats - start_seats).round();
                (npc.name.clone(), npc.seats, change)
            })
            .filter(|(_, _, change)| *change != 0.)
            .collect::<Vec<_>>()
    });
    let honeymoon_pc = with_state!(|game, ui| {
        if game.world.year
            < ui.start_year + consts::HONEYMOON_YEARS
        {
            consts::HONEYMOON_PC as isize
        } else {
            0
        }
    });
    let requests_fulfilled = with_state!(|game, ui| {
        // TODO
        // return game.checkRequests().map(([kind, id, active, bounty]) => {
        //   let text;
        //   if (kind == 'Project') {
        //     let project = state.gameState.projects[id];
        //     text = t(`Completed Request: ${active ? 'Implement' : 'Stop'} {name}`, {name: t(project.name)});
        //   } else if (kind == 'Process') {
        //     let process = state.gameState.processes[id];
        //     text = t(`Completed Request: ${active ? 'Unban' : 'Ban'} {name}`, {name: t(process.name)});
        //   }
        //   this.pcChange += bounty;
        //   return {text, bounty};
        // });
        Vec::<Request>::new()
    });

    let upgrade_projects = write_state!(|game, ui| {
        for (id, queued) in ui.queued_upgrades.iter_mut() {
            if *queued {
                *queued = false;
                game.upgrade_project(*id);
            }
        }
    });

    let update_processes = write_state!(|game, ui| {
        let mut rem_pts = consts::PROCESS_POINTS_PER_CYCLE;
        let mut add_pts = consts::PROCESS_POINTS_PER_CYCLE;

        // TODO for each output in processmixchanges...
        // let removePoints = consts.processPointsPerCycle;
        // let addPoints = consts.processPointsPerCycle;
        // let changes = state.processMixChanges[output];
        // let totalChanges = Object.values(state.processMixChanges[output]).reduce((acc, change) => {
        //   return acc + Math.abs(change);
        // }, 0);
        // while (removePoints > 0 && addPoints > 0 && totalChanges > 0) {
        //   Object.keys(changes).forEach((processId) => {
        //     let change = changes[processId]
        //     if (change < 0 && removePoints > 0) {
        //       changes[processId] += 1;
        //       removePoints -= 1;
        //       game.changeProcessMixShare(processId, -1);
        //       totalChanges--;
        //     } else if (change > 0 && addPoints > 0) {
        //       addPoints -= 1;
        //       changes[processId] -= 1;
        //       game.changeProcessMixShare(processId, 1);
        //       totalChanges--;
        //     }
        //   });
        //
    });

    let warming_tip = || {
        tip(
            icons::WARMING,
            t!("The current global temperature anomaly. <strong>Increased warming</strong> will damage your political capital. <b class=\"tip-goal\">Your goal is to get this below 1°C.</b>")
        )
    };
    let biodiversity_tip = move || {
        // TODO
        // return factors.tips.biodiversity(
        //   t(`The current biodiversity pressure. <strong>Increased biodiversity pressure</strong> will cost you political capital. <b class="tip-goal">Your goal is to get this to below 20.</b>`));
        warming_tip()
    };
    let contentedness_tip = move || {
        // TODO
        // return factors.tips.contentedness(
        //   t(`How people around the world feel about the state of things. <strong>Increasing or maintaining contentedness</strong> will gain you political capital. <b class="tip-warn">If this goes below 0 you will be removed from power.</b>`));
        warming_tip()
    };
    let emissions_tip = move || {
        // TODO
        // return factors.tips.emissions(
        //   t(`Current annual emissions, in gigatonnes of CO2 equivalent. <strong>Reducing emissions</strong> will gain you political capital. <b class="tip-goal">Your goal is to get this to below 0.</b>`));
        warming_tip()
    };

    let temp_pc_change = with_state!(|game, ui| {
        let temp_change = game.world.temperature
            - ui.cycle_start_state.temperature;

        // Double temp change score for every degree above 1C
        let temp_change_multiplier =
            ((game.world.temperature.round() - 1.).max(0.)
                * 2.)
                .max(1.);

        // Temp scored for every 0.1C change
        let change = (temp_change * 10.).round()
            * -(consts::TEMPERATURE_PC as f32)
            * temp_change_multiplier;
        change as isize
    });
    let cont_pc_change = move || {
        let end = end_cont_int();
        consts::CONTENTEDNESS_PC.get(end).unwrap_or_else(|| {
            consts::CONTENTEDNESS_PC.last().unwrap()
        })
    };
    let ext_pc_change = move || {
        let end = end_exr_int();
        consts::EXTINCTION_PC.get(end).unwrap_or_else(|| {
            consts::EXTINCTION_PC.last().unwrap()
        })
    };
    let ghg_pc_change = with_state!(|game, ui| {
        let emissions_change = game.emissions_gt()
            - ui.cycle_start_state.emissions;
        (emissions_change * 2.).round() as isize
            * -(consts::EMISSIONS_PC as isize)
    });

    let pc_change = with_state!(|game, ui| {
        let mut pc_change = temp_pc_change()
            + cont_pc_change()
            + ext_pc_change()
            + ghg_pc_change();
        pc_change +=
            (ui.cycle_start_state.completed_projects.len()
                * consts::PC_PER_COMPLETED_PROJECT)
                as isize;
        pc_change += honeymoon_pc();
        pc_change
    });

    let completed_projects = with_state!(|game, ui| {
        ui.cycle_start_state
            .completed_projects
            .iter()
            .filter_map(|update| match update {
                Update::Project { id } => {
                    Some(game.world.projects[*id].clone())
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    });

    let next_phase = write_state!(move |game, ui| {
        game.change_political_capital(pc_change());

        // Apply process mix changes
        // update_processes() // TODO

        upgrade_projects();
        ui.points.refundable_research = 0;
        ui.phase = Phase::Interstitial;

        // Reset session plan changes
        ui.plan_changes.clear();
    });

    let temp_row = with_state!(|game, ui| {
        let start = format!(
            "{:+.1}°C",
            ui.cycle_start_state.temperature
        );
        let end = format!("{:+.1}°C", game.world.temperature);
        let pc_change = format!("{:+}", temp_pc_change());

        view! {
            <HasTip tip=warming_tip.into_signal()>
                <tr class="report--primary-change">
                    <td><img src=icons::WARMING /> {t!("Temperature")}</td>
                    <td>{start}</td>
                    <td><small><img src=icons::ARROW_RIGHT /></small></td>
                    <td>{end}</td>
                    <td><strong>{pc_change}</strong></td>
                </tr>
            </HasTip>
        }
    });
    let cont_row = with_state!(|game, ui| {
        let pc_change = format!("{:+}", cont_pc_change());
        view! {
            <HasTip tip=contentedness_tip.into_signal()>
                <tr class="report--primary-change">
                    <td><img src=icons::CONTENTEDNESS /> {t!("Contentedness")}</td>
                    <td>
                        <IntensityBar intensity=start_cont_int invert={true} />
                    </td>
                    <td><small><img src=icons::ARROW_RIGHT /></small></td>
                    <td>
                        <IntensityBar intensity=end_cont_int invert={true} />
                    </td>
                    <td><strong>{pc_change}</strong></td>
                </tr>
            </HasTip>
        }
    });
    let ext_row = with_state!(|game, ui| {
        let pc_change = format!("{:+}", ext_pc_change());
        view! {
            <HasTip tip=biodiversity_tip.into_signal()>
                <tr class="report--primary-change">
                    <td><img src=icons::EXTINCTION_RATE /> {t!("Extinction Rate")}</td>
                    <td>
                        <IntensityBar intensity=start_exr_int />
                    </td>
                    <td><small><img src=icons::ARROW_RIGHT /></small></td>
                    <td>
                        <IntensityBar intensity=end_exr_int />
                    </td>
                    <td><strong>{pc_change}</strong></td>
                </tr>
            </HasTip>
        }
    });
    let ghg_row = with_state!(|game, ui| {
        let start =
            format!("{:+.1}", ui.cycle_start_state.emissions);
        let end = format!("{:+.1}", game.emissions_gt());
        let pc_change = format!("{:+}", ghg_pc_change());
        view! {
            <HasTip tip=emissions_tip.into_signal()>
                <tr class="report--primary-change">
                    <td><img src=icons::EMISSIONS /> {t!("Emissions")}</td>
                    <td>{start}</td>
                    <td><small><img src=icons::ARROW_RIGHT /></small></td>
                    <td>{end}</td>
                    <td><strong>{pc_change}</strong></td>
                </tr>
            </HasTip>
        }
    });

    view! {
        <Hud/>
        <Events events on_advance=|_| {} on_done=|_| {}/>
        <div class="report">
            <h2>{t!("Report")}</h2>
            <div class="report--body">
                <div class="report--inner">
                    <section>
                        <table class="report--changes">
                            <tr>
                                <th>
                                    <strong>{t!("Changes")}</strong>
                                </th>
                                <th>
                                    <strong>{start_year}</strong>
                                </th>
                                <th>
                                    <small>
                                        <img src=icons::ARROW_RIGHT/>
                                    </small>
                                </th>
                                <th>
                                    <strong>{year}</strong>
                                </th>
                                <th>
                                    <img src=icons::POLITICAL_CAPITAL/>
                                </th>
                            </tr>
                            {temp_row}
                            {cont_row}
                            {ext_row}
                            {ghg_row}
                            <Show when=move || { honeymoon_pc() > 0 }>
                                <tr class="report--primary-change">
                                    <td>{t!("Post-Revolution Optimism")}</td>
                                    <td></td>
                                    <td></td>
                                    <td></td>
                                    <td>
                                        <strong>{move || format!("{:+}", honeymoon_pc())}</strong>
                                    </td>
                                </tr>

                            </Show>
                            <tr class="report-spacer"></tr>
                            <Show when=move || !completed_projects().is_empty()>
                                <tr class="report-header">
                                    <td>{t!("Completed Projects")}</td>
                                </tr>
                                <For
                                    each=move || completed_projects()
                                    key=|proj| proj.id
                                    children=|proj| {
                                        let name = proj.name.clone();
                                        let tip = tip(
                                                icons::PROJECT,
                                                t!("This project was completed."),
                                            )
                                            .card(proj);
                                        view! {
                                            <tr>
                                                <HasTip tip=tip>
                                                    <td colspan="4">{t!(& name)}</td>
                                                </HasTip>
                                                <td>
                                                    <strong>
                                                        {format!("{:+}", consts::PC_PER_COMPLETED_PROJECT)}
                                                    </strong>
                                                </td>

                                            </tr>
                                        }
                                    }
                                />

                                <tr class="report-spacer"></tr>
                            </Show>

                            <Show when=move || !requests_fulfilled().is_empty()>
                                <tr class="report-header">
                                    <td>{t!("Completed Requests")}</td>
                                </tr>
                                <For
                                    each=move || requests_fulfilled()
                                    key=|req| req.text.clone()
                                    children=|req| {
                                        view! {
                                            <tr>
                                                <td colspan="4">{t!(& req.text)}</td>
                                                <td>
                                                    <strong>{format!("{:+}", req.bounty)}</strong>
                                                </td>
                                            </tr>
                                        }
                                    }
                                />

                                <tr class="report-spacer"></tr>

                            </Show>
                            <tr class="report--total-change">
                                <td colspan="4">
                                    <img src=icons::POLITICAL_CAPITAL/>
                                    {t!("Total Change")}
                                </td>
                                <td>{move || format!("{:+}", pc_change())}</td>
                            </tr>
                        </table>
                    </section>
                    <section>
                        <table>
                            <Show when=move || !seat_changes().is_empty()>
                                <tr class="report-header">
                                    <td>{t!("Parliament")}</td>
                                </tr>
                                <For
                                    each=move || seat_changes()
                                    key=|(name, _, _)| name.clone()
                                    children=|(name, seats, change)| {
                                        view! {
                                            <tr>
                                                <td colspan="2">{t!(& name)}</td>
                                                <td>
                                                    <strong>{format!("{:+}", change)}</strong>
                                                </td>
                                                <td>{seats}</td>
                                            </tr>
                                        }
                                    }
                                />

                                <tr class="report-spacer"></tr>
                            </Show>

                            <Show when=move || !world_events().is_empty()>
                                <tr class="report-header">
                                    <td colspan="2">{t!("Events")}</td>
                                    <td colspan="3" class="report-header-desc">
                                        {t!("Tap for more details.")}
                                    </td>
                                </tr>
                                <For
                                    each=move || world_events()
                                    key=|(name, _)| name.clone()
                                    children=|(name, tip)| {
                                        view! {
                                            <HasTip tip>
                                                <tr>
                                                    <td colspan="5">{t!(& name)}</td>
                                                </tr>
                                            </HasTip>
                                        }
                                    }
                                />

                                <tr class="report-spacer"></tr>
                            </Show>

                            <Show when=move || !region_income_changes().is_empty()>
                                <tr class="report-header">
                                    <td>{t!("Regions")}</td>
                                </tr>

                                <For
                                    each=move || region_income_changes()
                                    key=|(name, _)| name.clone()
                                    children=|(name, inc)| {
                                        view! {
                                            <tr>
                                                <td colspan="4">
                                                    {t!(
                                                        "{region} is now {income} income.", region : t!(& name),
                                                        income : t!(inc.lower())
                                                    )}

                                                </td>
                                            </tr>
                                        }
                                    }
                                />

                                <tr class="report-spacer"></tr>
                            </Show>

                            <Show when=move || !region_disasters().is_empty()>
                                <tr class="report-header">
                                    <td colspan="2">{t!("Disasters")}</td>
                                    <td colspan="3" class="report-header-desc">
                                        <img src=icons::HABITABILITY/>
                                        {t!("Reduce the habitability of regions.")}
                                    </td>
                                </tr>
                                <For
                                    each=move || region_disasters()
                                    key=|(name, _)| name.clone()
                                    children=|(name, evs)| {
                                        view! {
                                            <tr>
                                                <td>{t!(& name)}</td>
                                                <td colspan="4" class="report-disasters">
                                                    {evs
                                                        .iter()
                                                        .map(|ev| {
                                                            view! { <img src=&ev.icon/> }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </td>
                                            </tr>
                                        }
                                    }
                                />

                            </Show>
                        </table>
                    </section>
                    <button class="btn" on:click=move |_| next_phase()>
                        {t!("Next")}
                    </button>
                </div>
            </div>
        </div>
    }
}
