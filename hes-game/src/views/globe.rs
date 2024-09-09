use crate::{display::FloatExt, memo, t, util::to_ws_el};
use gloo_utils::format::JsValueSerdeExt;
use hes_engine::State;
use leptos::*;
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/js/dist/globe.js")]
extern "C" {
    type Globe;

    #[wasm_bindgen(constructor)]
    fn new(el: &web_sys::HtmlElement) -> Globe;

    #[wasm_bindgen(method)]
    fn init(this: &Globe, tex_path: &str);

    #[wasm_bindgen(method)]
    fn stop(this: &Globe);

    #[wasm_bindgen(method)]
    fn start(this: &Globe);

    #[wasm_bindgen(method, js_name = setClouds)]
    fn set_clouds(this: &Globe, visible: bool);

    #[wasm_bindgen(method, js_name = setRotation)]
    fn set_rotation(this: &Globe, rotate: bool);

    #[wasm_bindgen(method, js_name = setZoom)]
    fn set_zoom(this: &Globe, zoom: f32);

    #[wasm_bindgen(method, js_name = highlightRegion)]
    fn highlight_region(this: &Globe, region_name: &str);

    #[wasm_bindgen(method, js_name = showIconEvent)]
    fn show_icon_event(
        this: &Globe,
        region_name: &str,
        include_coasts: bool,
        icon: &str,
        intensity: usize,
    );

    #[wasm_bindgen(method, js_name = updateSurface)]
    fn update_surface(this: &Globe, tex_path: &str);

    #[wasm_bindgen(method, js_name = onReady)]
    fn on_ready(this: &Globe, cb: &js_sys::Function);

    #[wasm_bindgen(method, js_name = onClick)]
    fn on_click(this: &Globe, cb: &js_sys::Function);
}

pub struct GlobeRef {
    inner: Rc<RefCell<Globe>>,
}
impl Clone for GlobeRef {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl GlobeRef {
    pub fn start(&self) {
        self.inner.borrow().start();
    }

    pub fn stop(&self) {
        self.inner.borrow().stop();
    }

    pub fn rotate(&self, rotate: bool) {
        self.inner.borrow().set_rotation(rotate);
    }

    pub fn clouds(&self, visible: bool) {
        self.inner.borrow().set_clouds(visible);
    }

    pub fn set_zoom(&self, zoom: f32) {
        self.inner.borrow().set_zoom(zoom);
    }

    pub fn highlight_region(&self, region_name: &str) {
        self.inner.borrow().highlight_region(region_name);
    }

    pub fn show_icon_event(
        &self,
        region_name: &str,
        include_coasts: bool,
        icon: &str,
        intensity: usize,
    ) {
        self.inner.borrow().show_icon_event(
            region_name,
            include_coasts,
            icon,
            intensity,
        );
    }
}

fn surface_path(tgav: f32) -> String {
    // Max range is -2 to 14.9.
    let tgav = tgav.max(-2.).min(14.9).round_to(1);
    let key = format!("{tgav:.1}");
    format!("/assets/surface/for_temp/{key}.png")
}

#[component]
pub fn Globe(
    #[prop(optional, default = "globe")] id: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(into)] on_ready: Callback<GlobeRef>,
    #[prop(into, optional)] on_click: Option<Callback<usize>>,
    #[prop(into, optional)] bg_color: Signal<String>,
) -> impl IntoView {
    let (loading, set_loading) = create_signal(true);
    let globe_ref = create_node_ref::<html::Div>();
    let globe_obj = store_value(None);

    let game = expect_context::<RwSignal<State>>();
    let tgav = memo!(game.world.temperature);

    create_effect(move |_| {
        let g = globe_ref.get().unwrap();

        let on_click = Closure::wrap(Box::new(
            move |region_idxs: &JsValue| {
                if let Some(on_click) = on_click {
                    let region_idxs: Vec<usize> =
                        region_idxs.into_serde().unwrap();

                    if !region_idxs.is_empty() {
                        on_click.call(region_idxs[0]);
                    }
                }
            },
        )
            as Box<dyn FnMut(&JsValue)>);

        let globe = Globe::new(&to_ws_el(g));

        let g = GlobeRef {
            inner: Rc::new(RefCell::new(globe)),
        };

        let g_copy = g.clone();
        let on_ready = Closure::wrap(Box::new(move || {
            set_loading.set(false);
            on_ready.call(g_copy.clone());
            g_copy.inner.borrow().start();
        })
            as Box<dyn FnMut()>);

        g.inner
            .borrow()
            .on_ready(on_ready.as_ref().unchecked_ref());
        g.inner
            .borrow()
            .on_click(on_click.as_ref().unchecked_ref());
        g.inner
            .borrow()
            .init(&surface_path(tgav.get_untracked()));
        on_ready.forget();
        on_click.forget();

        globe_obj.set_value(Some(g));
    });

    create_effect(move |_| {
        let tgav = tgav.get();
        if let Some(g) = globe_obj.get_value() {
            g.inner
                .borrow()
                .update_surface(&surface_path(tgav));
        }
    });

    on_cleanup(move || {
        if let Some(g) = globe_obj.get_value() {
            g.inner.borrow().stop();
        }
    });

    view! {
        <div
            id=id
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
