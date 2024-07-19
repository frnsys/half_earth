use std::{collections::HashMap, sync::LazyLock};

use serde::{Deserialize, Serialize};

pub static ICON_EVENTS: LazyLock<HashMap<usize, IconEvent>> =
    LazyLock::new(|| {
        let icon_event_data =
            include_str!("../../assets/icon_events.json");
        serde_json::from_str(icon_event_data).unwrap()
    });

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconEvent {
    pub name: String,
    pub icon: String,
    pub intensity: usize,
}
impl IconEvent {
    pub fn is_over_water(&self) -> bool {
        self.name == "Severe Hurricane"
    }
}
