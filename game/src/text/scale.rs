use egui::UiBuilder;

pub fn scale_text<R>(
    ui: &mut egui::Ui,
    max_size: egui::Vec2,
    contents: impl Fn(&mut egui::Ui) -> R,
) -> R {
    scale_text_impl(ui, max_size, contents, &mut 1.)
}

fn scale_text_impl<R>(
    ui: &mut egui::Ui,
    max_size: egui::Vec2,
    contents: impl Fn(&mut egui::Ui) -> R,
    factor: &mut f32,
) -> R {
    const MIN_FACTOR: f32 = 0.5;

    // Create a separate UI for measuring the UI size.
    // Set the clip rect so that nothing actually renders.
    let mut measure_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                max_size,
            ))
            .layout(*ui.layout()),
    );
    let empty = egui::Rect::ZERO;
    measure_ui.set_clip_rect(empty);

    // Scale font sizes and spacing according to the factor,
    // then draw and measure the UI.
    scale_text_styles(measure_ui.style_mut(), *factor);
    let _ = contents(&mut measure_ui);
    let size =
        measure_ui.min_rect().size().max(egui::vec2(1.0, 1.0));

    // Check if the UI fits.
    let sx = (max_size.x / size.x).min(1.0);
    let sy = (max_size.y / size.y).min(1.0);
    let font_scale = sx.min(sy);

    // It doesn't fit and we haven't yet reached the limit,
    // so shrink and try again.
    if font_scale < 1. && *factor > MIN_FACTOR {
        *factor *= 0.9;
        scale_text_impl(ui, max_size, contents, factor)

    // It fits (or we reached the limit), so draw at the estimated size.
    } else {
        ui.scope(|ui| {
            // Vertically center
            let height_diff = max_size.y - size.y;
            ui.add_space(height_diff / 2.);
            scale_text_styles(ui.style_mut(), *factor);
            contents(ui)
        })
        .inner
    }
}

fn scale_text_styles(style: &mut egui::Style, factor: f32) {
    for (_, font) in style.text_styles.iter_mut() {
        font.size = (font.size * factor).max(1.0);
    }
    style.spacing.item_spacing.y *= factor;
}
