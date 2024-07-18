use std::rc::Rc;

use leptos::{
    expect_context,
    on_cleanup,
    provide_context,
    SignalGetUntracked,
};
use wasm_bindgen::prelude::*;

use crate::state::Settings;

#[wasm_bindgen(module = "/public/js/audio.js")]
extern "C" {
    type AudioManager;

    #[wasm_bindgen(constructor)]
    fn new() -> AudioManager;

    #[wasm_bindgen(method)]
    fn start_soundtrack(
        this: &AudioManager,
        file: &str,
        fade: bool,
    );

    #[wasm_bindgen(method)]
    fn stop_soundtrack(this: &AudioManager, fade: bool);

    #[wasm_bindgen(method)]
    fn start_atmosphere(
        this: &AudioManager,
        file: &str,
        fade: bool,
    );

    #[wasm_bindgen(method)]
    fn stop_atmosphere(this: &AudioManager, fade: bool);

    #[wasm_bindgen(method)]
    fn play_one_shot(this: &AudioManager, file: &str);

    #[wasm_bindgen(method)]
    fn mute(this: &AudioManager);

    #[wasm_bindgen(method)]
    fn unmute(this: &AudioManager);
}

pub fn init_audio() {
    let manager = AudioManager::new();
    let (settings, _) = Settings::rw();
    if settings.get_untracked().sound {
        manager.mute();
    }
    provide_context(Rc::new(manager));
}

pub fn play_phase_music(fname: &str, fade: bool) {
    let manager = expect_context::<Rc<AudioManager>>();
    manager.start_soundtrack(fname, fade);
    on_cleanup(|| {
        manager.stop_soundtrack(fade);
    });
}

pub fn play_one_shot(fname: &str) {
    let manager = expect_context::<Rc<AudioManager>>();
    manager.play_one_shot(fname);
}

pub fn play_atmosphere(fname: &str) {
    let manager = expect_context::<Rc<AudioManager>>();
    manager.start_atmosphere(fname, true);
    on_cleanup(|| {
        manager.stop_atmosphere(true);
    });
}
