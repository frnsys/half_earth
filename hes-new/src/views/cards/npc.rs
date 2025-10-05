use super::AsCard;
use egui::{Color32, Stroke};
use egui_taffy::{TuiBuilderLogic, taffy};
use hes_engine::NPC;
use rust_i18n::t;

use crate::{
    consts,
    display::{icon_from_slug, icons, speaker_icon},
    views::{
        cards::CardState,
        game::as_speaker,
        parts::{
            flex_justified,
            raised_frame_impl,
            raised_frame_no_shadow_impl,
        },
        tip,
        tips::add_tip,
    },
};

impl AsCard for NPC {
    fn bg_color(&self) -> Color32 {
        Color32::from_rgb(0x72, 0x46, 0x80)
    }

    fn fg_color(&self) -> Color32 {
        Color32::WHITE
    }

    fn header(&self, ui: &mut egui::Ui, ctx: &CardState) {
        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(6, 6))
            .show(ui, |ui| {
                flex_justified(ui, &self.name, |tui| {
                    tui.style(taffy::Style {
                        flex_grow: 1.,
                        ..Default::default()
                    })
                    .label(
                        egui::RichText::new(
                            t!("Parliament").to_uppercase(),
                        )
                        .monospace(),
                    );

                    tui.ui(|ui| {
                        let name = t!(&self.name);
                        let tip = tip(
                            icons::RELATIONSHIP,
                            t!(
                                "Your relationship with %{name}. Increase it by implementing projects they like. At 5 hearts or more they will join your coalition.",
                                name = name
                            ),
                        );
                        add_tip(
                            tip,
                            ui.horizontal(|ui| {
                                ui.style_mut().spacing.item_spacing.x = 1.;
                                for i in 0..consts::MAX_RELATIONSHIP {
                                    let icon = if i as f32 <= self.relationship
                                    {
                                        icons::RELATIONSHIP
                                    } else {
                                        icons::RELATIONSHIP_EMPTY
                                    };
                                    ui.add(icon.size(16.));
                                }
                            })
                            .response,
                        );

                    });
                });
            });
    }

    fn figure(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let speaker = as_speaker(&self.name);
        let portrait = speaker_icon(&speaker);
        ui.horizontal(|ui| {
            ui.add_space(6.);
            raised_frame_no_shadow_impl(
                ui,
                Color32::from_rgb(0x4e, 0x2c, 0x59),
                Color32::from_rgb(0xB0, 0x93, 0xBA),
                |ui| {
                    egui::Frame::NONE
                        .fill(Color32::from_rgb(
                            0x72, 0x46, 0x80,
                        ))
                        .corner_radius(5)
                        .show(ui, |ui| {
                            ui.set_width(
                                ui.available_width() - 6.,
                            );
                            ui.vertical_centered(|ui| {
                                ui.add(
                                    egui::Image::new(portrait)
                                        .fit_to_exact_size(
                                            egui::Vec2::splat(
                                                164.,
                                            ),
                                        ),
                                )
                            });
                        })
                        .response
                },
            );
        });

        // TODO
        // let rel_icon = match self.relationship_name() {
        //     "Ally" => icons::ALLY,
        //     "Friendly" => icons::FRIENDLY,
        //     "Nemesis" => icons::NEMESIS,
        //     "Neutral" => icons::NEUTRAL,
        //     _ => unreachable!(),
        // };
        // let rel_name = t!(&self.relationship_name());
        //
        // ui.image(rel_icon);
        // ui.label(rel_name);
    }

    fn name(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let name = t!(&self.name);
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new(name).heading());
        });
    }

    fn body(&self, ui: &mut egui::Ui, ctx: &CardState) {
        egui::Frame::NONE
            .outer_margin(egui::Margin {
                left: 6,
                right: 6,
                top: 0,
                bottom: 6,
            })
            .inner_margin(egui::Margin::symmetric(8, 8))
            .corner_radius(4)
            .stroke(Stroke::new(
                1.,
                Color32::from_black_alpha(64),
            ))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());
                let effects = t!(&self.flavor.effects);
                if self.is_ally() {
                    ui.label(effects);
                } else {
                    // TODO inactive styling
                    let tip = tip(
                        icons::RELATIONSHIP,
                        t!(
                            "Improve your relationship with %{name} to activate this ability.",
                            name = t!(&self.name)
                        ),
                    );
                    add_tip(tip, {
                        ui.set_opacity(0.35);
                        ui.label(effects)
                    });
                }
            });
    }

    fn top_back(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let speaker = as_speaker(&self.name);
        let portrait = speaker_icon(&speaker);
        ui.image(portrait);

        let desc = t!(&self.flavor.description);
        ui.label(desc);
    }

    fn bottom_back(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let likes = t!(&self.flavor.likes);
        ui.label(t!("Likes"));
        ui.label(likes);

        let dislikes = t!(&self.flavor.dislikes);
        ui.label(t!("Dislikes"));
        ui.label(dislikes);
    }
}
