mod active_plan;
mod processes;
mod projects;
mod region_item;
mod tabs;

pub use active_plan::ActivePlan;
use hes_engine::{
    events::{Flag, Phase as EventPhase},
    Game,
};
pub use processes::Processes;
pub use projects::Projects;
use tabs::{Dashboard, Parliament, Plan, Regions};

use crate::{
    audio,
    debug::get_debug_opts,
    memo,
    state::{GameExt, Tutorial, UIState},
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
    let game = expect_context::<RwSignal<Game>>();
    let ui = expect_context::<RwSignal<UIState>>();

    audio::play_phase_music("/assets/music/planning.mp3", true);

    let events = create_rw_signal(vec![]);
    game.update_untracked(|game| {
        let mut evs =
            game.roll_events(EventPhase::PlanningStart, None);
        evs.extend(
            game.roll_events(EventPhase::PlanningPlan, None),
        );

        if get_debug_opts().skip_to_planning {
            evs.retain(|ev| ev.name != "Planning Intro");
        }

        events.set_untracked(evs);
    });

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

        game.update_untracked(|game| {
            tracing::debug!("Rolling planning page events.");
            events.set(game.roll_events(phase, None));
        });
    };

    let cur_tutorial = memo!(ui.tutorial);
    let tab = move |label: &'static str,
                    p: Page,
                    tutorial: Tutorial| {
        tracing::debug!("Tab: {}", label);
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
                game.update_untracked(|game| {
                    events.set(game.roll_events(EventPhase::PlanningPlanChange, None));
                });
            } on_page_change=move |phase| {
                tracing::debug!("Plan page changed.");
                game.update_untracked(|game| {
                    update!(|events| {
                        events.extend(game.roll_events(phase, None));
                    });
                });
            }/> }
        }
        Page::Parliament => view! { <Parliament/> },
        Page::Dashboard => view! { <Dashboard/> },
        Page::Regions => view! { <Regions/> },
    };

    let on_done = move |_| {
        tracing::debug!("Planning events finished.");

        update!(|game, ui| {
            if game.flags.contains(&Flag::SkipTutorial) {
                tracing::debug!("Skipping tutorial.");
                ui.tutorial = Tutorial::Ready;
            } else if game.flags.contains(&Flag::RepeatTutorial)
                && !ui.tutorial_restarted
            {
                tracing::debug!("Restarting tutorial.");
                ui.tutorial_restarted = true;
                ui.tutorial = Tutorial::Projects;
                events.set(game.roll_events(
                    EventPhase::PlanningStart,
                    None,
                ));
            }

            tracing::debug!("Checking tutorial.");
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
            }
            tracing::debug!("Done calling on done.");
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
