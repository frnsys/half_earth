use std::{rc::Rc, time::Duration};

use crate::{util::card_scale, anim::animation};
use leptos::*;
use leptos_use::use_intersection_observer;

// TODO

#[derive(Clone, Copy)]
pub struct DragRect {
    pub top_y: f32,
    pub bot_y: f32,
}

#[component]
pub fn Draggable(
    children: Children,
    id: Signal<String>,
    draggable: Signal<bool>,
    #[prop(into)] y_bounds: Signal<[f32; 2]>,
    #[prop(into)] on_drag: Callback<DragRect>,
    #[prop(into)] on_drag_stop: Callback<()>,
) -> impl IntoView {
    let (dragging, set_dragging) = create_signal(false);

    // let start_drag = move |_| {};

    // Keep track of the top offset from the element's starting y position;
    // this is updated as the component is dragged
    let (top, set_top) = create_signal(0.);

    // Whether or not dragging is started,
    // i.e. the component has been clicked or touched
    let (down, set_down) = create_signal(false);

    // Cache the starting y position of the element
    let mut el_y = 0;

    // Current position of the cursor
    let (pos, set_pos) = create_signal((0, 0));

    let (top_y, set_top_y) = create_signal(0.);
    let (height, set_height) = create_signal(0.);

    // if draggable.get() {
    // }

    let el_ref = create_node_ref::<html::Div>();

    use_intersection_observer(
        el_ref,
        move |entries, _observer| {
            let rect = entries[0].bounding_client_rect();
            set_top_y.set(rect.y());
            set_height.set(rect.height());
            // TODO disconnect?
        },
    );

    // Whether or not dragging is enabled
    let (is_enabled, set_is_enabled) = create_signal(false);
    let enable = move || {
        if is_enabled.get() {
            return
        }
        set_is_enabled.set(true);

        // document.body.addEventListener('touchmove', this.dragHandler, {passive: true});
        // document.body.addEventListener('mousemove', this.dragHandler, {passive: true});
      // window.addEventListener('mouseup', this.stopDrag);
      // window.addEventListener('touchend', this.stopDrag);

      // this.getPosition(); with the observer, is this necessary?
    };
    let disable = move || {
        if !is_enabled.get() {
            return
        }
        set_is_enabled.set(false);
      // document.body.removeEventListener('touchmove', this.dragHandler, {passive: true});
      // document.body.removeEventListener('mousemove', this.dragHandler, {passive: true});
      // window.removeEventListener('mouseup', this.stopDrag);
      // window.removeEventListener('touchend', this.stopDrag);
    };

    let (anim_stop, set_anim_stop) = create_signal::<Option<Rc<dyn Fn()>>>(None);

    // this.getPosition(); with the observer, is this necessary?
    // let resize_handle = window_event_listener(ev::resize, |ev| getPosition());
    on_cleanup(move || {
        disable();
        // resize_handle.remove();
    });

    let start_drag = move |x: i32, y: i32| {
        if !draggable.get() {
            return
        }
        set_down.set(true);

      // Stop snap-back animation if there is one
        if let Some(stop_anim) = anim_stop.get() {
            stop_anim();
        }

        if let Some(el) = el_ref.get() {
            el.style("cursor", "grab");
        }

        // Update current mouse position.
        set_pos.set((x, y));
    };
    let drag = move |ev: ev::MouseEvent| {
        if !down.get() {
            return
        }
        let (x, y) = pos.get();
        let dx = ev.client_x() - x;
        let dy = ev.client_y() - y;
        let [min_y, max_y] = y_bounds.get();
        if dy.abs() > dx.abs() {
            set_dragging.set(true);
            let top = top.get();
            let y = top_y.get() + top;
            if y > min_y as f64 && y < max_y as f64 {
                let base_y = y - top;
                let min_dy = min_y as f64 - base_y;
                let max_dy = max_y as f64 - base_y;
                let dy = max_dy.min(min_dy.max(dy as f64));
                if let Some(el) = el_ref.get() {
                    el.style("transform", format!("scale({}), translate(0, {dy}px)", card_scale()));
                }
                set_top.set(dy);
            }
            on_drag.call(DragRect {
                top_y: y as f32,
                bot_y: (y + height.get()) as f32,
            });
        }
    };
    let stop_drag = move || {
        if !down.get() {
            return
        }
        set_down.set(false);
        set_dragging.set(false);

        let (_, stop, vals) = animation([top.get() as f32], [0.], 100., || {}, false);
        set_anim_stop.set(Some(Rc::new(stop)));
        create_effect(move |_| {
            if let Some(el) = el_ref.get() {
                let top = vals.get()[0];
                el.style("transform", format!("scale({}), translate(0, {top}px)", card_scale()));
            }
        });

        on_drag_stop.call(());
    };

    create_effect(move |_| {
        if draggable.get() {
            enable();

                // Hacky...double-check position
                // after animations have finished
                set_timeout(move || {
                    // this.getPosition(); with the observer, is this necessary?
                }, Duration::from_millis(400));

        // If not draggable, disable dragging events
        } else {
            disable();
            stop_drag();
        }
    });

    view! {
        <div
            id=id
            ref=el_ref
            class="draggable"
            class:dragging=dragging
            style:transform=format!("scale({})", card_scale())
            class:active=draggable
            on:mousedown=move |ev| start_drag(ev.client_x(), ev.client_y())
            on:touchstart=move |ev| {
                if let Some(touch) = ev.touches().get(0) {
                    start_drag(touch.client_x(), touch.client_y());
                }
            }
        >
            {children()}
        </div>
    }
}
