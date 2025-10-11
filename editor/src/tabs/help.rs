use egui_commonmark::{CommonMarkCache, commonmark_str};

pub fn help(ui: &mut egui::Ui) {
    let mut cache = CommonMarkCache::default();
    commonmark_str!(ui, &mut cache, "editor/help.md");
}
