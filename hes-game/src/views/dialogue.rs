use std::collections::HashMap;

use hes_engine::events::{Effect, Phase};
use leptos::*;

use super::cards::Image;

#[derive(Clone)]
pub struct Event {
    ref_id: String,
    phase: Phase,
    name: String,
    arc: String,
    image: Image,
    factors: Vec<String>,
    effects: Vec<Effect>,
    pub dialogue: DialogueData,
}

#[derive(Clone)]
pub struct DialogueData {
    root: usize,
    lines: Vec<DialogueLine>,
    effects: Vec<Effect>,
    event_id: Option<usize>,
    region_id: Option<usize>,
}

#[derive(Clone)]
pub struct DialogueLine {
    decision: bool,
    id: usize,
    next: usize,
    speaker: String,
    text: String,
}

#[component]
pub fn Dialogue(
    #[prop(into)] on_advance: Callback<()>,
    #[prop(into)] on_done: Callback<()>,
    #[prop(into)] context: MaybeSignal<HashMap<String, String>>,
    #[prop(into)] dialogue: MaybeSignal<DialogueData>,
) -> impl IntoView {
    // TODO
    todo!()
}
