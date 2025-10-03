use std::sync::LazyLock;

use egui::mutex::RwLock;

pub static DEBUG: LazyLock<RwLock<DebugOpts>> =
    LazyLock::new(|| RwLock::new(DebugOpts::default()));

#[derive(Default, Debug)]
pub struct DebugOpts {
    pub skip_events: bool,
    pub skip_tutorial: bool,
    pub show_all_projects: bool,
    pub show_all_processes: bool,
    pub fast_years: bool,
    pub skip_to_planning: bool,
    pub always_skip_world: bool,
    pub check_events: bool,
    pub very_popular: bool,
    pub no_globe: bool,
    pub no_hector: bool,
}
