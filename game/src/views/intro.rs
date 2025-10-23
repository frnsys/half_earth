use hes_engine::{EventPhase, State};
use rust_i18n::t;

use crate::{
    audio,
    parts::{button, set_full_bg_image},
    state::StateExt,
    views::events::{EventResult, Events},
};

const IMAGES: &[(&str, f32, f32)] = &[
    ("pexels-lt-chan-2833366.webp", 1800., 1200.),
    ("gosplant_world.webp", 1800., 900.),
    ("gosplant_world.webp", 1800., 900.),
    ("gosplant_world.webp", 1800., 900.),
    ("pexels-marco-allasio-4275996.webp", 1200., 1800.),
    ("pexels-mentatdgt-1185433.webp", 1800., 1200.),
    ("hasan-almasi-OwqLxCvoVxI-unsplash.webp", 1800., 1200.),
    (
        "matthew-tenbruggencate-0HJWobhGhJs-unsplash.webp",
        1800.,
        1200.,
    ),
    ("hasan-almasi-OwqLxCvoVxI-unsplash.webp", 1800., 1200.),
    ("kelly-sikkema-VpcSDucAYjw-unsplash.webp", 1200., 1800.),
];

pub struct Intro {
    img_idx: usize,
    events: Events,
}
impl Intro {
    pub fn new(state: &mut State) -> Self {
        let events = StateExt::roll_events(state, EventPhase::CutsceneIntro);

        audio::soundtrack(audio::Track::Intro);

        Self {
            img_idx: 0,
            events: Events::new(events, state),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut State) -> bool {
        if let Some((img, x, y)) = IMAGES.get(self.img_idx) {
            set_full_bg_image(ui, hes_images::intro_image(img), egui::vec2(*x, *y));
        }

        let result = self.events.render(ui, state);
        if result == Some(EventResult::Advanced) {
            self.img_idx += 1;
        }
        if self.events.is_finished {
            return true;
        }

        egui::Area::new(egui::Id::new("cutscene-skip"))
            .order(egui::Order::Tooltip)
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-10., -10.))
            .show(ui.ctx(), |ui| ui.add(button(t!("Skip"))).clicked())
            .inner
    }
}
