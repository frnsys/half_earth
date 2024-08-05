use crate::{memo, t, util::to_ws_el};
use gloo_utils::format::JsValueSerdeExt;
use hes_engine::Game;
use js_sys::Uint8Array;
use leptos::*;
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/js/dist/globe.js")]
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

    let game = expect_context::<RwSignal<Game>>();
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
                calc_surface(tgav.get_untracked())
                    .await
                    .unwrap();
            g.inner.borrow().init(
                width,
                height,
                pixels.as_slice().into(),
            );
            g.inner.borrow().render();
        });

        on_ready.forget();
        on_click.forget();
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

#[server(prefix = "/compute", endpoint = "surface")]
pub async fn calc_surface(
    tgav: f32,
) -> Result<(usize, usize, Vec<u8>), ServerFnError> {
    let mut surface = crate::server::STARTING_SURFACE.clone();
    let width = surface.width();
    let height = surface.height();

    surface.update_biomes(tgav);
    surface.update_surface();
    Ok((width, height, surface.pixels))
}
