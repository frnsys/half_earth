mod cards;
mod draggable;
mod effects;
mod process;
mod project;

use hes_engine::{state::State, Id};
use leptos::*;
use leptos_use::use_resize_observer;
use std::{rc::Rc, time::Duration};
use wasm_bindgen::prelude::*;
use web_sys::Animation;

use crate::util::to_ws_el;
use draggable::{DragRect, Draggable};

pub use cards::ScannerCards;
pub use process::ProcessScanner;
pub use project::ProjectScanner;

#[derive(Clone)]
pub struct ScannerControls {
    reject_scan: Rc<dyn Fn() + 'static>,
    pub progress_elem: HtmlElement<html::Div>,
}
impl ScannerControls {
    pub fn reject_scan(&self) {
        (self.reject_scan)();
        effects::shake_progress(to_ws_el(
            self.progress_elem.clone(),
        ));
    }

    pub fn pulse_card(&self) {
        effects::pulse_card();
    }

    pub fn pulse_level(&self) {
        effects::pulse_level();
    }

    pub fn shrink_pulse_card(&self) {
        effects::shrink_pulse_card();
    }

    pub fn shake_screen(&self) {
        effects::shake_screen();
    }
}

#[component(transparent)]
pub fn Scanner(
    children: Children,
    scan_time: f32,
    reveal_target: f32,
    #[prop(into)] should_show: Signal<bool>,
    #[prop(into)] scan_allowed: Signal<bool>,
    #[prop(into)] drag_rect: Signal<Option<DragRect>>,
    #[prop(into)] on_finish_scan: Callback<
        ScannerControls,
        bool,
    >,
    #[prop(into)] target_ref: NodeRef<html::Div>,
    #[prop(into)] progress_ref: NodeRef<html::Div>,
    #[prop(into)] set_y_bound: Callback<(f32, f32)>,
) -> impl IntoView {
    let (is_scanning, set_is_scanning) = create_signal(false);

    let (top_y, set_top_y) = create_signal(0.);
    let (bot_y, set_bot_y) = create_signal(0.);
    let get_edges = move || {
        tracing::debug!("Scanner > Getting Edges");
        if let Some(target) = target_ref.get_untracked() {
            let rect =
                to_ws_el(target).get_bounding_client_rect();
            let top_y = rect.y() as f32 + reveal_target;
            let bot_y = top_y + rect.height() as f32;
            set_top_y.set(top_y);
            set_bot_y.set(bot_y);
            set_y_bound.call((bot_y, top_y));
        }
    };

    use_resize_observer(
        target_ref,
        move |_entries, _observer| {
            get_edges();
        },
    );

    let (scanning_anim, set_scanning_anim) =
        create_signal(None::<Animation>);
    let stop_scanning_card = move |_| {
        set_is_scanning.set(false);
        if let Some(target) = target_ref.get_untracked() {
            target
                .class_list()
                .remove_2("scanning", "no-scan")
                .unwrap();
            if let Some(elem) = target.parent_element() {
                let _ = elem.class_list().remove_1("scan-ok");
            }
        }
        if let Some(elem) = document()
            .query_selector(".draggable.active")
            .unwrap()
        {
            let _ = elem.class_list().remove_1("scan-reject");
        }

        if let Some(fill_anim) = scanning_anim.get_untracked() {
            fill_anim.cancel();
        }
    };

    let reject_scan = move || {
        if let Some(target) = target_ref.get_untracked() {
            if let Some(elem) = target.parent_element() {
                let _ = elem.class_list().add_1("scan-fail");
                set_timeout(
                    move || {
                        let _ = elem
                            .class_list()
                            .remove_1("scan-fail");
                    },
                    Duration::from_millis(500),
                );
            }
            let _ = target.class_list().add_1("no-scan");
        }
        if let Some(elem) = document()
            .query_selector(".draggable.active")
            .unwrap()
        {
            let _ = elem.class_list().add_1("scan-reject");
        }
    };

    // TODO
    // create_effect(move |_| {
    //     get_edges();

    // Hacky...double-check position
    // after animations have finished
    // set_timeout(
    //     move || get_edges(),
    //     Duration::from_millis(500),
    // );
    // });

    let scan_card = move || {
        if let Some(progress) = progress_ref.get_untracked() {
            let anim =
                effects::fill_bar(&progress, scan_time as f64);

            let controls = ScannerControls {
                reject_scan: Rc::new(reject_scan),
                progress_elem: progress.clone(),
            };

            let on_finish = Closure::wrap(Box::new(move |_| {
                let keep_scanning =
                    on_finish_scan.call(controls.clone());
                if keep_scanning {
                    set_scanning_anim.update(|anims| {
                        if let Some(anim) = anims {
                            let playback_rate =
                                anim.playback_rate();
                            let multiplier = (playback_rate
                                * (5. / 4.))
                                .min(2.0);
                            anim.set_playback_rate(multiplier);
                            let _ = anim.play();
                        }
                    });
                } else {
                    stop_scanning_card(());
                }
            })
                as Box<dyn FnMut(JsValue)>);

            anim.set_onfinish(Some(
                on_finish.as_ref().unchecked_ref(),
            ));
            set_scanning_anim.set(Some(anim));

            // Keep the closure alive
            on_finish.forget();
        }
    };

    let stop_drag = move || {
        stop_scanning_card(());
        if let Some(target) = target_ref.get_untracked() {
            let _ =
                target.style("transform", "translate(0, 0)");
        }
    };

    // Movement handling
    let check_drag = move |drag_rect: DragRect| {
        if should_show.get() {
            if let Some(target) = target_ref.get_untracked() {
                let target = target
                    .style("visibility", "visible")
                    .style(
                        "transform",
                        format!(
                            "translate(0, {reveal_target}px)"
                        ),
                    );

                let intersects = drag_rect.top_y
                    < bot_y.get_untracked()
                    && drag_rect.bot_y > top_y.get();
                if intersects {
                    if scan_allowed.get_untracked()
                        && !is_scanning.get_untracked()
                    {
                        set_is_scanning.set(true);
                        if let Some(elem) =
                            target.parent_element()
                        {
                            let _ = elem
                                .class_list()
                                .add_1("scan-ok");
                        }
                        scan_card();
                    } else {
                        reject_scan();
                    }
                } else {
                    stop_scanning_card(());
                }
            }
        }
    };

    create_effect(move |_| {
        if let Some(rect) = drag_rect.get() {
            check_drag(rect);
        } else {
            stop_drag();
        }
    });

    children()
}

