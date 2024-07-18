mod active_plan;
mod processes;
mod projects;
mod region_item;
mod tabs;

pub use active_plan::ActivePlan;
use hes_engine::events::Phase as EventPhase;
pub use processes::Processes;
pub use projects::Projects;
use tabs::{Dashboard, Parliament, Plan, Regions};

use crate::{
    audio,
    state::Tutorial,
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
    // TODO
    // game.updateProduction();
    // game.updateFactors();

    audio::play_phase_music("/assets/music/planning.mp3", true);

    let (events, set_events) = create_signal(vec![]);
    create_effect(move |_| {
        let state = expect_context::<
            RwSignal<crate::state::GameState>,
        >();
        state.update(|state| {
            let mut events = state.game.roll_events_for_phase(
                EventPhase::PlanningStart,
                None,
            );
            events.extend(state.game.roll_events_for_phase(
                EventPhase::PlanningPlan,
                None,
            ));
            set_events.set(events);
        });
    });

    let has_changes = ui!(has_any_process_mix_changes());

    let (page, set_page) = create_signal(Page::Plan);
    let select_page = move |page: Page| {
        set_page.set(page);

        // TODO
        // set_events.set(game.roll.planning(this.page));
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

    // TODO
    let page_view = move || {
        match page.get() {
            Page::Plan => view! { <Plan/> },
            Page::Parliament => view! { <Parliament/> },
            Page::Dashboard => view! { <Dashboard/> },
            Page::Regions => view! { <Regions/> },
        }

        // let on_plan_change = || {
        //     // TODO
        //     // this.events = game.roll.planning('PlanChange');
        // };
        // let on_plan_subpage = || {
        //     // TODO
        //     // this.events = game.roll.planning(p);
        // };
        // TODO
        // <Plan v-if="page == PAGES.PLAN" @page="pageEvents" @change="planChangeEvents" />
        // <Parliament v-else-if="page == PAGES.PARLIAMENT" />
        // <Dashboard v-else-if="page == PAGES.DASHBOARD" />
        // <Regions v-else-if="page == PAGES.REGIONS" />
    };

    // TODO may no longer be necessary?
    // let on_done = write_state!(|state, ui| {
    //   if (state.gameState.flags.includes('SkipTutorial')) {
    //     state.tutorial = tutorial.READY + 1;
    //   } else if (state.gameState.flags.includes('RepeatTutorial') && !state.tutorialRestarted) {
    //     state.tutorialRestarted = true;
    //     state.tutorial = 0;
    //     // Re-roll for tutorial start
    //     this.events = game.roll.planning('Start');
    //     this.showEvent();
    //   }
    // });

    view! {
        <Hud/>
        <Events
            events
            on_advance=|_| {}
            on_done=move |_| { set_events.set(vec![]) }
        />
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
