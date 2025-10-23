use std::borrow::Cow;

use egui::{Color32, Margin, RichText, TextStyle};

use crate::display::icon_from_slug;

mod animate;
mod parse;
mod scale;

pub use scale::{scale_text, scale_text_styles, scale_text_ui};

pub fn bbcode(text: &str) -> impl FnOnce(&mut egui::Ui) -> egui::Response {
    |ui| {
        ui.horizontal_wrapped(|ui| {
            let style = ui.style();
            let font_id = TextStyle::Body.resolve(style);
            let text_height = ui.fonts(|f| f.row_height(&font_id));
            ui.style_mut().spacing.item_spacing.x = 0.;

            let (_, nodes) = parse::parse_bbcode(text).unwrap();
            for node in nodes {
                node.render_static(ui, text_height);
            }
        })
        .response
    }
}

#[derive(Default)]
pub struct BbCodeAnimator {
    animator: animate::NodesAnimator,
}
impl BbCodeAnimator {
    pub fn finished(&self) -> bool {
        self.animator.finished()
    }

    pub fn finish(&mut self) {
        self.animator.finish();
    }

    pub fn render(&mut self, ui: &mut egui::Ui, text: &str, available_width: f32) {
        ui.horizontal_wrapped(|ui| {
            let style = ui.style();
            let font_id = TextStyle::Body.resolve(style);
            let text_height = ui.fonts(|f| f.row_height(&font_id));

            ui.style_mut().spacing.item_spacing.x = 0.;

            let (_, nodes) = parse::parse_bbcode(text).unwrap();
            self.animator
                .animate(ui, nodes, text_height, available_width);
        });
    }
}

#[derive(Debug, PartialEq)]
enum Tag {
    Bold,
    Image,
    UnknownParam,
    TypeTotal,
    Card,
    EffectFeature,
    TipWarn,
    TipGoal,
}

#[derive(Debug, PartialEq)]
enum Node<'a> {
    Text(&'a str),
    Tagged { tag: Tag, children: Vec<Node<'a>> },
}
impl<'a> Node<'a> {
    fn text(&'a self) -> Cow<'a, str> {
        match self {
            Node::Text(text) => (*text).into(),
            Node::Tagged { children, .. } => {
                let mut text = String::new();
                for ch in children {
                    text.push_str(&ch.text());
                }
                text.into()
            }
        }
    }
}

impl Node<'_> {
    fn render_static(self, ui: &mut egui::Ui, text_height: f32) {
        match self {
            Node::Text(text) => {
                ui.label(text);
            }
            Node::Tagged { tag, children } => {
                match tag {
                    Tag::Bold => {
                        let mut text = String::new();
                        for ch in children {
                            text.push_str(&ch.text());
                        }
                        let mut text = RichText::new(text).strong().underline();
                        if let Some(color) = ui.style().visuals.override_text_color {
                            text = text.color(color);
                        }
                        ui.label(text);
                    }
                    Tag::Image => {
                        ui.add(inline_image(&children, text_height));
                    }
                    Tag::UnknownParam => {
                        egui::Frame::NONE
                            .inner_margin(Margin::symmetric(6, 1))
                            .corner_radius(12)
                            .fill(Color32::from_black_alpha(180))
                            .show(ui, |ui| {
                                ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                                for ch in children {
                                    ch.render_static(ui, text_height);
                                }
                            });
                    }
                    Tag::TypeTotal => {
                        let text = inner_text(&children);
                        ui.colored_label(Color32::from_white_alpha(150), text);
                    }
                    Tag::Card => {
                        for ch in children {
                            ch.render_static(ui, text_height);
                        }
                    }
                    Tag::EffectFeature => {
                        for ch in children {
                            ch.render_static(ui, text_height);
                        }
                    }
                    Tag::TipWarn => {
                        let text = inner_text(&children);
                        ui.colored_label(Color32::from_rgb(0xeb, 0x39, 0x41), text);
                    }
                    Tag::TipGoal => {
                        let text = inner_text(&children);
                        ui.colored_label(Color32::from_rgb(0x43, 0xcc, 0x70), text);
                    }
                };
            }
        }
    }
}

fn inner_text(nodes: &[Node<'_>]) -> String {
    let mut text = String::new();
    for ch in nodes {
        text.push_str(&ch.text());
    }
    text
}

fn inline_image<'a>(nodes: &'a [Node<'_>], text_height: f32) -> egui::Image<'a> {
    let text = inner_text(nodes);

    // Special treatment for the Gosplant mark,
    // which is the only non-square icon.
    if text == "gosplant" {
        egui::Image::new(icon_from_slug(&text)).max_height(text_height)
    } else {
        icon_from_slug(&text).size(text_height - 4.)
    }
}
