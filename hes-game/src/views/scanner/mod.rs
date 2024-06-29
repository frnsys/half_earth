use std::time::Duration;

use leptos::*;

use crate::{
    anim::animation,
    util::{card_scale, to_ws_el},
};

#[component]
pub fn RemoveScanner() -> impl IntoView {
    todo!()
}

#[component]
pub fn AddScanner() -> impl IntoView {
    let progress_ref = create_node_ref::<html::Div>();
    let target_ref = create_node_ref::<html::Div>();

    const REVEAL_TARGET: f64 = 65.;

    let get_edges = move || {
        if let Some(target) = target_ref.get() {
            let rect = to_ws_el(target).get_bounding_client_rect();
            let top_y = rect.y() + REVEAL_TARGET;
            let bot_y = top_y + rect.height();
        }
    };

    // TODO
    // window.addEventListener('resize', this.getEdges);
    // beforeUnmount() {
    //   window.removeEventListener('resize', this.getEdges);
    // },

    view! {
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
    }
}

fn shake_screen() {
    document().body().map(|body| {
        // TODO
        // window.audioManager.playOneShot('/assets/sounds/impact.mp3');
        body.class_list().add_1("shake").unwrap();
        set_timeout(
            move || {
                body.class_list().remove_1("shake").unwrap();
            },
            Duration::from_millis(500),
        );
    });
}

fn shake_progress(elem: web_sys::HtmlElement) {
    if let Some(elem) = elem.parent_element() {
        elem.class_list().add_2("scan-error", "shake");
        set_timeout(
            move || {
                elem.class_list().remove_2("scan-error", "shake").unwrap();
            },
            Duration::from_millis(350),
        );
    }
}

fn pulse_card() {
    if let Some(elem) = document().query_selector(".draggable.active").unwrap() {
        let from = card_scale();
        let to = from * 1.05;
        // animation([from],[to], 100., Some(||))

        // TODO
        // animate(consts.cardScale, consts.cardScale*1.05, 100, (val) => {
        //   updateTransform(el, {scale: val});
        // }, () => {
        //   animate(consts.cardScale*1.05, consts.cardScale, 100, (val) => {
        //     updateTransform(el, {scale: val});
        //   });
        // });
    }
}

fn shrink_pulse_card() {
    if let Some(elem) = document().query_selector(".draggable.active").unwrap() {
        let from = card_scale();
        let to = from * 0.95;
        // TODO
        // animate(consts.cardScale, consts.cardScale*0.95, 100, (val) => {
        //   updateTransform(el, {scale: val});
        // }, () => {
        //   animate(consts.cardScale*0.95, consts.cardScale, 100, (val) => {
        //     updateTransform(el, {scale: val});
        //   });
        // });
    }
}
