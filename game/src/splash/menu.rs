use std::path::PathBuf;

use super::super::parts::set_full_bg_image;
use crate::{image, parts::glow, state::Settings, text::scale_text_ui};
use egui::{
    Align2, Color32, CursorIcon, FontFamily, FontId, Layout, Margin, OpenUrl, RichText, Sense,
    Stroke, TextFormat, text::LayoutJob,
};
use hes_engine::World;
use rust_i18n::t;

pub enum MenuAction {
    Credits,
    Continue,
    NewGame(Box<World>),
    ToggleSound,
}

#[derive(Clone)]
enum WorldStatus {
    Default,
    Custom(String, Box<World>),
    FailedToRead,
    FailedToParse,
}

struct WorldPicker {
    world: WorldStatus,
}
impl WorldPicker {
    fn new() -> Self {
        Self {
            world: WorldStatus::Default,
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            let mut frame = egui::Frame::NONE
                .corner_radius(4)
                .inner_margin(Margin::symmetric(2, 4))
                .begin(ui);

            let label = match &self.world {
                WorldStatus::Default => t!("Default World").to_string(),
                WorldStatus::Custom(name, _) => {
                    format!("{}: {name}", t!("Custom World"))
                }
                WorldStatus::FailedToRead => {
                    t!("Error reading world, please re-select").to_string()
                }
                WorldStatus::FailedToParse => {
                    t!("Error parsing world, please re-select").to_string()
                }
            };
            frame
                .content_ui
                .label(egui::RichText::new(label).heading().size(12.));

            let resp = frame.allocate_space(ui).interact(Sense::click());
            if resp.hovered() {
                frame.frame.stroke = Stroke::new(1., Color32::WHITE);
            }
            frame.paint(ui);

            if resp.clicked() {
                self.pick_and_load_world();
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pick_and_load_world(&mut self) {
        let file = rfd::FileDialog::new()
            .add_filter("World", &["world"])
            .pick_file();
        if let Some(path) = file {
            self.load_world(path);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn pick_and_load_world(&mut self) {
        use pollster::FutureExt as _;
        let future = async {
            let file = rfd::AsyncFileDialog::new()
                .add_filter("World", &["world"])
                .pick_file()
                .await;

            if let Some(file) = file {
                let name: String = file.file_name();
                let data: Vec<u8> = file.read().await;
                Some((name, data))
            } else {
                None
            }
        };
        let data: Option<(String, Vec<u8>)> = future.block_on();
        if let Some((name, data)) = data {
            self.world = match serde_json::from_slice::<World>(&data) {
                Ok(world) => WorldStatus::Custom(name, Box::new(world)),
                Err(_) => WorldStatus::FailedToParse,
            };
        }
    }

    fn load_world(&mut self, path: PathBuf) {
        self.world = match std::fs::read_to_string(&path) {
            Ok(data) => match serde_json::from_str::<World>(&data) {
                Ok(world) => {
                    let name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    WorldStatus::Custom(name, Box::new(world))
                }
                Err(_) => WorldStatus::FailedToParse,
            },
            Err(_) => WorldStatus::FailedToRead,
        }
    }
}

pub struct Menu {
    picker: WorldPicker,
}
impl Menu {
    pub fn new() -> Self {
        Self {
            picker: WorldPicker::new(),
        }
    }

    fn world(&self) -> Box<World> {
        match &self.picker.world {
            WorldStatus::Default | WorldStatus::FailedToRead | WorldStatus::FailedToParse => {
                Box::new(World::default())
            }
            WorldStatus::Custom(_, world) => world.clone(),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        prefs: &Settings,
        has_save: bool,
    ) -> Option<MenuAction> {
        let mut action = None;

        // Show git commit for this build.
        egui::Area::new("git-hash".into())
            .anchor(Align2::RIGHT_BOTTOM, egui::vec2(-8., -8.))
            .show(ui.ctx(), |ui| {
                let git_hash = env!("GIT_HASH");
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                ui.label(RichText::new(git_hash).size(10.));
            });

        set_full_bg_image(
            ui,
            hes_images::background_image("start.png"),
            egui::vec2(715., 973.),
        );

        ui.painter().rect_filled(
            ui.ctx().screen_rect(),
            0.,
            Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );
        ui.vertical(|ui| {
            const WIDTH: f32 = 360.;
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.set_width(WIDTH);
                let image = image!("intro.svg");
                ui.add(
                    egui::Image::new(image)
                        .maintain_aspect_ratio(true)
                        .fit_to_exact_size(egui::vec2(300., 300.)),
                );
                ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                ui.style_mut().visuals.widgets.noninteractive.bg_stroke =
                    Stroke::new(1., Color32::WHITE);

                ui.add_space(18.);

                ui.label(t!("A Planetary Crisis Planning Game"));

                ui.add_space(18.);

                if has_save {
                    ui.separator();

                    if button(ui, &t!("Continue"), WIDTH) {
                        action = Some(MenuAction::Continue);
                    }
                }

                ui.separator();

                if button(ui, &t!("New Game"), WIDTH) {
                    action = Some(MenuAction::NewGame(self.world()));
                }
                self.picker.render(ui);

                ui.separator();

                ui.horizontal_top(|ui| {
                    ui.style_mut().spacing.item_spacing.x = 0.;

                    let sound = format!(
                        "{}: {}",
                        t!("Sound"),
                        if prefs.sound { t!("On") } else { t!("Off") }
                    );
                    if button(ui, &sound, WIDTH / 2. - 2.) {
                        action = Some(MenuAction::ToggleSound);
                    }
                    if button(ui, &t!("Credits"), WIDTH / 2. - 2.) {
                        action = Some(MenuAction::Credits);
                    }
                });

                book_button(ui, WIDTH);
            })
        });

        action
    }
}

fn button(ui: &mut egui::Ui, label: &str, width: f32) -> bool {
    let mut frame = egui::Frame::NONE
        .corner_radius(3.)
        .inner_margin(egui::Margin::symmetric(3, 12))
        .begin(ui);
    let clicked = {
        frame.content_ui.set_width(width);
        frame.content_ui.set_height(32.);
        frame.content_ui.vertical_centered(|ui| {
            let size = egui::vec2(width, 32.);
            scale_text_ui(ui, size, |ui| {
                let text = RichText::new(label).heading();
                ui.label(text);
            });
        });

        let resp = frame.allocate_space(ui);
        let resp = resp.interact(egui::Sense::click());
        if resp.hovered() {
            frame
                .content_ui
                .ctx()
                .set_cursor_icon(CursorIcon::PointingHand);
            frame.frame.fill = egui::Color32::from_black_alpha(96);
        }
        resp.clicked()
    };
    frame.end(ui);
    clicked
}

fn book_button(ui: &mut egui::Ui, width: f32) {
    let is_hovered: bool =
        ui.memory(|mem| mem.data.get_temp("book-hover".into()).unwrap_or_default());
    let mut frame = egui::Frame::NONE
        .corner_radius(3.)
        .inner_margin(egui::Margin::symmetric(3, 12))
        .begin(ui);
    {
        frame.content_ui.set_width(width);
        frame.content_ui.set_height(24.);

        read_label(&mut frame.content_ui, is_hovered);

        let resp = frame.allocate_space(ui);
        let resp = resp.interact(egui::Sense::click());
        if resp.clicked() {
            ui.ctx().open_url(OpenUrl::new_tab(
                "https://www.versobooks.com/books/3818-half-earth-socialism",
            ));
        }
        if resp.hovered() {
            frame
                .content_ui
                .ctx()
                .set_cursor_icon(CursorIcon::PointingHand);
            frame.frame.fill = egui::Color32::from_rgb(0xB9, 0xF8, 0x0D);
            glow(ui, resp.rect, egui::Color32::from_rgb(0xB9, 0xF8, 0x0D));
            if !is_hovered {
                ui.memory_mut(|mem| mem.data.insert_temp("book-hover".into(), true));
            }
        } else if is_hovered {
            ui.memory_mut(|mem| mem.data.insert_temp("book-hover".into(), false));
        }
    }
    frame.end(ui);
}

fn read_label(ui: &mut egui::Ui, is_hovered: bool) {
    let color = if is_hovered {
        Color32::BLACK
    } else {
        Color32::WHITE
    };
    ui.with_layout(
        Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| {
            let mut job = LayoutJob::default();

            job.append(
                "Read the book: ",
                0.0,
                TextFormat {
                    font_id: FontId::new(16., FontFamily::Name("TimesTen".into())),
                    color,
                    ..Default::default()
                },
            );

            job.append(
                "Half-Earth Socialism.",
                0.0,
                TextFormat {
                    font_id: FontId::new(16., FontFamily::Name("TimesTen-Italic".into())),
                    color,
                    ..Default::default()
                },
            );

            ui.label(job);
        },
    );
}