#[component]
pub fn AddScanner(
    scan_time: f32,
    #[prop(into)] should_show: Signal<bool>,
    #[prop(into)] scan_allowed: Signal<bool>,
    #[prop(into)] drag_rect: Signal<Option<DragRect>>,
    #[prop(into)] on_finish_scan: Callback<
        ScannerControls,
        bool,
    >,
    #[prop(into)] set_y_bound: Callback<(f32, f32)>,
) -> impl IntoView {
    let progress_ref = create_node_ref::<html::Div>();
    let target_ref = create_node_ref::<html::Div>();

    view! {
        <Scanner
            reveal_target=65.
            scan_time
            should_show
            scan_allowed
            drag_rect
            on_finish_scan
            set_y_bound
            target_ref
            progress_ref
        >
            <div class="scanbar-wrapper" ref=target_ref>
                <div class="mini-scanbar">
                    <div class="scanbar-base">
                        <div class="scan-progress-bar" ref=progress_ref></div>
                    </div>
                    <div class="scanbar-led scanbar-led-ok"></div>
                    <div class="scanbar-led scanbar-led-bad"></div>
                    <div class="card-scan-target"></div>
                </div>
            </div>
        </Scanner>
    }
}

#[component]
pub fn RemoveScanner(
    scan_time: f32,
    #[prop(into)] label: Signal<String>,
    #[prop(into)] should_show: Signal<bool>,
    #[prop(into)] scan_allowed: Signal<bool>,
    #[prop(into)] drag_rect: Signal<Option<DragRect>>,
    #[prop(into)] on_finish_scan: Callback<
        ScannerControls,
        bool,
    >,
    #[prop(into)] set_y_bound: Callback<(f32, f32)>,
) -> impl IntoView {
    let progress_ref = create_node_ref::<html::Div>();
    let target_ref = create_node_ref::<html::Div>();

    view! {
        <Scanner
            reveal_target=-60.
            scan_time
            should_show
            scan_allowed
            drag_rect
            on_finish_scan
            set_y_bound
            target_ref
            progress_ref
        >
            <div class="card-withdraw-target" ref=target_ref>
                {label}
                <div class="withdraw-bar" ref=progress_ref></div>
            </div>
        </Scanner>
    }
}

pub trait Scannable:
    std::fmt::Debug + Clone + PartialEq + 'static
{
    fn id(&self) -> &Id;
    fn as_card(item: Signal<Self>) -> View;
    fn get_from_state(id: &Id, state: &State) -> Self;
}

pub trait ScannerSpec {
    type Item: Scannable;

    fn add_props(
        &self,
        item: RwSignal<Option<Self::Item>>,
    ) -> CardScanProps;
    fn rem_props(
        &self,
        item: RwSignal<Option<Self::Item>>,
    ) -> CardScanProps;
}

pub struct CardScanProps {
    pub label: Option<Signal<String>>,
    pub should_show: Signal<bool>,
    pub scan_allowed: Signal<bool>,
    pub on_finish_scan: Callback<ScannerControls, bool>,
    pub scan_time: f32,
}
