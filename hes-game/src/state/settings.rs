use super::Tutorial;
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

const SETTINGS_KEY: &str = "hes.settings";

/// Settings that persist across sessions.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub read_help: Vec<String>,
    pub hide_help: bool,
    pub sound: bool,
    pub runs_played: usize,
    pub tutorial: Tutorial,
    pub language: String,
}
impl Settings {
    pub fn rw() -> (Signal<Settings>, WriteSignal<Settings>) {
        let (read, write, _) = use_local_storage::<
            Settings,
            JsonSerdeCodec,
        >(SETTINGS_KEY);
        (read, write)
    }
}
