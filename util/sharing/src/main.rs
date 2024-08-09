use std::fs::File;

use ab_glyph::{FontRef, PxScale};
use hes_engine::State;
use image::{
    codecs::jpeg::JpegEncoder,
    DynamicImage,
    ImageBuffer,
    Rgba,
    RgbaImage,
};
use imageproc::drawing::{draw_text_mut, text_size};

const MSGS_LOSE: &[&str] = &[
    "were ousted from power by a furious mob",
    "were exiled by popular revolutionaries",
    "were assassinated by a global coalition",
];

const MSGS_WIN: &[&str] =
    &["ushered the world into a prosperous future"];

const MSGS_DIED: &[&str] =
    &["failed to make the world a better place in time"];

const MSGS_COUP: &[&str] = &[
    "were pushed out of parliament in a coup",
    "were executed by a reactionary force formed by the former wealthy nations",
];

const BGS_WIN: &[&[u8]] = &[
    include_bytes!("../assets/win/win1.jpg"),
    include_bytes!("../assets/win/win2.jpg"),
    include_bytes!("../assets/win/win3.jpg"),
    include_bytes!("../assets/win/win4.jpg"),
    include_bytes!("../assets/win/win5.jpg"),
    include_bytes!("../assets/win/win6.jpg"),
    include_bytes!("../assets/win/win7.jpg"),
    include_bytes!("../assets/win/win9.jpg"),
];
const BGS_COUP: &[&[u8]] = &[
    include_bytes!("../assets/lose/coup/coup.jpg"),
    include_bytes!("../assets/lose/coup/coup1.jpg"),
];
const BGS_DIED: &[&[u8]] =
    &[include_bytes!("../assets/lose/death/death.jpg")];
const BGS_LOSE: &[&[u8]] = &[
    include_bytes!("../assets/lose/generic/lose4.jpg"),
    include_bytes!("../assets/lose/generic/lose5.jpg"),
    include_bytes!("../assets/lose/generic/lose6.jpg"),
    include_bytes!("../assets/lose/generic/lose7.jpg"),
    include_bytes!("../assets/lose/generic/lose8.jpg"),
    include_bytes!("../assets/lose/generic/lose9.jpg"),
    include_bytes!("../assets/lose/generic/lose11.jpg"),
    include_bytes!("../assets/lose/generic/lose12.jpg"),
];

fn generate_image(
    msg: &str,
    bg: &[u8],
    faction: &str,
) -> RgbaImage {
    let size = (1200, 675);
    let v_padding = 10;

    let lg_font = FontRef::try_from_slice(include_bytes!(
        "../assets/fonts/TimesTen.ttf"
    ))
    .unwrap();
    let sm_font = FontRef::try_from_slice(include_bytes!(
        "../assets/fonts/Inter-Medium.ttf"
    ))
    .unwrap();

    let mut bg = image::load_from_memory(bg).unwrap();
    bg = bg.resize_to_fill(
        size.0,
        size.1,
        image::imageops::FilterType::Nearest,
    );

    let mut img = ImageBuffer::from_pixel(
        size.0,
        size.1,
        Rgba([0, 0, 0, 255]),
    );
    image::imageops::overlay(&mut img, &bg, 0, 0);

    let msg = format!("Me and {}s {}.", faction, msg);
    let para = textwrap::wrap(&msg, 26);

    let scale = PxScale { x: 86.0, y: 86.0 };
    let line_height: u32 =
        text_size(scale, &lg_font, &msg).1 + 3;
    let n_lines: u32 = para.len() as u32;
    let height =
        n_lines * line_height + (n_lines - 1) * v_padding;
    let mut y: i32 = (size.1 / 2 - height / 2 - line_height / 2
        + 30 / 2) as i32
        - 20;

    let outline_width = 3;
    for line in para {
        let w = text_size(scale, &lg_font, &line).0;

        for dy in -outline_width..outline_width + 1 {
            for dx in -outline_width..outline_width + 1 {
                if dx * dx + dy * dy
                    >= outline_width * outline_width
                {
                    continue;
                }

                imageproc::drawing::draw_text_mut(
                    &mut img,
                    Rgba([0, 0, 0, 255]),
                    ((size.0 - w) / 2) as i32 + dx,
                    y + dy,
                    scale,
                    &lg_font,
                    &line,
                );
            }
        }

        draw_text_mut(
            &mut img,
            Rgba([255, 255, 255, 255]),
            ((size.0 - w) / 2) as i32,
            y,
            scale,
            &lg_font,
            &line,
        );
        y += (line_height + v_padding) as i32;
    }

    // Draw footer text
    let footer_text = "Play at half.earth";
    let footer_scale = PxScale { x: 42.0, y: 42.0 };
    let (w, h) = text_size(footer_scale, &sm_font, footer_text);

    for dy in -outline_width..outline_width + 1 {
        for dx in -outline_width..outline_width + 1 {
            if dx * dx + dy * dy
                >= outline_width * outline_width
            {
                continue;
            }

            imageproc::drawing::draw_text_mut(
                &mut img,
                Rgba([0, 0, 0, 255]),
                ((size.0 - w) / 2) as i32 + dx,
                (size.1 - h - 32) as i32 + dy,
                footer_scale,
                &sm_font,
                footer_text,
            );
        }
    }

    draw_text_mut(
        &mut img,
        Rgba([254, 192, 7, 255]),
        ((size.0 - w) / 2) as i32,
        (size.1 - h - 32) as i32,
        footer_scale,
        &sm_font,
        footer_text,
    );

    img
}

