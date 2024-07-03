use std::rc::Rc;

use leptos::{provide_context, SignalGet};
use wasm_bindgen::prelude::*;

use crate::state::Settings;

#[wasm_bindgen(module = "/public/js/audio.js")]
extern "C" {
    type AudioManager;

    #[wasm_bindgen(constructor)]
    fn new() -> AudioManager;

    #[wasm_bindgen(method)]
    fn start_soundtrack(this: &AudioManager, file: &str, fade: bool);

    #[wasm_bindgen(method)]
    fn stop_soundtrack(this: &AudioManager, fade: bool);

    #[wasm_bindgen(method)]
    fn start_atmosphere(this: &AudioManager, file: &str, fade: bool);

    #[wasm_bindgen(method)]
    fn stop_atmosphere(this: &AudioManager, fade: bool);

    #[wasm_bindgen(method)]
    fn play_one_shot(this: &AudioManager);

    #[wasm_bindgen(method)]
    fn mute(this: &AudioManager);

    #[wasm_bindgen(method)]
    fn unmute(this: &AudioManager);
}

pub fn init_audio() {
    let manager = AudioManager::new();
    let (settings, _) = Settings::get();
    if settings.get().sound {
        manager.mute();
    }
    provide_context(Rc::new(manager));
}
