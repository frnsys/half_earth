use crate::views::phases::cards::{CardScanProps, Scannable, ScannerCards, ScannerControls};
use crate::views::{parts::Help, scanner::*};
use crate::{
    consts,
    icons::{self, HasIcon},
    state,
    state::{GameExt, Tutorial},
    state_with, t,
    util::to_ws_el,
    views::cards::CardFocusArea,
    write_state,
};
use hes_engine::projects::{Project, Status, Type};
use leptos::*;
use std::collections::HashMap;

impl Scannable for Project {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn as_card(&self) -> View {
        todo!() // TODO
    }
}

#[component]
pub fn Projects(
    #[prop(into)] on_kind_change: Callback<Type>,
    #[prop(into)] on_change: Callback<()>,
    #[prop(into)] close: Callback<()>,
) -> impl IntoView {
    let (kind, set_kind) = create_signal(Type::Research);
    create_effect(move |_| {
        on_kind_change.call(kind.get());
    });

    let scan_tip = t!("↑ Swipe this card up and hold to add it to your plan ↑");
    let scroll_tip = t!("⟵ Swipe sideways to see other projects ⟶ ");

    let back_disabled = state!(|state, ui| { ui.tutorial < Tutorial::ProjectsBack });
    let back_highlighted = state!(|state, ui| { ui.tutorial == Tutorial::ProjectsBack });

    let PROJECT_LOCKERS: HashMap<usize, usize> = todo!(); // TODO

    let projects = state!(|state, ui| {
        let kind = kind.get();
        state
            .world
            .projects
            .iter()
            .filter(|p| {
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
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let project_order = move || {
        let projects = projects();
        let mut idxs: Vec<_> = projects.iter().enumerate().collect();
        idxs.sort_by(|a, b| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()));
        idxs.into_iter().map(|(i, _)| i).collect::<Vec<usize>>()
    };

    view! {
        <div class="plan-change-select planning--page">
            <Help text=scan_tip x=0.5 y=150. center=true/>
            <Help text=scroll_tip x=0.5 y=250. center=true/>

            <div class="planning--page-tabs">
                <div
                    class="planning-sub-tab"
                    on:click=move |_| set_kind.set(Type::Research)
                    class:selected=move || kind.get() == Type::Research
                >
                    <img src=icons::RESEARCH/>
                    <div>{t!("Research")}</div>
                </div>
                <div
                    class="planning-sub-tab"
                    on:click=move |_| set_kind.set(Type::Initiative)
                    class:selected=move || kind.get() == Type::Initiative
                >
                    <img src=icons::INITIATIVE/>
                    <div>{t!("Infrastructure")}</div>
                </div>
                <div
                    class="planning-sub-tab"
                    on:click=move |_| set_kind.set(Type::Policy)
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

            <ProjectScanner projects=projects.into_signal() on_change/>
        </div>
    }
}

#[component]
pub fn ProjectScanner(
    #[prop(into)] projects: MaybeSignal<Vec<Project>>,
    #[prop(into)] on_change: Callback<()>,
) -> impl IntoView {
    let (project, set_project) =
        create_signal::<Option<Project>>(projects.with(|projects| projects.get(0).cloned()));
    let kind =
        move || project.with(|project| project.as_ref().map(|p| p.kind).unwrap_or(Type::Research));

    let upgrade_queued = state!(move |state, ui| {
        project.with(|p| {
            p.as_ref()
                .is_some_and(|p| ui.queued_upgrades.get(&p.id) == Some(&true))
        })
    });

    let add_scan_allowed = state!(move |state, ui| {
        project.with(|p| {
            if let Some(p) = p {
                let player_seats = state.player_seats() as f32;
                if p.required_majority > 0. && player_seats < p.required_majority {
                    false
                } else if upgrade_queued() {
                    false
                } else if p.next_upgrade().is_some() {
                    true
                } else if p.kind == Type::Policy && p.status == Status::Active {
                    false
                } else {
                    true
                }
            } else {
                false
            }
        })
    });

    let finish_scan_add = move |controls: ScannerControls| {
        let state = expect_context::<RwSignal<crate::state::GameState>>();
        state
            .try_update(|state| {
                let ui = &mut state.ui;
                let state = &mut state.game;
                set_project.try_update(|p| {
                    let mut keep_scanning = false;
                    if let Some(p) = p {
                        let changes = ui.plan_changes.entry(p.id).or_default();

                        // Upgrading projects
                        if p.is_online() && p.next_upgrade().is_some() {
                            let free = changes.downgrades > 0;
                            if free {
                                changes.downgrades -= 1;
                            }
                            if state.upgrade_project_x(p, free, &mut ui.queued_upgrades) {
                                on_change.call(());
                            }
                            pulse_level();
                            if p.next_upgrade().is_some() {
                                keep_scanning = true;
                            }

                            // Refundable upgrade
                            changes.upgrades += 1;

                        // Adding points to Research/Infrastructure
                        } else if p.kind != Type::Policy && state.buy_point(p, &mut ui.points) {
                            if ui.tutorial == Tutorial::Projects {
                                ui.tutorial.advance();
                            }
                            state.assign_point(p, &mut ui.points);
                            on_change.call(());
                            pulse_card();
                            keep_scanning = true;

                            // Refundable points
                            changes.points += 1;

                            // Passing Policies
                            // Free if withdrawn in this same session (i.e. undo the withdraw)
                        } else if p.kind == Type::Policy
                            && (changes.withdrawn || state.pay_points(p))
                        {
                            if ui.tutorial == Tutorial::Projects {
                                ui.tutorial.advance();
                            }
                            state.pass_policy(p);
                            on_change.call(());

                            pulse_card();
                            shake_screen();

                            // Refundable
                            if changes.withdrawn {
                                changes.withdrawn = false;
                            } else {
                                changes.passed = true;
                            }

                            if p.next_upgrade().is_some() {
                                keep_scanning = true;
                            }

                        // If not enough PC
                        } else {
                            (controls.reject_scan)();
                            shake_progress(to_ws_el(controls.progress_elem));
                            keep_scanning = false;
                        }
                    }
                    keep_scanning
                })
            })
            .flatten()
            .unwrap_or(false)
    };

    let add_props = CardScanProps {
        should_show: (|| true).into_signal(),
        scan_allowed: add_scan_allowed.into_signal(),
        scan_time: consts::PROJECT_CARD_SCAN_TIME,
        on_finish_scan: finish_scan_add.into(),
    };

    let refundable = state!(move |state, ui| {
        project.with(|p| {
            p.as_ref().is_some_and(|p| {
                if let Some(changes) = ui.plan_changes.get(&p.id) {
                    changes.upgrades > 0
                        || changes.points > 0
                        || changes.passed
                        || changes.withdrawn
                } else {
                    false
                }
            })
        })
    });
    let can_remove =
        move || project.with(|p| p.as_ref().is_some_and(|p| p.is_haltable())) || refundable();

    let finish_scan_rem = move |controls| {
        let state = expect_context::<RwSignal<crate::state::GameState>>();
        state
            .try_update(|state| {
                let ui = &mut state.ui;
                let state = &mut state.game;
                set_project.try_update(|p| {
                    let mut keep_scanning = false;
                    if let Some(p) = p {
                        shrink_pulse_card();
                        let mut keep_withdrawing = false;
                        let changes = ui.plan_changes.entry(p.id).or_default();
                        if refundable() {
                            if changes.upgrades > 0 {
                                state.downgrade_project_x(p, &mut ui.queued_upgrades);
                                changes.upgrades -= 1;
                                keep_withdrawing = changes.upgrades > 0 || changes.passed;
                            } else if changes.passed {
                                state.stop_policy(p);
                                changes.passed = false;
                            } else {
                                let points = changes.points;
                                let refund = state.next_point_cost(&p.kind) * points;

                                // Don't allow stored research-only points to be converted into PC,
                                // instead convert them back into research points
                                if p.kind == Type::Research {
                                    let excess_points =
                                        points.saturating_sub(ui.points.refundable_research);
                                    let refund =
                                        state.next_point_cost(&p.kind) * (points - excess_points);
                                    ui.points.refundable_research =
                                        ui.points.refundable_research.saturating_sub(points);
                                    ui.points.research += excess_points as isize;
                                }
                                state.unassign_points(p, points);
                                state.change_political_capital(refund as isize);
                                changes.points = 0;
                            }
                        } else if p.can_downgrade() {
                            state.downgrade_project(p.id);
                            keep_withdrawing = p.level > 0;
                            changes.downgrades += 1;
                        } else {
                            state.stop_project(p.id);
                            changes.withdrawn = true;
                        }
                        on_change.call(());

                        if keep_withdrawing {
                            keep_scanning = true
                        } else {
                            keep_scanning = false
                        }
                    } else {
                        keep_scanning = false
                    }
                    keep_scanning
                })
            })
            .flatten()
            .unwrap_or(false)
    };

    let rem_props = CardScanProps {
        should_show: can_remove.into_signal(),
        scan_allowed: can_remove.into_signal(),
        scan_time: consts::PROJECT_CARD_WITHDRAW_TIME,
        on_finish_scan: finish_scan_rem.into(),
    };
    let rem_label = move || {
        project.with(|p| {
            if refundable() {
                t!("Undo")
            } else if p.as_ref().is_some_and(|p| p.can_downgrade()) {
                t!("Downgrade")
            } else {
                t!("Withdraw")
            }
        })
    };

    view! {
        <ScannerCards
            // TODO hacky
            items=move || projects.get()
            remove_label=rem_label
            add_props
            remove_props=rem_props
        />

        <footer>
            <Points kind/>
        </footer>
    }
}

#[component]
fn Points(#[prop(into)] kind: Signal<Type>) -> impl IntoView {
    let pc_points = state!(|state, ui| { state.political_capital });
    let available_points = state_with!(|state, ui, kind| {
        match kind {
            Type::Policy => state.political_capital,
            Type::Initiative => ui.points.initiative,
            Type::Research => ui.points.research,
        }
    });
    let next_point_cost = state_with!(|state, ui, kind| { state.next_point_cost(kind) });
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
