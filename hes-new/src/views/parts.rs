use std::sync::LazyLock;

use crate::image;
use egui::{
    Color32,
    Image,
    ImageSource,
    Margin,
    Rect,
    Shadow,
    TextureOptions,
    Vec2,
    ahash::HashMap,
    mutex::Mutex,
};
use egui_taffy::{Tui, taffy, tui};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/images/content"]
struct ContentImages;

fn hash_to_hex(data: &[u8]) -> String {
    let hash = blake3::hash(data);
    hash.to_hex().to_string()
}

fn ext_from_mime(mime: &str) -> Option<&'static str> {
    match mime {
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpg"),
        "image/gif" => Some("gif"),
        "image/webp" => Some("webp"),
        "image/svg+xml" => Some("svg"),
        _ => None,
    }
}

static IMAGES: LazyLock<
    Mutex<HashMap<String, ImageSource<'static>>>,
> = LazyLock::new(|| Mutex::new(HashMap::default()));

pub fn flavor_image(
    image: &hes_engine::flavor::Image,
) -> egui::Image {
    let mut images = IMAGES.lock();

    let fname = match &image.data {
        hes_engine::flavor::ImageData::File(fname) => {
            fname.to_string()
        }
        hes_engine::flavor::ImageData::Data { bytes, mime } => {
            let fname = hash_to_hex(&bytes);
            let ext = ext_from_mime(&mime);
            if let Some(ext) = ext {
                format!("{fname}.{ext}")
            } else {
                tracing::warn!("Unrecognized mimetype: {mime}");
                return Image::new(image!(
                    "content/DEFAULT.jpg"
                ));
            }
        }
    };

    let image = match images.get(&fname) {
        Some(image) => Image::new(image.clone()),
        None => {
            let source = match &image.data {
                hes_engine::flavor::ImageData::File(fname) => {
                    match ContentImages::get(&fname) {
                        Some(image) => ImageSource::Bytes {
                            uri: format!("bytes:://{fname}")
                                .into(),
                            bytes: image.data.to_vec().into(),
                        },
                        None => image!("content/DEFAULT.jpg"),
                    }
                }
                hes_engine::flavor::ImageData::Data {
                    bytes,
                    ..
                } => ImageSource::Bytes {
                    uri: format!("bytes:://{fname}").into(),
                    bytes: bytes.clone().into(),
                },
            };
            images.insert(fname, source.clone());
            Image::new(source)
        }
    };

    image
        .show_loading_spinner(false)
        .texture_options(TextureOptions::LINEAR)
}

/// Full cover background image.
fn full_bg_image(
    ui: &mut egui::Ui,
    image: egui::ImageSource<'_>,
    image_size: Vec2,
) {
    // Get the target rect (e.g., the whole screen)
    let target_rect = ui.ctx().screen_rect();
    let target_size = target_rect.size();

    // Compute aspect ratios
    let image_aspect = image_size.x / image_size.y;
    let target_aspect = target_size.x / target_size.y;

    // Compute size to draw to match `background-size: cover`
    let draw_size = if image_aspect > target_aspect {
        // Image is wider than target → match height, crop width
        Vec2::new(target_size.y * image_aspect, target_size.y)
    } else {
        // Image is taller than target → match width, crop height
        Vec2::new(target_size.x, target_size.x / image_aspect)
    };

    // Center the image
    let center = target_rect.center();
    let draw_rect = Rect::from_center_size(center, draw_size);

    egui::Image::new(image)
        .show_loading_spinner(false)
        .texture_options(TextureOptions::NEAREST)
        .paint_at(ui, draw_rect);
}

pub fn set_full_bg_image(
    ui: &mut egui::Ui,
    image: egui::ImageSource<'static>,
    image_size: Vec2,
) {
    ui.memory_mut(|mem| {
        mem.data.insert_temp(
            "bg-image".into(),
            (image, image_size),
        );
    });
}

pub fn draw_bg_image(ui: &mut egui::Ui) {
    if let Some((image, size)) =
        ui.memory(|mem| mem.data.get_temp("bg-image".into()))
    {
        full_bg_image(ui, image, size);
    }
}

pub fn raised_frame(
    ui: &mut egui::Ui,
    contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::NONE
        .fill(Color32::from_gray(70))
        .shadow(Shadow {
            offset: [2, 2],
            blur: 8,
            spread: 2,
            color: Color32::from_black_alpha(128),
        })
        .inner_margin(Margin {
            top: 1,
            left: 1,
            ..Default::default()
        })
        .corner_radius(5)
        .show(ui, |ui| {
            egui::Frame::NONE
                .fill(Color32::from_gray(0))
                .corner_radius(5)
                .inner_margin(Margin {
                    bottom: 2,
                    right: 2,
                    ..Default::default()
                })
                .show(ui, |ui| {
                    egui::Frame::NONE
                        .fill(Color32::from_gray(22))
                        .corner_radius(5)
                        .inner_margin(Margin::symmetric(8, 8))
                        .show(ui, |ui| contents(ui))
                });
        });
}

pub fn glow(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    color: Color32,
) {
    let painter = ui.painter();
    for i in 1..=4 {
        let expanded = rect.expand(i as f32);
        let alpha = 40 / i; // fade out
        painter.rect_stroke(
            expanded,
            8.0,
            egui::Stroke::new(
                i as f32 * 2.,
                color.linear_multiply(alpha as f32 / 255.0),
            ),
            egui::StrokeKind::Outside,
        );
    }
}

pub fn center_center<T>(
    ui: &mut egui::Ui,
    id: &str,
    inner: impl FnOnce(&mut Tui) -> T,
) -> T {
    tui(ui, ui.id().with(id))
        .reserve_available_space()
        .style(taffy::Style {
            flex_grow: 1.,
            flex_direction: taffy::FlexDirection::Column,
            min_size: taffy::Size {
                width: taffy::prelude::percent(1.),
                height: taffy::prelude::percent(1.),
            },
            align_items: Some(taffy::AlignItems::Center),
            justify_content: Some(
                taffy::JustifyContent::SpaceAround,
            ),
            ..Default::default()
        })
        .show(inner)
}

pub fn flex_justified(
    ui: &mut egui::Ui,
    id: &str,
    inner: impl FnOnce(&mut Tui),
) {
    tui(ui, ui.id().with(id))
        .reserve_available_width()
        .style(taffy::Style {
            flex_grow: 1.,
            flex_direction: taffy::FlexDirection::Row,
            min_size: taffy::Size {
                width: taffy::prelude::percent(1.),
                height: taffy::prelude::auto(),
            },
            align_items: Some(taffy::AlignItems::Center),
            justify_content: Some(
                taffy::JustifyContent::SpaceBetween,
            ),
            ..Default::default()
        })
        .show(inner);
}

pub fn flex_spaced(
    ui: &mut egui::Ui,
    id: &str,
    inner: impl FnOnce(&mut Tui),
) {
    tui(ui, ui.id().with(id))
        .reserve_available_width()
        .style(taffy::Style {
            flex_grow: 1.,
            flex_direction: taffy::FlexDirection::Row,
            min_size: taffy::Size {
                width: taffy::prelude::percent(1.),
                height: taffy::prelude::auto(),
            },
            align_items: Some(taffy::AlignItems::Center),
            justify_content: Some(
                taffy::JustifyContent::SpaceAround,
            ),
            ..Default::default()
        })
        .show(inner);
}
