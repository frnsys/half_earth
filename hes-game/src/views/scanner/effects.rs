use std::time::Duration;

use gloo_utils::format::JsValueSerdeExt;
use leptos::*;
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::{
    Animation,
    AnimationEffect,
    Element,
    KeyframeEffect,
    KeyframeEffectOptions,
};

use crate::util::card_scale;

#[derive(Serialize)]
struct KeyframeOpts {
    /// In milliseconds.
    duration: f32,

    /// "forwards", "backwards", or "none".
    fill: &'static str,

    /// "linear", "ease-in", etc.
    easing: &'static str,
}

fn animate<F: Serialize>(
    elem: &Element,
    frames: &[F],
    duration_ms: f64,
    linear: bool,
) -> Animation {
    let frames = JsValue::from_serde(frames).unwrap();
    let mut opts = KeyframeEffectOptions::new();
    opts.duration(&JsValue::from_f64(duration_ms))
        .easing(if linear { "linear" } else { "ease-in-out" });
    let effect = KeyframeEffect::new_with_opt_element_and_keyframes_and_keyframe_effect_options(
        Some(elem), Some(&frames.into()), &opts).unwrap();
    let effect = AnimationEffect::from(effect);
    let anim =
        Animation::new_with_effect(Some(&effect)).unwrap();
    anim.play();
    anim
}

fn shake(elem: &Element, duration_ms: f64) {
    #[derive(Serialize)]
    struct TranslateKeyframe {
        translate: &'static str,
        offset: f32,
    }

    let frames = vec![
        TranslateKeyframe {
            translate: "0 0",
            offset: 0.0,
        },
        TranslateKeyframe {
            translate: "-2px 1px",
            offset: 0.25,
        },
        TranslateKeyframe {
            translate: "1px 2px",
            offset: 0.35,
        },
        TranslateKeyframe {
            translate: "3px 1px",
            offset: 0.55,
        },
        TranslateKeyframe {
            translate: "-1px 2px",
            offset: 0.75,
        },
        TranslateKeyframe {
            translate: "0 0",
            offset: 1.0,
        },
    ];
    animate(&elem, &frames, duration_ms, false);
}
fn pulse(elem: &Element, from: f32, to: f32, duration_ms: f64) {
    #[derive(Serialize)]
    struct ScaleKeyframe {
        scale: f32,
        offset: f32,
    }
    let frames = vec![
        ScaleKeyframe {
            scale: from,
            offset: 0.0,
        },
        ScaleKeyframe {
            scale: to,
            offset: 0.5,
        },
        ScaleKeyframe {
            scale: from,
            offset: 1.0,
        },
    ];
    animate(&elem, &frames, duration_ms, false);
}

pub fn fill_bar(elem: &Element, duration_ms: f64) -> Animation {
    #[derive(Serialize)]
    struct WidthKeyframe {
        width: &'static str,
        offset: f32,
    }
    let frames = vec![
        WidthKeyframe {
            width: "0%",
            offset: 0.0,
        },
        WidthKeyframe {
            width: "100%",
            offset: 1.0,
        },
    ];
    animate(&elem, &frames, duration_ms, true)
}

pub fn glow(elem: &Element, duration_ms: f64) -> Animation {
    #[derive(Serialize)]
    struct BoxShadowKeyframe {
        box_shadow: &'static str,
        offset: f32,
    }
    let frames = vec![
        BoxShadowKeyframe {
            box_shadow:
                "0 0 2px #6bff66, inset 1px 0px 8px #6bff66",
            offset: 0.0,
        },
        BoxShadowKeyframe {
            box_shadow:
                "0 0 24px #6bff66, inset 1px 0px 8px #6bff66",
            offset: 1.0,
        },
    ];
    animate(&elem, &frames, duration_ms, true)
}

pub fn shake_screen() {
    document().body().map(|body| {
        // TODO
        // window.audioManager.playOneShot('/assets/sounds/impact.mp3');
        shake(&body.into(), 350.0);
    });
}

pub fn shake_progress(elem: web_sys::HtmlElement) {
    if let Some(elem) = elem.parent_element() {
        shake(&elem, 350.0);
    }
}

pub fn pulse_card() {
    if let Some(elem) =
        document().query_selector(".draggable.active").unwrap()
    {
        let from = card_scale();
        pulse(&elem, from, from * 1.05, 100.);
    }
}

pub fn shrink_pulse_card() {
    if let Some(elem) =
        document().query_selector(".draggable.active").unwrap()
    {
        let from = card_scale();
        pulse(&elem, from, from * 0.95, 100.);
    }
}

pub fn pulse_level() {
    if let Some(elem) = document()
        .query_selector(".draggable.active .project-cost")
        .unwrap()
    {
        pulse(&elem, 1.0, 1.2, 200.);
    }
}
