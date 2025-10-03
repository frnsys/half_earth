use super::AsCard;
use egui::Color32;
use hes_engine::NPC;
use rust_i18n::t;

use crate::{
    consts,
    display::{icon_from_slug, icons, speaker_icon},
    views::{
        cards::CardState,
        game::as_speaker,
        tip,
        tips::add_tip,
    },
};

impl AsCard for NPC {
    fn bg_color(&self) -> Color32 {
        Color32::from_rgb(0x72, 0x46, 0x80)
    }

    fn header(&self, ui: &mut egui::Ui, ctx: &CardState) {
        ui.label(t!("Parliament"));

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
                for i in 0..consts::MAX_RELATIONSHIP {
                    let icon = if i as f32 <= self.relationship
                    {
                        icons::RELATIONSHIP
                    } else {
                        icons::RELATIONSHIP_EMPTY
                    };
                    ui.image(icon);
                }
            })
            .response,
        );
    }

    fn figure(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let speaker = as_speaker(&self.name);
        let portrait = speaker_icon(&speaker);
        ui.image(portrait);
    }

    fn name(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let rel_icon = match self.relationship_name() {
            "Ally" => icons::ALLY,
            "Friendly" => icons::FRIENDLY,
            "Nemesis" => icons::NEMESIS,
            "Neutral" => icons::NEUTRAL,
            _ => unreachable!(),
        };
        let name = t!(&self.name);
        let rel_name = t!(&self.relationship_name());

        ui.image(rel_icon);
        ui.label(rel_name);
        ui.label(name);
    }

    fn body(&self, ui: &mut egui::Ui, ctx: &CardState) {
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
            add_tip(tip, ui.label(effects));
        }
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
