use std::collections::BTreeMap;

use crate::{
    consts,
    state,
    state::{GameExt, PlanChange, Tutorial},
    t,
    ui,
    views::cards::ProjectCard,
};
use hes_engine::{
    projects::{Project, Status, Type},
    state::State,
    Id,
};
use leptos::*;

use super::{
    CardScanProps,
    Scannable,
    ScannerControls,
    ScannerSpec,
};

impl Scannable for Project {
    fn id(&self) -> &Id {
        &self.id
    }

    fn get_from_state(id: &Id, state: &State) -> Self {
        state.world.projects[id].clone()
    }

    fn as_card(project: Signal<Self>) -> View {
        view! { <ProjectCard project/> }.into_view()
    }
}

pub struct ProjectScanner {
    on_change: Callback<()>,
    player_seats: Memo<f32>,
    plan_changes: Memo<BTreeMap<Id, PlanChange>>,
    queued_upgrades: Memo<BTreeMap<Id, bool>>,
}
impl ProjectScanner {
    pub fn new(on_change: Callback<()>) -> Self {
        Self {
            on_change,
            plan_changes: ui!(plan_changes.clone()),
            queued_upgrades: ui!(queued_upgrades.clone()),
            player_seats: state!(player_seats()),
        }
    }
}

impl ScannerSpec for ProjectScanner {
    type Item = Project;

