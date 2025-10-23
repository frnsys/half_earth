use super::{Node, Tag, inner_text};
use eframe::epaint::{
    FontId,
    text::{LayoutJob, TextFormat},
};
use egui::Color32;

#[derive(Default)]
pub struct NodesAnimator {
    states: Vec<bool>,
    animator: Option<NodeAnimator>,
}
impl NodesAnimator {
    pub fn finished(&self) -> bool {
        self.states.iter().all(|finished| *finished)
    }

    pub fn reset<'a>(&mut self, nodes: &[Node<'a>]) {
        self.states = vec![false; nodes.len()];
        self.animator = None;
    }

    pub fn finish(&mut self) {
        for state in &mut self.states {
            *state = true;
        }
    }

    // NOTE: This assumes `nodes` is stable from frame to frame.
    pub fn animate<'a>(
        &mut self,
        ui: &mut egui::Ui,
        nodes: Vec<Node<'a>>,
        text_height: f32,
        available_width: f32,
    ) {
        ui.set_width(available_width);
        if nodes.len() != self.states.len() {
            self.reset(&nodes);
        }

        let mut found = false;
        for (node, finished) in nodes.into_iter().zip(self.states.iter_mut()) {
            if *finished {
                // For visual consistency
                node.render_partial(ui, usize::MAX, 1.0, text_height);
            } else if !found {
                let animator = self.animator.get_or_insert_default();
                animator.advance(ui.ctx());
                *finished = animator.is_animation_finished();
                if *finished {
                    self.animator = None;
                    node.render_partial(ui, usize::MAX, 1.0, text_height);
                } else {
                    animator.render(ui, node, text_height);
                    ui.ctx().request_repaint();
                }
                found = true;
            } else {
                // TODO this is buggy
                // ui.scope(|ui| {
                //     ui.set_opacity(0.);
                //     node.render_static(ui, text_height);
                // });
            }
        }
    }
}

#[derive(Clone, Debug)]
struct NodeAnimator {
    timer: f32,
    speed: f32,
    animation_finished: bool,
}

impl Default for NodeAnimator {
    fn default() -> Self {
        Self {
            timer: 0.0,
            speed: 50.,
            animation_finished: false,
        }
    }
}

impl NodeAnimator {
    fn advance(&mut self, ctx: &egui::Context) {
        if self.animation_finished {
            return;
        }

        let dt = ctx.input(|i| i.unstable_dt);
        self.timer += dt;
    }

    fn is_animation_finished(&self) -> bool {
        self.animation_finished
    }

    fn render(&mut self, ui: &mut egui::Ui, node: Node<'_>, text_height: f32) {
        if matches!(
            node,
            Node::Tagged {
                tag: Tag::Image,
                ..
            }
        ) {
            self.animation_finished = true;
        } else {
            let num_chars = node.len();
            let total_time = num_chars as f32 * 1. / self.speed;
            let p = self.timer / total_time;
            let visible_chars_float = p * num_chars as f32;
            let visible_chars = visible_chars_float.floor() as usize;
            let progress = visible_chars_float - visible_chars_float.floor();
            node.render_partial(ui, visible_chars, progress, text_height);
            if p >= 1. {
                self.animation_finished = true;
            }
        }
    }
}

impl Node<'_> {
    /// Count of how many "chars" or char-likes to animate for this node.
    fn len(&self) -> usize {
        match self {
            Node::Text(text) => text.len(),
            Node::Tagged { tag, children } => {
                match tag {
                    // ignore nested
                    Tag::Bold => {
                        let mut text = String::new();
                        for ch in children {
                            text.push_str(&ch.text());
                        }
                        text.len()
                    }
                    Tag::Image => 1,
                    Tag::UnknownParam => 1,

                    // Everything else is just treated as normal text when animated
                    _ => {
                        let mut text = String::new();
                        for ch in children {
                            text.push_str(&ch.text());
                        }
                        text.len()
                    }
                }
            }
        }
    }

    pub fn render_partial(
        self,
        ui: &mut egui::Ui,
        visible_chars: usize,
        progress: f32,
        text_height: f32,
    ) {
        match self {
            Node::Text(text) => {
                let style = current_text_style(ui);
                partial_text(ui, text, style, visible_chars, progress);
            }
            Node::Tagged { tag, children } => {
                match tag {
                    Tag::Bold => {
                        let style = current_text_style(ui);
                        let text = inner_text(&children);
                        partial_text(ui, &text, style, visible_chars, progress);
                    }
                    Tag::Image => {
                        ui.scope(|ui| {
                            ui.set_opacity(progress);
                            ui.add(super::inline_image(&children, text_height));
                        });
                    }
                    Tag::TipWarn => {
                        let mut style = current_text_style(ui);
                        style.color = Color32::from_rgb(0xeb, 0x39, 0x41);
                        let text = inner_text(&children);
                        partial_text(ui, &text, style, visible_chars, progress);
                    }
                    Tag::TipGoal => {
                        let mut style = current_text_style(ui);
                        style.color = Color32::from_rgb(0x43, 0xcc, 0x70);
                        let text = inner_text(&children);
                        partial_text(ui, &text, style, visible_chars, progress);
                    }

                    // Everything else is just treated as normal text when animated
                    _ => {
                        let style = current_text_style(ui);
                        let text = inner_text(&children);
                        partial_text(ui, &text, style, visible_chars, progress);
                    }
                }
            }
        }
    }
}

fn current_text_style(ui: &mut egui::Ui) -> TextFormat {
    let style = ui.style();
    let font_id = style
        .override_font_id
        .clone()
        .unwrap_or_else(|| FontId::monospace(18.));
    let color = style.visuals.override_text_color.unwrap_or(Color32::GRAY);
    TextFormat {
        font_id,
        color,
        ..Default::default()
    }
}

fn partial_text(
    ui: &mut egui::Ui,
    text: &str,
    style: TextFormat,
    visible_chars: usize,
    progress: f32,
) {
    let chars: Vec<char> = text.chars().collect();
    let num_chars = chars.len();
    let mut job = LayoutJob::default();
    for (i, ch) in chars.iter().enumerate() {
        let char_alpha_f32 = if i < visible_chars {
            1.0
        } else if i == visible_chars && i < num_chars {
            progress
        } else {
            0.0
        };
        let mut format = style.clone();
        format.color = format.color.gamma_multiply(char_alpha_f32);
        job.append(&ch.to_string(), 0.0, format);
    }
    ui.label(job);
}
