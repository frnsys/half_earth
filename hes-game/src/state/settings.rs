use super::Tutorial;
use leptos::*;
use leptos_use::{
    storage::use_local_storage,
    utils::JsonCodec,
};
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
}
impl Settings {
    pub fn rw() -> (Signal<Settings>, WriteSignal<Settings>) {
        let (read, write, _) = use_local_storage::<
            Settings,
            JsonCodec,
        >(SETTINGS_KEY);
        (read, write)
    }
}
