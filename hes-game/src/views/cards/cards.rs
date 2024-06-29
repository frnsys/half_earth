use std::time::Duration;

use crate::util::{detect_center_element, is_safari, to_children_vec, to_ws_el};
use leptos::*;

/// Renders child elements side-by-side with drag-to-scroll.
/// Children should be wrapped in <li> tags.
#[component]
pub fn Cards(
    children: Children,
    #[prop(into)] disabled: Signal<bool>,
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
    let (countdown, set_countdown) = create_signal(SCROLL_COUNTDOWN);

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
    let drag_start = move |ev: ev::MouseEvent| {
        if disabled.get() {
            return;
        }
        set_down.set(true);

        if let Some(scroller) = scroller_ref.get() {
            let left = scroller.scroll_left();
            let x = ev.client_x();
            set_pos.set((left, x));
        }
    };
    let drag = move |ev: ev::MouseEvent| {
        if disabled.get() {
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
    let drag_stop = move |ev: ev::MouseEvent| {
        if dragging.get() {
            // Necessary for firefox to snap to the nearest card
            if let Some(scroller) = scroller_ref.get() {
                scroller.scroll();
            }

            ev.prevent_default();
            ev.stop_immediate_propagation();
        }
        set_down.set(false);
        set_dragging.set(false);
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
        if disabled.get() {
            "hidden"
        } else {
            "visible"
        }
    };
    let padding_bottom = move || {
        if disabled.get() {
            format!("{}px", scrollbar_height())
        } else {
            "0px".to_string()
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
                    let children = to_children_vec(scroller.children());
                    let idx = detect_center_element(to_ws_el(scroller), &children);
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
                let nextLast = scroller.scroll_left();
                let children = to_children_vec(scroller.children());
                let idx = detect_center_element(to_ws_el(scroller), &children);
                on_focus.call(idx);
                on_scroll_end.call(());
                set_scrolling.set(false);
                set_countdown.set(SCROLL_COUNTDOWN);
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
            on:scroll=on_scroll
            on:mousedown=drag_start
            on:mousemove=drag
            on:click=drag_stop
        >
            {children()}
            <Show when=|| is_safari()>
                <div class="safari-scroll-margin"></div>
            </Show>
        </div>
    }
}
