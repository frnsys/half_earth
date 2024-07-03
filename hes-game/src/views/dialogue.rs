use std::{collections::HashMap, rc::Rc};

use crate::t;
use crate::views::effects::EffectThing;
use crate::views::Effects;
use ev::{KeyboardEvent, MouseEvent};
use hes_engine::{
    events::{Effect, Phase},
    flavor::{self, Branch, DialogueNext},
};
use leptos::*;
use leptos_dom::helpers::IntervalHandle;
use leptos_use::{use_document, use_event_listener};
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

#[component]
pub fn Dialogue(
    #[prop(into)] on_done: Callback<()>,
    #[prop(into, optional)] context: Signal<HashMap<String, String>>,
    #[prop(into)] dialogue: Signal<flavor::Dialogue>,
    #[prop(into, optional, default=(|_| {}).into())] on_advance: Callback<()>,
    #[prop(into, optional, default=(|_| {}).into())] on_start: Callback<()>,
    #[prop(into, optional)] effects: Signal<Option<Vec<EffectThing>>>,
    #[prop(into, optional)] event_id: Signal<Option<usize>>,
    #[prop(into, optional)] region_id: Signal<Option<usize>>,
) -> impl IntoView {
    let (revealed, set_revealed) = create_signal(false);
    let (stop_anim, set_stop_anim) = create_signal::<Option<Rc<dyn Fn() + 'static>>>(None);
    let text_ref = create_node_ref::<html::Div>();

    let (current, set_current) = create_signal(dialogue.get().root);
    let line = move || dialogue.get().lines[current.get()].clone();

    let text = move || {
        let line = line();
        if context.get().is_empty() {
            t!(&line.text)
        } else {
            // TODO fill in variables and icons
            // parse_text = return display.fillIcons(display.fillVars(text, context));
            // parse_text(t!(&line.text), context.get())
            t!(&line.text)
        }
    };

    let play = move || {
        if let Some(text_ref) = text_ref.get() {
            set_revealed.set(false);
            let on_reveal_start = Closure::wrap(Box::new(move |start_callback: JsValue| {
                let start_callback: js_sys::Function = start_callback.into();
                let stop_anim = move || {
                    start_callback.call0(&JsValue::NULL).unwrap();
                };
                set_stop_anim.set(Some(Rc::new(stop_anim)));
            }) as Box<dyn FnMut(JsValue)>);
            let on_reveal_finish = Closure::wrap(Box::new(move || {
                set_revealed.set(true);
            }) as Box<dyn FnMut()>);

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

    create_effect(move |_| {
        let dialogue = dialogue.get();
        set_revealed.set(false);
        set_current.set(dialogue.root);
        on_start.call(());
        // TODO emit started
        // next tick:
        play();
    });

    let profile = move || format!("/public/assets/characters/{}.webp", line().speaker);
    let is_last_line = move || line().next.is_none();
    let has_decision = move || line().has_decision();

    let end = move || {
        if let Some(stop_anim) = stop_anim.get() {
            stop_anim();
            set_revealed.set(false);
            // TODO emit done
        }
    };

    let next_line = move || {
        let line = line();
        let mut can_advance = false;
        if let Some(next) = line.next {
            match next {
                DialogueNext::Line { id } => {
                    set_current.set(id);
                }
                DialogueNext::Branches(branches) => {
                    if let Some(event_id) = event_id.get() {
                        let state = expect_context::<RwSignal<crate::state::GameState>>();
                        let branch = state.with(|state| {
                            branches.iter().find(|b| {
                                state
                                    .game
                                    .eval_branch_conditions(event_id, region_id.get(), b.id)
                            })
                        });
                        if let Some(branch) = branch {
                            if let Some(line_id) = branch.line_id {
                                set_current.set(line_id);
                            }
                        }
                    }
                }
            }
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
                text_ref.inner_html(text());
                set_revealed.set(true);
            }
        }
    };
    let advance = move || {
        if revealed.get() && !is_last_line() && !has_decision() {
            next_line();
            // emit(advanced)
        } else {
            skip_reveal();
        }
    };

    let select_choice = move |ev: MouseEvent, branch: &Branch| {
        ev.stop_immediate_propagation();
        let state = expect_context::<RwSignal<crate::state::GameState>>();

        // this.eventID will be undefined
        // for project outcome dialogues.
        // The whole dialogue system was really written with
        // events in mind; it'd be a pretty big rewrite to
        // fully support project dialogues with branch effects.
        // So we just assume project dialogues won't have branch effects
        // which, at time of writing, none of them do.
        if let Some(event_id) = event_id.get() {
            state.update(|state| {
                state
                    .game
                    .apply_branch_effects(event_id, region_id.get(), branch.id);
            });
            if let Some(line_id) = branch.line_id {
                set_current.set(line_id);
                play();
            } else {
                end();
            }
        }
    };

    use_event_listener(use_document(), ev::keydown, move |ev| {
        if ev.key() == "Enter" {
            if is_last_line() {
                end();
            } else if !has_decision() {
                advance();
            }
        }
    });

    let speaker = move || line().speaker;

    let actions = move || {
        if is_last_line() {
            view! {
                <div class="dialogue--choice" on:click=move |_| end()>
                    ({t!("Continue")})
                </div>
            }
            .into_view()
        } else if let Some(DialogueNext::Branches(branches)) = line().next {
            branches
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
                    ({t!("Next")})
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
                    <Show when=move || line().speaker != "[GAME]">
                        <div class="dialogue--speaker">
                            <img src=profile/>
                        </div>
                    </Show>
                    <div class="dialogue--body" on:click=move |_| advance()>
                        <Show when=move || line().speaker != "[GAME]">
                            <div class="dialogue--speaker-name">
                                {move || t!(& line().speaker)}
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
