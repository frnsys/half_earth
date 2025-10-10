use std::collections::BTreeMap;

use hes_engine::{Flag, Id, Project, ProjectType, Status};
use rust_i18n::t;

use crate::{
    consts,
    state::{GameState, PlanChange, StateExt, Tutorial},
    views::scanner::ScanResult,
};

use super::Scannable;

fn is_refundable(
    project: &Project,
    plan_changes: &BTreeMap<Id, PlanChange>,
) -> bool {
    if let Some(changes) = plan_changes.get(&project.id) {
        changes.upgrades > 0
            || changes.points > 0
            || changes.passed
            || changes.withdrawn
    } else {
        false
    }
}

fn is_subtractable(
    project: &Project,
    plan_changes: &BTreeMap<Id, PlanChange>,
) -> bool {
    project.is_haltable()
        || is_refundable(project, plan_changes)
}

impl Scannable for Project {
    fn add_label(&self, _state: &GameState) -> Option<String> {
        None
    }

    fn add_scan_time(&self) -> f32 {
        consts::PROJECT_CARD_SCAN_TIME
    }

    fn add_scan_done(
        &self,
        state: &mut GameState,
    ) -> ScanResult {
        let result;
        let changes =
            state.ui.plan_changes.entry(self.id).or_default();

        // Upgrading projects
        if self.is_online() && self.next_upgrade().is_some() {
            let free = changes.downgrades > 0;
            if free {
                changes.downgrades -= 1;
            }
            state.core.upgrade_project_x(
                &self.id,
                free,
                &mut state.ui.queued_upgrades,
            );

            if self.next_upgrade().is_some() {
                result = ScanResult::SuccessContinue;
            } else {
                result = ScanResult::SuccessStop;
            }

            // Refundable upgrade
            changes.upgrades += 1;

        // Adding points to Research/Infrastructure
        } else if self.kind != ProjectType::Policy
            && state
                .core
                .buy_point(&self.id, &mut state.ui.points)
        {
            if state.ui.tutorial == Tutorial::Projects {
                state.ui.tutorial.advance();
            }
            state
                .core
                .assign_point(&self.id, &mut state.ui.points);

            // Refundable points
            changes.points += 1;

            result = ScanResult::SuccessContinue;

            // Passing Policies
            // Free if withdrawn in this same session (i.e. undo the withdraw)
        } else if self.kind == ProjectType::Policy
            && (changes.withdrawn
                || state.core.pay_points(&self.id))
        {
            if state.ui.tutorial == Tutorial::Projects {
                state.ui.tutorial.advance();
            }
            state.core.pass_policy(&self.id);

            // Refundable
            if changes.withdrawn {
                changes.withdrawn = false;
            } else {
                changes.passed = true;
            }

            if self.next_upgrade().is_some() {
                result = ScanResult::SuccessContinue;
            } else {
                result = ScanResult::SuccessStop;
            }

        // If not enough PC
        } else {
            result = ScanResult::Rejected;
        }

        result
    }

    fn is_add_visible(&self, _state: &GameState) -> bool {
        true
    }

    fn is_add_allowed(&self, state: &GameState) -> bool {
        let player_seats = state.npcs.coalition_seats();
        let parliament_suspended =
            state.flags.contains(&Flag::ParliamentSuspended);

        // Does this project already have an upgrade queued/under construction?
        let upgrade_queued =
            state.ui.queued_upgrades.get(&self.id)
                == Some(&true);

        // Points can be added to a project if:
        // - The player has enough of a majority, if required.
        // - No upgrade is queued for the project.
        // - An upgrade exists for the project.
        // - If the project is a policy, only if it's not already implemented.
        if (self.required_majority > 0.
            && player_seats < self.required_majority)
            && !parliament_suspended
        {
            false
        } else if upgrade_queued {
            false
        } else if self.next_upgrade().is_some() {
            true
        } else if self.kind == ProjectType::Policy
            && self.status == Status::Active
        {
            false
        } else if self.status == Status::Finished {
            false
        } else {
            true
        }
    }

    fn rem_label(&self, state: &GameState) -> Option<String> {
        let label =
            if is_refundable(self, &state.ui.plan_changes) {
                t!("Undo")
            } else if self.can_downgrade() {
                t!("Downgrade")
            } else {
                t!("Withdraw")
            };

        Some(label.to_string())
    }

    fn rem_scan_time(&self) -> f32 {
        consts::PROJECT_CARD_WITHDRAW_TIME
    }

    fn rem_scan_done(
        &self,
        state: &mut GameState,
    ) -> ScanResult {
        let mut keep_withdrawing = false;
        let refundable =
            is_refundable(self, &state.ui.plan_changes);
        let changes =
            state.ui.plan_changes.entry(self.id).or_default();
        if refundable {
            if changes.upgrades > 0 {
                state.core.downgrade_project_x(
                    &self.id,
                    &mut state.ui.queued_upgrades,
                );
                changes.upgrades -= 1;
                keep_withdrawing =
                    changes.upgrades > 0 || changes.passed;
            } else if changes.passed {
                state.core.stop_policy(&self.id);
                changes.passed = false;
            } else {
                let points = changes.points;
                let mut refund =
                    state.core.next_point_cost(&self.kind)
                        * points;

                // Don't allow stored research-only points to be converted into PC,
                // instead convert them back into research points
                if self.kind == ProjectType::Research {
                    let excess_points = points.saturating_sub(
                        state.ui.points.refundable_research,
                    );
                    refund =
                        state.core.next_point_cost(&self.kind)
                            * (points - excess_points);
                    state.ui.points.refundable_research = state
                        .ui
                        .points
                        .refundable_research
                        .saturating_sub(points);
                    state.ui.points.research +=
                        excess_points as isize;
                }
                state.core.unassign_points(&self.id, points);
                state
                    .core
                    .change_political_capital(refund as isize);
                changes.points = 0;
            }
        } else if self.can_downgrade() {
            state.core.downgrade_project(&self.id);
            keep_withdrawing = self.level > 0;
            changes.downgrades += 1;
        } else {
            state.core.stop_project(&self.id);
            changes.withdrawn = true;
        }

        if keep_withdrawing {
            ScanResult::SuccessContinue
        } else {
            ScanResult::SuccessStop
        }
    }

    fn is_rem_visible(&self, state: &GameState) -> bool {
        self.is_rem_allowed(state)
    }

    fn is_rem_allowed(&self, state: &GameState) -> bool {
        is_subtractable(self, &state.ui.plan_changes)
    }
}
