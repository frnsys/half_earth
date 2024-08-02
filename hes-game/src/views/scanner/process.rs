use std::collections::BTreeMap;

use crate::{
    consts,
    state::Tutorial,
    t,
    views::cards::ProcessCard,
};
use enum_map::EnumMap;
use hes_engine::{
    kinds::Output,
    production::Process,
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
        let state = expect_context::<
            RwSignal<crate::state::GameState>,
        >();

        let addable = move || {
            state.with_untracked(
                |crate::state::GameState {
                     game: state,
                     ui,
                 }| {
                    if let Some(process) =
                        process.get_untracked()
                    {
                        let max_share =
                            state.process_max_share(&process);
                        let change = ui.process_mix_changes
                            [process.output]
                            .get(&process.id)
                            .unwrap_or(&0);
                        points.get() != 0
                            && (*change + 1)
                                < max_share as isize
                    } else {
                        false
                    }
                },
            )
        };

        let on_finish_scan =
            move |controls: ScannerControls| {
                if addable() {
                    state.update(|state| {
                        let ui = &mut state.ui;
                        if ui.tutorial == Tutorial::Processes {
                            ui.tutorial.advance();
                        }
                    });

                    let mut available_points = points.get();
                    if let Some(process) = process.get() {
                        state.update(|state| {
                            let max_share = state
                                .process_max_share(&process);
                            state.ui.add_point(
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

        let subtractable = move || {
            if let Some(process) = process.get() {
                let changes = mix_changes.get();
                let change = changes[process.output]
                    .get(&process.id)
                    .unwrap_or(&0);
                process.mix_share as isize + *change != 0
            } else {
                false
            }
        };

        let state = expect_context::<
            RwSignal<crate::state::GameState>,
        >();
        let on_finish_scan =
            move |_controls: ScannerControls| {
                let mut available_points = points.get();
                if let Some(process) = process.get() {
                    state.update(|state| {
                        state.ui.remove_point(
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
