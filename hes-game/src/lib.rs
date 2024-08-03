#![feature(let_chains)]
#![feature(extract_if)]

mod app;
mod audio;
mod consts;
mod debug;
mod display;
mod eval;
mod i18n;
mod icons;
mod state;
mod tgav;
mod util;
mod vars;
mod views;

pub use views::CalcSurface;

#[cfg(feature = "server")]
mod server;

use app::Root;

use leptos::*;
use tracing::Level;
use tracing_wasm::WASMLayerConfigBuilder;

#[cfg_attr(
    feature = "client",
    wasm_bindgen::prelude::wasm_bindgen
)]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let config = WASMLayerConfigBuilder::new()
        .set_max_level(Level::DEBUG)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);
    mount_to_body(|| {
        view! { <Root/> }
    })
}
