mod active_plan;
mod processes;
mod projects;
mod region_item;
mod tabs;

pub use active_plan::ActivePlan;
use hes_engine::{EventPhase, Flag, State};
use leptos_hotkeys::use_hotkeys;
pub use processes::Processes;
pub use projects::Projects;
use tabs::{Dashboard, Parliament, Plan, Regions};
use crate::util::send_click;

use crate::{
    audio,
    debug::get_debug_opts,
    memo,
    state::{
        update_factors,
        Settings,
        StateExt,
        Tutorial,
        UIState,
    },
    t,
    views::{hud::Hud, Events},
};
use leptos::*;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Plan,
    Parliament,
    Dashboard,
    Regions,
}
impl std::fmt::Display for Page {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plan => "Plan",
                Self::Parliament => "Parliament",
                Self::Dashboard => "Dashboard",
                Self::Regions => "Regions",
            }
        )
    }
}

#[component]
pub fn Planning() -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

    use_hotkeys!(("keyp") => move |_| {
        send_click("tab-Plan");
    });

    use_hotkeys!(("keyg") => move |_| {
        send_click("tab-Parliament");
    });

    use_hotkeys!(("keys") => move |_| {
        send_click("tab-Dashboard");
    });

    use_hotkeys!(("keyw") => move |_| {
        send_click("tab-Regions");
    });

    audio::play_phase_music("/assets/music/planning.mp3", true);

    let events = create_rw_signal(vec![]);
    game.update_untracked(|game| {
        ui.update_untracked(|ui| {
            update_factors(game);

            let points = game.collect_research_points();
            ui.points.research += points;
        });

        let mut evs = [
            StateExt::roll_events(
                game,
                EventPhase::PlanningStart,
            ),
            StateExt::roll_events(
                game,
                EventPhase::PlanningPlan,
            ),
        ]
        .concat();

        if get_debug_opts().skip_to_planning {
            evs.retain(|ev| {
                ev.name != "Planning Intro"
                    && ev.name != "Welcome Back"
            });
        }

        events.set_untracked(evs);
    });

    let (page, set_page) = create_signal(Page::Plan);
    let select_page = move |page: Page| {
        set_page.set(page);

        let phase = match page {
            Page::Plan => EventPhase::PlanningPlan,
            Page::Regions => EventPhase::PlanningRegions,
            Page::Dashboard => EventPhase::PlanningDashboard,
            Page::Parliament => EventPhase::PlanningParliament,
        };

        game.update_untracked(|game| {
            events.set(StateExt::roll_events(game, phase));
        });
    };

    let cur_tutorial = memo!(ui.tutorial);
    let tab =
        move |label: String, p: Page, tutorial: Tutorial| {
            let active = page.get() == p;
            let highlight = cur_tutorial.get() == tutorial;
            let disabled = cur_tutorial.get() < tutorial;
            view! {
                <div
                    id=format!("tab-{}", p)
                    class="planning--tab"
                    class:active=active
                    class:highlight=highlight
                    class:disabled=disabled
                    on:click=move |_| {
                        select_page(p);
                    }
                >

                    {label}
                </div>
            }
        };

    let on_plan_change = move |_| {
        game.update_untracked(|game| {
            events.set(StateExt::roll_events(
                game,
                EventPhase::PlanningPlanChange,
            ));
        });
    };

    let page_view = move || match page.get() {
        Page::Plan => {
            view! { <Plan on_plan_change
            on_page_change=move |phase| {
                game.update_untracked(|game| {
                    update!(|events| {
                        events.extend(StateExt::roll_events(game, phase));
                    });
                });
            }/> }
        }
        Page::Parliament => view! { <Parliament/> },
        Page::Dashboard => view! { <Dashboard/> },
        Page::Regions => view! { <Regions/> },
    };

    let on_done = move |_| {
        update!(|ui| {
            game.update_untracked(|game| {
                if game.flags.contains(&Flag::SkipTutorial) {
                    ui.tutorial = Tutorial::Ready;
                    let (_, settings) = Settings::rw();
                    settings.update(|state| {
                        state.tutorial = Tutorial::Ready;
                        if state.runs_played == 0 {
                            state.runs_played = 1;
                        }
                    });
                } else if game
                    .flags
                    .contains(&Flag::RepeatTutorial)
                    && !ui.tutorial_restarted
                {
                    ui.tutorial_restarted = true;
                    ui.tutorial = Tutorial::Projects;
                    events.set(StateExt::roll_events(
                        game,
                        EventPhase::PlanningStart,
                    ));
                }
            });

            let should_advance = match page.get_untracked() {
                Page::Parliament => {
                    ui.tutorial == Tutorial::Parliament
                }
                Page::Dashboard => {
                    ui.tutorial == Tutorial::Dashboard
                }
                Page::Regions => {
                    ui.tutorial == Tutorial::Regions
                }
                Page::Plan => ui.tutorial == Tutorial::Plan,
            };
            if should_advance {
                ui.tutorial.advance();
                if ui.tutorial == Tutorial::Ready {
                    let (_, settings) = Settings::rw();
                    settings.update(|state| {
                        state.tutorial = Tutorial::Ready;
                        if state.runs_played == 0 {
                            state.runs_played = 1;
                        }
                    });
                }
            }
        });
    };

    view! {
        <Hud/>
        <Events events on_done />
        <div class="planning">
            <header>
                {move || tab(t!("Plan"), Page::Plan, Tutorial::Plan)}
                {move || tab(
                    t!("Govt"),
                    Page::Parliament,
                    Tutorial::Parliament,
                )}
                {move || tab(t!("Stats"), Page::Dashboard, Tutorial::Dashboard)}
                {move || tab(t!("World"), Page::Regions, Tutorial::Regions)}
            </header>
            {page_view}
        </div>
    }
}
