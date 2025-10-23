use std::collections::BTreeMap;

use enum_map::EnumMap;
use hes_engine::{Id, Output, Process, State};

use crate::{
    consts,
    state::{GameState, Tutorial},
    views::scanner::ScanResult,
};

use super::Scannable;

fn is_subtractable(process: &Process, mix_changes: &EnumMap<Output, BTreeMap<Id, isize>>) -> bool {
    let change = mix_changes[process.output].get(&process.id).unwrap_or(&0);
    process.mix_share as isize + *change != 0
}

fn is_addable(
    process: &Process,
    state: &State,
    process_mix_changes: &EnumMap<Output, BTreeMap<Id, isize>>,
    points: isize,
) -> bool {
    let max_share = state.process_max_share(&process.id);
    let change = process_mix_changes[process.output]
        .get(&process.id)
        .unwrap_or(&0);
    points != 0 && *change < max_share as isize
}

fn remove_point(
    points: &mut isize,
    process: &Process,
    mix_changes: &mut EnumMap<Output, BTreeMap<Id, isize>>,
) {
    let change = mix_changes[process.output].entry(process.id).or_default();
    if process.mix_share as isize + *change > 0 {
        *points += 1;
        *change -= 1;
    }
}

// Returns the point change.
fn add_point(
    points: &mut isize,
    process: &Process,
    max_share: usize,
    mix_changes: &mut EnumMap<Output, BTreeMap<Id, isize>>,
) {
    if *points > 0 {
        let change = mix_changes[process.output].entry(process.id).or_default();
        if *change < max_share as isize {
            *points -= 1;
            *change += 1;
        }
    }
}

impl Scannable for Process {
    fn add_scan_time(&self) -> f32 {
        consts::PROCESS_CARD_SCAN_TIME
    }

    fn add_scan_done(&self, state: &mut GameState) -> ScanResult {
        let addable = is_addable(
            self,
            &state.core,
            &state.ui.process_mix_changes,
            state.ui.process_points,
        );
        if addable {
            if state.ui.tutorial == Tutorial::Processes {
                state.ui.tutorial.advance();
            }

            let max_share = state.core.process_max_share(&self.id);

            add_point(
                &mut state.ui.process_points,
                self,
                max_share,
                &mut state.ui.process_mix_changes,
            );

            if state.ui.process_points > 0 {
                ScanResult::SuccessContinue
            } else {
                ScanResult::SuccessStop
            }
        } else {
            ScanResult::Rejected
        }
    }

    fn is_add_allowed(&self, state: &GameState) -> bool {
        is_addable(
            self,
            &state.core,
            &state.ui.process_mix_changes,
            state.ui.process_points,
        )
    }

    fn rem_scan_time(&self) -> f32 {
        consts::PROCESS_CARD_WITHDRAW_TIME
    }

    fn rem_scan_done(&self, state: &mut GameState) -> ScanResult {
        remove_point(
            &mut state.ui.process_points,
            self,
            &mut state.ui.process_mix_changes,
        );

        // If still subtractable, continue scanning
        if is_subtractable(self, &state.ui.process_mix_changes) {
            ScanResult::SuccessContinue
        } else {
            ScanResult::SuccessStop
        }
    }

    fn is_rem_allowed(&self, state: &GameState) -> bool {
        is_subtractable(self, &state.ui.process_mix_changes)
    }
}
