use std::time::Duration;

use crate::util::{
    collection_to_elements,
    detect_center_element,
    is_safari,
    to_ws_el,
};
use leptos::*;

/// Renders child elements side-by-side with drag-to-scroll.
/// Children should be wrapped in <li> tags.
#[component]
pub fn Cards(
    children: Children,
    #[prop(into)] enabled: Signal<bool>,
    #[prop(into)] on_focus: Callback<Option<usize>>,
    #[prop(into)] on_scroll_start: Callback<()>,
    #[prop(into)] on_scroll_end: Callback<()>,
) -> impl IntoView {
    // How many scroll events we wait until
    // declaring that scrolling has started
    const SCROLL_COUNTDOWN: i32 = 10;

    let (pos, set_pos) = create_signal((0, 0));
    let (down, set_down) = create_signal(false);
    let (dragging, set_dragging) = create_signal(false);
    let (countdown, set_countdown) =
        create_signal(SCROLL_COUNTDOWN);

    // We use this to determine if the scrolling
    // (and its momentum) have stopped
    let (last, set_last) = create_signal(0);

    // Just a flag to identify if we just started scrolling
    // as opposed to if we're in the middle of scrolling
    let (scrolling, set_scrolling) = create_signal(false);

    let scroller_ref = create_node_ref::<html::Div>();

    let on_scroll = move |_| {
        // If we're not already in a scroll action
        // and a scroll event is fired, that means
        // we started scrolling.
        // But we wait until seeing a certain number of
        // scroll events until firing a scrollStart event
        // to deal with some timing issues that cause
        // mobile dragging to be wonky
        if !scrolling.get() {
            if countdown.get() > 0 {
                set_countdown.update(|cd| *cd -= 1);
            } else {
                set_scrolling.set(true);
                on_scroll_start.call(());
            }
        }
    };

    // Drag to scroll horizontally on desktop
    let drag_start = move |ev: ev::PointerEvent| {
        // if let Some(elem) = scroller_ref.get() {
        //     elem.set_pointer_capture(ev.pointer_id());
        // }

        if !enabled.get() {
            return;
        }
        set_down.set(true);

        if let Some(scroller) = scroller_ref.get() {
            let left = scroller.scroll_left();
            let x = ev.client_x();
            set_pos.set((left, x));
        }
    };
    let drag_stop = move |ev: ev::PointerEvent| {
        // if let Some(elem) = scroller_ref.get() {
        //     elem.release_pointer_capture(ev.pointer_id());
        // }

        if dragging.get() {
            // Necessary for firefox to snap to the nearest card
            if let Some(scroller) = scroller_ref.get() {
                scroller.scroll();
            }

            ev.prevent_default();
            // ev.stop_immediate_propagation();
        }
        set_down.set(false);
        set_dragging.set(false);
    };
    let drag = move |ev: ev::PointerEvent| {
        if !enabled.get() {
            return;
        }

        // Button no longer pressed
        if ev.pressure() < 0.5 {
            drag_stop(ev);
            return;
        }

        let (left, x) = pos.get();
        let dx = ev.client_x() - x;
        if down.get() && dx.abs() > 10 {
            set_dragging.set(true);
            if let Some(scroller) = scroller_ref.get() {
                scroller.set_scroll_left(left - dx);
            }
        }
    };

    // Calculate scroll bar height so we can accommodate it
    // when it disappears when overflowX is set to hidden.
    // This prevents the layout from shifting when a card is being dragged.
    let scrollbar_height = move || {
        scroller_ref
            .get()
            .map(|s| s.offset_height() - s.client_height())
            .unwrap_or(0)
    };

    let overflow_x = move || {
        if enabled.get() {
            "visible"
        } else {
            "hidden"
        }
    };
    let padding_bottom = move || {
        if enabled.get() {
            "0px".to_string()
        } else {
            format!("{}px", scrollbar_height())
        }
    };

    // Hack to start with first card focused
    create_effect(move |_| {
        if let Some(scroller) = scroller_ref.get() {
            let width = scroller.client_width();
            scroller.set_scroll_left(width / 2);
        }
    });

    // Fallback for if the focused card detection messes up
    let focus_handle = set_interval_with_handle(
        move || {
            if !scrolling.get() {
                if let Some(scroller) = scroller_ref.get() {
                    let children = collection_to_elements(
                        scroller.children(),
                    );
                    let idx = detect_center_element(
                        to_ws_el(scroller),
                        &children,
                    );
                    on_focus.call(idx);
                }
            }
        },
        Duration::from_millis(100),
    )
    .unwrap();

    // Wait to see if we've stopped scrolling
    // If so, figure out what the focused/centered child is.
    let timeout_handle = set_interval_with_handle(
        move || {
            if let Some(scroller) = scroller_ref.get() {
                let next_last = scroller.scroll_left();

                // If we are still within a scroll action and
                // momentum/snapping has finished
                // (i.e. the scroll left position hasn't changed),
                // we're done scrolling.
                if scrolling.get() && last.get() == next_last {
                    let children = collection_to_elements(
                        scroller.children(),
                    );
                    let idx = detect_center_element(
                        to_ws_el(scroller),
                        &children,
                    );
                    on_focus.call(idx);
                    on_scroll_end.call(());
                    set_scrolling.set(false);
                    set_countdown.set(SCROLL_COUNTDOWN);
                } else {
                    set_last.set(next_last);
                }
            }
        },
        Duration::from_millis(16),
    )
    .unwrap();

    on_cleanup(move || {
        focus_handle.clear();
        timeout_handle.clear();
    });

    // Horizontally dragging for desktop
    // Use 'click' instead of 'mouseup' so we can properly
    // intercept other click events to the cards
    view! {
        <div
            class="cards"
            ref=scroller_ref
            class:unlock-scroll=dragging
            class:is-dragging=dragging
            on:scroll=on_scroll
            on:pointerdown=drag_start
            on:pointermove=drag
            on:pointerup=drag_stop
        >
            {children()}
            <Show when=|| is_safari()>
                <div class="safari-scroll-margin"></div>
            </Show>
        </div>
    }
}

#[component]
pub fn CardFocusArea() -> impl IntoView {
    view! {
        <div class="card-focus-area wrapper">
            <div class="inner">
                <small class="helper top">"▲"</small>
                <small class="helper bottom">"▼"</small>
            </div>
        </div>
    }
}
