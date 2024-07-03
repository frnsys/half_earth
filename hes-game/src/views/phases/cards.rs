use crate::{
    anim::animation,
    state,
    util::{detect_center_element, nodelist_to_elements, to_ws_el},
    views::cards::{CardFocusArea, Cards, DragRect, Draggable},
    write_state,
};
use leptos::*;
use std::{rc::Rc, time::Duration};
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct ScannerControls {
    pub reject_scan: Rc<dyn Fn() + 'static>,
    pub progress_elem: HtmlElement<html::Div>,
}

fn scan_card_inner(
    controls: ScannerControls,
    scan_time: f32,
    scan_time_multiplier: f32,
    is_scanning: ReadSignal<bool>,
    set_stop_anim: WriteSignal<Option<Rc<dyn Fn()>>>,
    on_finish_scan: Callback<ScannerControls, bool>,
    stop_scanning_card: Callback<()>,
) {
    let duration = scan_time * 1000. * scan_time_multiplier;
    let (_, stop, vals) = animation(
        [0.],
        [100.],
        duration,
        move || {
            if is_scanning.get() {
                let scan_time_multiplier = (scan_time_multiplier * 4. / 5.).max(0.2);
                let keep_scanning = on_finish_scan.call(controls.clone());
                if keep_scanning {
                    scan_card_inner(
                        controls.clone(),
                        scan_time,
                        scan_time_multiplier,
                        is_scanning,
                        set_stop_anim,
                        on_finish_scan,
                        stop_scanning_card,
                    );
                } else {
                    stop_scanning_card.call(());
                }
            }
        },
        true,
    );
    set_stop_anim.set(Some(Rc::new(stop)));
}

