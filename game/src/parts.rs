use std::time::Duration;

use crate::image;
use egui::{
    Color32,
    CornerRadius,
    Margin,
    Order,
    Rect,
    Sense,
    Shadow,
    TextureOptions,
    Vec2,
};
use egui_animation::{animate_repeating, easing};
use egui_taffy::{Tui, TuiBuilderLogic, taffy, tui};

pub fn bg_cover_image(
    ui: &mut egui::Ui,
    image: egui::Image<'_>,
    target_rect: Rect,
) {
    if let Some(image_size) =
        image.load_and_calc_size(ui, ui.available_size())
    {
        let target_size = target_rect.size();

        // Compute aspect ratios
        let image_aspect = image_size.x / image_size.y;
        let target_aspect = target_size.x / target_size.y;

        let draw_size = if image_aspect > target_aspect {
            egui::Vec2::new(
                target_size.y * image_aspect,
                target_size.y,
            )
        } else {
            egui::Vec2::new(
                target_size.x,
                target_size.x / image_aspect,
            )
        };

        let draw_rect = egui::Rect::from_center_size(
            target_rect.center(),
            draw_size,
        );
        ui.scope(|ui| {
            ui.shrink_clip_rect(target_rect);
            image.paint_at(ui, draw_rect);
        });
    }
}

/// Full cover background image.
fn full_bg_image(
    ui: &mut egui::Ui,
    image: egui::ImageSource<'_>,
    image_size: Vec2,
    tint: Option<Color32>,
) {
    // Get the target rect (e.g., the whole screen)
    let target_rect = ui.ctx().screen_rect();
    let target_size = target_rect.size();

    // Compute aspect ratios
    let image_aspect = image_size.x / image_size.y;
    let target_aspect = target_size.x / target_size.y;

    let draw_size = if image_aspect > target_aspect {
        Vec2::new(target_size.y * image_aspect, target_size.y)
    } else {
        Vec2::new(target_size.x, target_size.x / image_aspect)
    };

    // Center the image
    let center = target_rect.center();
    let draw_rect = Rect::from_center_size(center, draw_size);

    let mut image = egui::Image::new(image)
        .show_loading_spinner(false)
        .texture_options(TextureOptions::LINEAR);

    if let Some(tint) = tint {
        image = image.tint(tint);
    }

    image.paint_at(ui, draw_rect);
}

pub fn set_full_bg_image(
    ui: &mut egui::Ui,
    image: egui::ImageSource<'static>,
    image_size: Vec2,
) {
    ui.memory_mut(|mem| {
        mem.data.insert_temp(
            "bg-image".into(),
            (image, image_size, None::<Color32>),
        );
    });
}

pub fn set_full_bg_image_tinted(
    ui: &mut egui::Ui,
    image: egui::ImageSource<'static>,
    image_size: Vec2,
    tint: Color32,
) {
    ui.memory_mut(|mem| {
        mem.data.insert_temp(
            "bg-image".into(),
            (image, image_size, Some(tint)),
        );
    });
}

pub fn draw_bg_image(ui: &mut egui::Ui) {
    if let Some((image, size, tint)) =
        ui.memory(|mem| mem.data.get_temp("bg-image".into()))
    {
        full_bg_image(ui, image, size, tint);
    }
}

pub struct RaisedFrame {
    top_color: Color32,
    bot_color: Color32,
    radius: CornerRadius,
    inner_color: Color32,
    inner_margin: Margin,
    hover_color: Option<Color32>,
    shadow: Option<Shadow>,
    highlight: Option<Color32>,
    font_color: Option<Color32>,
}
pub fn raised_frame() -> RaisedFrame {
    RaisedFrame {
        top_color: Color32::from_gray(70),
        bot_color: Color32::from_gray(0),
        inner_color: Color32::from_gray(22),
        radius: 5.into(),
        inner_margin: 8.into(),
        shadow: None,
        hover_color: None,
        highlight: None,
        font_color: None,
    }
}
impl RaisedFrame {
    pub fn colors(
        mut self,
        top: Color32,
        bot: Color32,
        inner: Color32,
    ) -> Self {
        self.top_color = top;
        self.bot_color = bot;
        self.inner_color = inner;
        self
    }

    pub fn margin(mut self, margin: impl Into<Margin>) -> Self {
        self.inner_margin = margin.into();
        self
    }

