use std::collections::BTreeMap;

use egui::{Align, Color32, Layout, Margin};
use hes_engine::{
    Id,
    State,
    flavor::{DialogueLine, DialogueNext, Response, Speaker},
};
use rust_i18n::t;

use crate::{
    display::{DisplayEffect, speaker_icon},
    text::BbCodeAnimator,
    views::{events::render_effects, parts::button},
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
    context: BTreeMap<String, String>,
    event_id: Option<Id>,
    region_id: Option<Id>,
}
impl Dialogue {
    pub fn new(
        dialogue: hes_engine::flavor::Dialogue,
        effects: Option<Vec<DisplayEffect>>,
        context: BTreeMap<String, String>,
        event_id: Option<Id>,
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
            context,
            event_id,
            region_id: None,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
    ) -> Option<DialogueResult> {
        let line = &self.current_line;
        let profile = speaker_icon(&line.speaker);
        let is_last_line = line.next.is_none();
        let has_decision = line.has_decision();

        egui::Frame::NONE
            .fill(Color32::WHITE)
            .inner_margin(Margin::symmetric(6, 6))
            .show(ui, |ui| {
                ui.style_mut().visuals.override_text_color =
                    Some(Color32::BLACK);
                if line.speaker != Speaker::Game {
                    ui.image(profile);
                    let text = t!(line.speaker.to_string());
                    ui.label(text);
                }

                // TODO apply context
                //             fill_icons(&fill_vars(&t!(&line.text), context))
                self.animator.render(ui, &line.text);
            });

        let revealed = self.animator.finished();

        let mut result = None;
        if revealed {
            if is_last_line {
                if let Some(effects) = &self.effects {
                    render_effects(ui, state, effects);
                }

                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.add_space(26.);
                        if ui
                            .add(button(t!("Continue")))
                            .clicked()
                        {
                            result =
                                Some(DialogueResult::Finished);
                        }
                    },
                );
            } else if let Some(DialogueNext::Responses(
                responses,
            )) = line.next.clone()
            {
                for branch in responses {
                    let text = t!(&branch.text);
                    if ui.add(button(text)).clicked() {
                        self.select_choice(&branch, state);
                    }
                }
            } else {
                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.add_space(26.);
                        if ui.add(button(t!("Next"))).clicked()
                        {
                            self.advance_line(state);
                            result =
                                Some(DialogueResult::Advanced);
                        }
                    },
                );
            }
        }
        result
    }

    fn advance_line(&mut self, state: &State) {
        let next = &self.current_line.next;
        if let Some(next) = next {
            match next {
                DialogueNext::Line { id } => {
                    self.current_line =
                        self.dialogue.lines[*id].clone();
                }
                DialogueNext::Responses(responses) => {
                    if self.event_id.is_some() {
                        let branch =
                            responses.iter().find(|b| {
                                state.eval_conditions(
                                    &b.conditions,
                                    self.region_id,
                                )
                            });
                        if let Some(branch) = branch {
                            if let Some(line_id) =
                                branch.next_line
                            {
                                self.current_line = self
                                    .dialogue
                                    .lines[line_id]
                                    .clone();
                            }
                        }
                    }
                }
            }
        }
    }

    fn select_choice(
        &mut self,
        response: &Response,
        state: &mut State,
    ) {
        state.apply_effects(&response.effects, self.region_id);

        if let Some(line_id) = response.next_line {
            self.current_line =
                self.dialogue.lines[line_id].clone();
        }
    }
}
