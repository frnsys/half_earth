use leptos::*;

/// SSR compatibe `is_supported`
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_supported, js};
/// # use wasm_bindgen::JsValue;
/// #
/// # pub fn Demo() -> impl IntoView {
/// let is_supported = use_supported(
///     || js!("getBattery" in &window().navigator())
/// );
///
/// if is_supported.get() {
///     // do something
/// }
/// #    view! { }
/// # }
/// ```
pub fn use_supported(callback: impl Fn() -> bool + 'static) -> Signal<bool> {
    #[cfg(feature = "ssr")]
    {
        let _ = callback;
        Signal::derive(|| false)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Signal::derive(callback)
    }
}
