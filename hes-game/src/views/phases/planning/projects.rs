use crate::{
    debug::get_debug_opts,
    icons::{self, HasIcon},
    state,
    state::{GameExt, GameState, Tutorial},
    t,
    ui,
    views::{scanner::*, Help},
    with_state,
};
use hes_engine::projects::{Status, Type};
use leptos::*;

#[component]
pub fn Projects(
    #[prop(into)] on_kind_change: Callback<Type>,
    #[prop(into)] on_change: Callback<()>,
    #[prop(into)] close: Callback<()>,
) -> impl IntoView {
    let (kind, set_kind) = create_signal(Type::Research);
    let set_kind = move |kind: Type| {
        set_kind.set(kind);
        on_kind_change.call(kind);
    };
    on_kind_change.call(kind.get_untracked());

    let scan_tip = t!("↑ Swipe this card up and hold to add it to your plan ↑");
    let scroll_tip =
        t!("⟵ Swipe sideways to see other projects ⟶ ");

    let back_disabled =
        ui!(tutorial.lt(&Tutorial::ProjectsBack));
    let back_highlighted =
        ui!(tutorial.eq(&Tutorial::ProjectsBack));

    let debug = get_debug_opts();
    let state = expect_context::<RwSignal<GameState>>();
    let projects = move || {
        let mut projects = with!(|state, kind| {
            state
                .game
                .world
                .projects
                .iter()
                .filter(|p| {
                    p.kind == *kind && (!p.locked || debug.show_all_projects)

                // Filter out finished projects
                && p.status != Status::Finished

                // Filter out finished policies
                // but only ones added before
                // this planning session
                && (p.status != Status::Active || state.ui.plan_changes.contains_key(&p.id))

                // Filter out projects that are mutually exclusive
                // with active projects
                    && state.game.world.project_lockers.get(&p.id)
                    .map(|locker_id| {
                        // Is the locker satisfied?
                        match state.game.world.projects[locker_id].status {
                            Status::Building | Status::Active | Status::Finished => false,
                            _=> true
                        }
                    }).unwrap_or(true)
                })
                .cloned()
                .collect::<Vec<_>>()
        });
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
                    on:click=move |_| set_kind(Type::Research)
                    class:selected=move || kind.get() == Type::Research
                >
                    <img src=icons::RESEARCH/>
                    <div>{t!("Research")}</div>
                </div>
                <div
                    class="planning-sub-tab"
                    on:click=move |_| set_kind(Type::Initiative)
                    class:selected=move || kind.get() == Type::Initiative
                >
                    <img src=icons::INITIATIVE/>
                    <div>{t!("Infrastructure")}</div>
                </div>
                <div
                    class="planning-sub-tab"
                    on:click=move |_| set_kind(Type::Policy)
                    class:selected=move || kind.get() == Type::Policy
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
fn Points(#[prop(into)] kind: Signal<Type>) -> impl IntoView {
    let pc_points = state!(political_capital);
    let init_points = ui!(points.initiative);
    let research_points = ui!(points.research);
    let available_points = move || match kind.get() {
        Type::Policy => pc_points.get(),
        Type::Initiative => init_points.get(),
        Type::Research => research_points.get(),
    };
    let next_point_cost = state!(next_point_cost(&kind.get()));
    let icon = move || kind.get().icon();

    view! {
        <div class="pips">
            <div class="pips-group">
                {pc_points} <img class="pip" src=icons::POLITICAL_CAPITAL/>
            </div>
            <Show when=move || kind.get() != Type::Policy>
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
