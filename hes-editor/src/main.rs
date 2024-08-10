use hes_editor::App;
use leptos::*;
use tracing::Level;
use tracing_wasm::WASMLayerConfigBuilder;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let config = WASMLayerConfigBuilder::new()
        .set_max_level(Level::DEBUG)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
