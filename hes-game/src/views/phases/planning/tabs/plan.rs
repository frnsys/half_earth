use crate::{
    display::*,
    icons::{self, HasIcon},
    memo,
    state::{Phase, Tutorial, UIState},
    t,
    views::{
        cards::{MiniProcess, MiniProject},
        phases::planning::{ActivePlan, Processes, Projects},
        tip,
        HasTip,
        Help,
    },
};
use enum_map::EnumMap;
use hes_engine::{
    EventPhase,
    Feedstock,
    KindMap,
    Output,
    ProjectType,
    Resource,
    State,
};
use leptos::*;
use leptos_use::{use_document, use_event_listener};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Overview,
    Processes,
    Projects,
    All,
}

fn calc_slots() -> usize {
    let width =
        window().inner_width().unwrap().as_f64().unwrap();
    if width > 680. {
        9
    } else if width > 560. {
        7
    } else {
        5
    }
}

#[component]
fn Check() -> impl IntoView {
    view! {
        <svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M20 6L9 17L4 12" stroke="#03A781" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
    }
}

#[component]
fn Warn() -> impl IntoView {
    view! {
        <svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
         <path d="M11.9998 8.99999V13M11.9998 17H12.0098M10.6151 3.89171L2.39019 18.0983C1.93398 18.8863 1.70588 19.2803 1.73959 19.6037C1.769 19.8857 1.91677 20.142 2.14613 20.3088C2.40908 20.5 2.86435 20.5 3.77487 20.5H20.2246C21.1352 20.5 21.5904 20.5 21.8534 20.3088C22.0827 20.142 22.2305 19.8857 22.2599 19.6037C22.2936 19.2803 22.0655 18.8863 21.6093 18.0983L13.3844 3.89171C12.9299 3.10654 12.7026 2.71396 12.4061 2.58211C12.1474 2.4671 11.8521 2.4671 11.5935 2.58211C11.2969 2.71396 11.0696 3.10655 10.6151 3.89171Z" stroke="#DD0000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
         </svg>
    }
}

