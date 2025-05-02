use std::panic::PanicHookInfo;

use hes_game::Root;
use leptos::*;
use tracing::Level;
use tracing_wasm::WASMLayerConfigBuilder;
use web_sys::{window, Document};

fn display_panic(info: &PanicHookInfo) {
    let document: Document =
        window().unwrap().document().unwrap();
    document.body().unwrap().set_inner_html(&format!(
        r#"
        <h1 style="margin:0.5em;color:white;font-family:sans-serif;font-size:15px;">
            Fatal error: {}
            </h1>
    "#,
        info
    ));
    console_error_panic_hook::hook(info);
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);

    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        std::panic::set_hook(Box::new(display_panic));
    });

    let config = WASMLayerConfigBuilder::new()
        .set_max_level(Level::DEBUG)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);
    mount_to_body(|| {
        view! { <Root /> }
    })
}