    pub fn radius(
        mut self,
        radius: impl Into<CornerRadius>,
    ) -> Self {
        self.radius = radius.into();
        self
    }

    pub fn hover(mut self, color: Color32) -> Self {
        self.hover_color = Some(color);
        self
    }

    pub fn shadow(mut self) -> Self {
        self.shadow = Some(Shadow {
            offset: [2, 2],
            blur: 8,
            spread: 2,
            color: Color32::from_black_alpha(128),
        });
        self
    }

    pub fn glow(mut self, color: Color32, radius: u8) -> Self {
        self.shadow = Some(Shadow {
            offset: [0, 0],
            blur: radius * 3,
            spread: radius,
            color,
        });
        self
    }

    pub fn highlight(mut self) -> Self {
        self.highlight =
            Some(Color32::from_rgb(0xeb, 0x40, 0x34));
        self.font_color = Some(Color32::BLACK);
        self.colors(
            Color32::from_rgb(0xe5, 0xfa, 0xaf),
            Color32::from_rgb(0x99, 0xb5, 0x51),
            Color32::from_rgb(0xda, 0xfc, 0x83),
        )
        .hover(Color32::from_rgb(0xd3, 0xfc, 0x68))
    }

    pub fn maybe_highlight(self, highlight: bool) -> Self {
        if highlight { self.highlight() } else { self }
    }

    pub fn show(
        mut self,
        ui: &mut egui::Ui,
        inner: impl FnOnce(&mut egui::Ui),
    ) -> egui::Response {
        let mut frame = egui::Frame::NONE.fill(self.top_color);

        if let Some(glow) = self.highlight {
            let t = animate_repeating(
                ui,
                easing::roundtrip,
                Duration::from_millis(750),
                0.,
            );
            self = self.glow(glow, (6. * t).round() as u8);
        }

        if let Some(shadow) = self.shadow {
            frame = frame.shadow(shadow);
        }

        frame
            .inner_margin(Margin {
                top: 1,
                left: 1,
                ..Default::default()
            })
            .corner_radius(self.radius)
            .show(ui, |ui| {
                egui::Frame::NONE
                    .fill(self.bot_color)
                    .corner_radius(self.radius)
                    .inner_margin(Margin {
                        bottom: 2,
                        right: 2,
                        ..Default::default()
                    })
                    .show(ui, |ui| {
                        if let Some(color) = self.font_color {
                            ui.style_mut()
                                .visuals
                                .override_text_color =
                                Some(color);
                        }
                        let mut frame = egui::Frame::NONE
                            .fill(self.inner_color)
                            .corner_radius(self.radius)
                            .inner_margin(self.inner_margin)
                            .begin(ui);
                        inner(&mut frame.content_ui);

                        let resp = frame.allocate_space(ui);
                        if let Some(color) = self.hover_color
                            && resp.hovered()
                        {
                            frame.frame.fill = color;
                        }
                        frame.paint(ui);
                    })
            })
            .response
    }
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

pub fn h_center<T>(
    ui: &mut egui::Ui,
    id: &str,
    inner: impl FnOnce(&mut Tui) -> T,
) -> T {
    tui(ui, ui.id().with(id))
        .reserve_available_space()
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

pub struct Button {
    frame: RaisedFrame,
    text: String,
    full_width: bool,
}
pub fn button(text: impl Into<String>) -> Button {
    Button {
        frame: button_frame(),
        text: text.into(),
        full_width: false,
    }
}
impl Button {
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    pub fn colors(
        mut self,
        top: Color32,
        bot: Color32,
        inner: Color32,
        hover: Color32,
    ) -> Self {
        self.frame =
            self.frame.colors(top, bot, inner).hover(hover);
        self
    }

    pub fn maybe_highlight(mut self, highlight: bool) -> Self {
        self.frame = self.frame.maybe_highlight(highlight);
        self
    }
}
impl egui::Widget for Button {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let text = egui::RichText::new(self.text)
            .heading()
            .size(14.)
            .color(Color32::BLACK);

        let resp = self.frame.show(ui, |ui| {
            if self.full_width {
                ui.set_width(ui.available_width());
                ui.vertical_centered(|ui| {
                    ui.label(text);
                });
            } else {
                ui.label(text);
            }
        });
        resp.interact(Sense::click())
    }
}

pub fn button_frame() -> RaisedFrame {
    raised_frame()
        .colors(
            Color32::WHITE,
            Color32::from_gray(0xBB),
            Color32::from_gray(0xEE),
        )
        .hover(Color32::from_gray(0xCC))
        .margin(Margin::symmetric(6, 4))
}

pub fn overlay(
    ui: &mut egui::Ui,
    inner: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> bool {
    egui::Area::new("overlay".into())
        .order(Order::Foreground)
        .default_size(ui.ctx().screen_rect().size())
        .movable(false)
        .show(ui.ctx(), |ui| {
            egui::Frame::NONE
                .fill(Color32::from_black_alpha(200))
                .inner_margin(Margin::symmetric(18, 18))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());
                    center_center(
                        ui,
                        "overlay-content".into(),
                        |tui| {
                            tui.ui(|ui| {
                                let resp = inner(ui);
                                resp.clicked_elsewhere()
                            })
                        },
                    )
                })
                .inner
        })
        .inner
}