#[component(transparent)]
pub fn Scanner(
    children: Children,
    scan_time: f32,
    reveal_target: f32,
    #[prop(into)] should_show: Signal<bool>,
    #[prop(into)] scan_allowed: Signal<bool>,
    #[prop(into)] drag_rect: Signal<Option<DragRect>>,
    #[prop(into)] on_finish_scan: Callback<ScannerControls, bool>,
    #[prop(into)] target_ref: NodeRef<html::Div>,
    #[prop(into)] progress_ref: NodeRef<html::Div>,
    #[prop(into)] set_y_bound: Callback<(f32, f32)>,
) -> impl IntoView {
    let (scan_time_multiplier, set_scan_time_multiplier) = create_signal(1.);
    let (is_scanning, set_is_scanning) = create_signal(false);
    let (scan_anim, set_scan_anim) = create_signal::<Option<Rc<dyn Fn()>>>(None);

    let (top_y, set_top_y) = create_signal(0.);
    let (bot_y, set_bot_y) = create_signal(0.);
    let get_edges = move || {
        if let Some(target) = target_ref.get() {
            let rect = to_ws_el(target).get_bounding_client_rect();
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

    create_effect(move |_| {
        get_edges();

        // Hacky...double-check position
        // after animations have finished
        set_timeout(move || get_edges(), Duration::from_millis(500));
    });

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
        if let Some(elem) = document().query_selector(".draggable.active").unwrap() {
            elem.class_list().add_1("scan-reject");
        }
    };

    let stop_scanning_card = move |_| {
        set_is_scanning.set(false);
        if let Some(target) = target_ref.get() {
            target.class_list().remove_2("scanning", "no-scan").unwrap();
            if let Some(elem) = target.parent_element() {
                elem.class_list().remove_1("scan-ok");
            }
        }
        if let Some(elem) = document().query_selector(".draggable.active").unwrap() {
            elem.class_list().remove_1("scan-reject");
        }
        if let Some(stop_anim) = scan_anim.get() {
            stop_anim();
            set_scan_anim.set(None);
            if let Some(progress) = progress_ref.get() {
                progress.style("width", "0");
            }
        }
    };

    let scan_card = move || {
        if let Some(progress) = progress_ref.get() {
            let controls = ScannerControls {
                reject_scan: Rc::new(reject_scan),
                progress_elem: progress,
            };
            scan_card_inner(
                controls,
                scan_time,
                1.,
                is_scanning,
                set_scan_anim,
                on_finish_scan,
                stop_scanning_card.into(),
            );
            // TODO
            // create_effect(move |_| {
            //     if let Some(progress) = progress_ref.get() {
            //         let p = vals.get()[0];
            //         let width = format!("{p}%");
            //         progress.style("width", width);
            //     }
            // });
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
                    .style("transform", format!("translate(0, {reveal_target}px)"));

                let intersects = drag_rect.top_y < bot_y.get() && drag_rect.bot_y > top_y.get();
                if intersects {
                    let scan_ok = scan_allowed.get();
                    if is_scanning.get() && scan_ok {
                        set_is_scanning.set(true);
                        if let Some(elem) = target.parent_element() {
                            elem.class_list().add_1("scan-ok");
                        }
                        target.class_list().add_1("scanning");
                        scan_card();
                    } else if !scan_ok {
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

    view! { children }
}

#[component]
pub fn AddScanner(
    scan_time: f32,
    #[prop(into)] should_show: Signal<bool>,
    #[prop(into)] scan_allowed: Signal<bool>,
    #[prop(into)] drag_rect: Signal<Option<DragRect>>,
    #[prop(into)] on_finish_scan: Callback<ScannerControls, bool>,
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
    #[prop(into)] on_finish_scan: Callback<ScannerControls, bool>,
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

pub trait Scannable: Clone + 'static {
    fn id(&self) -> String;
    fn as_card(&self) -> View;
}

pub struct CardScanProps {
    pub should_show: Signal<bool>,
    pub scan_allowed: Signal<bool>,
    pub on_finish_scan: Callback<ScannerControls, bool>,
    pub scan_time: f32,
}

#[component]
pub fn ScannerCards<I: Scannable>(
    #[prop(into)] items: Signal<Vec<I>>,
    #[prop(into)] remove_label: Signal<String>,
    add_props: CardScanProps,
    remove_props: CardScanProps,
) -> impl IntoView {
    let (allow_scroll, set_allow_scroll) = create_signal(true);
    let (allow_swipe, set_allow_swipe) = create_signal(true);
    let (focused_idx, set_focused_idx) = create_signal(None); // TODO initialize
    let (drag_rect, set_drag_rect) = create_signal(None);
    let (card_height, set_card_height) = create_signal(0.);

    let on_scroll_start = move |_| {
        set_allow_swipe.set(false);
    };
    let on_scroll_end = move |_| {
        set_allow_swipe.set(true);
    };
    let on_drag = move |rect: DragRect| {
        // This triggers the scanner functionalities
        set_drag_rect.set(Some(rect));
        set_allow_scroll.set(false);
    };
    let on_drag_stop = move |_| {
        // This stops/cancels the scanner functionalities
        set_allow_scroll.set(true);
        set_drag_rect.set(None);
    };

    let update_focused = move || {
        // Figure out what the focused card is
        // TODO next_tick?
        // TODO use refs?
        if let Some(scroller) = document().query_selector(".cards").unwrap() {
            let els = document().query_selector_all(".draggable").unwrap();
            if els.length() == 0 {
                let els = nodelist_to_elements(els);
                if let Some(idx) = detect_center_element(
                    scroller
                        .dyn_into::<web_sys::HtmlElement>()
                        .expect("Is an html element"),
                    &els,
                ) {
                    set_card_height.set(els[idx].get_bounding_client_rect().height());
                    set_focused_idx.set(Some(idx));
                }
            }
        }
    };

    let focused = move || {
        // TODO
        // items(focused_idx.get())
        focused_idx.get().map(|idx| idx)
    };

    let (top_y_bound, set_top_y_bound) = create_signal(0.);
    let (bot_y_bound, set_bot_y_bound) = create_signal(0.);
    let y_bounds = move || [top_y_bound.get(), bot_y_bound.get()];

    // TODO
    // let y_bounds = move || {
    //   if (this.$refs.addScanner && this.$refs.removeScanner) {
    //     return [
    //       this.$refs.addScanner.botY - 10,
    //       this.$refs.removeScanner.topY + 10 - this.cardHeight,
    //     ];
    //   } else {
    //     return null;
    //   }
    // };

    let on_focus = move |idx| {
        write_state!(|state, ui| {
            set_focused_idx.set(idx);
            if let Some(idx) = idx {
                let item: I = items.with(|items| items[idx].clone());
                let id = item.id();
                if ui.viewed.contains(&id) {
                    ui.viewed.push(id);
                }
            }
        });
    };

    view! {
        <Show when=move || focused().is_some()>
            <AddScanner
                scan_time=add_props.scan_time
                drag_rect
                should_show=add_props.should_show
                scan_allowed=add_props.scan_allowed
                on_finish_scan=add_props.on_finish_scan
                set_y_bound=move |(top, bot)| {
                    set_top_y_bound.set(bot - 10.);
                }
            />

            <RemoveScanner
                scan_time=remove_props.scan_time
                drag_rect
                label=remove_label
                should_show=remove_props.should_show
                scan_allowed=remove_props.scan_allowed
                on_finish_scan=remove_props.on_finish_scan
                set_y_bound=move |(top, bot)| {
                    set_bot_y_bound.set(top + 10. - card_height.get() as f32);
                }
            />

        </Show>
        <Cards
            disabled=allow_scroll
            on_focus
            on_scroll_start
            on_scroll_end
        >
            <For
                each=move || items.get().into_iter().enumerate()
                key=|(_, item)| item.id()
                children=move |(i, item)| {
                    let draggable = move || {
                        allow_swipe.get() && focused_idx.get() == Some(i)
                    };
                    let card = item.as_card();
                    let id = move || item.id();
                    view! {
                        <Draggable
                            on_drag
                            on_drag_stop
                            y_bounds
                            id=id.into_signal()
                            draggable=draggable.into_signal()
                        >
                            {card}
                        </Draggable>
                    }
                }
            />

        </Cards>

        <CardFocusArea/>
    }
}

// <ProjectCard
//   :project="projects[i]"
//   @change="$emit('change')" />

// onScrollStarted() {
//   state.help[scrollTip] = true;
//   this.onScrollStart();
// },
// onDragStarted(rect) {
//   state.help[scanTip] = true;
//   this.onDrag(rect);
// },
//
