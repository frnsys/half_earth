use std::collections::BTreeMap;

use egui::ahash::HashSet;
use egui_taffy::TuiBuilderLogic;
use hes_engine::{Effect, Project, ProjectType, State, Status};
use rust_i18n::t;

use crate::{
    display::{DisplayEffect, DisplayEvent, fill_icons, icons},
    text::bbcode,
    views::{
        dialogue::Dialogue,
        parts::center_center,
        tip,
        tips::add_tip,
    },
};

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
    ) -> bool {
        let mut just_finished = false;
        if !self.events.is_empty() {
            let event = self.events[self.idx].clone();
            let go_to_next =
                center_center(ui, "events", |tui| {
                    tui.ui(|ui| {
                        self.render_event(ui, state, &event)
                    })
                });
            if go_to_next {
                if self.idx < self.events.len() - 1 {
                    self.idx += 1;
                    let event = self.events[self.idx].clone();
                    self.dialogue = Some(Dialogue::from(event));
                } else {
                    self.is_finished = true;
                    just_finished = true;
                }
            }
        }
        just_finished
    }

    fn render_event(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        event: &DisplayEvent,
    ) -> bool {
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
            false
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
                ui.add(fac);
            }
        });

        if show_effects {
            render_effects(ui, state, effects);
        }
    });
}

fn outcome_effects(project: &Project) -> Vec<DisplayEffect> {
    struct Count {
        effect: DisplayEffect,
        count: usize,
        hashes: HashSet<Vec<u8>>,
    }
    let mut all_effects: BTreeMap<String, Count> =
        BTreeMap::default();
    let n_outcomes = project.outcomes.len();
    for outcome in &project.outcomes {
        for effect in &outcome.effects {
            // TODO effect.is_hidden
            let key = effect.fingerprint();
            let mut efx = DisplayEffect::from(effect);
            let hash = bincode::serialize(&effect).unwrap();
            efx.likelihood =
                Some(outcome.probability.likelihood);
            let count =
                all_effects.entry(key).or_insert_with(|| {
                    Count {
                        effect: efx,
                        count: 0,
                        hashes: HashSet::default(),
                    }
                });
            count.count += 1;
            count.hashes.insert(hash);
        }
    }

    all_effects
        .into_values()
        .map(
            |Count {
                 mut effect,
                 count,
                 hashes,
             }| {
                effect.is_unknown =
                    count != n_outcomes || hashes.len() > 1;
                effect
            },
        )
        .collect()
}

pub fn active_effects(project: &Project) -> Vec<DisplayEffect> {
    let mut effects = vec![];

    if project.kind == ProjectType::Policy
        && !project.is_active()
    {
        // Project outcome effects are secret and delayed
        effects.extend(
            project.effects.iter().map(DisplayEffect::from),
        );
    } else if project.status == Status::Inactive
        || project.status == Status::Building
    {
        effects.extend(
            project.effects.iter().map(DisplayEffect::from),
        );
        effects.extend(outcome_effects(project).into_iter());
    } else {
        effects.extend(
            project
                .active_effects()
                .iter()
                .map(DisplayEffect::from),
        );
        if let Some(id) = project.active_outcome {
            effects.extend(
                project.outcomes[id]
                    .effects
                    .iter()
                    .map(DisplayEffect::from),
            );
        }
    }

    effects
}

pub fn render_effects(
    ui: &mut egui::Ui,
    state: &State,
    effects: &Vec<DisplayEffect>,
) {
    let mut effects = effects
        .iter()
        .filter(|effect| !effect.is_hidden)
        .filter_map(|effect| {
            effect.tip(state).ok().map(|mut details| {
                if effect.is_unknown {
                    details.tip.supicon = Some(icons::CHANCE);
                }
                details.text = fill_icons(&details.text);
                details
            })
        })
        .collect::<Vec<_>>();
    effects.sort_by_key(|effect| effect.text.clone());
    for effect in effects {
        let resp = bbcode(ui, &effect.text);
        add_tip(effect.tip, resp);
    }
}
