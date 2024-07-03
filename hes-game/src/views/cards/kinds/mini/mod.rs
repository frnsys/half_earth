mod npc;
mod process;
mod project;
mod region;

pub use npc::MiniNPC;
pub use process::MiniProcess;
pub use project::MiniProject;
pub use region::MiniRegion;

use leptos::{ev::MouseEvent, *};
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

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
    let (expanded, set_expanded) = create_signal(false);
    let overlay_ref = create_node_ref::<html::Div>();
    let expand = move |_| {
        set_expanded.set(true);
    };
    let collapse = move |ev: MouseEvent| {
        if let Some(target) = ev.target() {
            if let Some(elem) = target.dyn_ref::<HtmlDivElement>() {
                if let Some(overlay) = overlay_ref.get() {
                    if let Some(overlay_elem) = overlay.dyn_ref::<HtmlDivElement>() {
                        if elem == overlay_elem {
                            set_expanded.set(false);
                        }
                    }
                }
            }
        }
    };

    view! {
        <div
            class="minicard"
            style:border=border
            class=class
            on:click=expand
        >
            (body.children)().into_view()
        </div>
        <AnimatedShow
            when=expanded
            show_class="opacityfade-enter-active"
            hide_class="opacityfade-leave-to"
            hide_delay=Duration::from_millis(1000)
        >
            <div
                class="minicard--expanded"
                on:click=collapse
                ref=overlay_ref
            >
                <AnimatedShow
                    when=expanded
                    show_class="appear-bounceup-enter-active"
                    hide_class="appear-bounceup-leave-active"
                    hide_delay=Duration::from_millis(1000)
                >
                    <div>(expanded.children)().into_view()</div>
                </AnimatedShow>
            </div>
        </AnimatedShow>
    }
}
