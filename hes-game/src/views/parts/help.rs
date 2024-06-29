use crate::{state::Settings, t};
use leptos::*;

/// A tutorial help tip.
#[component]
pub fn Help(text: String, x: f32, y: f32, center: bool) -> impl IntoView {
    let text = store_value(text);
    let (settings, set_settings) = Settings::get();
    let show = move || {
        let settings = settings.get();
        !settings.hide_help && settings.read_help.contains(&text.get_value())
    };

    view! {
        <Show when=show>
            <div
                class="help-tip--outer"
                style:left=format!("{x}px")
                style:top=format!("{y}px")
                on:click=move |_| {
                    set_settings
                        .update(|settings| {
                            settings.read_help.push(text.get_value());
                        });
                }
            >
                <div class="help-tip--inner" class:center=center>
                    <img src="/public/assets/icons/help.svg"/>
                    {t!(& text.get_value())}
                </div>
            </div>
        </Show>
    }
}
