use super::ui::Tutorial;
use serde::{Deserialize, Serialize};

/// Settings that persist across sessions.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub sound: bool,
    pub runs_played: usize,
    pub tutorial: Tutorial,
    pub language: String,
}
