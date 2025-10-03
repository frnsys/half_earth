use hes_engine::{EventPhase, State};
use rust_i18n::t;

use crate::{
    display::DisplayEvent,
    image,
    views::{events::Events, parts::set_full_bg_image},
};

const IMAGES: &[(egui::ImageSource<'static>, f32, f32)] = &[
    (image!("intro/pexels-lt-chan-2833366.jpg"), 1800., 1200.),
    (image!("intro/gosplant_world.jpg"), 1800., 900.),
    (image!("intro/gosplant_world.jpg"), 1800., 900.),
    (image!("intro/gosplant_world.jpg"), 1800., 900.),
    (
        image!("intro/pexels-marco-allasio-4275996.jpg"),
        1200.,
        1800.,
    ),
    (
        image!("intro/pexels-mentatdgt-1185433.jpg"),
        1800.,
        1200.,
    ),
    (
        image!("intro/hasan-almasi-OwqLxCvoVxI-unsplash.jpg"),
        1800.,
        1200.,
    ),
    (
        image!(
            "intro/matthew-tenbruggencate-0HJWobhGhJs-unsplash.jpg"
        ),
        1800.,
        1200.,
    ),
    (
        image!("intro/hasan-almasi-OwqLxCvoVxI-unsplash.jpg"),
        1800.,
        1200.,
    ),
    (
        image!("intro/kelly-sikkema-VpcSDucAYjw-unsplash.jpg"),
        1200.,
        1800.,
    ),
];

pub struct Intro {
    img_idx: usize,
    events: Events,
}
impl Intro {
    pub fn new(state: &mut State) -> Self {
        let events = state
            .roll_events(EventPhase::CutsceneIntro)
            .into_iter()
            .map(|ev| DisplayEvent::new(ev, state))
            .collect();
        // TODO this deadlocks, we should use a queue
        // let sound_data = audio!("music/intro.mp3");
        // let _ = AUDIO.write().play(sound_data);
        Self {
            img_idx: 0,
            events: Events::new(events),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
    ) -> bool {
        if let Some((img, x, y)) = IMAGES.get(self.img_idx) {
            set_full_bg_image(
                ui,
                img.clone(),
                egui::vec2(*x, *y),
            );
        }

        self.events.render(ui, state);
        if self.events.is_finished {
            // TODO fade out
            return true;
        }

        egui::Area::new(egui::Id::new("cutscene-skip"))
            .anchor(
                egui::Align2::RIGHT_BOTTOM,
                egui::Vec2::new(-10., -10.),
            )
            .show(ui.ctx(), |ui| {
                if ui.button(t!("Skip")).clicked() {
                    // TODO fade out
                    true
                } else {
                    false
                }
            })
            .inner
    }
}
