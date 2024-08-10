use leptos::*;

include!(concat!(env!("OUT_DIR"), "/help_text.rs"));

#[component]
pub fn Help() -> impl IntoView {
    view! {
        <div id="help" inner_html={HTML_CONTENT} />
    }
}
