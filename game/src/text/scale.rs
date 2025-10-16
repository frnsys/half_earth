use egui::UiBuilder;

pub fn scale_text_ui<R>(
    ui: &mut egui::Ui,
    max_size: egui::Vec2,
    contents: impl Fn(&mut egui::Ui) -> R,
) -> R {
    scale_text_impl(ui, max_size, contents, &mut 1.)
}

pub fn scale_text(
    ui: &mut egui::Ui,
    max_size: egui::Vec2,
    text: egui::RichText,
    start_size: f32,
) -> egui::Response {
    let text = SizedText {
        text,
        size: start_size,
    };
    scale_text_impl(ui, max_size, text, &mut 1.)
}

trait Resizeable<R> {
    fn render_scaled(
        &mut self,
        ui: &mut egui::Ui,
        factor: f32,
    ) -> R;
}

impl<F, R> Resizeable<R> for F
where
    F: Fn(&mut egui::Ui) -> R,
{
    fn render_scaled(
        &mut self,
        ui: &mut egui::Ui,
        factor: f32,
    ) -> R {
        scale_text_styles(ui.style_mut(), factor);
        self(ui)
    }
}

struct SizedText {
    text: egui::RichText,
    size: f32,
}

impl Resizeable<egui::Response> for SizedText {
    fn render_scaled(
        &mut self,
        ui: &mut egui::Ui,
        factor: f32,
    ) -> egui::Response {
        let text = self.text.clone();
        let text = text.size(self.size * factor);
        ui.label(text)
    }
}

fn scale_text_impl<R, U: Resizeable<R>>(
    ui: &mut egui::Ui,
    max_size: egui::Vec2,
    mut contents: U,
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
    let _ = contents.render_scaled(&mut measure_ui, *factor);
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
            contents.render_scaled(ui, *factor)
        })
        .inner
    }
}

pub fn scale_text_styles(style: &mut egui::Style, factor: f32) {
    for (_, font) in style.text_styles.iter_mut() {
        font.size = (font.size * factor).max(1.0);
    }
    style.spacing.item_spacing.y *= factor;
}
