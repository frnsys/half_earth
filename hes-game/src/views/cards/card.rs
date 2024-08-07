use leptos::*;

use crate::util::{scale_text, to_ws_el};
use leptos::wasm_bindgen::JsCast;

#[derive(Clone)]
#[slot]
pub struct Name {
    children: ChildrenFn,
}

#[derive(Clone)]
#[slot]
pub struct Header {
    children: ChildrenFn,
}

#[derive(Clone)]
#[slot]
pub struct Figure {
    children: ChildrenFn,
}

#[derive(Clone)]
#[slot]
pub struct Body {
    children: ChildrenFn,
}

#[derive(Clone)]
#[slot]
pub struct TopBack {
    children: ChildrenFn,
}

#[derive(Clone)]
#[slot]
pub struct BottomBack {
    children: ChildrenFn,
}

#[derive(Clone)]
#[slot]
pub struct ProcessMix {
    children: ChildrenFn,
}

#[component]
pub fn Card(
    body: Body,
    #[prop(into, optional, default = "#fff".into())]
    color: MaybeSignal<&'static str>,
    #[prop(into, optional, default = "#222".into())] background: MaybeSignal<&'static str>,
    #[prop(optional)] top_back: Option<TopBack>,
    #[prop(optional)] bottom_back: Option<BottomBack>,
    #[prop(optional)] name: Option<Name>,
    #[prop(optional)] header: Option<Header>,
    #[prop(optional)] figure: Option<Figure>,
    #[prop(optional)] process_mix: Option<ProcessMix>,
    #[prop(optional, into)] class: MaybeSignal<String>,
) -> impl IntoView {
    let (flipped, set_flipped) = create_signal(false);

    let flippable = top_back.is_some() || bottom_back.is_some();
    let flip = move |_| {
        if flippable {
            set_flipped.set(!flipped.get());
        }
    };
    let is_process = process_mix.is_some();

    let card_ref = create_node_ref::<html::Div>();
    let name_ref = create_node_ref::<html::Div>();
    let body_ref = create_node_ref::<html::Div>();

    // Fit texts
    create_effect(move |_| {
        if !is_process && let Some(name) = name_ref.get() {
            scale_text(to_ws_el(name), 16);
        }

        // Can't target the body as a whole,
        // mainly because the card body can contain
        // more than just a list of effects, and if it does,
        // things break. E.g. if a project is Building/Researching,
        // the HTML that includes the tag indicating that breaks everything
        if let Some(body) = body_ref.get() {
            if let Some(effects) = body
                .query_selector(".solo-effects")
                .expect("Valid selector")
            {
                let effects = effects
                    .dyn_into::<web_sys::HtmlElement>()
                    .expect("We know this is an HTML element");
                scale_text(effects, 7);
            }
        }

        if let Some(card) = card_ref.get() {
            if let Some(desc) = card
                .query_selector(".card-desc")
                .expect("Valid selector")
            {
                let desc = desc
                    .dyn_into::<web_sys::HtmlElement>()
                    .expect("We know this is an HTML element");
                scale_text(desc, 11);
            }
        }
    });

    view! {
        <div
            class=move || format!("card {}", class.get())
            ref=card_ref
            on:click=flip
            class:flipped=move || flipped.get()
            class:process=is_process
        >
            <div class="card-front">
                {header
                    .clone()
                    .zip(figure)
                    .map(|(header, figure)| {
                        view! {
                            <div
                                class="card-top"
                                style:background=background
                                style:color=color
                            >
                                <header style:color=color>
                                    {(header.children)().into_view()}
                                </header>
                                <figure>{(figure.children)().into_view()}</figure>
                            </div>
                        }
                    })}
                {name
                    .map(|name| {
                        view! {
                            <div
                                class="card-mid card--name"
                                style:background=background
                                style:color=color
                            >
                                <div class="name-wrapper" ref=name_ref>
                                    {(name.children)().into_view()}
                                </div>
                            </div>
                        }
                    })}
                <div
                    class="card-bot"
                    style:background=background
                    style:color=color
                >
                    <div class="card--body" style:color=color ref=body_ref>
                        {(body.children)().into_view()}
                    </div>
                </div>
            </div>

            {top_back
                .zip(bottom_back)
                .map(|(top, bottom)| {
                    view! {
                        <div class="card-back">
                            <div
                                class="card-top"
                                style:background=background
                                style:color=color
                            >
                                {header
                                    .map(|header| {
                                        Some(
                                            view! {
                                                <header style:color=color>
                                                    {(header.children)().into_view()}
                                                </header>
                                            },
                                        )
                                    })}

                                <div
                                    class="card-top-back"
                                    style:background=background
                                    style:color=color
                                >
                                    {(top.children)().into_view()}
                                </div>
                            </div>
                            <div
                                class="card-mid card--name"
                                style:background=background
                                style:color=color
                            >
                                <div></div>
                            </div>
                            <div
                                class="card-bot"
                                style:background=background
                                style:color=color
                            >
                                <div class="card-bot-back">
                                    {(bottom.children)().into_view()}
                                </div>
                            </div>
                        </div>
                    }
                })}

            {process_mix
                .map(|mix| {
                    view! {
                        <div class="process-mix-bar">
                            {(mix.children)().into_view()}
                        </div>
                    }
                })}

        </div>
    }
}
