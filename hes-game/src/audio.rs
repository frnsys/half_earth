use leptos::{on_cleanup, SignalGetUntracked};
use wasm_bindgen::prelude::*;

use crate::state::Settings;

#[wasm_bindgen(module = "/public/js/audio.js")]
extern "C" {
    type AudioManager;

    fn get_audio_manager() -> AudioManager;

    #[wasm_bindgen(method, js_name = startSoundtrack)]
    fn start_soundtrack(
        this: &AudioManager,
        file: &str,
        fade: bool,
    );

    #[wasm_bindgen(method, js_name = stopSoundtrack)]
    fn stop_soundtrack(this: &AudioManager, fade: bool);

    #[wasm_bindgen(method, js_name = startAtmosphere)]
    fn start_atmosphere(
        this: &AudioManager,
        file: &str,
        fade: bool,
    );

    #[wasm_bindgen(method, js_name = stopAtmosphere)]
    fn stop_atmosphere(this: &AudioManager, fade: bool);

    #[wasm_bindgen(method, js_name = playOneShot)]
    fn play_one_shot(this: &AudioManager, file: &str);

    #[wasm_bindgen(method)]
    fn mute(this: &AudioManager);

    #[wasm_bindgen(method)]
    fn unmute(this: &AudioManager);
}

pub fn init_audio() {
    let (settings, _) = Settings::rw();
    if !settings.get_untracked().sound {
        get_audio_manager().mute();
    }
}

pub fn play_phase_music(fname: &str, fade: bool) {
    let manager = get_audio_manager();
    manager.start_soundtrack(fname, fade);
    on_cleanup(move || {
        manager.stop_soundtrack(fade);
    });
}

pub fn play_one_shot(fname: &str) {
    get_audio_manager().play_one_shot(fname);
}

pub fn play_atmosphere(fname: &str) {
    let manager = get_audio_manager();
    manager.start_atmosphere(fname, true);
    on_cleanup(move || {
        manager.stop_atmosphere(true);
    });
}

pub fn mute() {
    get_audio_manager().mute();
}

pub fn unmute() {
    get_audio_manager().unmute();
}
