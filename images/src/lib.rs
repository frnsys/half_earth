use std::sync::LazyLock;

use egui::{
    Image,
    ImageSource,
    TextureOptions,
    ahash::HashMap,
    mutex::Mutex,
};
use rust_embed::Embed;

static IMAGES: LazyLock<
    Mutex<HashMap<String, ImageSource<'static>>>,
> = LazyLock::new(|| Mutex::new(HashMap::default()));

#[derive(Embed)]
#[folder = "images"]
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
        _ => None,
    }
}

const DEFAULT_IMAGE: ImageSource<'static> = egui::include_image!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/DEFAULT.webp",)
);

pub fn flavor_image(
    image: &hes_engine::flavor::Image,
) -> egui::Image<'_> {
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
                return Image::new(DEFAULT_IMAGE);
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
                        None => DEFAULT_IMAGE,
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
