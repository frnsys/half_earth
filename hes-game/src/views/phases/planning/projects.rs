use crate::{state, state_with, state::{Tutorial, GameExt}, views::cards::CardFocusArea, icons::{self, HasIcon}, consts};
use hes_engine::projects::{Status, Type};
use std::collections::HashMap;
use leptos::*;
use crate::views::phases::cards::{ScannerCards, CardScanProps};

#[component]
pub fn Projects(#[prop(into)] on_kind_change: Callback<Type>) -> impl IntoView {
    let (kind, set_kind) = create_signal(Type::Research);
    create_effect(move |_| {
        on_kind_change.call(kind.get());
    });

    let back_disabled = state!(|state, ui| { ui.tutorial < Tutorial::ProjectsBack });
    let back_highlighted = state!(|state, ui| { ui.tutorial == Tutorial::ProjectsBack });

    let PROJECT_LOCKERS: HashMap<usize, usize> = todo!(); // TODgO
    // import PROJECT_LOCKERS from 'content/project_lockers.json';

    let projects = state!(|state, ui| {
        let kind = kind.get();
        state.world.projects.iter().filter(|p| {
            // TODO (!p.locked || debug.show_all_projects)
            p.kind == kind && !p.locked

            // Filter out finished projects
            && p.status != Status::Finished

            // Filter out finished policies
            // but only ones added before
            // this planning session
            && (p.status != Status::Active || ui.plan_changes.contains_key(&p.id))

            // Filter out projects that are mutually exclusive
            // with active projects
                && PROJECT_LOCKERS.get(&p.id)
                .map(|locker_id| {
                    // Is the locker satisfied?
                    match state.world.projects[*locker_id].status {
                        Status::Building | Status::Active | Status::Finished => false,
                        _=> true
                    }
                }).unwrap_or(true)
        }).cloned().collect::<Vec<_>>()
    });

    let project_order = move || {
        let projects = projects();
        let mut idxs: Vec<_> = projects.iter().enumerate().collect();
        idxs.sort_by(|a, b| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()));
        idxs.into_iter().map(|(i, _)| i).collect::<Vec<usize>>()
    };

    let add_props = CardScanProps {
        should_show: (|| true).into_signal(),
        scan_time: consts::PROJECT_CARD_SCAN_TIME,
    };

    view! {
        <ScannerCards />
    }
}

#[component]
fn Points(kind: Signal<Type>) -> impl IntoView {
    let pc_points = state!(|state, ui| {
        state.political_capital
    });
    let available_points = state_with!(|state, ui, kind| {
        match kind {
            Type::Policy => state.political_capital,
            Type::Initiative => ui.initiative_points,
            Type::Research => ui.research_points,
        }
    });
    let next_point_cost = state_with!(|state, ui, kind| {
        state.next_point_cost(kind)
    });
    let icon = move || kind.get().icon();

    view! {
        <div class="pips">
          <div class="pips-group">
            {pc_points}<img class="pip" src=icons::POLITICAL_CAPITAL />
          </div>
        <Show when=move || kind.get() != Type::Policy>
            <div class="pips-group">
                <Show when=move || { available_points() > 0 }
                    fallback=move || {
                        view! {
                            {next_point_cost}<img class="pip" src=icons::POLITICAL_CAPITAL /> <img src=icons::ARROW_RIGHT class="pip-arrow"/> <img class="pip" src=icon />
                        }
                    }>
                  {available_points}<img class="pip" src=icon />
                </Show>
            </div>
        </Show>
    </div>

    }
}
