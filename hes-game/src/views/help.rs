use crate::{state::Settings, t};
use leptos::*;

/// A tutorial help tip.
#[component]
pub fn Help(
    text: String,
    x: f32,
    y: f32,
    center: bool,
) -> impl IntoView {
    let (settings, set_settings) = Settings::get();
    let text = store_value(text);
    let show = move || {
        settings.with(|s| {
            !s.hide_help
                && s.read_help.contains(&text.get_value())
        })
    };

    let left = move || {
        if x < 1. {
            format!("{}%", x * 100.)
        } else {
            format!("{x}px")
        }
    };
    let top = move || {
        if y < 1. {
            format!("{}%", y * 100.)
        } else {
            format!("{y}px")
        }
    };

    view! {
        <Show when=show>
            <div
                class="help-tip--outer"
                style:left=left
                style:top=top
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
