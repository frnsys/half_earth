use std::sync::LazyLock;

use egui::{Image, ImageSource, TextureOptions, ahash::HashMap, mutex::Mutex};

static IMAGES: LazyLock<Mutex<HashMap<String, ImageSource<'static>>>> =
    LazyLock::new(|| Mutex::new(HashMap::default()));

#[cfg(not(target_arch = "wasm32"))]
mod content {
    use egui::ImageSource;
    use rust_embed::{Embed, RustEmbed};

    #[derive(Embed)]
    #[folder = "assets/content"]
    struct ContentImages;

    pub fn load<'a>(fname: &str) -> ImageSource<'a> {
        match ContentImages::get(fname) {
            Some(image) => ImageSource::Bytes {
                uri: format!("bytes:://{fname}").into(),
                bytes: image.data.to_vec().into(),
            },
            None => super::DEFAULT_IMAGE,
        }
    }

    fn rand_image<'a, D: RustEmbed>(faction: &str) -> Option<ImageSource<'a>> {
        let image_opts: Vec<_> = D::iter().filter(|path| path.contains(faction)).collect();
        fastrand::choice(&image_opts).and_then(|path| {
            D::get(path).map(|file| ImageSource::Bytes {
                uri: format!("bytes:://{path}").into(),
                bytes: file.data.to_vec().into(),
            })
        })
    }

    #[derive(Embed)]
    #[folder = "assets/sharing/win"]
    struct WinImages;

    pub fn win_image<'a>(faction: &str) -> Option<ImageSource<'a>> {
        rand_image::<WinImages>(faction)
    }

    #[derive(Embed)]
    #[folder = "assets/sharing/lose/death"]
    struct DeathImages;

    pub fn death_image<'a>(faction: &str) -> Option<ImageSource<'a>> {
        rand_image::<DeathImages>(faction)
    }

    #[derive(Embed)]
    #[folder = "assets/sharing/lose/coup"]
    struct CoupImages;

    pub fn coup_image<'a>(faction: &str) -> Option<ImageSource<'a>> {
        rand_image::<CoupImages>(faction)
    }

    #[derive(Embed)]
    #[folder = "assets/sharing/lose/generic"]
    struct LoseImages;

    pub fn lose_image<'a>(faction: &str) -> Option<ImageSource<'a>> {
        rand_image::<LoseImages>(faction)
    }
}

#[cfg(target_arch = "wasm32")]
mod content {
    use egui::ImageSource;
    use web_sys::window;

    fn get_origin() -> String {
        window().and_then(|w| w.location().origin().ok()).unwrap()
    }

    pub fn load<'a>(fname: &str) -> ImageSource<'a> {
        let origin = get_origin();
        ImageSource::Uri(format!("{origin}/images/content/{fname}").into())
    }

    include!(concat!(env!("OUT_DIR"), "/sharing.rs"));

    fn rand_image<'a>(opts: &[&'static str], faction: &str) -> Option<ImageSource<'a>> {
        let image_opts: Vec<_> = opts.iter().filter(|path| path.contains(faction)).collect();
        fastrand::choice(&image_opts).map(|path| {
            let origin = get_origin();
            ImageSource::Uri(format!("{origin}/{path}").into())
        })
    }

    pub fn win_image<'a>(faction: &str) -> Option<ImageSource<'a>> {
        rand_image(&WIN, faction)
    }

    pub fn death_image<'a>(faction: &str) -> Option<ImageSource<'a>> {
        rand_image(&DEATH, faction)
    }

    pub fn coup_image<'a>(faction: &str) -> Option<ImageSource<'a>> {
        rand_image(&COUP, faction)
    }

    pub fn lose_image<'a>(faction: &str) -> Option<ImageSource<'a>> {
        rand_image(&LOSE, faction)
    }
}

pub use content::{coup_image, death_image, lose_image, win_image};

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

const DEFAULT_IMAGE: ImageSource<'static> =
    egui::include_image!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/DEFAULT.webp",));

pub fn locale_image<'a>(fname: &'static str) -> egui::ImageSource<'a> {
    let mut images = IMAGES.lock();
    images
        .entry(fname.to_string())
        .or_insert_with(|| {
            #[cfg(target_arch = "wasm32")]
            let fname = fname.replace("webp", "jpg");

            content::load(&format!("locales/{fname}"))
        })
        .clone()
}

pub fn intro_image<'a>(fname: &'static str) -> egui::ImageSource<'a> {
    let mut images = IMAGES.lock();
    images
        .entry(fname.to_string())
        .or_insert_with(|| {
            #[cfg(target_arch = "wasm32")]
            let fname = fname.replace("webp", "jpg");
            content::load(&format!("intro/{fname}"))
        })
        .clone()
}

pub fn background_image<'a>(fname: &'static str) -> egui::ImageSource<'a> {
    let mut images = IMAGES.lock();
    images
        .entry(fname.to_string())
        .or_insert_with(|| content::load(&format!("backgrounds/{fname}")))
        .clone()
}

pub fn flavor_image<'a>(image: &hes_engine::flavor::Image) -> egui::Image<'a> {
    let mut images = IMAGES.lock();

    let fname = match &image.data {
        hes_engine::flavor::ImageData::File(fname) => fname.to_string(),
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
                    #[cfg(target_arch = "wasm32")]
                    let path = format!(
                        "flavor/web/{}",
                        fname.replace("webp", "jpg").replace("png", "jpg")
                    );

                    #[cfg(not(target_arch = "wasm32"))]
                    let path = format!("flavor/{fname}");

                    content::load(&path)
                }
                hes_engine::flavor::ImageData::Data { bytes, .. } => ImageSource::Bytes {
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
