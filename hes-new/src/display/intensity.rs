use std::borrow::Cow;

use egui::{Color32, CornerRadius, Pos2, Stroke, StrokeKind};
use hes_engine::Output;
use rust_i18n::t;

use crate::{
    state::base_demand_by_income_levels,
    vars::{Impact, OutputKind},
};

const BASE_WORLD_OUTLOOK: f32 = 20.;
const BASE_REGIONAL_OUTLOOK: f32 = 10.;
const BASE_REGIONAL_HABITABILITY: f32 = 10.;

pub enum Variable {
    Outlook,
    Extinction,
    Habitability,
    WorldOutlook,
    Warming,
}

fn impact_stops(key: Impact, kind: OutputKind) -> [f32; 4] {
    match key {
        Impact::Land => match kind {
            OutputKind::Energy => [0., 0.001, 0.01, 0.1],
            OutputKind::Calories => [0., 0.001, 0.002, 0.01],
        },
        Impact::Energy => match kind {
            // Enhancement: Take into account EROI
            OutputKind::Energy => [0., 0.001, 0.01, 0.1],
            OutputKind::Calories => {
                [0., 0.00015, 0.0005, 0.001]
            }
        },
        Impact::Water => match kind {
            OutputKind::Energy => [0., 1., 2., 5.],
            OutputKind::Calories => [0., 1., 2., 3.],
        },
        Impact::Emissions => match kind {
            OutputKind::Energy => [-2000., 0., 200., 800.],
            OutputKind::Calories => [-1., 0., 0.5, 1.],
        },
        Impact::Biodiversity => match kind {
            OutputKind::Energy => [0., 1e-15, 1e-14, 1.5e-14],
            OutputKind::Calories => [0., 1e-16, 1e-15, 1e-14],
        },
        Impact::Electricity | Impact::Fuel => {
            let output = key
                .as_output()
                .expect("Checked they're valid outputs");
            base_demand_by_income_levels(output)
        }
    }
}

pub fn impact_intensity(
    val: f32,
    key: Impact,
    kind: OutputKind,
) -> usize {
    let stops = impact_stops(key, kind);
    if val < stops[0] {
        0
    } else {
        stops
            .windows(2)
            .enumerate()
            .find(|(_, stops)| {
                val >= stops[0] && val < stops[1]
            })
            .map(|(i, _)| i + 1)
            .unwrap_or(stops.len())
    }
}

fn output_stops(key: Output) -> [f32; 4] {
    base_demand_by_income_levels(key)
}

pub fn output_intensity(val: f32, key: Output) -> usize {
    let stops = output_stops(key);
    stops
        .windows(2)
        .enumerate()
        .find(|(_, stops)| val >= stops[0] && val < stops[1])
        .map(|(i, _)| i + 1)
        .unwrap_or(stops.len())
}

pub const N_PIPS: usize = 5;

pub fn color(
    mut intensity: usize,
    invert: bool,
) -> egui::Color32 {
    if invert {
        intensity = N_PIPS.saturating_sub(intensity);
    }
    if intensity <= 1 {
        Color32::from_rgb(0x2F, 0xE8, 0x63)
    } else if intensity == 2 {
        Color32::from_rgb(0xFB, 0xC0, 0x11)
    } else if intensity == 3 {
        Color32::from_rgb(0xf2, 0x84, 0x35)
    } else {
        Color32::from_rgb(0xEF, 0x38, 0x38)
    }
}

pub fn describe(intensity: usize) -> Cow<'static, str> {
    match intensity {
        0 => t!("Very Low"),
        1 => t!("Low"),
        2 => t!("Moderate"),
        3 => t!("High"),
        _ => t!("Very High"),
    }
}

pub fn scale(val: f32, key: Variable) -> usize {
    let val = match key {
        Variable::Outlook => {
            (val / BASE_REGIONAL_OUTLOOK * 4.).round().max(1.)
        }
        Variable::Extinction => {
            (val / 100. * 4.).round().max(0.)
        }
        Variable::Habitability => {
            (val / BASE_REGIONAL_HABITABILITY * 4.)
                .round()
                .max(0.)
        }
        Variable::WorldOutlook => (val
            / (BASE_REGIONAL_OUTLOOK + BASE_WORLD_OUTLOOK)
            * 4.)
            .round()
            .max(1.),
        Variable::Warming => val.floor() + 1.,
    };
    val as usize
}

// TODO replace this with a fluent builder
pub fn render_intensity_bar(
    ui: &mut egui::Ui,
    intensity: usize,
    invert: bool,
) {
    render_intensity_bar_with_pips(
        ui, intensity, invert, N_PIPS,
    );
}

pub fn render_intensity_bar_with_seg_width(
    ui: &mut egui::Ui,
    intensity: usize,
    invert: bool,
    seg_w: f32,
) {
    let color = color(intensity, invert);
    draw_segmented_pill(ui, N_PIPS, color, intensity, seg_w);
}

pub fn render_intensity_bar_with_pips(
    ui: &mut egui::Ui,
    intensity: usize,
    invert: bool,
    max_pips: usize,
) {
    let color = color(intensity, invert);
    draw_segmented_pill(ui, max_pips, color, intensity, 8.);
}

fn draw_segmented_pill(
    ui: &mut egui::Ui,
    n: usize,
    fill: Color32,
    fill_to: usize,
    seg_w: f32,
) {
    let seg_h = 6.;
    let spacing = 1.;
    let radius = 2;

    let total_width =
        (seg_w * n as f32) + (spacing * (n - 1) as f32);
    let size = egui::vec2(total_width, seg_h);
    let (rect, _resp) =
        ui.allocate_exact_size(size, egui::Sense::hover());

    let painter = ui.painter();

    let mut x = rect.left();
    for i in 0..n {
        let x0 = x;
        let x1 =
            if i == n - 1 { rect.right() } else { x0 + seg_w };
        let seg_rect = egui::Rect::from_min_max(
            Pos2::new(x0, rect.top()),
            Pos2::new(x1, rect.bottom()),
        );

        // Corner rounding per segment
        let rounding = if i == 0 {
            // left cap
            CornerRadius {
                nw: radius,
                ne: 0,
                sw: radius,
                se: 0,
            }
        } else if i == n - 1 {
            // right cap
            CornerRadius {
                nw: 0,
                ne: radius,
                sw: 0,
                se: radius,
            }
        } else {
            CornerRadius::ZERO
        };

        let stroke = Stroke::NONE;
        let color = if i < fill_to {
            fill
        } else {
            Color32::from_rgb(0x45, 0x3E, 0x3E)
        };
        painter.rect(
            seg_rect,
            rounding,
            color,
            stroke,
            StrokeKind::Outside,
        );

        x += seg_w + spacing;
    }
}
