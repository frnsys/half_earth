use leptos::*;
use leptos_dom::helpers::AnimationFrameRequestHandle;
use leptos_use::{use_raf_fn, use_raf_fn_with_options, utils::Pausable, UseRafFnOptions};
use std::rc::Rc;

/// Ease-in-out quadratic function
/// <https://gist.github.com/andjosh/6764939>
fn ease_in_out_quad(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut t = t / (d / 2.0);
    if t < 1.0 {
        return c / 2.0 * t * t + b;
    }
    t -= 1.0;
    -c / 2.0 * (t * (t - 2.0) - 1.0) + b
}

#[derive(Clone)]
pub struct Anim {
    frame: Rc<dyn Fn() + 'static>,
    handle: RwSignal<Option<AnimationFrameRequestHandle>>,
    stopped: RwSignal<bool>,
}
impl Anim {
    pub fn start(&self) {
        self.call();
    }

    pub fn stop(&self) {
        if let Some(Some(handle)) = self.handle.try_get() {
            handle.cancel();
            self.stopped.set(true);
        }
    }

    pub fn call(&self) {
        let stopped = self.stopped.try_get();
        if stopped.is_none() || stopped == Some(true) {
            return;
        }
        let copy = self.clone();
        let handle = request_animation_frame_with_handle(move || {
            (copy.frame)();
            copy.call();
        })
        .unwrap();
        self.handle.set(Some(handle));
    }
}

/// Takes an array of start values and an array of end values,
/// then animates between the values.
/// When the animation is finished the optional callback is called.
/// The duration is in ms.
pub fn animation<const N: usize, CB: Fn() + 'static>(
    start: [f32; N],
    end: [f32; N],
    duration: f32,
    callback: CB,
    linear: bool,
) -> (Anim, ReadSignal<[f32; N]>) {
    #[derive(Default, Clone, Copy)]
    struct Value {
        start: f32,
        min: f32,
        max: f32,
        delta: f32,
    }

    let mut deltas = [Value::default(); N];
    for i in 0..start.len() {
        deltas[i] = Value {
            start: start[i],
            min: start[i].min(end[i]),
            max: start[i].max(end[i]),
            delta: end[i] - start[i],
        };
    }

    // If timestamp is very large it can cause
    // the value to overshoot the end target,
    // so clamp it in case.
    let lerp: Box<dyn Fn(f32) -> [f32; N]> = if linear {
        Box::new(move |elapsed| {
            deltas.map(|v| {
                let val: f32 = (elapsed / duration * v.delta);
                val.clamp(v.min, v.max)
            })
        })
    } else {
        Box::new(move |elapsed| {
            deltas
                .map(|v| ease_in_out_quad(elapsed, v.start, v.delta, duration).clamp(v.min, v.max))
        })
    };

    let start_time = window().performance().unwrap().now();
    let (vals, set_vals) = create_signal(start);
    let (is_done, set_is_done) = create_signal(false);

    let anim = Anim {
        frame: Rc::new(move || {
            let timestamp = window().performance().unwrap().now();
            let elapsed = (timestamp - start_time) as f32;
            if elapsed < duration {
                let new_vals = lerp(elapsed);
                set_vals.update(|vals| *vals = new_vals);
            } else {
                set_vals.update(|vals| *vals = end);
                set_is_done.set(true);
            }
        }),
        handle: create_rw_signal(None),
        stopped: create_rw_signal(false),
    };

    // Note: Calling the callback
    // within the request animation frame function
    // doesn't work because it doesn't let us access
    // context signals, so we set up the `is_done`
    // signal and effect instead.
    // let pause_ = pause.clone();
    let anim_ = anim.clone();
    create_effect(move |_| {
        if is_done.get() {
            anim_.stop();
            callback();
        }
    });

    (anim, vals)
}

/// Convenience function to animate from 1.0 to 0.0.
pub fn fade_out<CB: Fn() + 'static>(duration: f32, callback: CB) -> (Anim, Signal<f32>) {
    let (anim, anim_vals) = animation([1.0], [0.0], duration, callback, false);
    let opacity = move || anim_vals.with(|vals| vals[0]);
    (anim, opacity.into_signal())
}