    fn add_props(
        &self,
        project: RwSignal<Option<Self::Item>>,
    ) -> CardScanProps {
        let on_change = self.on_change.clone();
        let player_seats = self.player_seats.clone();
        let queued_upgrades = self.queued_upgrades.clone();

        // Does this project already have an upgrade queued/under construction?
        let upgrade_queued = move || {
            project.with(|project| {
                project.as_ref().is_some_and(|p| {
                    queued_upgrades.get().get(&p.id)
                        == Some(&true)
                })
            })
        };

        // Points can be added to a project if:
        // - The player has enough of a majority, if required.
        // - No upgrade is queued for the project.
        // - An upgrade exists for the project.
        // - If the project is a policy, only if it's not already implemented.
        let addable = move || {
            project.with(|p| {
                if let Some(p) = p {
                    let player_seats =
                        player_seats.get() as f32;
                    if p.required_majority > 0.
                        && player_seats < p.required_majority
                    {
                        false
                    } else if upgrade_queued() {
                        false
                    } else if p.next_upgrade().is_some() {
                        true
                    } else if p.kind == Type::Policy
                        && p.status == Status::Active
                    {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
        };

        let state = expect_context::<
            RwSignal<crate::state::GameState>,
        >();
        let on_finish_scan =
            move |controls: ScannerControls| {
                state
                    .try_update(|state| {
                        let ui = &mut state.ui;
                        let state = &mut state.game;
                        let mut keep_scanning = false;
                        project.with(|project| {
                            if let Some(p) = project {
                                let changes = ui
                                    .plan_changes
                                    .entry(p.id)
                                    .or_default();

                                // Upgrading projects
                                if p.is_online()
                                    && p.next_upgrade()
                                        .is_some()
                                {
                                    let free =
                                        changes.downgrades > 0;
                                    if free {
                                        changes.downgrades -= 1;
                                    }
                                    if state.upgrade_project_x(
                                        &p.id,
                                        free,
                                        &mut ui.queued_upgrades,
                                    ) {
                                        on_change.call(());
                                    }
                                    controls.pulse_level();
                                    if p.next_upgrade()
                                        .is_some()
                                    {
                                        keep_scanning = true;
                                    }

                                    // Refundable upgrade
                                    changes.upgrades += 1;

                                    // Adding points to Research/Infrastructure
                                } else if p.kind != Type::Policy
                                    && state.buy_point(
                                        &p.id,
                                        &mut ui.points,
                                    )
                                {
                                    if ui.tutorial
                                        == Tutorial::Projects
                                    {
                                        ui.tutorial.advance();
                                    }
                                    state.assign_point(
                                        &p.id,
                                        &mut ui.points,
                                    );

                                    on_change.call(());

                                    controls.pulse_card();

                                    keep_scanning = true;

                                    // Refundable points
                                    changes.points += 1;

                                    // Passing Policies
                                    // Free if withdrawn in this same session (i.e. undo the withdraw)
                                } else if p.kind == Type::Policy
                                    && (changes.withdrawn
                                        || state
                                            .pay_points(&p.id))
                                {
                                    if ui.tutorial
                                        == Tutorial::Projects
                                    {
                                        ui.tutorial.advance();
                                    }
                                    state.pass_policy(&p.id);
                                    on_change.call(());

                                    controls.pulse_card();
                                    controls.shake_screen();

                                    // Refundable
                                    if changes.withdrawn {
                                        changes.withdrawn =
                                            false;
                                    } else {
                                        changes.passed = true;
                                    }

                                    if p.next_upgrade()
                                        .is_some()
                                    {
                                        keep_scanning = true;
                                    }

                                    // If not enough PC
                                } else {
                                    controls.reject_scan();
                                    keep_scanning = false;
                                }
                            }
                            keep_scanning
                        })
                    })
                    .unwrap_or(false)
            };

        CardScanProps {
            label: None,
            should_show: (|| true).into_signal(),
            scan_allowed: addable.into_signal(),
            scan_time: consts::PROJECT_CARD_SCAN_TIME,
            on_finish_scan: on_finish_scan.into(),
        }
    }

    fn rem_props(
        &self,
        project: RwSignal<Option<Self::Item>>,
    ) -> CardScanProps {
        let on_change = self.on_change.clone();
        let plan_changes = self.plan_changes.clone();

        let refundable = move || {
            project.with(|p| {
                p.as_ref().is_some_and(|p| {
                    if let Some(changes) =
                        plan_changes.get().get(&p.id)
                    {
                        changes.upgrades > 0
                            || changes.points > 0
                            || changes.passed
                            || changes.withdrawn
                    } else {
                        false
                    }
                })
            })
        };

        let subtractable = move || {
            project.with(|p| {
                p.as_ref().is_some_and(|p| p.is_haltable())
            }) || refundable()
        };

        let state = expect_context::<
            RwSignal<crate::state::GameState>,
        >();
        let on_finish_scan =
            move |controls: ScannerControls| {
                state
                .try_update(|state| {
                    let ui = &mut state.ui;
                    let state = &mut state.game;
                    let mut keep_scanning = false;
                    project.with(|project| {
                        if let Some(p) = project {
                            controls.shrink_pulse_card();
                            let mut keep_withdrawing = false;
                            let changes = ui.plan_changes.entry(p.id).or_default();
                            if refundable() {
                                if changes.upgrades > 0 {
                                    state.downgrade_project_x(&p.id, &mut ui.queued_upgrades);
                                    changes.upgrades -= 1;
                                    keep_withdrawing = changes.upgrades > 0 || changes.passed;
                                } else if changes.passed {
                                    state.stop_policy(&p.id);
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
                                            state.next_point_cost(&p.kind) * (points - excess_points); // TODO why is this unused
                                        ui.points.refundable_research =
                                            ui.points.refundable_research.saturating_sub(points);
                                        ui.points.research += excess_points as isize;
                                    }
                                    state.unassign_points(&p.id, points);
                                    state.change_political_capital(refund as isize);
                                    changes.points = 0;
                                }
                            } else if p.can_downgrade() {
                                state.downgrade_project(&p.id);
                                keep_withdrawing = p.level > 0;
                                changes.downgrades += 1;
                            } else {
                                state.stop_project(&p.id);
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
                .unwrap_or(false)
            };

        let label = move || {
            project.with(|p| {
                if refundable() {
                    t!("Undo")
                } else if p
                    .as_ref()
                    .is_some_and(|p| p.can_downgrade())
                {
                    t!("Downgrade")
                } else {
                    t!("Withdraw")
                }
            })
        };

        CardScanProps {
            label: Some(label.into_signal()),
            should_show: subtractable.into_signal(),
            scan_allowed: subtractable.into_signal(),
            scan_time: consts::PROJECT_CARD_WITHDRAW_TIME,
            on_finish_scan: on_finish_scan.into(),
        }
    }
}
