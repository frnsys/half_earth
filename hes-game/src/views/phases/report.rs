use crate::{
    consts,
    display::{self, *},
    icons,
    memo,
    state::{Phase, StateExt, UIState},
    t,
    vars::Var,
    views::{
        events::Events,
        factors::factors_card,
        hud::Hud,
        intensity::{self, IntensityBar},
        tip,
        DisplayEvent,
        HasTip,
    },
};
use hes_engine::{EventPhase, NPCRequest, State};
use leptos::*;

pub struct Request {
    text: String,
    bounty: isize,
}

#[component]
pub fn Report() -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

    let events = create_rw_signal::<Vec<DisplayEvent>>(vec![]);
    game.update_untracked(|game| {
        events.set(StateExt::roll_events(
            game,
            EventPhase::ReportStart,
        ));
    });
    ui.update_untracked(|ui| {
        ui.session_start_state = game.get_untracked();
    });

    let year = memo!(game.world.year);
    let start_year = memo!(ui.cycle_start_state.year);

    let finished_requests = store_value(vec![]);
    game.update_untracked(|game| {
        finished_requests.set_value(game.check_requests());
    });

    let outlook = memo!(game.outlook());
    let start_outlook =
        memo!(ui.cycle_start_state.contentedness);
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

    let exr = memo!(game.world.extinction_rate);
    let start_exr = memo!(ui.cycle_start_state.extinction_rate);
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

    let regions = memo!(game.world.regions);
    let start_region_incomes =
        memo!(ui.cycle_start_state.region_incomes);
    let region_income_changes = move || {
        with!(|regions, start_region_incomes| {
            regions
                .iter()
                .zip(start_region_incomes.iter())
                .filter(|(reg, inc)| reg.income != **inc)
                .map(|(reg, _)| (reg.name.clone(), reg.income))
                .collect::<Vec<_>>()
        })
    };

    let region_events = memo!(ui.annual_region_events);
    let region_disasters = move || {
        with!(|regions, region_events| {
            region_events
                .iter()
                .map(|(idx, events)| {
                    let reg = regions[idx].name.clone();
                    (reg, events.clone())
                })
                .collect::<Vec<_>>()
        })
    };

    let recent_world_events = memo!(ui.world_events);
    let world_events = move || {
        with!(|recent_world_events| {
            recent_world_events.iter().map(|ev| {
                (
                    ev.name.clone(),
                    tip(
                        icons::CHANCE,
                        t!("This event occurred during this planning cycle.")
                    ).card(ev.clone())
                )
            }).collect::<Vec<_>>()
        })
    };

    let npcs = memo!(game.npcs);
    let start_parliament =
        memo!(ui.cycle_start_state.parliament);
    let seat_changes = move || {
        with!(|start_parliament, npcs| {
            start_parliament
                .iter()
                .enumerate()
                .map(|(i, start_seats)| {
                    let npc = &npcs.by_idx(i);
                    let change =
                        (npc.seats - start_seats).round();
                    (npc.name.clone(), npc.seats, change)
                })
                .filter(|(_, _, change)| *change != 0.)
                .collect::<Vec<_>>()
        })
    };
    let honeymoon_pc = move || {
        with!(|year, start_year| {
            if *year < *start_year + consts::HONEYMOON_YEARS {
                consts::HONEYMOON_PC as isize
            } else {
                0
            }
        })
    };

    let projects = memo!(game.world.projects);
    let processes = memo!(game.world.processes);
    let requests_fulfilled = move || {
        with!(|projects, processes| {
            finished_requests.get_value().into_iter().map(|(kind, id, active, bounty)| {
                match kind {
                    NPCRequest::Project => {
                        let project = &projects[&id];
                        Request {
                            bounty: bounty as isize,
                            text: if active {
                                t!("Completed Request: Implement {name}", name: t!(&project.name))
                            } else {
                                t!("Completed Request: Stop {name}", name: t!(&project.name))
                            }
                        }
                    }
                    NPCRequest::Process => {
                        let process = &processes[&id];
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
        })
    };

    let warming_tip = || {
        tip(
            icons::WARMING,
            t!(
                r#"The current global temperature anomaly. <strong>Increased warming</strong> will damage your political capital. <b class="tip-goal">Your goal is to get this below 1°C.</b>"#
            ),
        )
    };
    let biodiversity_tip = move || {
        let tip_text = t!(
            r#"The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse. <b class="tip-goal">Your goal is to get this to below 20.</b>"#
        );
        crate::views::tip(icons::EXTINCTION_RATE, tip_text)
            .card(with!(|game| factors_card(
                None,
                Var::Biodiversity,
                &game,
            )))
    };

    let emissions_gt = memo!(game.emissions.display());
    let emissions_tip = move || {
        let tip_text = t!(r#"Current annual emissions are {emissions}. <b class="tip-goal">Your goal is to get this to below 0.</b>"#, emissions: emissions_gt.get());
        crate::views::tip(icons::EMISSIONS, tip_text).card(
            with!(|game| factors_card(
                None,
                Var::Emissions,
                &game,
            )),
        )
    };

    let contentedness_tip = move || {
        let tip_text = t!(
            r#"How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions. <b class="tip-warn">If this goes below 0 you will be removed from power.</b>"#
        );
        crate::views::tip(icons::CONTENTEDNESS, tip_text).card(
            with!(|game| factors_card(
                None,
                Var::Contentedness,
                &game,
            )),
        )
    };

    let temp = memo!(game.world.temperature);
    let start_temp = memo!(ui.cycle_start_state.temperature);
    let temp_pc_change = move || {
        with!(|temp, start_temp| {
            let temp_change = temp - start_temp;

            // Double temp change score for every degree above 1C
            let temp_change_multiplier =
                ((temp.round() - 1.).max(0.) * 2.).max(1.);

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
        let change = start_exr.get() - exr.get();
        let end = end_exr_int();
        consts::EXTINCTION_PC.get(end).unwrap_or_else(|| {
            consts::EXTINCTION_PC.last().unwrap()
        }) + (change.round() as isize * consts::BIODIVERSITY_PC)
            .max(0)
    };
    let emissions = memo!(game.emissions.as_gtco2eq());
    let start_emissions = memo!(ui.cycle_start_state.emissions);
    let ghg_pc_change = move || {
        with!(|emissions, start_emissions| {
            let emissions_change = emissions - start_emissions;
            (emissions_change * 2.).round() as isize
                * -consts::EMISSIONS_PC
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
        with!(|ui| {
            let mut pc_change = temp_pc_change()
                + cont_pc_change()
                + ext_pc_change()
                + ghg_pc_change()
                + req_pc_change();
            pc_change +=
                (ui.cycle_start_state.completed_projects.len()
                    * consts::PC_PER_COMPLETED_PROJECT)
                    as isize;
            pc_change += honeymoon_pc();
            pc_change
        })
    };

    let recent_completed_projects =
        memo!(ui.cycle_start_state.completed_projects);
    let completed_projects = move || {
        with!(|projects, recent_completed_projects| {
            recent_completed_projects
                .iter()
                .map(|project_id| projects[project_id].clone())
                .collect::<Vec<_>>()
        })
    };

    let next_phase = move || {
        let pc_change = pc_change();
        game.update_untracked(|game| {
            game.change_political_capital(pc_change);

            ui.update_untracked(|ui| {
                // Reset session plan changes
                ui.plan_changes.clear();
                ui.points.refundable_research = 0;
            });
        });

        ui.update(|ui| {
            ui.phase = Phase::Interstitial;
        });
    };

    let temp_row = move || {
        with!(|temp, start_temp| {
            let start = display::temp(*start_temp);
            let end = display::temp(*temp);
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
        })
    };
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
    let ghg_row = move || {
        with!(|emissions, start_emissions| {
            let start = format!("{:+.1}", start_emissions);
            let end = format!("{:+.1}", emissions);
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
        })
    };

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
                                                <td colspan="4">{req.text}</td>
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