pub fn new_icon(
    card_rect: Rect,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    let size = egui::vec2(48., 48.);
    let rect = egui::Rect::from_min_size(
        card_rect.left_top() - egui::vec2(16., 16.),
        size,
    );
    let new_icon = image!("new.svg");
    move |ui| {
        ui.place(
            rect,
            egui::Image::new(new_icon)
                .fit_to_exact_size(size)
                .rotate(-0.5, egui::Vec2::splat(0.5)),
        )
    }
}

pub struct CenteredText<'a> {
    text: String,
    image: Option<egui::Image<'a>>,
    font_size: f32,
    font_family: egui::FontFamily,
}
pub fn center_text<'a>(
    text: impl Into<String>,
) -> CenteredText<'a> {
    CenteredText {
        text: text.into(),
        image: None,
        font_size: 14.,
        font_family: egui::FontFamily::Proportional,
    }
}
impl<'a> CenteredText<'a> {
    pub fn image(mut self, image: egui::Image<'a>) -> Self {
        self.image = Some(image);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn family(mut self, family: egui::FontFamily) -> Self {
        self.font_family = family;
        self
    }
}
impl egui::Widget for CenteredText<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let font_id =
            egui::FontId::new(self.font_size, self.font_family);

        let galley = ui.fonts(|f| {
            f.layout_delayed_color(
                self.text,
                font_id,
                f32::INFINITY,
            )
        });
        let mut content_width = galley.size().x;
        let width = ui.available_width();

        if let Some(image) = &self.image {
            let spacing = ui.style().spacing.item_spacing.x;
            let image_width =
                image.calc_size(ui.available_size(), None).x;
            content_width += image_width + spacing;
        }

        let offset = width / 2. - content_width / 2.;
        ui.horizontal(|ui| {
            ui.add_space(offset);
            if let Some(image) = self.image {
                ui.add(image);
            }
            ui.label(galley);
        })
        .response
    }
}

pub fn calc_text_width(
    ui: &mut egui::Ui,
    text: String,
    size: f32,
    family: egui::FontFamily,
) -> f32 {
    let font_id = egui::FontId::new(size, family);
    let galley = ui.fonts(|f| {
        f.layout_delayed_color(text, font_id, f32::INFINITY)
    });
    galley.size().x
}

pub struct FillBar {
    width: f32,
    height: f32,
    filled: f32,
    fill_color: Color32,
    back_color: Color32,
}
impl FillBar {
    pub fn fill_color(mut self, color: Color32) -> Self {
        self.fill_color = color;
        self
    }

    pub fn back_color(mut self, color: Color32) -> Self {
        self.back_color = color;
        self
    }
}
impl egui::Widget for FillBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, resp) = ui.allocate_exact_size(
            egui::vec2(self.width, self.height),
            Sense::empty(),
        );
        let painter = ui.painter();
        painter.rect_filled(rect, 2, self.back_color);

        let mut inner = rect.shrink(1.);
        inner.set_width(inner.width() * self.filled);
        painter.rect_filled(inner, 2, self.fill_color);
        resp
    }
}

pub fn fill_bar(
    (width, height): (f32, f32),
    filled: f32,
) -> FillBar {
    FillBar {
        width,
        height,
        filled,
        fill_color: Color32::from_rgb(0x2F, 0xE8, 0x63),
        back_color: Color32::WHITE,
    }
}