fn main() {
    let state = State::default();

    for faction in state.npcs.iter() {
        let mut i = 0;
        for msg in MSGS_WIN {
            for bg in BGS_WIN {
                let img =
                    generate_image(msg, bg, &faction.name);
                let img =
                    DynamicImage::ImageRgba8(img).to_rgb8();
                let fname =
                    format!("hes-game/public/assets/sharing/win/{faction}-{i}.jpg");
                let mut file = File::create(fname).unwrap();
                let encoder = JpegEncoder::new_with_quality(
                    &mut file, 30,
                );
                img.write_with_encoder(encoder).unwrap();
                i += 1;
            }
        }
    }

    for faction in state.npcs.iter() {
        let mut i = 0;
        for msg in MSGS_DIED {
            for bg in BGS_DIED {
                let img =
                    generate_image(msg, bg, &faction.name);
                let img =
                    DynamicImage::ImageRgba8(img).to_rgb8();
                let fname =
                    format!("hes-game/public/assets/sharing/lose/death/{faction}-{i}.jpg");
                let mut file = File::create(fname).unwrap();
                let encoder = JpegEncoder::new_with_quality(
                    &mut file, 30,
                );
                img.write_with_encoder(encoder).unwrap();
                i += 1;
            }
        }
    }

    for faction in state.npcs.iter() {
        let mut i = 0;
        for msg in MSGS_COUP {
            for bg in BGS_COUP {
                let img =
                    generate_image(msg, bg, &faction.name);
                let img =
                    DynamicImage::ImageRgba8(img).to_rgb8();
                let fname =
                    format!("hes-game/public/assets/sharing/lose/coup/{faction}-{i}.jpg");
                let mut file = File::create(fname).unwrap();
                let encoder = JpegEncoder::new_with_quality(
                    &mut file, 30,
                );
                img.write_with_encoder(encoder).unwrap();
                i += 1;
            }
        }
    }

    for faction in state.npcs.iter() {
        let mut i = 0;
        for msg in MSGS_LOSE {
            for bg in BGS_LOSE {
                let img =
                    generate_image(msg, bg, &faction.name);
                let img =
                    DynamicImage::ImageRgba8(img).to_rgb8();
                let fname =
                    format!("hes-game/public/assets/sharing/lose/generic/{faction}-{i}.jpg");
                let mut file = File::create(fname).unwrap();
                let encoder = JpegEncoder::new_with_quality(
                    &mut file, 30,
                );
                img.write_with_encoder(encoder).unwrap();
                i += 1;
            }
        }
    }
}