#[component]
pub fn Plan(
    #[prop(into)] on_plan_change: Callback<()>,
    #[prop(into)] on_page_change: Callback<EventPhase>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

    let (slots, set_slots) = create_signal(calc_slots());
    let max_width = move || match slots.get() {
        5 => "s",
        7 => "m",
        9 => "l",
        _ => "l",
    };
    let _ = use_event_listener(
        use_document(),
        ev::resize,
        move |_| {
            set_slots.set(calc_slots());
        },
    );

    let processes_disabled =
        memo!(ui.tutorial.lt(&Tutorial::Processes));
    let processes_highlighted =
        memo!(ui.tutorial.eq(&Tutorial::Processes));
    let ready_disabled =
        memo!(ui.tutorial.lt(&Tutorial::Ready));
    let ready_highlighted =
        memo!(ui.tutorial.eq(&Tutorial::Ready));
    let projects_highlighted =
        memo!(ui.tutorial.eq(&Tutorial::Projects));

    let projects = memo!(game.world.projects);
    let active_projects = move || {
        with!(|projects| {
            projects.part_of_plan().cloned().collect::<Vec<_>>()
        })
    };
    let n_projects = move || {
        let projs = active_projects();
        if projs.len() > slots.get() {
            // Save one spot for "View All"
            slots.get() - 1
        } else {
            projs.len()
        }
    };
    let placeholders = move || {
        (slots.get() as isize
            - active_projects().len() as isize)
            .max(0) as usize
    };

    let viewed = memo!(ui.viewed);
    let processes = memo!(game.world.processes);
    let any_new_projects = move || {
        with!(|projects, viewed| {
            projects.unlocked().any(|p| !viewed.contains(&p.id))
        })
    };
    let any_new_processes = move || {
        with!(|processes, viewed| {
            processes
                .unlocked()
                .any(|p| !viewed.contains(&p.id))
        })
    };
    let max_for_output = move |output: Output| {
        with!(|processes| {
            processes
                .iter()
                .filter(|p| p.output == output)
                .max_by_key(|p| p.mix_share)
                .cloned()
                .unwrap()
        })
    };
    let max_processes = move || {
        [
            max_for_output(Output::Electricity),
            max_for_output(Output::Fuel),
            max_for_output(Output::PlantCalories),
            max_for_output(Output::AnimalCalories),
        ]
    };
    let processes_over_limit = move || {
        with!(|game| game
            .world
            .processes
            .over_limit(
                game.output_demand.total(),
                game.feedstocks.available
            )
            .map(|p| t!(&p.name))
            .collect::<Vec<_>>())
    };

    let produced = memo!(game.produced);
    let output_demand = memo!(game.output_demand.total());
    let production_shortages = move || {
        let problems = with!(|produced, output_demand| {
            let mut problems: EnumMap<Output, f32> =
                EnumMap::from_array([1.; 4]);
            for output in Output::iter() {
                tracing::debug!(
                    "{output:?}: produced={}, demand={}",
                    crate::display::output(
                        produced.of(output),
                        output
                    ),
                    crate::display::output(
                        output_demand[output],
                        output
                    )
                );
                let met =
                    produced.of(output) / output_demand[output];
                if met >= 0.99 {
                    continue;
                } else {
                    if met < problems[output] {
                        problems[output] = met;
                    }
                }
            }
            problems
        });

        enum Severity {
            Mild,
            Alarming,
            Severe,
            Critical,
        }
        impl Severity {
            fn class(&self) -> String {
                format!(
                    "shortage-{}",
                    match self {
                        Severity::Mild => "mild",
                        Severity::Alarming => "alarming",
                        Severity::Severe => "severe",
                        Severity::Critical => "critical",
                    }
                )
            }
        }

        let problems: Vec<_> = problems
            .into_iter()
            .filter(|(_, met)| *met < 1.)
            .map(|(output, met)| {
                (
                    output,
                    if met >= 0.85 {
                        Severity::Mild
                    } else if met >= 0.75 {
                        Severity::Alarming
                    } else if met >= 0.5 {
                        Severity::Severe
                    } else {
                        Severity::Critical
                    },
                )
            })
            .collect();
        if problems.is_empty() {
            None
        } else {
            if problems.len() == 1 {
                let (output, severity) = &problems[0];
                let desc = match severity {
                    Severity::Mild => t!("There is a mild production shortage"),
                    Severity::Alarming => t!("There is a alarming production shortage"),
                    Severity::Severe => t!("There is a severe production shortage"),
                    Severity::Critical => t!("There is a critical production shortage"),
                };
                let class = severity.class();
                let details = format!(
                    "<b class={class}>{}</b>",
                    t!(&output.title())
                );
                Some(format!("{desc}: {details}"))
            } else {
                let list = problems
                    .into_iter()
                    .map(|(output, severity)| {
                        let class = severity.class();
                        let severity = match severity {
                            Severity::Mild => t!("mild"),
                            Severity::Alarming => t!("alarming"),
                            Severity::Severe => t!("severe"),
                            Severity::Critical => t!("critical"),
                        };
                        let title = t!(&output.title());
                        format!("<b class={class}>{title} ({severity})</b>")
                    })
                    .collect::<Vec<_>>().join("\n");
                let desc = t!(
                    "There are multiple production shortages:"
                );
                Some(format!("{desc} {list}"))
            }
        }
    };

    let resources = memo!(game.resources);
    let feedstocks = memo!(game.feedstocks);
    let input_shortages = move || {
        let resources: Vec<_> = with!(|resources| {
            Resource::iter()
                .filter(|res| resources.has_shortage(*res))
                .map(|r| t!(r.title()))
                .collect()
        });
        let feedstock: Vec<_> = with!(|feedstocks| {
            Feedstock::iter()
                .filter(|res| {
                    feedstocks.has_shortage(*res)
                        && *res != Feedstock::Other
                        && *res != Feedstock::Soil
                })
                .map(|r| t!(r.title()))
                .collect()
        });
        let shortages = [resources, feedstock].concat();
        if shortages.is_empty() {
            None
        } else {
            Some(
                t!("There is not enough {resources}. You should change your production mixes to use less of these or reduce demand elsewhere.", resources: shortages.join(", ")),
            )
        }
    };

    // Save when starting the planning session.
    game.with_untracked(move |game| {
        ui.with_untracked(move |ui| {
            crate::state::save(game, ui);
        });
    });

    let (_, set_phase) = slice!(ui.phase);
    let enter_world = move || {
        game.with_untracked(|game| {
            crate::state::save(game, &ui.get_untracked());
        });
        set_phase.set(Phase::Events);
    };

    let (page, set_page) = create_signal(Page::Overview);
    let close = move || {
        update!(|ui| {
            let page = page.get();
            if page == Page::Projects
                && ui.tutorial == Tutorial::ProjectsBack
            {
                ui.tutorial.advance();
            } else if page == Page::Processes
                && ui.tutorial == Tutorial::ProcessesBack
            {
                ui.tutorial.advance();
            }
        });
        set_page.set(Page::Overview);
        on_page_change.call(EventPhase::PlanningPlan);
    };
    let select_page = move |page| {
        set_page.set(page);
        let phase = match page {
            Page::Overview => EventPhase::PlanningPlan,
            Page::Projects => EventPhase::PlanningAdd,
            Page::Processes => EventPhase::PlanningProcesses,
            Page::All => EventPhase::PlanningPlan,
        };
        on_page_change.call(phase);
    };
    let on_kind_change = move |kind: ProjectType| {
        let phase = match kind {
            ProjectType::Policy => EventPhase::PlanningPolicies,
            ProjectType::Research => {
                EventPhase::PlanningResearch
            }
            ProjectType::Initiative => {
                EventPhase::PlanningInitiatives
            }
        };
        on_page_change.call(phase);
    };

    let process_over_limit_tip = move || {
        tip(
            icons::ALERT,
            t!("The following processes can't produce as much as they need to: {processesOverLimit}", processesOverLimit: processes_over_limit().join(", ")),
        )
    };
    let shortages_tip = move || {
        tip(
            icons::ALERT,
            format!(
                "{}. {}",
                production_shortages().unwrap_or(String::new()),
                input_shortages().unwrap_or(String::new())
            ),
        )
    };

    let card_slots = move || {
        (0..placeholders())
            .map(|_| {
                view! {
                    <div class="plan--change">
                        <div class="plan--change-placeholder"></div>
                    </div>
                }
            })
            .collect::<Vec<_>>()
    };

    let protected_land = memo!(game.protected_land);
    let resource_demand = memo!(game.resource_demand);
    let starting_resources =
        memo!(game.world.starting_resources);
    let resource_status = move || {
        with!(|resources,
               resource_demand,
               starting_resources,
               protected_land| {
            resource_demand.total().items().map(|(k, demand)| {
                let demand = match k {
                    Resource::Electricity | Resource::Fuel => {
                        to_energy_units(demand)
                    },
                    Resource::Water => {
                        resource(demand, k, resources.available)
                    }
                    Resource::Land => {
                        // For land we add in protected land as well.
                        let protected = protected_land * 100.;
                        resource(demand, k, *starting_resources) + protected
                    }
                };
                let available = match k {
                    Resource::Electricity | Resource::Fuel => {
                        to_energy_units(resources.available[k])
                    },
                    Resource::Land | Resource::Water => {
                        100.
                    }
                };
                let inner = view! {
                    <div class="resources-info-pill" class:not-enough={demand > available}>
                        <img src=k.icon()/>
                        {format!("{:.0}", demand)}/{format!("{:.0}", available)}
                    </div>
                };
                if production_shortages().is_some() {
                    view!{
                        <HasTip tip=shortages_tip.into_signal()>
                            {inner}
                        </HasTip>
                    }.into_view()
                } else {
                    inner.into_view()
                }
            }).to_vec()
        })
    };

    view! {
        <div class="planning--page plan">
            <Show when=move || page.get() == Page::Projects>
                <Projects on_kind_change on_change=on_plan_change close=move |_| close()/>
            </Show>
            <Show when=move || page.get() == Page::Processes>
                <Processes on_change=on_plan_change close=move |_| close()/>
            </Show>
            <Show when=move || page.get() == Page::All>
                <ActivePlan
                    close=move |_| close()
                    add=move |_| select_page(Page::Projects)
                />
            </Show>
            <Show when=move || page.get() == Page::Overview>
                <div class=format!("plan--changes {}", max_width())>
                    <Help
                        text=t!("Add some cards to get started")
                        x=0.5
                        y=220.0
                        center=true
                    />
                    <Show when=any_new_projects>
                        <img
                            class="plan-new-icon plan-new-projects-icon"
                            src="/assets/new.svg"
                        />
                    </Show>
                    <div class="plan--change">
                        <div
                            class="plan--add-change minicard"
                            on:click=move |_| select_page(Page::Projects)
                            class:highlight=projects_highlighted
                        >
                            <div>
                                <img src=icons::ADD/>
                                <div class="plan--action">{t!("Add")}</div>
                            </div>
                        </div>
                    </div>
                    <For
                        each=move || {
                            active_projects().into_iter().take(n_projects())
                        }

                        key=|proj| proj.id
                        children=move |project| {
                            let (project, _) = create_signal(project);
                            view! {
                                <div class="plan--change">
                                    <MiniProject project/>
                                </div>
                            }
                        }
                    />

                    {card_slots}
                    <Show when=move || {
                        active_projects().len() > slots.get()
                    }>
                        <div class="plan--change">
                            <div
                                class="plan--change-view-all btn"
                                on:click=move |_| select_page(Page::All)
                            >
                                {t!("View All")}
                            </div>
                        </div>
                    </Show>
                </div>
                <div class="plan--production">
                    <div class="plan--production-icons">
                        <Show when=any_new_processes>
                            <img class="plan-new-icon" src="/assets/new.svg"/>
                        </Show>
                        <Show when=move || !processes_over_limit().is_empty()>
                            <HasTip tip=process_over_limit_tip.into_signal()>
                                <img class="plan-alert" src=icons::ALERT/>
                            </HasTip>
                        </Show>
                        <Show when=move || production_shortages().is_some()>
                            <HasTip tip=shortages_tip.into_signal()>
                                <img class="plan-alert" src=icons::ALERT/>
                            </HasTip>
                        </Show>
                    </div>

                    <div class="plan--production--processes">
                        <For
                            each=move || max_processes()
                            key=|proc| proc.id
                            children=move |process| {
                                let (process, _) = create_signal(process);
                                let produced = with!(|process, produced| {
                                    crate::display::output(produced.of(process.output), process.output)
                                });
                                let demand = with!(|process, output_demand| {
                                    crate::display::output(output_demand[process.output], process.output)
                                });

                                let icon = with!(|process| process.output.icon());
                                view! {
                                    <div>
                                        <MiniProcess process/>
                                        <div class="production-info">
                                            {move || {
                                                if produced/demand < 0.99 {
                                                    view! {
                                                        <HasTip tip=shortages_tip.into_signal()>
                                                            <Warn />
                                                        </HasTip>
                                                    }
                                                } else {
                                                    view! {
                                                        <Check />
                                                    }
                                                }
                                            }}
                                            {format!("{:.0}", produced)}/{format!("{:.0}", demand)}
                                        </div>
                                    </div>
                                }
                            }
                        />

                    </div>
                    <div class="resources-info">
                        {resource_status}
                    </div>
                    <div
                        class="plan--production-button btn"
                        class:disabled=processes_disabled
                        class:highlight=processes_highlighted
                        on:click=move |_| select_page(Page::Processes)
                    >
                        {t!("Change Production")}
                    </div>
                </div>
                <div class="plan--ready-outer">
                    <div class="plan--ready-inner">
                        <button
                            class="plan--ready"
                            class:disabled=ready_disabled
                            class:highlight=ready_highlighted
                            on:click=move |_| enter_world()
                        >
                            {t!("Ready")}
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}
