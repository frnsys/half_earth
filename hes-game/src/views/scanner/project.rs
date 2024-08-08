use std::collections::BTreeMap;

use crate::{
    consts,
    memo,
    state::{PlanChange, StateExt, Tutorial, UIState},
    t,
    views::cards::ProjectCard,
};
use hes_engine::{Id, Project, ProjectType, State, Status};
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
        let ui = expect_context::<RwSignal<UIState>>();
        let game = expect_context::<RwSignal<State>>();
        Self {
            on_change,
            plan_changes: memo!(ui.plan_changes.clone()),
            queued_upgrades: memo!(ui.queued_upgrades.clone()),
            player_seats: memo!(game.npcs.coalition_seats()),
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
            project.with_untracked(|project| {
                project.as_ref().is_some_and(|p| {
                    queued_upgrades.get_untracked().get(&p.id)
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
            project.with_untracked(|p| {
                if let Some(p) = p {
                    let player_seats =
                        player_seats.get_untracked() as f32;
                    if p.required_majority > 0.
                        && player_seats < p.required_majority
                    {
                        false
                    } else if upgrade_queued() {
                        false
                    } else if p.next_upgrade().is_some() {
                        true
                    } else if p.kind == ProjectType::Policy
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

        let game = expect_context::<RwSignal<State>>();
        let ui = expect_context::<RwSignal<UIState>>();

        let on_finish_scan =
            move |controls: ScannerControls| {
                let mut changed = false;
                let keep_scanning = game
                    .try_update(|game| {
                        ui.try_update(|ui| {
                            let mut keep_scanning = false;
                            project.with_untracked(|project| {
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
                                        if game.upgrade_project_x(
                                            &p.id,
                                            free,
                                            &mut ui.queued_upgrades,
                                        ) {
                                            changed = true;
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
                                    } else if p.kind != ProjectType::Policy
                                        && game.buy_point(
                                            &p.id,
                                            &mut ui.points,
                                        )
                                    {
                                        if ui.tutorial
                                            == Tutorial::Projects
                                        {
                                            ui.tutorial.advance();
                                        }
                                        game.assign_point(
                                            &p.id,
                                            &mut ui.points,
                                        );

                                        changed = true;

                                        controls.pulse_card();

                                        keep_scanning = true;

                                        // Refundable points
                                        changes.points += 1;

                                        // Passing Policies
                                        // Free if withdrawn in this same session (i.e. undo the withdraw)
                                    } else if p.kind == ProjectType::Policy
                                        && (changes.withdrawn
                                            || game
                                            .pay_points(&p.id))
                                    {
                                        if ui.tutorial
                                            == Tutorial::Projects
                                        {
                                            ui.tutorial.advance();
                                        }
                                        game.pass_policy(&p.id);
                                        changed = true;

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
                    })
                .unwrap_or(false);
                if changed {
                    on_change.call(());
                }
                keep_scanning
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
            project.with_untracked(|p| {
                p.as_ref().is_some_and(|p| {
                    if let Some(changes) =
                        plan_changes.get_untracked().get(&p.id)
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
            project.with_untracked(|p| {
                p.as_ref().is_some_and(|p| p.is_haltable())
            }) || refundable()
        };

        let game = expect_context::<RwSignal<State>>();
        let ui = expect_context::<RwSignal<UIState>>();

        let on_finish_scan =
            move |controls: ScannerControls| {
                let mut changed = false;
                let keep_scanning = game
                    .try_update(|game| {
                        ui.try_update(|ui| {
                            let mut keep_scanning = false;
                            project.with_untracked(|project| {
                                if let Some(p) = project {
                                    controls.shrink_pulse_card();
                                    let mut keep_withdrawing = false;
                                    let changes = ui.plan_changes.entry(p.id).or_default();
                                    if refundable() {
                                        if changes.upgrades > 0 {
                                            game.downgrade_project_x(&p.id, &mut ui.queued_upgrades);
                                            changes.upgrades -= 1;
                                            keep_withdrawing = changes.upgrades > 0 || changes.passed;
                                        } else if changes.passed {
                                            game.stop_policy(&p.id);
                                            changes.passed = false;
                                        } else {
                                            let points = changes.points;
                                            let mut refund = game.next_point_cost(&p.kind) * points;

                                            // Don't allow stored research-only points to be converted into PC,
                                            // instead convert them back into research points
                                            if p.kind == ProjectType::Research {
                                                let excess_points =
                                                    points.saturating_sub(ui.points.refundable_research);
                                                refund =
                                                    game.next_point_cost(&p.kind) * (points - excess_points);
                                                ui.points.refundable_research =
                                                    ui.points.refundable_research.saturating_sub(points);
                                                ui.points.research += excess_points as isize;
                                            }
                                            game.unassign_points(&p.id, points);
                                            game.change_political_capital(refund as isize);
                                            changes.points = 0;
                                        }
                                    } else if p.can_downgrade() {
                                        game.downgrade_project(&p.id);
                                        keep_withdrawing = p.level > 0;
                                        changes.downgrades += 1;
                                    } else {
                                        game.stop_project(&p.id);
                                        changes.withdrawn = true;
                                    }
                                    changed = true;

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
                        }).unwrap_or(false)
                    })
                .unwrap_or(false);
                if changed {
                    on_change.call(());
                }
                keep_scanning
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
