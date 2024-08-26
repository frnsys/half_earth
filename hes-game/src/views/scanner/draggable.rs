use std::time::Duration;

use crate::util::{card_scale, to_ws_el};
use leptos::*;
use leptos_use::{
    use_document,
    use_event_listener,
    use_intersection_observer,
    use_resize_observer,
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
    #[prop(into)] on_drag: Callback<DragRect>,
    #[prop(into)] on_drag_stop: Callback<()>,
) -> impl IntoView {
    let dragging = store_value(false);

    // Whether or not dragging is started,
    // i.e. the component has been clicked or touched.
    let down = store_value(false);

    // Starting cursor position when starting dragging.
    let pos = store_value((0, 0));

    // At-rest top-y position.
    let top_y = store_value(0.);
    let height = store_value(0.);
    let win_height = store_value(0.);

    let el_ref = create_node_ref::<html::Div>();

    let update_rect = move || {
        if let Some(el) = el_ref.get_untracked() {
            let rect = to_ws_el(el).get_bounding_client_rect();
            top_y.set_value(rect.y());
            height.set_value(rect.height());
            let wh = window()
                .inner_height()
                .unwrap()
                .as_f64()
                .unwrap();
            win_height.set_value(wh);
        }
    };

    // Apply card scaling on mount...
    create_effect(move |ok| {
        if ok.is_none()
            && let Some(el) = el_ref.get_untracked()
        {
            let el = el.style(
                "transform",
                format!("scale({})", card_scale()),
            );
            update_rect();
        }
    });

    // ...and when the window size changes.
    use_resize_observer(
        document().body().expect("We have a body element"),
        move |entries, observer| {
            if let Some(el) = el_ref.get_untracked() {
                let _ = el.style(
                    "transform",
                    format!("scale({})", card_scale()),
                );
            }
        },
    );

    use_intersection_observer(
        el_ref,
        move |entries, _observer| {
            update_rect();
        },
    );

    let start_drag = move |ev: ev::PointerEvent| {
        if !draggable.get_untracked() {
            return;
        }

        ev.prevent_default();
        if let Some(el) = el_ref.get_untracked() {
            let _ = el.set_pointer_capture(ev.pointer_id());
            let _ = el.style("cursor", "grab");
        }

        let x = ev.client_x();
        let y = ev.client_y();

        down.set_value(true);

        // Update current mouse position.
        pos.set_value((x, y));
    };

    // Eat click events so they don't trigger other behaviors
    // while dragging, e.g. flipping cards.
    let _ = use_event_listener(
        use_document(),
        ev::click,
        move |ev| {
            if dragging.get_value() {
                ev.stop_immediate_propagation();
            }
        },
    );

    // Throttle this so it doesn't run roughly more than once per frame.
    let drag_handle = use_throttle_fn_with_arg(
        move |ev: ev::PointerEvent| {
            if !down.get_value() {
                return;
            }
            let (x, y) = pos.get_value();
            let dx = ev.client_x() - x;
            let dy = ev.client_y() - y;
            if dy.abs() > dx.abs() {
                dragging.set_value(true);
                if let Some(el) = el_ref.get() {
                    let new_top_y =
                        top_y.get_value() + dy as f64;
                    let new_bot_y = top_y.get_value()
                        + height.get_value()
                        + dy as f64;
                    if new_top_y >= 35.
                        && new_bot_y
                            <= win_height.get_value() - 25.
                    {
                        let _ = el.style(
                            "transform",
                            format!(
                                "scale({}) translate(0, {dy}px)",
                                card_scale()
                            ),
                        );
                    }

                    on_drag.call(DragRect {
                        top_y: new_top_y as f32,
                        bot_y: (new_top_y + height.get_value())
                            as f32,
                    });
                }
            }
        },
        10.,
    );
    let drag = move |ev: ev::PointerEvent| {
        drag_handle(ev);
    };
    let stop_drag = move || {
        if !down.get_value() {
            return;
        }
        down.set_value(false);

        // Snap-back animation.
        if let Some(el) = el_ref.get_untracked() {
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
                dragging.set_value(false);
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
                if let Some(elem) = el_ref.get_untracked() {
                    let _ = elem.release_pointer_capture(ev.pointer_id());
                }
                stop_drag();
            }
        >

            {children()}
        </div>
    }
}
