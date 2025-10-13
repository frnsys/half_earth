use egui_commonmark::{CommonMarkCache, commonmark_str};

use crate::parts::h_center;

pub fn help(ui: &mut egui::Ui) {
    h_center(ui, "help", |ui| {
        ui.vertical(|ui| {
            ui.set_max_width(640.);
            let mut cache = CommonMarkCache::default();
            commonmark_str!(ui, &mut cache, "editor/help.md");
        });
    });
    ui.add_space(64.);
}
