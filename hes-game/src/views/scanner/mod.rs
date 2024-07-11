mod cards;
mod draggable;
mod effects;
mod process;
mod project;

use hes_engine::state::State;
use leptos::*;
use std::{rc::Rc, time::Duration};

use crate::util::to_ws_el;
use draggable::{DragRect, Draggable};

pub use cards::ScannerCards;
pub use process::ProcessScanner;
pub use project::ProjectScanner;

fn adding_animation(scan_speed: f32) -> String {
    format!("scanning 0.35s ease-in-out infinite alternate, fill-bar {scan_speed}s linear infinite forwards")
}

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
        effects::pulse_card();
    }

    pub fn shake_screen(&self) {
        effects::shake_screen();
    }
}

#[component(transparent)]
pub fn Scanner(
    id: &'static str,
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
    let (scan_time_multiplier, set_scan_time_multiplier) =
        create_signal(1.);
    let (is_scanning, set_is_scanning) = create_signal(false);

    let (top_y, set_top_y) = create_signal(0.);
    let (bot_y, set_bot_y) = create_signal(0.);
    let get_edges = move || {
        if let Some(target) = target_ref.get() {
            let rect =
                to_ws_el(target).get_bounding_client_rect();
            let top_y = rect.y() as f32 + reveal_target;
            let bot_y = top_y + rect.height() as f32;
            set_top_y.set(top_y);
            set_bot_y.set(bot_y);
            set_y_bound.call((bot_y, top_y));
        }
    };

    // TODO
    // window.addEventListener('resize', this.getEdges);
    // beforeUnmount() {
    //   window.removeEventListener('resize', this.getEdges);
    // },

    let stop_scanning_card = move |_| {
        set_is_scanning.set(false);
        if let Some(target) = target_ref.get() {
            target
                .class_list()
                .remove_2("scanning", "no-scan")
                .unwrap();
            if let Some(elem) = target.parent_element() {
                elem.class_list().remove_1("scan-ok");
            }
        }
        if let Some(elem) = document()
            .query_selector(".draggable.active")
            .unwrap()
        {
            elem.class_list().remove_1("scan-reject");
        }

        if let Some(progress) = progress_ref.get() {
            progress.style("animation", "");
        }
    };

    let reject_scan = move || {
        if let Some(target) = target_ref.get() {
            if let Some(elem) = target.parent_element() {
                elem.class_list().add_1("scan-fail");
                set_timeout(
                    move || {
                        elem.class_list().remove_1("scan-fail");
                    },
                    Duration::from_millis(500),
                );
            }
            target.class_list().add_1("no-scan");
        }
        if let Some(elem) = document()
            .query_selector(".draggable.active")
            .unwrap()
        {
            elem.class_list().add_1("scan-reject");
        }
    };

    let (scan_time_multiplier, set_scan_time_multiplier) =
        create_signal::<f32>(1.);
    create_effect(move |_| {
        if let Some(progress) = progress_ref.get() {
            let controls = ScannerControls {
                reject_scan: Rc::new(reject_scan),
                progress_elem: progress.clone(),
            };
            progress.on(
                ev::animationiteration,
                move |ev: ev::AnimationEvent| {
                    if ev.animation_name() == "fill-bar" {
                        logging::log!(
                            "ELAPSED TIME: {}",
                            ev.elapsed_time()
                        );
                        if let Some(progress) =
                            progress_ref.get()
                        {
                            // TODO needs work
                            // basically what seems to happen is changing the animation property
                            // immediately triggers another animationiteration event;
                            // if you look at two subsequent ev.elapsed_time() they'll be almost
                            // identical
                            logging::log!(
                                "CALLING ON FINISH SCAN"
                            );
                            let keep_scanning = on_finish_scan
                                .call(controls.clone());
                            logging::log!(
                                "KEEP SCANNING: {}",
                                keep_scanning
                            );
                            // let progress = progress.style("animation", "");
                            if keep_scanning {
                                // set_timeout(move || {
                                logging::log!(
                                    "UPDATING MULTIPLIER"
                                );
                                logging::log!(
                                    "SCAN TIME BASE: {}",
                                    scan_time
                                );
                                let multiplier =
                                    (scan_time_multiplier
                                        .get_untracked()
                                        * 4.
                                        / 5.)
                                        .max(0.2);
                                // TODO this triggers way too quickly in the middle
                                if multiplier
                                    != scan_time_multiplier
                                        .get_untracked()
                                {
                                    progress.style(
                                        "animation",
                                        adding_animation(
                                            scan_time
                                                * multiplier,
                                        ),
                                    );
                                    set_scan_time_multiplier
                                        .set_untracked(
                                            multiplier,
                                        );
                                }
                                logging::log!(
                                    "MULTIPLIER UPDATED: {:?}",
                                    multiplier
                                );
                                // }, Duration::from_millis(16));
                            } else {
                                stop_scanning_card(());
                                progress.style("animation", "");
                            }
                        }
                    }
                },
            );
        }
    });

    create_effect(move |_| {
        get_edges();

        // Hacky...double-check position
        // after animations have finished
        set_timeout(
            move || get_edges(),
            Duration::from_millis(500),
        );
    });

    let scan_card = move || {
        if let Some(progress) = progress_ref.get() {
            let progress = progress.style(
                "animation",
                adding_animation(scan_time),
            );
        }
    };

    let stop_drag = move || {
        stop_scanning_card(());
        if let Some(target) = target_ref.get() {
            target.style("transform", "translate(0, 0)");
        }
    };

    // Movement handling
    let check_drag = move |drag_rect: DragRect| {
        if should_show.get() {
            if let Some(target) = target_ref.get() {
                let target = target
                    .style("visibility", "visible")
                    .style(
                        "transform",
                        format!(
                            "translate(0, {reveal_target}px)"
                        ),
                    );

                // logging::log!("INTERSECT CHECK: {:?} [bot: {} top: {}]", drag_rect, bot_y.get(), top_y.get());
                let intersects = drag_rect.top_y < bot_y.get()
                    && drag_rect.bot_y > top_y.get();
                if intersects {
                    if scan_allowed.get() && !is_scanning.get()
                    {
                        set_is_scanning.set(true);
                        if let Some(elem) =
                            target.parent_element()
                        {
                            elem.class_list().add_1("scan-ok");
                        }
                        // target.class_list().add_1("scanning");
                        logging::log!(
                            "SCAN TRIGGERED FOR {id}"
                        );
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

    view! { {children()} }
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
            id="add-scanner"
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
            id="remove-scanner"
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
    fn id(&self) -> usize;
    fn as_card(item: Signal<Self>) -> View;
    fn get_from_state(id: usize, state: &State) -> Self;
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
