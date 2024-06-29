use leptos::*;

#[component]
pub fn CardFocusArea() -> impl IntoView {
    view! {
        <div class="card-focus-area wrapper">
            <div class="inner">
                <small class="helper top">"▲"</small>
                <small class="helper bottom">"▼"</small>
            </div>
        </div>
    }
}
