use ab_glyph::{FontRef, PxScale};
use base64::prelude::*;
use image::{DynamicImage, ImageBuffer, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};

use crate::eval::{Badge, Ending, Summary};

const MSGS_LOSE: &[&str] = &[
    "were ousted from power by a furious mob in {year}",
    "were exiled by popular revolutionaries {year}",
    "were assassinated by a global coalition in {year}",
];

const MSGS_WIN: &[&str] =
    &["ushered the world into a prosperous future"];

const MSGS_DIED: &[&str] =
    &["failed to make the world a better place in time"];

const MSGS_COUP: &[&str] = &[
    "were pushed out of parliament in a coup in {year}",
    "were executed by a reactionary force formed by the former wealthy nations in {year}",
];

const BGS_WIN: &[&[u8]] = &[
    include_bytes!("../../public/assets/sharing/win/win1.jpg"),
    include_bytes!("../../public/assets/sharing/win/win2.jpg"),
    include_bytes!("../../public/assets/sharing/win/win3.jpg"),
    include_bytes!("../../public/assets/sharing/win/win4.jpg"),
    include_bytes!("../../public/assets/sharing/win/win5.jpg"),
    include_bytes!("../../public/assets/sharing/win/win6.jpg"),
    include_bytes!("../../public/assets/sharing/win/win7.jpg"),
    include_bytes!("../../public/assets/sharing/win/win9.jpg"),
    include_bytes!("../../public/assets/sharing/win/win12.jpg"),
];
const BGS_COUP: &[&[u8]] = &[
    include_bytes!(
        "../../public/assets/sharing/lose/coup/coup.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/coup/coup1.jpg"
    ),
];
const BGS_DIED: &[&[u8]] = &[include_bytes!(
    "../../public/assets/sharing/lose/death/death.jpg"
)];
const BGS_LOSE: &[&[u8]] = &[
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose1.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose2.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose3.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose4.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose5.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose6.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose7.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose8.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose9.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose10.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose11.jpg"
    ),
    include_bytes!(
        "../../public/assets/sharing/lose/generic/lose12.jpg"
    ),
];

impl Badge {
    fn image(&self) -> DynamicImage {
        let data: &[u8] = match self {
            Self::Seceded => include_bytes!(
                "../../public/assets/badges/seceded.png"
            ),
            Self::Aliens => include_bytes!(
                "../../public/assets/badges/aliens.png"
            ),
            Self::Biodiversity => include_bytes!(
                "../../public/assets/badges/biodiversity.png"
            ),
            Self::Electrification => include_bytes!(
                "../../public/assets/badges/electrification.png"
            ),
            Self::Extinction => include_bytes!(
                "../../public/assets/badges/extinction.png"
            ),
            Self::FossilFuels => include_bytes!(
                "../../public/assets/badges/fossil_fuels.png"
            ),
            Self::Meat => {
                include_bytes!(
                    "../../public/assets/badges/meat.png"
                )
            }
            Self::Nuclear => include_bytes!(
                "../../public/assets/badges/nuclear.png"
            ),
            Self::Renewables => include_bytes!(
                "../../public/assets/badges/renewables.png"
            ),
            Self::Space => {
                include_bytes!(
                    "../../public/assets/badges/space.png"
                )
            }
            Self::Vegan => {
                include_bytes!(
                    "../../public/assets/badges/vegan.png"
                )
            }
        };
        image::load_from_memory(data).unwrap()
    }
}

pub fn generate_image(summary: &Summary) -> String {
    let (msg, bg) = match summary.ending {
        Ending::Win => (
            fastrand::choice(MSGS_WIN),
            fastrand::choice(BGS_WIN),
        ),
        Ending::Died => (
            fastrand::choice(MSGS_DIED),
            fastrand::choice(BGS_DIED),
        ),
        Ending::Coup => (
            fastrand::choice(MSGS_COUP),
            fastrand::choice(BGS_COUP),
        ),
        Ending::LostOther => (
            fastrand::choice(MSGS_LOSE),
            fastrand::choice(BGS_LOSE),
        ),
    };

    let size = (1200, 675);
    let badge_size: u32 = 52;
    let badge_spacing: u32 = 8;
    let v_padding = 10;

    let lg_font = FontRef::try_from_slice(include_bytes!(
        "../../public/assets/sharing/fonts/TimesTen.ttf"
    ))
    .unwrap();
    let sm_font = FontRef::try_from_slice(include_bytes!(
        "../../public/assets/sharing/fonts/Inter-Medium.ttf"
    ))
    .unwrap();

    let mut bg = image::load_from_memory(bg.unwrap()).unwrap();
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

    let msg =
        format!("Me and {} {}.", summary.faction, msg.unwrap());
    let para = textwrap::wrap(&msg, 26);

    let n_badges = summary.badges.len() as u32;
    let badges_width: u32 =
        n_badges * badge_size + (n_badges - 1) * badge_spacing;
    let mut x: i64 = ((size.0 / 2) - (badges_width / 2)) as i64;
    let by: i64 = 16;

    for badge in &summary.badges {
        let badge =
            badge.image().thumbnail(badge_size, badge_size);
        image::imageops::overlay(&mut img, &badge, x, by);
        x += (badge_size + badge_spacing) as i64;
    }

    let scale = PxScale { x: 86.0, y: 86.0 };
    let line_height: u32 = text_size(scale, &lg_font, &msg).1;
    let n_lines: u32 = para.len() as u32;
    let height =
        n_lines * line_height + (n_lines - 1) * v_padding;
    let mut y: i32 = (size.1 / 2 - height / 2 - line_height / 2
        + (badge_size + 30) / 2) as i32;

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
                (size.1 - h - 16) as i32 + dy,
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
        (size.1 - h - 16) as i32,
        footer_scale,
        &sm_font,
        footer_text,
    );

    let mut buf: Vec<u8> = vec![];
    img.write_to(
        &mut std::io::Cursor::new(&mut buf),
        image::ImageFormat::Png,
    )
    .unwrap();
    let b64 = BASE64_STANDARD.encode(&buf);
    format!("data:image/png;base64,{}", b64)
}
