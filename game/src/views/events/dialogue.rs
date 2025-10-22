use egui::{Color32, Key, Margin, PointerButton};
use egui_taffy::TuiBuilderLogic;
use hes_engine::{
    Id, State,
    flavor::{DialogueLine, DialogueNext, Response, Speaker},
};
use rust_i18n::t;

use crate::{
    display::{DisplayEffect, speaker_icon},
    parts::{button, r_align},
    text::BbCodeAnimator,
    views::events::render_effects,
};

#[derive(Debug, PartialEq)]
pub enum DialogueResult {
    Advanced,
    Finished,
}

pub struct Dialogue {
    dialogue: hes_engine::flavor::Dialogue,
    animator: BbCodeAnimator,
    current_line: DialogueLine,
    effects: Option<Vec<DisplayEffect>>,
    event_id: Option<Id>,
    region_id: Option<Id>,
    region_name: Option<String>,
}
impl Dialogue {
    pub fn new(
        dialogue: hes_engine::flavor::Dialogue,
        effects: Option<Vec<DisplayEffect>>,
        event_id: Option<Id>,
        region_name: Option<String>,
    ) -> Self {
        let start_line = dialogue
            .lines
            .get(dialogue.root)
            .unwrap_or(&DialogueLine::default())
            .clone();

        Self {
            dialogue,
            animator: BbCodeAnimator::default(),
            current_line: start_line,
            effects,
            event_id,
            region_id: None,
            region_name,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        mut width: f32,
    ) -> Option<DialogueResult> {
        let mut result = None;

        let is_last_line = self.current_line.next.is_none();
        let has_decision = self.current_line.has_decision();

        let enter_pressed = ui.input(|inp| inp.key_pressed(Key::Enter));
        let clicked = ui.input(|inp| inp.pointer.button_clicked(PointerButton::Primary));
        let anim_finished = self.animator.finished();

        // Finish animation on click/Enter
        if !anim_finished && (clicked || enter_pressed) {
            self.animator.finish();
        } else if anim_finished && enter_pressed {
            // Advance line
            if !is_last_line {
                self.advance_line(state);
                result = Some(DialogueResult::Advanced);

            // Can also press Enter to finish the dialogue,
            // but only if no decision has to be made.
            } else if !has_decision {
                result = Some(DialogueResult::Finished);
            }
        }

        let line = &self.current_line;
        let profile = speaker_icon(&line.speaker);

        ui.horizontal_top(|ui| {
            if line.speaker != Speaker::Game {
                ui.add(profile.fit_to_exact_size(egui::Vec2::splat(64.)));
                width -= 84.;
            }

            egui::Frame::NONE
                .fill(Color32::WHITE)
                .inner_margin(Margin::symmetric(6, 6))
                .show(ui, |ui| {
                    if line.speaker != Speaker::Game {
                        ui.set_min_height(64.);
                    }
                    ui.style_mut().visuals.override_text_color = Some(Color32::BLACK);

                    ui.vertical(|ui| {
                        if line.speaker != Speaker::Game {
                            let text = t!(line.speaker.to_string());
                            ui.label(egui::RichText::new(text.to_uppercase()).size(11.));
                            ui.add_space(4.);
                        }

                        let mut text = t!(&line.text).to_string();
                        if let Some(region_name) = &self.region_name {
                            text = text.replace("{region}", &t!(region_name.as_str()));
                        }
                        self.animator.render(ui, &text, width);
                    });
                });
        });

        let revealed = self.animator.finished();

        if revealed {
            if is_last_line {
                if let Some(effects) = &self.effects {
                    render_effects(ui, state, effects);
                }

                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                r_align(ui, "dialogue-opts", |tui| {
                    tui.ui(|ui| {
                        if ui.add(button(t!("Continue"))).clicked() {
                            result = Some(DialogueResult::Finished);
                        }
                    });
                });
            } else if let Some(DialogueNext::Responses(responses)) = line.next.clone() {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                r_align(ui, "dialogue-opts", |tui| {
                    for branch in responses {
                        tui.ui(|ui| {
                            let text = t!(&branch.text);
                            if ui.add(button(text)).clicked() {
                                let done = self.select_choice(&branch, state);
                                if done {
                                    result = Some(DialogueResult::Finished);
                                }
                            }
                            ui.add_space(1.);
                        });
                    }
                });
            } else {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                r_align(ui, "dialogue-opts", |tui| {
                    tui.ui(|ui| {
                        if ui.add(button(t!("Next"))).clicked() {
                            self.advance_line(state);
                            result = Some(DialogueResult::Advanced);
                        }
                    });
                });
            }
        }
        result
    }

    fn advance_line(&mut self, state: &State) {
        let next = &self.current_line.next;
        if let Some(next) = next {
            match next {
                DialogueNext::Line { id } => {
                    self.current_line = self.dialogue.lines[*id].clone();
                }
                DialogueNext::Responses(responses) => {
                    if self.event_id.is_some() {
                        let branch = responses
                            .iter()
                            .find(|b| state.eval_conditions(&b.conditions, self.region_id));
                        if let Some(branch) = branch {
                            if let Some(line_id) = branch.next_line {
                                self.current_line = self.dialogue.lines[line_id].clone();
                            }
                        }
                    }
                }
            }
        }
    }

    fn select_choice(&mut self, response: &Response, state: &mut State) -> bool {
        state.apply_effects(&response.effects, self.region_id);

        if let Some(line_id) = response.next_line {
            self.current_line = self.dialogue.lines[line_id].clone();
            false
        } else {
            true
        }
    }
}
