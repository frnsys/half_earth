use std::{collections::BTreeMap, rc::Rc};

use crate::{
    icons::fill_icons,
    t,
    views::{effects::DisplayEffect, Effects},
};
use ev::MouseEvent;
use hes_engine::{flavor::*, Id, State};
use leptos::*;
use leptos_use::{use_document, use_event_listener};
use regex_lite::Regex;
use wasm_bindgen::prelude::*;

// The dialogue animation functionality is easier
// to just handle directly in javascript.
#[wasm_bindgen(module = "/public/js/dialogue.js")]
extern "C" {
    fn playText(
        textEl: &web_sys::HtmlElement,
        text: &str,
        onStart: &js_sys::Function,
        onFinish: &js_sys::Function,
    );
}

fn fill_vars(
    text: &str,
    context: &BTreeMap<String, String>,
) -> String {
    let re = Regex::new(r"\{([a-z_]+)\}").unwrap();
    let mut result = String::from(text);
    for caps in re.captures_iter(text) {
        if let Some(var) = caps.get(1) {
            if let Some(replacement) = context.get(var.as_str())
            {
                result = result.replace(&caps[0], replacement);
            }
        }
    }

    result
}

#[component]
pub fn Dialogue(
    #[prop(into)] on_done: Callback<()>,
    #[prop(into, optional)] context: Signal<
        BTreeMap<String, String>,
    >,
    #[prop(into)] dialogue: Signal<Dialogue>,
    #[prop(into, optional, default=(|_| {}).into())] on_advance: Callback<()>,
    #[prop(into, optional, default=(|_| {}).into())]
    on_start: Callback<()>,
    #[prop(into, optional)] effects: Signal<
        Option<Vec<DisplayEffect>>,
    >,
    #[prop(into, optional)] event_id: Signal<Option<Id>>,
    #[prop(into, optional)] region_id: Signal<Option<Id>>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();

    let (revealed, set_revealed) = create_signal(false);
    let (stop_anim, set_stop_anim) =
        create_signal::<Option<Rc<dyn Fn() + 'static>>>(None);
    let text_ref = create_node_ref::<html::Div>();

    // If no lines in the dialogue,
    // we'll just present the "Continue" button.
    let start_line = {
        let dialogue = dialogue.get();
        dialogue
            .lines
            .get(dialogue.root)
            .unwrap_or(&DialogueLine::default())
            .clone()
    };
    let (line, set_line) = create_signal(start_line);

    let text = move || {
        let line = line.get();
        with!(|context| {
            if context.is_empty() {
                fill_icons(&t!(&line.text))
            } else {
                fill_icons(&fill_vars(&t!(&line.text), context))
            }
        })
    };

    let play = move || {
        if let Some(text_ref) = text_ref.get() {
            set_revealed.set(false);
            let on_reveal_start = Closure::wrap(Box::new(
                move |start_callback: JsValue| {
                    let start_callback: js_sys::Function =
                        start_callback.into();
                    let stop_anim = move || {
                        start_callback
                            .call0(&JsValue::NULL)
                            .unwrap();
                    };
                    set_stop_anim.set(Some(Rc::new(stop_anim)));
                },
            )
                as Box<dyn FnMut(JsValue)>);
            let on_reveal_finish =
                Closure::wrap(Box::new(move || {
                    set_revealed.set(true);
                })
                    as Box<dyn FnMut()>);

            // Call the JavaScript function
            playText(
                &text_ref,
                &text(),
                on_reveal_start.as_ref().unchecked_ref(),
                on_reveal_finish.as_ref().unchecked_ref(),
            );

            // Keep the closures alive
            on_reveal_start.forget();
            on_reveal_finish.forget();
        }
    };

    create_effect(move |last| {
        // Only run on mount.
        if last.is_some() {
            return;
        }

        set_revealed.set(false);
        on_start.call(());
        play();
    });

    let profile = move || {
        format!(
            "/assets/characters/{}.webp",
            line.get().speaker
        )
    };
    let is_last_line = move || line.get().next.is_none();
    let has_decision = move || line.get().has_decision();

    let end = move || {
        if let Some(stop_anim) = stop_anim.get() {
            stop_anim();
            set_revealed.set(false);
        }
        on_done.call(());
    };

    let next_line = move || {
        let line = line.get();
        let mut can_advance = false;
        if let Some(next) = line.next {
            match next {
                DialogueNext::Line { id } => {
                    let dialogue = dialogue.get();
                    let line = dialogue.lines[id].clone();
                    set_line.set(line);
                }
                DialogueNext::Responses(responses) => {
                    if event_id.get().is_some() {
                        let branch =
                            game.with_untracked(|game| {
                                responses.iter().find(|b| {
                                    game.eval_conditions(
                                        &b.conditions,
                                        region_id.get(),
                                    )
                                })
                            });
                        if let Some(branch) = branch {
                            if let Some(line_id) =
                                branch.next_line
                            {
                                let dialogue = dialogue.get();
                                let line = dialogue.lines
                                    [line_id]
                                    .clone();
                                set_line.set(line);
                            }
                        }
                    }
                }
            }
            can_advance = true;
        }
        if can_advance {
            play();
        } else {
            end();
        }
    };

    let skip_reveal = move || {
        if let Some(stop_anim) = stop_anim.get() {
            stop_anim();
            if let Some(text_ref) = text_ref.get() {
                let _ = text_ref.inner_html(text());
                set_revealed.set(true);
            }
        }
    };
    let advance = move || {
        if revealed.get() && !is_last_line() && !has_decision()
        {
            next_line();
            on_advance.call(());
        } else {
            skip_reveal();
        }
    };

    let select_choice =
        move |ev: MouseEvent, response: &Response| {
            ev.stop_immediate_propagation();
            // this.eventID will be undefined
            // for project outcome dialogues.
            // The whole dialogue system was really written with
            // events in mind; it'd be a pretty big rewrite to
            // fully support project dialogues with branch effects.
            // So we just assume project dialogues won't have branch effects
            // which, at time of writing, none of them do.
            if event_id.get().is_some() {
                update!(|game| {
                    game.apply_effects(
                        &response.effects,
                        region_id.get(),
                    );
                });
            }

            if let Some(line_id) = response.next_line {
                let dialogue = dialogue.get();
                let line = dialogue.lines[line_id].clone();
                set_line.set(line);
                play();
            } else {
                end();
            }
        };

    let _ = use_event_listener(
        use_document(),
        ev::keydown,
        move |ev| {
            if ev.key() == "Enter" {
                if is_last_line() {
                    end();
                } else if !has_decision() {
                    advance();
                }
            }
        },
    );

    let speaker = move || with!(|line| line.speaker.clone());

    let actions = move || {
        if is_last_line() {
            view! {
                <div class="dialogue--choice" on:click=move |_| end()>
                    {t!("Continue")}
                </div>
            }
            .into_view()
        } else if let Some(DialogueNext::Responses(responses)) =
            line.get().next
        {
            responses
                .iter()
                .cloned()
                .map(|branch| {
                    let (sig, _) = create_signal(branch);
                    view! {
                        <div
                            class="dialogue--choice"
                            on:click=move |ev| select_choice(ev, &sig.get())
                        >
                            {move || t!(& sig.get().text)}
                        </div>
                    }
                })
                .collect::<Vec<_>>()
                .into_view()
        } else {
            view! {
                <div class="dialogue--choice" on:click=move |_| advance()>
                    {t!("Next")}
                </div>
            }
            .into_view()
        }
    };
    let dialogue_effects = move || {
        effects.get().map(|effects| {
            let (sig, _) = create_signal(effects);
            view! {
                <div class="dialogue--effects">
                    <Effects effects=sig/>
                </div>
            }
        })
    };

    view! {
        <div class="dialogue">
            <div class="dialogue--inner">
                <div class="dialogue--speech">
                    <Show when=move || speaker() != Speaker::Game>
                        <div class="dialogue--speaker">
                            <img src=profile/>
                        </div>
                    </Show>
                    <div class="dialogue--body" on:click=move |_| advance() class:hidden=move || line.get().text.is_empty()>
                        <Show when=move || speaker() != Speaker::Game>
                            <div class="dialogue--speaker-name">
                                {move || t!(& speaker().to_string())}
                            </div>
                        </Show>
                        <div class="dialogue--text" ref=text_ref></div>
                        <Show when=move || {
                            revealed.get() && is_last_line()
                        }>{dialogue_effects}</Show>
                    </div>
                </div>
                <div class="dialogue--choices">
                    <Show when=move || revealed.get()>{actions}</Show>
                </div>
            </div>
        </div>
    }
}
