mod audio;
mod climate;
mod consts;
mod debug;
mod display;
mod locales;
mod parts;
mod splash;
mod state;
mod style;
mod text;
mod tips;
mod vars;
mod views;

use std::sync::Arc;

use egui::Key;

use debug::DEBUG;
use splash::{Start, StartAction};
use state::Settings;

use crate::{
    audio::AudioSystem,
    parts::{Sizing, draw_bg_image, set_full_bg_image},
    state::{GameState, Tutorial, prepare_game},
    views::{GameAction, GameView},
};

#[cfg(not(target_arch = "wasm32"))]
use hes_editor::WorldEditor;

rust_i18n::i18n!("/dev/null", fallback = "en", backend = locales::Backend);

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

enum View {
    Start(Start),
    Game(GameView),

    #[cfg(not(target_arch = "wasm32"))]
    Editor(WorldEditor),
}

pub struct App {
    view: View,
    state: GameState,
    prefs: Settings,
    audio: AudioSystem,
    ctx: Arc<three_d::context::Context>,

    has_save: bool,
}
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        style::configure_style(&cc.egui_ctx);

        let mut prefs = load_prefs(cc.storage).unwrap_or_default();
        let state = load_game(cc.storage);
        let has_save = state.is_some();
        let mut state = state.unwrap_or_default();

        if DEBUG.skip_tutorial {
            prefs.tutorial.finish();
        }

        let audio = AudioSystem::new(!prefs.sound);

        let ctx = cc.gl.clone().unwrap();
        Self {
            view: if DEBUG.open_editor {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    View::Editor(WorldEditor::default())
                }

                #[cfg(target_arch = "wasm32")]
                {
                    View::Start(Start::default())
                }
            } else if DEBUG.view.is_some() {
                prepare_game(&mut state, &prefs);
                View::Game(GameView::new(&mut state, &ctx))
            } else {
                View::Start(Start::default())
            },
            audio,
            prefs,
            state,
            has_save,
            ctx,
        }
    }
}

impl App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.save_prefs(storage);
        self.save_game(storage);
    }

    fn save_game(&mut self, storage: &mut dyn eframe::Storage) {
        // NOTE: For some reason serializing the state directly by
        // passing it to `set_value` doesn't load/deserialize;
        // loading always returns `None`. There might be something wrong
        // with the RON de/serializer. It works fine using JSON though.
        let s = serde_json::to_string(&self.state).unwrap();
        eframe::set_value(storage, "state", &s);
    }

    fn save_prefs(&mut self, storage: &mut dyn eframe::Storage) {
        self.prefs.tutorial = self.state.ui.tutorial;
        if self.prefs.tutorial == Tutorial::Ready && self.prefs.runs_played == 0 {
            self.prefs.runs_played = 1;
        }
        eframe::set_value(storage, "prefs", &self.prefs);
    }
}

fn load_game(storage: Option<&dyn eframe::Storage>) -> Option<GameState> {
    storage
        .and_then(|storage| eframe::get_value::<String>(storage, "state"))
        // See note in `save_game`.
        .map(|s| serde_json::from_str(&s).unwrap())
}

fn load_prefs(storage: Option<&dyn eframe::Storage>) -> Option<Settings> {
    storage.and_then(|storage| eframe::get_value(storage, "prefs"))
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|inp| inp.key_released(Key::Q) && inp.modifiers.ctrl) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        if let Err(err) = self.audio.update() {
            tracing::error!("Audio error: {err}");
        }

        let is_small_screen = ctx.screen_rect().width() <= 450.;
        let scale = if is_small_screen { 0.7 } else { 1. };
        let sizing = Sizing {
            scale,
            normal: 14. * scale,
            is_small: is_small_screen,
        };
        ctx.memory_mut(|mem| mem.data.insert_temp(egui::Id::NULL, sizing));

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.inner_margin(0.0))
            .show(ctx, |ui| {
                draw_bg_image(ui);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    match &mut self.view {
                        View::Start(start) => {
                            if let Some(action) = start.render(ui, &mut self.prefs, self.has_save) {
                                match action {
                                    StartAction::Continue => {
                                        self.state = load_game(frame.storage()).unwrap_or_default();
                                        prepare_game(&mut self.state, &self.prefs);
                                        self.view = View::Game(GameView::from_save(
                                            &mut self.state,
                                            &self.ctx,
                                        ));
                                    }
                                    StartAction::NewGame(world) => {
                                        self.state = GameState::from_world(*world);
                                        prepare_game(&mut self.state, &self.prefs);
                                        self.view =
                                            View::Game(GameView::new(&mut self.state, &self.ctx));
                                    }

                                    #[cfg(not(target_arch = "wasm32"))]
                                    StartAction::OpenEditor => {
                                        self.view = View::Editor(WorldEditor::default());
                                    }
                                }
                            }
                        }
                        View::Game(view) => {
                            if let Some(action) = view.render(ui, &mut self.state, &mut self.prefs)
                            {
                                match action {
                                    GameAction::Restart => {
                                        self.view = View::Start(Start::default());
                                    }
                                    GameAction::ToggleSound => {
                                        self.prefs.sound = !self.prefs.sound;
                                        if self.prefs.sound {
                                            audio::unmute();
                                        } else {
                                            audio::mute();
                                        }
                                        if let Some(storage) = frame.storage_mut() {
                                            self.save_prefs(storage);
                                        }
                                    }
                                    GameAction::Save => {
                                        if let Some(storage) = frame.storage_mut() {
                                            self.save(storage);
                                        }
                                    }
                                }
                            }
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        View::Editor(editor) => {
                            set_full_bg_image(
                                ui,
                                hes_images::background_image("editor.jpg"),
                                egui::vec2(1200., 897.),
                            );
                            ui.add(editor);
                        }
                    }
                });
            });
    }
}
