mod dialogue;

use std::collections::BTreeMap;

use egui_taffy::TuiBuilderLogic;
use hes_engine::State;
use rust_i18n::t;

use crate::{
    display::{DisplayEvent, icons, render_effects},
    parts::center_center,
    tips::tip,
};

use dialogue::{Dialogue, DialogueResult};

#[derive(Debug, PartialEq)]
pub enum EventResult {
    Advanced,
    JustFinished,
    AlreadyFinished,
}

pub struct Events {
    idx: usize,
    events: Vec<DisplayEvent>,
    dialogue: Option<Dialogue>,
    pub is_finished: bool,
}
impl Events {
    pub fn new(events: Vec<DisplayEvent>) -> Self {
        if events.is_empty() {
            Self {
                idx: 0,
                dialogue: None,
                events,
                is_finished: true,
            }
        } else {
            Self {
                idx: 0,
                dialogue: Some(Dialogue::from(
                    events[0].clone(),
                )),
                events,
                is_finished: false,
            }
        }
    }

    pub fn replace(&mut self, events: Vec<DisplayEvent>) {
        *self = Events::new(events);
    }

    // Returns `true` if events finished this frame.
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
    ) -> Option<EventResult> {
        let mut result = None;
        if !self.events.is_empty() {
            let event = self.events[self.idx].clone();
            let dialogue_result =
                center_center(ui, "events", |tui| {
                    tui.ui(|ui| {
                        self.render_event(ui, state, &event)
                    })
                });
            if let Some(dialogue_result) = dialogue_result {
                match dialogue_result {
                    DialogueResult::Advanced => {
                        result = Some(EventResult::Advanced);
                    }
                    DialogueResult::Finished => {
                        if self.idx < self.events.len() - 1 {
                            self.idx += 1;
                            let event =
                                self.events[self.idx].clone();
                            self.dialogue =
                                Some(Dialogue::from(event));
                            result =
                                Some(EventResult::Advanced);
                        } else {
                            self.is_finished = true;
                            result =
                                Some(EventResult::JustFinished);
                        }
                    }
                }
            }
        } else {
            result = Some(EventResult::AlreadyFinished);
        }
        result
    }

    fn render_event(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        event: &DisplayEvent,
    ) -> Option<DialogueResult> {
        if let Some(dialogue) = &mut self.dialogue {
            ui.set_width(400.);
            let go_to_next = ui
                .vertical(|ui| {
                    if event.show_as_card() {
                        render_event_card(ui, state, event);
                    }
                    dialogue.render(ui, state)
                })
                .inner;
            go_to_next
        } else {
            None
        }
    }
}

impl From<DisplayEvent> for Dialogue {
    fn from(event: DisplayEvent) -> Self {
        let mut ctx = BTreeMap::default();
        if let Some((_, name)) = &event.region {
            ctx.insert("region".to_string(), name.to_string());
        };

        // Only show effects in the dialogue if there's
        // no event card being shown.
        let effects = if event.show_as_card() {
            None
        } else {
            Some(event.effects.clone())
        };

        Dialogue::new(
            event.flavor.dialogue.clone(),
            effects,
            ctx,
            Some(event.id),
        )
    }
}

fn render_event_card(
    ui: &mut egui::Ui,
    state: &State,
    event: &DisplayEvent,
) {
    // TODO
    // let factor_tip =
    //     store_value(t!("The factors behind this event.↓"));
    // let (_, set_settings) = Settings::rw();
    // on_cleanup(move || {
    //     set_settings.update(|settings| {
    //         settings.read_help.push(factor_tip.get_value());
    //     });
    // });

    // TODO how to get as ImageSource
    // let image_src =
    //     event.flavor.image.as_ref().map(|image| image.data);

    let attrib = event
        .flavor
        .image
        .as_ref()
        .map(|image| &image.attribution);

    let show_effects = event.has_visible_effects();

    let arc = t!(&event.flavor.arc);
    let name = t!(&event.name);
    let factors_list = event
        .factors
        .iter()
        .cloned()
        .map(|(icon, factor)| {
            tip(
                icons::icon_from_slug(&icon),
                factor.to_string(),
            )
        })
        .collect::<Vec<_>>();

    let effects = &event.effects;

    let attribution = if let Some(attrib) = attrib {
        if attrib.trim().is_empty() {
            "".into()
        } else {
            format!("{} {attrib}", t!("Image:"))
        }
    } else {
        "".into()
    };

    ui.vertical(|ui| {
        // TODO background image
        // bg_image(); image_src

        // TODO
        // <Help text={factor_tip.get_value()} x=0.55 y=-18.0 center=false/>

        //
        ui.label(arc);
        ui.label(name);
        ui.label(attribution);

        ui.horizontal(|ui| {
            for fac in factors_list {
                fac.render(ui.ctx());
            }
        });

        if show_effects {
            render_effects(ui, state, effects);
        }
    });
}
