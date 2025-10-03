use super::super::parts::set_full_bg_image;
use crate::{image, state::STATE, views::parts::glow};
use egui::{
    Align2,
    Color32,
    CursorIcon,
    FontFamily,
    FontId,
    Layout,
    RichText,
    Stroke,
    TextFormat,
    text::LayoutJob,
};
use rust_i18n::t;

pub enum MenuAction {
    Credits,
    Continue,
    NewGame,
    ToggleSound,
}

pub struct Menu;
impl Menu {
    pub fn render(ui: &mut egui::Ui) -> Option<MenuAction> {
        let mut action = None;

        // Show git commit for this build.
        egui::Area::new("git-hash".into())
            .anchor(Align2::RIGHT_BOTTOM, egui::vec2(-8., -8.))
            .show(ui.ctx(), |ui| {
                let git_hash = env!("GIT_HASH");
                ui.label(RichText::new(git_hash).size(10.));
            });

        set_full_bg_image(
            ui,
            image!("start.png"),
            egui::vec2(715., 973.),
        );

        ui.painter().rect_filled(
            ui.ctx().screen_rect(),
            0.,
            Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );
        ui.vertical(|ui| {
            const WIDTH: f32 = 320.;
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.set_width(WIDTH);
                let image = image!("intro.svg");
                ui.add(
                    egui::Image::new(image)
                        .maintain_aspect_ratio(true)
                        .fit_to_exact_size(egui::vec2(
                            300., 300.,
                        )),
                );
                ui.style_mut().visuals.override_text_color =
                    Some(Color32::WHITE);
                ui.style_mut()
                    .visuals
                    .widgets
                    .noninteractive
                    .bg_stroke =
                    Stroke::new(1., Color32::WHITE);

                ui.add_space(18.);

                ui.label("A Planetary Crisis Planning Game");

                ui.add_space(18.);
                ui.separator();

                if button(ui, "Continue", WIDTH) {
                    action = Some(MenuAction::Continue);
                }

                ui.separator();

                if button(ui, "New Game", WIDTH) {
                    action = Some(MenuAction::NewGame);
                }

                ui.separator();

                ui.horizontal_top(|ui| {
                    ui.style_mut().spacing.item_spacing.x = 0.;

                    let sound = format!(
                        "{}: {}",
                        t!("Sound"),
                        if STATE.read().prefs.sound {
                            t!("On")
                        } else {
                            t!("Off")
                        }
                    );
                    if button(ui, &sound, WIDTH / 2. - 2.) {
                        action = Some(MenuAction::ToggleSound);
                    }
                    if button(ui, "Credits", WIDTH / 2. - 2.) {
                        action = Some(MenuAction::Credits);
                    }
                });

                book_button(ui, WIDTH);

                // if ui.button("click me").clicked() {
                // let sound_data =
                //     StaticSoundData::from_cursor(Cursor::new(include_bytes!("../track.mp3")))
                //         .unwrap();
                // // Cloning the sound data will not use any extra memory.
                // let handle = self.audio.play(sound_data.clone()).unwrap();
                // }
            })
        });

        action
    }
}

fn button(ui: &mut egui::Ui, label: &str, width: f32) -> bool {
    let mut clicked = false;
    let mut frame = egui::Frame::NONE
        .corner_radius(3.)
        .inner_margin(egui::Margin::symmetric(3, 12))
        .begin(ui);
    {
        frame.content_ui.set_width(width);
        frame.content_ui.set_height(24.);
        let text = RichText::new(label).font(FontId::new(
            24.,
            FontFamily::Name("TimesTen".into()),
        ));
        let label = egui::Label::new(text).selectable(false);
        frame.content_ui.with_layout(
            Layout::centered_and_justified(
                egui::Direction::TopDown,
            ),
            |ui| {
                ui.add(label);
            },
        );

        let resp = frame.allocate_space(ui);
        let resp = resp.interact(egui::Sense::click());
        clicked = resp.clicked();
        if resp.hovered() {
            frame
                .content_ui
                .ctx()
                .set_cursor_icon(CursorIcon::PointingHand);
            frame.frame.fill =
                egui::Color32::from_black_alpha(96);
        }
    }
    frame.end(ui);
    clicked
}

fn book_button(ui: &mut egui::Ui, width: f32) {
    let is_hovered: bool = ui.memory(|mem| {
        mem.data
            .get_temp("book-hover".into())
            .unwrap_or_default()
    });
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
            let _ = webbrowser::open(
                "https://www.versobooks.com/books/3818-half-earth-socialism",
            );
        }
        if resp.hovered() {
            frame
                .content_ui
                .ctx()
                .set_cursor_icon(CursorIcon::PointingHand);
            frame.frame.fill =
                egui::Color32::from_rgb(0xB9, 0xF8, 0x0D);
            glow(
                ui,
                resp.rect,
                egui::Color32::from_rgb(0xB9, 0xF8, 0x0D),
            );
            if !is_hovered {
                ui.memory_mut(|mem| {
                    mem.data
                        .insert_temp("book-hover".into(), true)
                });
            }
        } else if is_hovered {
            ui.memory_mut(|mem| {
                mem.data.insert_temp("book-hover".into(), false)
            });
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
        Layout::centered_and_justified(
            egui::Direction::TopDown,
        ),
        |ui| {
            let mut job = LayoutJob::default();

            job.append(
                "Read the book: ",
                0.0,
                TextFormat {
                    font_id: FontId::new(
                        16.,
                        FontFamily::Name("TimesTen".into()),
                    ),
                    color,
                    ..Default::default()
                },
            );

            job.append(
                "Half-Earth Socialism.",
                0.0,
                TextFormat {
                    font_id: FontId::new(
                        16.,
                        FontFamily::Name(
                            "TimesTen-Italic".into(),
                        ),
                    ),
                    color,
                    ..Default::default()
                },
            );

            ui.label(job);
        },
    );
}
