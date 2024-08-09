mod npc;
mod process;
mod project;

pub use npc::MiniNPC;
pub use process::MiniProcess;
pub use project::MiniProject;

use leptos::{ev::MouseEvent, *};
use std::time::Duration;

#[derive(Clone)]
#[slot]
pub struct Body {
    children: ChildrenFn,
}

#[derive(Clone)]
#[slot]
pub struct Expanded {
    children: ChildrenFn,
}

#[component]
pub fn MiniCard(
    body: Body,
    expanded: Expanded,
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(optional, into)] border: MaybeSignal<String>,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) = create_signal(false);
    let expand = move |_| {
        set_is_expanded.set(true);
    };
    let collapse = move |ev: MouseEvent| {
        let target: web_sys::HtmlElement = event_target(&ev);
        let class = target.class_name();
        if class == "cards" || class == "minicard--expanded" {
            set_is_expanded.set(false);
        }
    };

    // So we can copy the children.
    let expanded = store_value(expanded);

    view! {
        <div
            class=format!("minicard {}", class.get())
            style:border=border
            on:click=expand
        >
            {(body.children)().into_view()}
        </div>
        <AnimatedShow
            when=is_expanded
            show_class="opacityfade-enter-active"
            hide_class="opacityfade-leave-to"
            hide_delay=Duration::from_millis(1000)
        >
            <div
                class="minicard--expanded"
                on:click=collapse
            >
                <AnimatedShow
                    when=is_expanded
                    show_class="appear-bounceup-enter-active"
                    hide_class="appear-bounceup-leave-active"
                    hide_delay=Duration::from_millis(1000)
                >
                    <div class="expanded-content">{(expanded.get_value().children)().into_view()}</div>
                </AnimatedShow>
            </div>
        </AnimatedShow>
    }
}
