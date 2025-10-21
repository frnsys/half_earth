use std::{cmp::Ordering, collections::BTreeMap};

use egui::{Color32, ImageSource};
use egui_taffy::TuiBuilderLogic;
use enum_map::EnumMap;
use hes_engine::*;
use hes_images::{coup_image, death_image, lose_image, win_image};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    display::{Icon, icons},
    parts::{button, h_center},
    state::{GameState, StateExt},
    tips::{add_tip, tip},
    views::events::Events,
};

pub struct End {
    lose: bool,
    events: Events,
    badges: Vec<Badge>,
    log: Vec<String>,
    image: Option<ImageSource<'static>>,
}
impl End {
    pub fn new(lose: bool, state: &mut GameState) -> Self {
        let events = if lose {
            StateExt::roll_events(&mut state.core, EventPhase::BreakStart)
        } else {
            StateExt::roll_events(&mut state.core, EventPhase::EndStart)
        };
        let summary = summarize(&state.core, !lose);

        let image = match summary.ending {
            Ending::Win => win_image(&summary.faction),
            Ending::Died => death_image(&summary.faction),
            Ending::Coup => coup_image(&summary.faction),
            Ending::LostOther => lose_image(&summary.faction),
        };

        let log = state
            .ui
            .change_history
            .iter()
            .zip(state.ui.process_mix_history.iter().map(|(_, mixes)| mixes))
            .map(|((year, changes), mixes)| format_year_log(*year, changes, mixes))
            .collect::<Vec<_>>();

        Self {
            lose,
            events: Events::new(events, &state.core),
            badges: eval_badges(state),
            log,
            image,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut GameState) -> bool {
        let mut restart = false;
        if !self.events.is_finished {
            self.events.render(ui, state);
        } else {
            ui.vertical_centered(|ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                let width = (ui.ctx().screen_rect().width() - 12.).min(480.);
                ui.set_max_width(width);

                ui.add_space(64.);

                h_center(ui, "badges", |tui| {
                    tui.ui(|ui| {
                        ui.horizontal(|ui| {
                            for badge in &self.badges {
                                let tip = tip(icons::HELP, t!(badge.to_string()));
                                add_tip(tip, ui.add(badge.icon().size(32.)));
                            }
                        });
                    });
                });

                ui.add_space(64.);

                h_center(ui, "message", |tui| {
                    tui.ui(|ui| {
                        let message = if self.lose {
                            t!("This is not the end...")
                        } else {
                            t!("Well Played!")
                        };
                        ui.label(
                            egui::RichText::new(message.to_uppercase())
                                .heading()
                                .italics()
                                .color(Color32::WHITE),
                        );
                    });
                });

                ui.add_space(64.);

                let resp = ui.add(button(t!("Try Again?")).full_width());
                restart = resp.clicked();

                ui.add_space(32.);

                h_center(ui, "history", |tui| {
                    tui.ui(|ui| {
                        ui.set_max_width(width);
                        if let Some(image) = &self.image {
                            ui.add(egui::Image::new(image.clone()));
                        }

                        ui.add_space(32.);

                        ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                        ui.style_mut().interaction.selectable_labels = true;
                        ui.monospace("Your History");
                        for line in &self.log {
                            ui.monospace(line);
                        }
                    });
                });

                ui.add_space(64.);
            });
        }
        restart
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Ending {
    Win,
    Died,
    Coup,
    LostOther,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
enum Badge {
    Seceded,
    Aliens,
    Biodiversity,
    Electrification,
    Extinction,
    FossilFuels,
    Meat,
    Nuclear,
    Renewables,
    Space,
    Vegan,
}
impl Badge {
    fn applies(&self, state: &State) -> bool {
        match self {
            Self::Seceded => state.world.regions.iter().any(|reg| reg.seceded),
            Self::Aliens => state.flags.contains(&Flag::AlienEncounter),
            Self::Biodiversity => state.world.extinction_rate <= 15.,
            Self::Extinction => state.world.extinction_rate >= 60.,
            Self::Electrification => state.world.projects.iter().any(|proj| {
                proj.name == "Mass Electrification"
                    && (proj.status == Status::Finished || proj.status == Status::Active)
            }),
            Self::FossilFuels => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| proc.features.contains(&ProcessFeature::IsFossil))
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    > 0
            }
            Self::Meat => {
                // Animal calories demand at least 80% of starting value
                state.output_demand.of(Output::AnimalCalories) >= 2e15
            }
            Self::Nuclear => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| {
                        proc.features.contains(&ProcessFeature::CanMeltdown)
                            || proc.features.contains(&ProcessFeature::MakesNuclearWaste)
                    })
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    >= 10
            }
            Self::Renewables => {
                state
                    .world
                    .processes
                    .iter()
                    .filter(|proc| proc.features.contains(&ProcessFeature::IsIntermittent))
                    .map(|proc| proc.mix_share)
                    .sum::<usize>()
                    >= 10
            }
            Self::Space => {
                state
                    .world
                    .projects
                    .iter()
                    .filter(|proj| {
                        proj.group == Group::Space
                            && (proj.status == Status::Finished || proj.status == Status::Active)
                    })
                    .count()
                    >= 3
            }
            Self::Vegan => {
                // Animal calories demand down to less than 10% of starting val
                state.output_demand.of(Output::AnimalCalories) < 2e14
            }
        }
    }

    fn icon(&self) -> Icon {
        match self {
            Self::Seceded => icons::BADGE_SECEDED,
            Self::Aliens => icons::BADGE_ALIENS,
            Self::Biodiversity => icons::BADGE_BIODIVERSITY,
            Self::Electrification => icons::BADGE_ELECTRIFICATION,
            Self::Extinction => icons::BADGE_EXTINCTION,
            Self::FossilFuels => icons::BADGE_FOSSILFUELS,
            Self::Meat => icons::BADGE_MEAT,
            Self::Nuclear => icons::BADGE_NUCLEAR,
            Self::Renewables => icons::BADGE_RENEWABLES,
            Self::Space => icons::BADGE_SPACE,
            Self::Vegan => icons::BADGE_VEGAN,
        }
    }
}
impl std::fmt::Display for Badge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Seceded => "At least one region seceded from Gosplant.",
            Self::Aliens => "You had an extraterrestrial encounter.",
            Self::Biodiversity => "Planetary life flourished under your tenure.",
            Self::Electrification => "You helped electrify the world.",
            Self::Extinction => "Planetary life suffered under your tenure.",
            Self::FossilFuels => "You kept on using fossil fuels.",
            Self::Meat => "Carnivorous diets were left intact.",
            Self::Nuclear => "Nuclear was your preferred form of energy.",
            Self::Renewables => "Renewables dominated energy production.",
            Self::Space => "You pushed humanity towards the stars.",
            Self::Vegan => "Global diets shifted towards vegan.",
        };
        write!(f, "{}", desc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Summary {
    pub ending: Ending,
    pub faction: String,
    pub badges: Vec<Badge>,
}

fn eval_badges(state: &State) -> Vec<Badge> {
    Badge::iter().filter(|badge| badge.applies(state)).collect()
}

fn summarize(state: &State, win: bool) -> Summary {
    let badges = eval_badges(state);
    let closest = state
        .npcs
        .iter()
        .max_by(|x, y| {
            x.relationship
                .partial_cmp(&y.relationship)
                .unwrap_or(Ordering::Equal)
        })
        .unwrap();
    let faction = closest.name.to_string();

    Summary {
        badges,
        faction,
        ending: if win {
            Ending::Win
        } else {
            if state.world.year >= state.death_year {
                Ending::Died
            } else if state.political_capital <= 0 {
                Ending::Coup
            } else {
                Ending::LostOther
            }
        },
    }
}

fn format_year_log(
    year: usize,
    changes: &[Change],
    mixes: &EnumMap<Output, BTreeMap<String, usize>>,
) -> String {
    [
        format!("\n[{year}]"),
        changes
            .iter()
            .map(|diff| diff.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        "Production Mix:".into(),
        mixes
            .iter()
            .map(|(output, mix)| {
                let mut parts = vec![format!("  [{output}]")];
                for (name, mix) in mix {
                    parts.push(format!("    {name}:{mix}"));
                }
                parts.join("\n")
            })
            .collect::<Vec<_>>()
            .join("\n"),
    ]
    .join("\n")
}
