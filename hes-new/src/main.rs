use hes_new::App;

#[cfg(not(target_arch = "wasm32"))]
fn run_gui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([350.0, 590.0]),
        ..Default::default()
    };
    eframe::run_native(
        "test",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn run_gui() -> eframe::Result {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect(
                "the_canvas_id was not a HtmlCanvasElement",
            );

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(App::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) =
            document.get_element_by_id("loading_text")
        {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
    Ok(())
}

fn main() {
    use sys_locale::get_locale;
    let locale =
        get_locale().unwrap_or_else(|| String::from("en-US"));
    rust_i18n::set_locale(&locale);
    run_gui().unwrap();
}
