use std::collections::BTreeMap;

use crate::{
    consts,
    state::{Tutorial, UIState},
    t,
    views::cards::ProcessCard,
};
use enum_map::EnumMap;
use hes_engine::{Id, Output, Process, State};
use leptos::*;

use super::{
    CardScanProps,
    Scannable,
    ScannerControls,
    ScannerSpec,
};

impl Scannable for Process {
    fn id(&self) -> &Id {
        &self.id
    }

    fn get_from_state(id: &Id, state: &State) -> Self {
        state.world.processes[id].clone()
    }

    fn as_card(process: Signal<Self>) -> View {
        view! { <ProcessCard process/> }.into_view()
    }
}

pub struct ProcessScanner {
    pub points: RwSignal<isize>,
    pub on_change: Callback<()>,
    pub mix_changes: Memo<EnumMap<Output, BTreeMap<Id, isize>>>,
}

impl ScannerSpec for ProcessScanner {
    type Item = Process;

    fn add_props(
        &self,
        process: RwSignal<Option<Self::Item>>,
    ) -> CardScanProps {
        let points = self.points.clone();
        let on_change = self.on_change.clone();

        let game = expect_context::<RwSignal<State>>();
        let ui = expect_context::<RwSignal<UIState>>();

        let process_max_share = create_memo(move |_| {
            with!(|game, process| process
                .as_ref()
                .map(|process| game
                    .process_max_share(&process.id))
                .unwrap_or(0))
        });

        let addable = move || {
            ui.with_untracked(|ui| {
                if let Some(process) = process.get_untracked() {
                    let max_share =
                        process_max_share.get_untracked();
                    let change = ui.process_mix_changes
                        [process.output]
                        .get(&process.id)
                        .unwrap_or(&0);
                    points.get_untracked() != 0
                        && (*change + 1) < max_share as isize
                } else {
                    false
                }
            })
        };

        let should_advance = create_memo(move |_| {
            with!(|ui| ui.tutorial == Tutorial::Processes)
        });
        let on_finish_scan =
            move |controls: ScannerControls| {
                if addable() {
                    if should_advance.get_untracked() {
                        ui.update(|ui| {
                            ui.tutorial.advance();
                        });
                    }

                    let mut available_points =
                        points.get_untracked();
                    if let Some(process) =
                        process.get_untracked()
                    {
                        update!(|ui| {
                            let max_share = process_max_share
                                .get_untracked();
                            ui.add_point(
                                &mut available_points,
                                &process,
                                max_share,
                            );
                        });

                        // Consider the process mix 'changed'
                        // when all points have been assigned
                        if available_points == 0 {
                            on_change.call(());
                        }

                        points.set(available_points);
                    }
                    controls.pulse_card();
                    true
                } else {
                    controls.reject_scan();
                    false
                }
            };

        CardScanProps {
            label: None,
            should_show: addable.into_signal(),
            scan_allowed: addable.into_signal(),
            scan_time: consts::PROCESS_CARD_SCAN_TIME,
            on_finish_scan: on_finish_scan.into(),
        }
    }

    fn rem_props(
        &self,
        process: RwSignal<Option<Self::Item>>,
    ) -> CardScanProps {
        let points = self.points.clone();
        let mix_changes = self.mix_changes.clone();

        let ui = expect_context::<RwSignal<UIState>>();

        let subtractable = move || {
            if let Some(process) = process.get_untracked() {
                let changes = mix_changes.get_untracked();
                let change = changes[process.output]
                    .get(&process.id)
                    .unwrap_or(&0);
                process.mix_share as isize + *change != 0
            } else {
                false
            }
        };

        let on_finish_scan =
            move |_controls: ScannerControls| {
                let mut available_points =
                    points.get_untracked();
                if let Some(process) = process.get_untracked() {
                    ui.update(|ui| {
                        ui.remove_point(
                            &mut available_points,
                            &process,
                        );
                    });
                    points.set(available_points);
                }

                // If still subtractable, continue scanning
                subtractable()
            };

        CardScanProps {
            label: Some(
                (move || t!("Remove points")).into_signal(),
            ),
            should_show: subtractable.into_signal(),
            scan_allowed: subtractable.into_signal(),
            scan_time: consts::PROCESS_CARD_WITHDRAW_TIME,
            on_finish_scan: on_finish_scan.into(),
        }
    }
}
