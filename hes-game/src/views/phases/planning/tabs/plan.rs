use crate::views::cards::{MiniProcess, MiniProject};
use crate::views::phases::planning::{ActivePlan, Processes, Projects};
use crate::views::{parts::Help, tip, HasTip};
use crate::{
    display::text::AsText,
    icons, state,
    state::{Phase, Tutorial},
    t, write_state,
};
use enum_map::EnumMap;
use hes_engine::kinds::{Feedstock, Output};
use hes_engine::ProjectType;
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

#[component]
pub fn Plan() -> impl IntoView {
    let save = write_state!(|state, ui| {
        // TODO
        // state.save();
    });

    let (slots, set_slots) = create_signal(5);
    let max_width = move || match slots.get() {
        5 => "s",
        7 => "m",
        9 => "l",
        _ => "l",
    };
    use_event_listener(use_document(), ev::resize, move |ev| {
        let width = window().inner_width().unwrap().as_f64().unwrap();
        if width > 680. {
            set_slots.set(9);
        } else if width > 560. {
            set_slots.set(7);
        } else {
            set_slots.set(5);
        }
    });

    let processes_disabled = state!(|state, ui| ui.tutorial < Tutorial::Processes);
    let processes_highlighted = state!(|state, ui| ui.tutorial == Tutorial::Processes);
    let ready_disabled = state!(|state, ui| ui.tutorial < Tutorial::Ready);
    let ready_highlighted = state!(|state, ui| ui.tutorial == Tutorial::Ready);
    let projects_highlighted = state!(|state, ui| ui.tutorial == Tutorial::Projects);

    let active_projects = state!(move |state, ui| {
        state
            .world
            .projects
            .iter()
            .filter(|p| p.is_online() || p.is_building())
            .cloned()
            .collect::<Vec<_>>()
    });
    let n_projects = move || {
        let projs = active_projects();
        if projs.len() > slots.get() {
            // Save one spot for "View All"
            slots.get() - 1
        } else {
            projs.len()
        }
    };
    let placeholders = move || (slots.get() - active_projects().len()).max(0);
    let any_new_projects = state!(|state, ui| {
        state
            .world
            .projects
            .iter()
            .filter(|p| !p.locked)
            .any(|p| !ui.viewed.contains(&p.ref_id))
    });
    let any_new_processes = state!(|state, ui| {
        state
            .world
            .processes
            .iter()
            .filter(|p| !p.locked)
            .any(|p| !ui.viewed.contains(&p.ref_id))
    });
    let max_for_output = move |output: Output| {
        let state = expect_context::<RwSignal<crate::state::GameState>>();
        state
            .get()
            .game
            .world
            .processes
            .iter()
            .filter(|p| p.output == output)
            .max_by_key(|p| p.mix_share)
            .cloned()
            .unwrap()
    };
    let max_processes = move || {
        [
            max_for_output(Output::Electricity),
            max_for_output(Output::Fuel),
            max_for_output(Output::PlantCalories),
            max_for_output(Output::AnimalCalories),
        ]
    };
    let processes_over_limit = state!(|state, ui| {
        state
            .world
            .processes
            .iter()
            .filter(|p| p.mix_share > 0 && p.mix_share > state.process_max_share(p))
            .map(|p| t!(&p.name))
            .collect::<Vec<_>>()
    });
    let production_shortages = state!(|state, ui| {
        let mut total = 0;
        let mut problems: EnumMap<Output, f32> = EnumMap::from_array([1.; 4]);
        for output in Output::iter() {
            let met = state.produced[output] / state.output_demand[output];
            if met >= 0.99 {
                continue;
            } else {
                if met < problems[output] {
                    problems[output] = met;
                }
            }
        }

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
                Some(view! {
                {t!(&format!("There is a {severity} production shortage"))}: <b class=class>output.title()</b>
                })
            } else {
                let list: Vec<_> = problems
                    .into_iter()
                    .map(|(output, severity)| {
                        let class = format!("shortage-{severity}");
                        view! {
                            <b class=class>output.title() ({t!(&severity)})</b>
                        }
                    })
                    .collect();
                Some(view! {
                    {t!("There are multiple production shortages:")}
                    {list}
                })
            }
        }
    });

    let input_shortages = state!(|state, ui| {
        let resources: Vec<_> = hes_engine::kinds::Resource::iter()
            .filter(|res| {
                let shortage = state.required_resources[*res] - state.resources[*res];
                shortage > 0.
            })
            .collect();
        let feedstock: Vec<_> = hes_engine::kinds::Feedstock::iter()
            .filter(|res| {
                let shortage = state.required_feedstocks[*res] - state.feedstocks[*res];
                shortage > 0. && *res != Feedstock::Other && *res != Feedstock::Soil
            })
            .collect();
        let shortages = resources
            .into_iter()
            .map(|r| t!(r.title()))
            .chain(feedstock.into_iter().map(|r| t!(r.title())))
            .collect::<Vec<_>>();
        if shortages.is_empty() {
            None
        } else {
            Some(view! {
                t!("There is not enough {resources}. You should change your production mixes to use less of these or reduce demand elsewhere.", resources: shortages.join(", "))
            })
        }
    });

    let enter_world = write_state!(|state, ui| {
        if ui.tutorial == Tutorial::Ready {
            ui.tutorial.advance();
        }
        // TODO
        // state.save();
        ui.phase = Phase::Events;
    });

    let (page, set_page) = create_signal(Page::Overview);
    let close = write_state!(move |state, ui| {
        let page = page.get();
        if page == Page::Projects && ui.tutorial == Tutorial::ProjectsBack {
            ui.tutorial.advance();
        } else if page == Page::Processes && ui.tutorial == Tutorial::ProcessesBack {
            ui.tutorial.advance();
        }
        set_page.set(Page::Overview);
        // this.$emit('page', 'Plan');
    });
    let select_page = move |page| {
        set_page.set(page);
        // this.$emit('page', page);
        // if page == Page::Projects {
        // state.help[addTip] = true;
        // }
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
            "TODO".into(),
            // format!("{}. {}", production_shortages(), input_shortages()), // TODO need to handle
            // the html here
        )
    };

    // TODO
    let on_kind_change = move |kind: ProjectType| {};
    let on_change = move |_| {};

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
                <div class="plan--changes" class=max_width>
                    <Help
                        text=t!("Add some cards to get started")
                        x=0.5
                        y=220.0
                        center=true
                    />
                    <Show when=any_new_projects>
                        <img
                            class="plan-new-icon plan-new-projects-icon"
                            src="/public/assets/new.svg"
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

                    <div class="plan--change" v-for="i in placeholders">
                        <div class="plan--change-placeholder"></div>
                    </div>
                    <div
                        class="plan--change"
                        v-if="activeProjects.length > this.slots"
                    >
                        <div
                            class="plan--change-view-all btn"
                            on:click=move |_| select_page(Page::All)
                        >
                            {t!("View All")}
                        </div>
                    </div>
                </div>
                <div class="plan--production">
                    <div class="plan--production-icons">
                        <Show when=any_new_processes>
                            <img class="plan-new-icon" src="/public/assets/new.svg"/>
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
                    <For
                        each=move || max_processes()
                        key=|proc| proc.id
                        children=move |process| {
                            let (process, _) = create_signal(process);
                            view! { <MiniProcess process/> }
                        }
                    />

                    <div class="plan--production--processes"></div>
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
