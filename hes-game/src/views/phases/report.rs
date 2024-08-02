use crate::{
    consts,
    display::*,
    icons,
    state,
    state::{GameExt, Phase},
    t,
    ui,
    vars::Var,
    views::{
        events::Events,
        factors::factors_card,
        hud::Hud,
        intensity::{self, IntensityBar},
        tip,
        HasTip,
    },
    with_state,
};
use hes_engine::events::{
    Phase as EventPhase,
    Request as EngineRequest,
};
use leptos::*;

pub struct Request {
    text: String,
    bounty: isize,
}

#[component]
pub fn Report() -> impl IntoView {
    let state =
        expect_context::<RwSignal<crate::state::GameState>>();

    let events = create_rw_signal(vec![]);
    state.update_untracked(|state| {
        events.set(
            state
                .game
                .roll_events(EventPhase::ReportStart, None),
        );
    });

    let year = state!(world.year);
    let start_year = ui!(cycle_start_state.year);

    let finished_requests = store_value(vec![]);
    state.update_untracked(|state| {
        finished_requests
            .set_value(state.game.check_requests());
    });

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
                let reg = game.world.regions[idx].name.clone();
                (reg, events.clone())
            })
            .collect::<Vec<_>>()
    });
    let world_events = with_state!(|_game, ui| {
        ui.world_events.iter().map(|ev| {
            (
                ev.name.clone(),
                tip(
                    icons::CHANCE,
                    t!("This event occurred during this planning cycle.")
                ).card(ev.clone())
            )
        }).collect::<Vec<_>>()
    });
    let seat_changes = with_state!(|game, ui| {
        ui.cycle_start_state
            .parliament
            .iter()
            .enumerate()
            .map(|(i, start_seats)| {
                let npc = &game.npcs.by_idx(i);
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
    let requests_fulfilled = with_state!(|game, _ui| {
        finished_requests.get_value().into_iter().map(|(kind, id, active, bounty)| {
            match kind {
                EngineRequest::Project => {
                    let project = &game.world.projects[&id];
                    Request {
                        bounty: bounty as isize,
                        text: if active {
                            t!("Completed Request: Implement {name}", name: t!(&project.name))
                        } else {
                            t!("Completed Request: Stop {name}", name: t!(&project.name))
                        }
                    }
                }
                EngineRequest::Process => {
                    let process = &game.world.processes[&id];
                    Request {
                        bounty: bounty as isize,
                        text: if active {
                            t!("Completed Request: Unban {name}", name: t!(&process.name))
                        } else {
                            t!("Completed Request: Ban {name}", name: t!(&process.name))
                        }
                    }
                }
            }
        }).collect::<Vec<_>>()
    });

    let warming_tip = || {
        tip(
            icons::WARMING,
            t!("The current global temperature anomaly. <strong>Increased warming</strong> will damage your political capital. <b class=\"tip-goal\">Your goal is to get this below 1°C.</b>")
        )
    };
    let biodiversity_tip = move || {
        let tip_text = t!(
            r#"The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse. <b class="tip-goal">Your goal is to get this to below 20.</b>"#
        );
        crate::views::tip(icons::EXTINCTION_RATE, tip_text)
            .card(with!(|state| factors_card(
                None,
                Var::Biodiversity,
                &state.game,
            )))
    };

    let emissions_gt = state!(emissions_gt());
    let emissions_tip = move || {
        let tip_text = t!(r#"Current annual emissions are {emissions} gigatonnes. <b class="tip-goal">Your goal is to get this to below 0.</b>"#, emissions: emissions_gt.get());
        crate::views::tip(icons::EMISSIONS, tip_text).card(
            with!(|state| factors_card(
                None,
                Var::Emissions,
                &state.game,
            )),
        )
    };

    let contentedness_tip = move || {
        let tip_text = t!(
            r#"How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions. <b class="tip-warn">If this goes below 0 you will be removed from power.</b>"#
        );
        crate::views::tip(icons::CONTENTEDNESS, tip_text).card(
            with!(|state| factors_card(
                None,
                Var::Contentedness,
                &state.game,
            )),
        )
    };

    let temp_pc_change = move || {
        with!(|state| {
            let temp_change = state.game.world.temperature
                - state.ui.cycle_start_state.temperature;

            // Double temp change score for every degree above 1C
            let temp_change_multiplier =
                ((state.game.world.temperature.round() - 1.)
                    .max(0.)
                    * 2.)
                    .max(1.);

            // Temp scored for every 0.1C change
            let change = (temp_change * 10.).round()
                * -(consts::TEMPERATURE_PC as f32)
                * temp_change_multiplier;
            change as isize
        })
    };
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
    let ghg_pc_change = move || {
        with!(|state| {
            let emissions_change =
                state.game.state.emissions_gt()
                    - state.ui.cycle_start_state.emissions;
            (emissions_change * 2.).round() as isize
                * -(consts::EMISSIONS_PC as isize)
        })
    };
    let req_pc_change = move || {
        finished_requests
            .get_value()
            .into_iter()
            .map(|(_, _, _, bounty)| bounty)
            .sum::<usize>() as isize
    };
    let pc_change = move || {
        with!(|state| {
            let mut pc_change = temp_pc_change()
                + cont_pc_change()
                + ext_pc_change()
                + ghg_pc_change()
                + req_pc_change();
            pc_change += (state
                .ui
                .cycle_start_state
                .completed_projects
                .len()
                * consts::PC_PER_COMPLETED_PROJECT)
                as isize;
            pc_change += honeymoon_pc();
            pc_change
        })
    };

    let completed_projects = with_state!(|game, ui| {
        ui.cycle_start_state
            .completed_projects
            .iter()
            .map(|project_id| {
                game.world.projects[project_id].clone()
            })
            .collect::<Vec<_>>()
    });

    let next_phase = move || {
        let pc_change = pc_change();
        update!(|state| {
            state.game.change_political_capital(pc_change);

            // Apply process mix changes
            // and project upgrades.
            state.update_processes();
            state.upgrade_projects();

            // Reset session plan changes
            state.ui.plan_changes.clear();

            state.ui.points.refundable_research = 0;
            state.ui.phase = Phase::Interstitial;
        });
    };

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
    let cont_row = move || {
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
    };
    let ext_row = move || {
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
    };
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
        <Events events />
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
                                                            view! { <img src={icons::disaster_icon(&ev.icon)} /> }
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
