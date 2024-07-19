use crate::{state, t, util::to_ws_el};
use gloo_utils::format::JsValueSerdeExt;
use js_sys::Uint8Array;
use leptos::*;
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/js/earth/globe.pkg.js")]
extern "C" {
    type Globe;

    #[wasm_bindgen(constructor)]
    fn new(el: &web_sys::HtmlElement) -> Globe;

    #[wasm_bindgen(method)]
    fn render(this: &Globe);

    #[wasm_bindgen(method)]
    fn init(
        this: &Globe,
        width: usize,
        height: usize,
        pixels: Uint8Array,
    );

    #[wasm_bindgen(method)]
    fn clear(this: &Globe);

    #[wasm_bindgen(method, js_name = hideClouds)]
    fn hide_clouds(this: &Globe);

    #[wasm_bindgen(method, js_name = stopRotation)]
    fn stop_rotation(this: &Globe);

    #[wasm_bindgen(method, js_name = setZoom)]
    fn set_zoom(this: &Globe, zoom: f32);

    #[wasm_bindgen(method, js_name = highlightRegion)]
    fn highlight_region(this: &Globe, region_name: &str);

    #[wasm_bindgen(method)]
    fn update_surface(this: &Globe, pixels: Uint8Array);

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
    pub fn clear(&self) {
        self.inner.borrow().clear();
    }

    pub fn stop_rotation(&self) {
        self.inner.borrow().stop_rotation();
    }

    pub fn hide_clouds(&self) {
        self.inner.borrow().hide_clouds();
    }

    pub fn set_zoom(&self, zoom: f32) {
        self.inner.borrow().set_zoom(zoom);
    }

    pub fn highlight_region(&self, region_name: &str) {
        self.inner.borrow().highlight_region(region_name);
    }
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

    let tgav = state!(world.temperature);

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

        logging::log!("RUST CALING GLOBE");
        let globe = Globe::new(&to_ws_el(g));

        let g = GlobeRef {
            inner: Rc::new(RefCell::new(globe)),
        };

        let g_copy = g.clone();
        let on_ready = Closure::wrap(Box::new(move || {
            logging::log!("GLOBE READY");
            set_loading.set(false);
            on_ready.call(g_copy.clone());
        })
            as Box<dyn FnMut()>);

        g.inner
            .borrow()
            .on_ready(on_ready.as_ref().unchecked_ref());
        g.inner
            .borrow()
            .on_click(on_click.as_ref().unchecked_ref());

        spawn_local(async move {
            let (width, height, pixels) =
                calc_surface(tgav.get()).await.unwrap();
            logging::log!("GLOBE INITED");
            g.inner.borrow().init(
                width,
                height,
                pixels.as_slice().into(),
            );
            logging::log!("GLOBE RENDERING");
            g.inner.borrow().render();
        });

        on_ready.forget();
        on_click.forget();

        // TODO
        // globe.scene.resize();
        // globe.resetCamera();
    });

    // on_cleanup(|| {
    // this.globe.active = false;
    // });

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

#[server(prefix = "/compute", endpoint = "surface")]
pub async fn calc_surface(
    tgav: f32,
) -> Result<(usize, usize, Vec<u8>), ServerFnError> {
    let mut surface = crate::globe::STARTING_SURFACE.clone();
    let width = surface.width();
    let height = surface.height();

    surface.update_biomes(tgav);
    surface.update_surface();
    Ok((width, height, surface.pixels))
}
