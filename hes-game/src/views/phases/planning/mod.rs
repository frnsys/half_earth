mod active_plan;
mod processes;
mod projects;
mod region_item;
mod tabs;

pub use active_plan::ActivePlan;
use hes_engine::events::{Flag, Phase as EventPhase};
pub use processes::Processes;
pub use projects::Projects;
use tabs::{Dashboard, Parliament, Plan, Regions};

use crate::{
    audio,
    debug::get_debug_opts,
    state::{GameExt, Tutorial},
    t,
    ui,
    ui_rw,
    views::{hud::Hud, Events},
    write_state,
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
    let state =
        expect_context::<RwSignal<crate::state::GameState>>();

    audio::play_phase_music("/assets/music/planning.mp3", true);

    let events = create_rw_signal(vec![]);
    state.update_untracked(|state| {
        let mut evs = state
            .game
            .roll_events(EventPhase::PlanningStart, None);
        evs.extend(
            state
                .game
                .roll_events(EventPhase::PlanningPlan, None),
        );

        if get_debug_opts().skip_to_planning {
            evs.retain(|ev| ev.name != "Planning Intro");
        }

        events.set_untracked(evs);
    });

    let has_changes = ui!(has_any_process_mix_changes());

    let (page, set_page) = create_signal(Page::Plan);
    let select_page = move |page: Page| {
        tracing::debug!("Selecting planning page.");
        set_page.set(page);

        let phase = match page {
            Page::Plan => EventPhase::PlanningPlan,
            Page::Regions => EventPhase::PlanningRegions,
            Page::Dashboard => EventPhase::PlanningDashboard,
            Page::Parliament => EventPhase::PlanningParliament,
        };

        state.update(|state| {
            tracing::debug!("Rolling planning page events.");
            events.set(state.game.roll_events(phase, None));
        });
    };

    let (cur_tutorial, set_tutorial) = ui_rw!(tutorial);
    let tab = move |label: &'static str,
                    p: Page,
                    tutorial: Tutorial| {
        let active = page.get() == p;
        let highlight = cur_tutorial.get() == tutorial;
        let disabled = cur_tutorial.get() < tutorial;
        view! {
            <div
                class="planning--tab"
                class:active=active
                class:highlight=highlight
                class:disabled=disabled
                on:click=move |_| {
                    select_page(p);
                    if active {
                        write_state!(| _, ui | { ui.tutorial.advance(); });
                    }
                }
            >

                {t!(label)}
            </div>
        }
    };

    let page_view = move || match page.get() {
        Page::Plan => {
            view! { <Plan on_plan_change=move |_| {
                tracing::debug!("Plan changed.");
                state.update(|state| {
                    events.set(state.game.roll_events(EventPhase::PlanningPlanChange, None));
                });
            } on_page_change=move |phase| {
                tracing::debug!("Plan page changed.");
                update!(|state, events| {
                    events.extend(state.game.roll_events(phase, None));
                });
            }/> }
        }
        Page::Parliament => view! { <Parliament/> },
        Page::Dashboard => view! { <Dashboard/> },
        Page::Regions => view! { <Regions/> },
    };

    let on_done = move |_| {
        tracing::debug!("Planning events finished.");
        update!(|state| {
            if state.game.flags.contains(&Flag::SkipTutorial) {
                tracing::debug!("Skipping tutorial.");
                state.ui.tutorial = Tutorial::Ready;
            } else if state
                .game
                .flags
                .contains(&Flag::RepeatTutorial)
                && !state.ui.tutorial_restarted
            {
                tracing::debug!("Restarting tutorial.");
                state.ui.tutorial_restarted = true;
                state.ui.tutorial = Tutorial::Projects;
                events.set(state.game.roll_events(
                    EventPhase::PlanningStart,
                    None,
                ));
            }

            let should_advance = match page.get_untracked() {
                Page::Parliament => {
                    state.ui.tutorial == Tutorial::Parliament
                }
                Page::Dashboard => {
                    state.ui.tutorial == Tutorial::Dashboard
                }
                Page::Regions => {
                    state.ui.tutorial == Tutorial::Regions
                }
                Page::Plan => {
                    state.ui.tutorial == Tutorial::Plan
                }
            };
            if should_advance {
                state.ui.tutorial.advance();
            }
        });
    };

    view! {
        <Hud/>
        <Events events on_done />
        <div class="planning">
            <header>
                {move || tab("Plan", Page::Plan, Tutorial::Plan)}
                {move || tab(
                    "Govt",
                    Page::Parliament,
                    Tutorial::Parliament,
                )}
                {move || tab("Stats", Page::Dashboard, Tutorial::Dashboard)}
                {move || tab("World", Page::Regions, Tutorial::Regions)}
            </header>
            {page_view}
        </div>
    }
}
