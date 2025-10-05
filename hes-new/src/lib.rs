mod consts;
mod debug;
mod display;
mod state;
mod style;
mod text;
mod vars;
mod views;

use std::sync::{Arc, LazyLock};

use egui::{Align2, Key, mutex::RwLock};
use hes_engine::{Process, Project};
use kira::{
    AudioManager,
    AudioManagerSettings,
    DefaultBackend,
};

use debug::DEBUG;
use state::{State, UIState};
use views::{Start, StartAction};

use crate::views::{
    CardState,
    draw_bg_image,
    game::{Card, Cards},
};

pub static AUDIO: LazyLock<Arc<RwLock<AudioManager>>> =
    LazyLock::new(|| {
        let manager = AudioManager::<DefaultBackend>::new(
            AudioManagerSettings::default(),
        )
        .unwrap();
        Arc::new(RwLock::new(manager))
    });

rust_i18n::i18n!("locales", fallback = "en");

#[macro_export]
macro_rules! image {
    ($path:literal) => {
        egui::include_image!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/images/",
            $path
        ))
    };
}

#[macro_export]
macro_rules! audio {
    ($path:literal) => {
        kira::sound::static_sound::StaticSoundData::from_cursor(
            std::io::Cursor::new(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/sounds/",
                $path
            ))),
        )
        .unwrap()
    };
}

enum ViewState {
    Start(Start),
    Game(UIState),
}

pub struct App {
    ui: ViewState,
    state: State,

    // TODO temp
    // cards: Vec<Card<Project>>,
    cards: Vec<Card<Process>>,
}
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        style::configure_style(&cc.egui_ctx);

        let state = if let Some(storage) = cc.storage
            && let Some(state) =
                eframe::get_value(storage, eframe::APP_KEY)
        {
            state
        } else {
            State::default()
        };

        // let cards = state
        //     .game
        //     .world
        //     .projects
        //     .iter()
        //     .map(|p| Card::new(p.clone()))
        //     .collect();

        let cards = state
            .game
            .world
            .processes
            .iter()
            .map(|p| Card::new(p.clone()))
            .collect();

        Self {
            ui: ViewState::Start(Start::default()),
            state,
            cards,
        }
    }
}

struct FadeIn {
    started: bool,
}
impl FadeIn {
    fn advance(
        &mut self,
        ui: &mut egui::Ui,
        duration: f32,
        contents: impl FnOnce(&mut egui::Ui),
    ) -> bool {
        let target = if self.started {
            1.0
        } else {
            self.started = true;
            0.0
        };

        let easing: fn(f32) -> f32 =
            egui_animation::easing::cubic_in_out;
        let value = egui_animation::animate_eased(
            ui.ctx(),
            "fade-in",
            target,
            duration,
            easing,
        );
        ui.scope(|ui| {
            ui.set_opacity(value);
            contents(ui);
        });

        value == 1.0
    }
}

struct CardIn {
    started: bool,
}
impl CardIn {
    fn advance(
        &mut self,
        ui: &mut egui::Ui,
        contents: impl FnOnce(&mut egui::Ui),
    ) -> bool {
        let target = if self.started {
            300.0
        } else {
            self.started = true;
            0.0
        };

        let easing: fn(f32) -> f32 =
            egui_animation::easing::quart_out;
        let y = egui_animation::animate_eased(
            ui.ctx(),
            "card-in",
            target,
            1.,
            easing,
        );
        egui::Area::new("card-overlay".into())
            .anchor(Align2::CENTER_BOTTOM, egui::vec2(0., -y))
            .show(ui.ctx(), |ui| {
                contents(ui);
            });

        y == 300.0
    }
}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        const start_year: usize = 2022; // TODO

        if ctx.input(|inp| {
            inp.key_released(Key::Q) && inp.modifiers.ctrl
        }) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.inner_margin(0.0))
            .show(ctx, |ui| {
                // let mut cards = Cards::new(&mut self.cards);
                //
                // let viewed = Default::default();
                // let plan_changes = Default::default();
                // let queued_upgrades = Default::default();
                // let process_mix_changes = Default::default();
                // let process_points = Default::default();
                // let ctx = CardState {
                //     state: &self.state.game,
                //     viewed: &viewed,
                //     plan_changes: &plan_changes,
                //     queued_upgrades: &queued_upgrades,
                //     process_mix_changes: &process_mix_changes,
                //     process_points: &process_points,
                // };
                // cards.render(ui, &ctx);

                draw_bg_image(ui);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    match &mut self.ui {
                        ViewState::Start(start) => {
                            if let Some(action) =
                                start.render(ui)
                            {
                                match action {
                                    StartAction::Continue => {
                                        todo!()
                                    }
                                    StartAction::NewGame => {
                                        self.ui =
                                            ViewState::Game(
                                                UIState::intro(
                                                    start_year,
                                                    &mut self
                                                        .state
                                                        .game,
                                                ),
                                            );
                                    }
                                }
                            }
                        }
                        ViewState::Game(game) => game
                            .render(ui, &mut self.state.game),
                    }
                });
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(
            storage,
            eframe::APP_KEY,
            &*state::STATE.read(),
        );
    }
}
