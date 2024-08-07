use crate::{
    display::*,
    icons,
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
            projects.changeable().cloned().collect::<Vec<_>>()
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
    let placeholders =
        move || (slots.get() - active_projects().len()).max(0);

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

        let problems: Vec<_> = problems
            .into_iter()
            .filter(|(_, met)| *met < 1.)
            .map(|(output, met)| {
                (
                    output,
                    if met >= 0.85 {
                        t!("mild")
                    } else if met >= 0.75 {
                        t!("alarming")
                    } else if met >= 0.5 {
                        t!("severe")
                    } else {
                        t!("critical")
                    },
                )
            })
            .collect();
        if problems.is_empty() {
            None
        } else {
            if problems.len() == 1 {
                let (output, severity) = &problems[0];
                let class = format!("shortage-{severity}");
                let desc = t!(&format!(
                    "There is a {severity} production shortage",
                ));
                let details = format!(
                    "<b class={class}>{}</b>",
                    t!(&output.title())
                );
                Some(format!("{desc}: {details}"))
            } else {
                let list = problems
                    .into_iter()
                    .map(|(output, severity)| {
                        let class = format!("shortage-{severity}");
                        let severity = t!(&severity);
                        let title = t!(&output.title());
                        format!("<b class={class}>{title} ({severity})</b>")
                    })
                    .collect::<Vec<_>>().join("\n");
                let desc =
                    "There are multiple production shortages:";
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
    let on_change = move |_| {
        on_plan_change.call(());
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

    view! {
        <div class="planning--page plan">
            <Show when=move || page.get() == Page::Projects>
                <Projects on_kind_change on_change close=move |_| close()/>
            </Show>
            <Show when=move || page.get() == Page::Processes>
                <Processes on_change close=move |_| close()/>
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
                                view! { <MiniProcess process/> }
                            }
                        />

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
