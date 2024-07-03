use crate::{t, util::to_ws_el};
use gloo_utils::format::JsValueSerdeExt;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/js/earth/globe.js")]
extern "C" {
    type Globe;

    #[wasm_bindgen(constructor)]
    fn new(el: &web_sys::HtmlElement) -> Globe;

    #[wasm_bindgen(method)]
    fn render(this: &Globe);

    #[wasm_bindgen(method)]
    fn init(this: &Globe);

    #[wasm_bindgen(method)]
    fn on_ready(this: &Globe, cb: &js_sys::Function);

    #[wasm_bindgen(method)]
    fn on_click(this: &Globe, cb: &js_sys::Function);
}

pub struct ThaGlobe {
    inner: Rc<RefCell<Globe>>,
}
impl Clone for ThaGlobe {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[component]
pub fn Globe(
    id: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(into)] on_ready: Callback<ThaGlobe>,
    #[prop(into, optional)] on_click: Option<Callback<usize>>,
    #[prop(into, optional)] bg_color: Signal<String>,
) -> impl IntoView {
    let (loading, set_loading) = create_signal(true);
    let globe_ref = create_node_ref::<html::Div>();

    create_effect(move |_| {
        let g = globe_ref.get().unwrap();
        let globe = Globe::new(&to_ws_el(g));
        globe.render();
        globe.init();

        let g = ThaGlobe {
            inner: Rc::new(RefCell::new(globe)),
        };

        let g2 = g.clone();
        let on_ready = Closure::wrap(Box::new(move || {
            set_loading.set(false);
            on_ready.call(g2.clone());
        }) as Box<dyn FnMut()>);
        g.inner.borrow().on_ready(on_ready.as_ref().unchecked_ref());
        on_ready.forget();

        let on_click = Closure::wrap(Box::new(move |region_idxs: &JsValue| {
            if let Some(on_click) = on_click {
                let region_idxs: Vec<usize> = region_idxs.into_serde().unwrap();

                if !region_idxs.is_empty() {
                    on_click.call(region_idxs[0]);
                }
            }
        }) as Box<dyn FnMut(&JsValue)>);
        // globe._onClick = []; // TODO this was in the original code, is this necessary?
        g.inner.borrow().on_click(on_click.as_ref().unchecked_ref());
        on_click.forget();

        // TODO
        // globe.scene.resize();
        // globe.resetCamera();
    });

    on_cleanup(|| {
        // this.globe.active = false;
    });

    // If using a cached globe instance:
    // globe.setEl(this.$el);
    // globe.active = true;
    // globe.render();
    // this.ready = true;
    // if (this.onReady) {
    //     this.onReady(globe);
    // }

    view! {
        <div
            id=format!("globe {}", id)
            class=class
            ref=globe_ref
            style:background-color=bg_color
        >
            <Show when=move || loading.get()>
                <div class="globe-loading loading-text">
                    {t!("Loading")}
                </div>
            </Show>
        </div>
    }
}
