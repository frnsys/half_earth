use crate::{
    debug::get_debug_opts,
    icons::{self, HasIcon},
    memo,
    state::{StateExt, Tutorial, UIState},
    t,
    views::{scanner::*, Help},
};
use hes_engine::{ProjectType, State, Status};
use leptos::*;

#[component]
pub fn Projects(
    #[prop(into)] on_kind_change: Callback<ProjectType>,
    #[prop(into)] on_change: Callback<()>,
    #[prop(into)] close: Callback<()>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();
    let (kind, set_kind) = create_signal(ProjectType::Research);
    let set_kind = move |kind: ProjectType| {
        set_kind.set(kind);
        on_kind_change.call(kind);
    };
    on_kind_change.call(kind.get_untracked());

    let scan_tip = t!("↑ Swipe this card up and hold to add it to your plan ↑");
    let scroll_tip = format!(
        "⟵ {}⟶ ",
        t!("Swipe sideways to see other projects")
    );

    let back_disabled =
        memo!(ui.tutorial.lt(&Tutorial::ProjectsBack));
    let back_highlighted =
        memo!(ui.tutorial.eq(&Tutorial::ProjectsBack));

    let debug = get_debug_opts();
    let projects = memo!(game.world.projects);
    let project_lockers = memo!(game.world.project_lockers);
    let plan_changes = memo!(ui.plan_changes);
    let projects = move || {
        let mut projects = with!(
            |projects, project_lockers, plan_changes, kind| {
                projects
                .iter()
                .filter(|p| {
                    p.kind == *kind && (!p.locked || debug.show_all_projects)

                // Filter out finished projects,
                // but show them if they have upgrades
                && (p.status != Status::Finished || !p.upgrades.is_empty())

                // Filter out finished policies
                // but only ones added before
                // this planning session
                && (p.status != Status::Active || plan_changes.contains_key(&p.id) || !p.upgrades.is_empty())

                // Filter out projects that are mutually exclusive
                // with active projects
                    && project_lockers.get(&p.id)
                    .map(|locker_id| {
                        // Is the locker satisfied?
                        match projects[locker_id].status {
                            Status::Building | Status::Active | Status::Finished => false,
                            _=> true
                        }
                    }).unwrap_or(true)
                })
                .cloned()
                .collect::<Vec<_>>()
            }
        );
        projects.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });
        projects
    };

    let scanner = ProjectScanner::new(on_change);

    view! {
        <div class="plan-change-select planning--page">
            <Help text=scan_tip x=0.5 y=150. center=true/>
            <Help text=scroll_tip x=0.5 y=250. center=true/>

            <div class="planning--page-tabs">
                <div
                    class="planning-sub-tab"
                    on:click=move |_| set_kind(ProjectType::Research)
                    class:selected=move || kind.get() == ProjectType::Research
                >
                    <img src=icons::RESEARCH/>
                    <div>{t!("Research")}</div>
                </div>
                <div
                    class="planning-sub-tab"
                    on:click=move |_| set_kind(ProjectType::Initiative)
                    class:selected=move || kind.get() == ProjectType::Initiative
                >
                    <img src=icons::INITIATIVE/>
                    <div>{t!("Infrastructure")}</div>
                </div>
                <div
                    class="planning-sub-tab"
                    on:click=move |_| set_kind(ProjectType::Policy)
                    class:selected=move || kind.get() == ProjectType::Policy
                >
                    <img src=icons::POLICY/>
                    <div>{t!("Policies")}</div>
                </div>
                <div
                    on:click=move |_| close.call(())
                    class:disabled=back_disabled
                    class:highlight=back_highlighted
                >
                    {t!("Back")}
                </div>
            </div>

            <ScannerCards
                spec=scanner
                items=projects
            />

            <footer>
                <Points kind/>
            </footer>
        </div>
    }
}

#[component]
fn Points(
    #[prop(into)] kind: Signal<ProjectType>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();
    let pc_points = memo!(game.political_capital);
    let init_points = memo!(ui.points.initiative);
    let research_points = memo!(ui.points.research);
    let available_points = move || match kind.get() {
        ProjectType::Policy => pc_points.get(),
        ProjectType::Initiative => init_points.get(),
        ProjectType::Research => research_points.get(),
    };
    let next_point_cost =
        memo!(game.next_point_cost(&kind.get()));
    let icon = move || kind.get().icon();

    view! {
        <div class="pips">
            <div class="pips-group">
                {pc_points} <img class="pip" src=icons::POLITICAL_CAPITAL/>
            </div>
            <Show when=move || kind.get() != ProjectType::Policy>
                <div class="pips-group">
                    <Show
                        when=move || { available_points() > 0 }
                        fallback=move || {
                            view! {
                                {next_point_cost}
                                <img class="pip" src=icons::POLITICAL_CAPITAL/>
                                <img src=icons::ARROW_RIGHT class="pip-arrow"/>
                                <img class="pip" src=icon/>
                            }
                        }
                    >

                        {available_points}
                        <img class="pip" src=icon/>
                    </Show>
                </div>
            </Show>
        </div>
    }
}
