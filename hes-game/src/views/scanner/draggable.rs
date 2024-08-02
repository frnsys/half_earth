use std::time::Duration;

use crate::util::card_scale;
use leptos::*;
use leptos_use::{
    use_document,
    use_event_listener,
    use_intersection_observer,
    use_throttle_fn_with_arg,
};

#[derive(Debug, Clone, Copy)]
pub struct DragRect {
    pub top_y: f32,
    pub bot_y: f32,
}

// NOTE: This is up/down dragging (i.e. scanning)
#[component]
pub fn Draggable(
    children: Children,
    draggable: Signal<bool>,
    #[prop(into)] y_bounds: Signal<[f32; 2]>,
    #[prop(into)] on_drag: Callback<DragRect>,
    #[prop(into)] on_drag_stop: Callback<()>,
) -> impl IntoView {
    let (dragging, set_dragging) = create_signal(false);

    // Keep track of the top offset from the element's starting y position;
    // this is updated as the component is dragged
    let (top, set_top) = create_signal(0.);

    // Whether or not dragging is started,
    // i.e. the component has been clicked or touched
    let (down, set_down) = create_signal(false);

    // Current position of the cursor
    let (pos, set_pos) = create_signal((0, 0));

    let (top_y, set_top_y) = create_signal(0.);
    let (height, set_height) = create_signal(0.);

    let el_ref = create_node_ref::<html::Div>();

    use_intersection_observer(
        el_ref,
        move |entries, _observer| {
            let rect = entries[0].bounding_client_rect();
            set_top_y.set(rect.y());
            set_height.set(rect.height());
        },
    );

    let start_drag = move |ev: ev::PointerEvent| {
        if !draggable.get() {
            return;
        }

        ev.prevent_default();
        if let Some(elem) = el_ref.get() {
            let _ = elem.set_pointer_capture(ev.pointer_id());
        }

        let x = ev.client_x();
        let y = ev.client_y();

        set_down.set(true);

        if let Some(el) = el_ref.get() {
            let _ = el.style("cursor", "grab");
        }

        // Update current mouse position.
        set_pos.set((x, y));
    };

    // Eat click events so they don't trigger other behaviors
    // while dragging, e.g. flipping cards.
    let _ = use_event_listener(
        use_document(),
        ev::click,
        move |ev| {
            if dragging.get() {
                ev.stop_immediate_propagation();
            }
        },
    );

    // Throttle this so it doesn't run roughly more than once per frame.
    let drag_handle = use_throttle_fn_with_arg(
        move |ev: ev::PointerEvent| {
            if !down.get() {
                return;
            }
            let (x, y) = pos.get();
            let dx = ev.client_x() - x;
            let dy = ev.client_y() - y;
            let [min_y, max_y] = y_bounds.get();
            if dy.abs() > dx.abs() {
                set_dragging.set(true);
                let top = top.get();
                let y = top_y.get() + top;
                let base_y = y - top;
                let min_dy = min_y as f64 - base_y;
                let max_dy = max_y as f64 - base_y;
                let dy = max_dy.min(min_dy.max(dy as f64));
                if let Some(el) = el_ref.get() {
                    let _ = el.style(
                        "transform",
                        format!(
                            "scale({}) translate(0, {dy}px)",
                            card_scale()
                        ),
                    );
                }
                set_top.set(dy);
                on_drag.call(DragRect {
                    top_y: y as f32,
                    bot_y: (y + height.get()) as f32,
                });
            }
        },
        15.,
    );
    let drag = move |ev: ev::PointerEvent| {
        drag_handle(ev);
    };
    let stop_drag = move || {
        if !down.get() {
            return;
        }
        set_down.set(false);

        // Snap-back animation.
        if let Some(el) = el_ref.get() {
            let _ = el
                .style("transition", "transform 0.15s")
                .style(
                    "transform",
                    format!(
                        "scale({}) translate(0, 0)",
                        card_scale()
                    ),
                );
        }

        // Set dragging to off after a slight delay,
        // so the click-eating handler can still prevent
        // the current click from propagating.
        set_timeout(
            move || {
                set_dragging.set(false);
            },
            Duration::from_millis(10),
        );

        on_drag_stop.call(());
    };

    view! {
        <div
            ref=el_ref
            class="draggable"
            class:active=draggable
            on:pointerdown=start_drag
            on:pointermove=drag
            on:pointerup=move |ev| {
                if let Some(elem) = el_ref.get() {
                    let _ = elem.release_pointer_capture(ev.pointer_id());
                }
                stop_drag();
            }
        >

            {children()}
        </div>
    }
}
