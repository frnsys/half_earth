use std::collections::BTreeMap;

use crate::{
    consts,
    display::{AsText, DisplayEffect, HasIcon, active_effects, group_color, icons, render_effects},
    image,
    parts::{flex_justified, new_icon},
    state::{GameState, PlanChange},
    text::scale_text_ui,
    tips::{Tip, add_tip, tip},
};

use super::AsCard;
use egui::{Color32, Margin, Rect, Response, Stroke, TextStyle};
use egui_taffy::{TuiBuilderLogic, taffy};
use hes_engine::{Collection, Effect as EngineEffect, Flag, Id, NPC, Project, ProjectType};
use rust_i18n::t;

impl AsCard for Project {
    fn id(&self) -> hes_engine::Id {
        self.id
    }

    fn bg_color(&self) -> egui::Color32 {
        let (bg, _) = group_color(&self.group);
        bg
    }

    fn fg_color(&self) -> egui::Color32 {
        let (_, fg) = group_color(&self.group);
        fg
    }

    fn header(&self, ui: &mut egui::Ui, state: &GameState) {
        let resp = egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(6, 6))
            .show(ui, |ui| {
                flex_justified(ui, &self.name, |tui| {
                    let group = t!(self.group.to_string());
                    tui.style(taffy::Style {
                        flex_grow: 1.,
                        ..Default::default()
                    })
                    .label(egui::RichText::new(group.to_uppercase()).monospace());

                    tui.ui(|ui| {
                        if self.is_online() {
                            let has_levels = !self.upgrades.is_empty();
                            if has_levels {
                                let level = self.level + 1;
                                ui.label(format!("{} {level}", t!("Level")));
                            } else {
                                ui.image(icons::CHECK_BLK);
                                ui.label(t!("Completed"));
                            }
                        } else {
                            // See other note why we need to get the canonical ref.
                            let p = &state.world.projects[&self.id];
                            let remaining_cost = remaining_cost(p, &state.ui.plan_changes);
                            let is_countdown = self.kind != ProjectType::Policy || p.is_building();
                            add_tip(
                                cost_tip(&self.kind, &remaining_cost),
                                ui.horizontal_centered(|ui| {
                                    ui.style_mut().spacing.item_spacing.x = 2.;
                                    if is_countdown {
                                        ui.add(icons::TIME.size(12.));
                                    } else if self.kind == ProjectType::Policy {
                                        ui.add(icons::POLITICAL_CAPITAL.size(14.));
                                    }
                                    ui.label(egui::RichText::new(remaining_cost).size(12.));
                                })
                                .response,
                            );
                        }
                    });
                });
            })
            .response;

        let is_new = !state.ui.viewed.contains(&self.id);
        if is_new {
            ui.add(new_icon(resp.rect));
        }

        // TODO
        // let barcode = image!("barcode.png");
        // ui.image(barcode);
    }

    fn figure(&self, ui: &mut egui::Ui, state: &GameState) {
        let rect = super::render_flavor_image(ui, &self.flavor.image).rect;

        // NOTE: this is hacky, but the cards actually display
        // a clone of the Project, so when we e.g. add points
        // to the project it's not reflected in `self`.
        // This is fine for most data displayed as they're static,
        // but for dynamic data we have to get a ref to the canonical
        // data in `state.core.world`. This could probably be redesigned
        // to directly use the canonical project ref but it's of low priority.
        let p = &state.world.projects[&self.id];

        let has_points = self.kind != ProjectType::Policy && p.is_building();
        if has_points {
            ui.place(rect, points(p.points, &self.kind));
        }

        let (opposers, supporters) = npc_stances(self, &state.npcs);
        npc_support(ui, rect, &opposers, &supporters);

        let parliament_suspended = state.flags.contains(&Flag::ParliamentSuspended);
        let player_seats = state.npcs.coalition_seats();
        let majority_satisfied = if parliament_suspended {
            true
        } else {
            player_seats as f32 > self.required_majority
        };
        let warn_majority = self.required_majority > 0. && !majority_satisfied;

        if warn_majority {
            ui.place(rect, majority_warning(rect));
        }

        let passed = self.kind == ProjectType::Policy && (p.is_building() || p.is_online());
        if passed {
            let rect = egui::Rect::from_min_size(
                rect.left_top() + egui::vec2(16., -48.),
                egui::Vec2::splat(228.),
            );
            let stamp = image!("stamp.svg");
            ui.place(
                rect,
                egui::Image::new(stamp)
                    .fit_to_exact_size(egui::Vec2::splat(228.))
                    .rotate(-0.25, egui::Vec2::splat(0.5)),
            );
        }
    }

    fn name(&self, ui: &mut egui::Ui, _state: &GameState) {
        super::card_title(ui, &self.name);
    }

    fn body(&self, ui: &mut egui::Ui, state: &GameState) {
        let visible_effect = |d: &DisplayEffect| -> bool {
            !matches!(
                d.effect,
                EngineEffect::ProjectRequest(..) | EngineEffect::ProcessRequest(..)
            )
        };

        let effects = active_effects(self)
            .into_iter()
            .filter(visible_effect)
            .collect::<Vec<_>>();

        let resp = egui::Frame::NONE
            .outer_margin(egui::Margin {
                left: 6,
                right: 6,
                top: 0,
                bottom: 6,
            })
            .inner_margin(egui::Margin::symmetric(4, 4))
            .corner_radius(4)
            .stroke(Stroke::new(1., Color32::from_black_alpha(64)))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());
                ui.style_mut().override_text_style = Some(TextStyle::Small);

                let is_active = self.is_active();

                let next_upgrade = if self.upgrades.is_empty() {
                    None
                } else {
                    let idx = self.level;
                    if idx >= self.upgrades.len() {
                        None
                    } else {
                        let upgrade = &self.upgrades[idx];
                        let mut cost = upgrade.cost;
                        if let Some(changes) = state.ui.plan_changes.get(&self.id) {
                            if changes.downgrades > 0 {
                                cost = 0;
                            }
                        }
                        let effects: Vec<DisplayEffect> =
                            upgrade.effects.iter().map(|e| e.into()).collect();
                        Some((cost, effects))
                    }
                };
                let is_upgrading = state.ui.queued_upgrades.get(&self.id) == Some(&true);

                let can_downgrade = self.kind == ProjectType::Policy && self.level > 0;
                let has_downgrade = self.is_active() && can_downgrade;
                let prev_upgrade = if can_downgrade {
                    let idx = self.level as isize - 2;
                    if idx < 0 {
                        let effects: Vec<DisplayEffect> = self
                            .effects
                            .iter()
                            .map(DisplayEffect::from)
                            .filter(visible_effect)
                            .collect();
                        Some((0, effects))
                    } else {
                        if let Some(upgrade) = self.upgrades.get(idx as usize) {
                            let effects: Vec<DisplayEffect> = upgrade
                                .effects
                                .iter()
                                .map(DisplayEffect::from)
                                .filter(visible_effect)
                                .collect();
                            Some((upgrade.cost, effects))
                        } else {
                            None
                        }
                    }
                } else {
                    None
                };

                // When this in 0 in generally means this is offscreen
                // so this won't work correctly.
                if ui.available_height() != 0. {
                    let max_size = egui::vec2(ui.available_width(), ui.available_height());
                    scale_text_ui(ui, max_size, move |ui| {
                        render_effects(ui, &state, &effects);

                        if is_active && let Some((cost, effects)) = &next_upgrade {
                            if is_upgrading {
                                ui.label(t!("Upgrading in one planning cycle."));
                            } else {
                                ui.horizontal_centered(|ui| {
                                    ui.label(t!("Next Level"));
                                    ui.label(cost.to_string());
                                    ui.image(icons::POLITICAL_CAPITAL);
                                });
                            }
                            render_effects(ui, &state, effects);
                        }

                        if has_downgrade {
                            ui.label(t!("Prev Level"));
                            if let Some((_, effects)) = &prev_upgrade {
                                render_effects(ui, &state, effects);
                            }
                        }
                    });
                }
            })
            .response;

        if self.is_building() {
            let rect = egui::Rect::from_min_size(
                resp.rect.center_bottom() + egui::vec2(-6., -10.),
                egui::vec2(0., 0.),
            );

            let building_term = match self.kind {
                ProjectType::Research => {
                    t!("Researching")
                }
                ProjectType::Initiative => {
                    t!("Building")
                }
                ProjectType::Policy => t!("Passing"),
            };

            ui.place(rect, |ui: &mut egui::Ui| {
                egui::Frame::NONE
                    .fill(Color32::WHITE)
                    .stroke(egui::Stroke::new(1., Color32::BLACK))
                    .corner_radius(6)
                    .inner_margin(Margin::symmetric(3, 1))
                    .show(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        ui.label(egui::RichText::new(building_term.to_uppercase()).size(11.));
                    })
                    .response
            });
        }
    }

    fn top_back(&self, ui: &mut egui::Ui, _state: &GameState) {
        ui.add_space(12.);
        super::card_desc(ui, &self.flavor.description);
    }

    fn bottom_back(&self, ui: &mut egui::Ui, state: &GameState) {
        ui.add_space(12.);
        let (opposers, supporters) = npc_stances(self, &state.npcs);
        let has_opposers = !opposers.is_empty();
        let has_supporters = !supporters.is_empty();

        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new(t!("Political Effects")).underline());
        });
        if has_opposers || has_supporters {
            ui.columns(2, |cols| {
                cols[0].vertical_centered(|ui| {
                    ui.label(t!("Nay"));
                    for npc in opposers {
                        npc_opposer(ui, npc);
                    }
                });
                cols[1].vertical_centered(|ui| {
                    ui.label(t!("Yea"));
                    for npc in supporters {
                        npc_supporter(ui, npc);
                    }
                });
            });
        }
        ui.add_space(24.);

        ui.vertical_centered(|ui| {
            let image_attrib = &self.flavor.image.attribution;
            if !image_attrib.is_empty() {
                ui.label(egui::RichText::new(format!("{} {image_attrib}", t!("Image:"))).size(11.));
            }
        });
    }
}

fn cost_tip(kind: &ProjectType, remaining_cost: &str) -> Tip {
    match kind {
        ProjectType::Policy => tip(
            icons::POLITICAL_CAPITAL,
            t!(
                "This policy costs %{remainingCost} political capital to implement.",
                remainingCost = remaining_cost
            ),
        ),
        ProjectType::Initiative => tip(
            icons::INITIATIVE,
            t!(
                "This will take about %{remainingCost} to finish. Allocate more %{kind} points to accelerate its progress.",
                remainingCost = remaining_cost,
                kind = t!(kind.lower())
            ),
        ),
        ProjectType::Research => tip(
            icons::RESEARCH,
            t!(
                "This will take about %{remainingCost} to finish. Allocate more %{kind} points to accelerate its progress.",
                remainingCost = remaining_cost,
                kind = t!(kind.lower())
            ),
        ),
    }
}

fn remaining_cost(project: &Project, plan_changes: &BTreeMap<Id, PlanChange>) -> String {
    if project.is_online() {
        0.to_string()
    } else if project.is_building() {
        match project.kind {
            ProjectType::Policy => t!("1 planning cycle left").to_string(),
            _ => {
                let years = project.years_remaining();
                t!("%{years} yrs left", years = years).to_string()
            }
        }
    } else {
        let cost = if project.points > 0 {
            project.estimate
        } else {
            project.cost
        };
        match project.kind {
            ProjectType::Policy => {
                if let Some(changes) = plan_changes.get(&project.id) {
                    if changes.withdrawn {
                        0.to_string()
                    } else {
                        cost.to_string()
                    }
                } else {
                    cost.to_string()
                }
            }
            _ => t!("%{cost} yrs", cost = cost).to_string(),
        }
    }
}

fn majority_warning(rect: Rect) -> impl FnOnce(&mut egui::Ui) -> Response {
    const PADDING: i8 = 6;
    const P: f32 = (PADDING * 2) as f32;

    move |ui| {
        egui::Frame::NONE
            .outer_margin(egui::Margin::symmetric(6, 0))
            .inner_margin(egui::Margin::symmetric(PADDING, PADDING))
            .corner_radius(4)
            .fill(Color32::from_black_alpha(128))
            .show(ui, |ui| {
                ui.set_width(rect.width() - 12. - P);
                ui.set_height(rect.height() - P);

                ui.vertical_centered(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                    ui.add_space(48.);
                    ui.add(icons::WARNING.size(24.));
                    ui.label(t!(
                        "Because of opposition, this requires a majority in parliament."
                    ));
                });
            })
            .response
    }
}

fn points(n: usize, kind: &ProjectType) -> impl FnOnce(&mut egui::Ui) -> Response {
    let icon = kind.icon();
    let tip = tip(
        icon,
        t!(
            "%{points} %{kind} points are allocated to this project",
            points = n,
            kind = kind.lower()
        ),
    );

    const ICON_SIZE: f32 = 18.;
    move |ui| {
        let resp = egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(12, 6))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(3., 3.);
                    ui.horizontal_wrapped(|ui| {
                        ui.set_max_width(ICON_SIZE * 2. + 5.);
                        for i in 0..consts::MAX_POINTS {
                            let empty = i >= n;
                            if empty {
                                ui.set_opacity(0.5);
                            }
                            ui.add(icon.size(ICON_SIZE));
                        }
                    })
                })
            })
            .response;
        add_tip(tip, resp)
    }
}

pub fn npc_icon(npc: &NPC, yea: bool) -> impl FnOnce(&mut egui::Ui) -> Response {
    let icon = npc.icon();
    move |ui| {
        let color = if yea {
            Color32::from_rgb(0x30, 0xE8, 0x63)
        } else {
            Color32::from_rgb(0xba, 0x30, 0x30)
        };

        egui::Frame::NONE
            .fill(color)
            .corner_radius(4)
            .inner_margin(Margin::symmetric(3, 3))
            .stroke(Stroke::new(1., color))
            .show(ui, |ui| {
                ui.set_width(24.);
                ui.add(icon.size(24.));
            })
            .response
    }
}

pub fn npc_support(ui: &mut egui::Ui, frame_rect: Rect, opposers: &[&NPC], supporters: &[&NPC]) {
    let has_opposers = !opposers.is_empty();
    let has_supporters = !supporters.is_empty();

    let npc_rect = egui::Rect::from_min_size(
        frame_rect.left_bottom() + egui::vec2(6. + 6., -36.),
        egui::vec2(128., 32.),
    );
    if has_opposers {
        ui.place(npc_rect, |ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing.x = 2.;
                for npc in opposers {
                    npc_opposer(ui, npc);
                }
            })
            .response
        });
    }

    let width = 32. * supporters.len() as f32;
    let npc_rect = egui::Rect::from_min_size(
        frame_rect.right_bottom() + egui::vec2(-(6. + 6.) - width, -36.),
        egui::vec2(256., 32.),
    );
    if has_supporters {
        ui.place(npc_rect, |ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing.x = 2.;
                for npc in supporters {
                    npc_supporter(ui, npc);
                }
            })
            .response
        });
    }
}

fn npc_stances<'a>(proj: &Project, npcs: &'a Collection<NPC>) -> (Vec<&'a NPC>, Vec<&'a NPC>) {
    let opposers = proj
        .opposers
        .iter()
        .map(|id| &npcs[id])
        .filter(|npc| !npc.locked)
        .collect::<Vec<_>>();
    let supporters = proj
        .supporters
        .iter()
        .map(|id| &npcs[id])
        .filter(|npc| !npc.locked)
        .collect::<Vec<_>>();
    (opposers, supporters)
}

fn npc_opposer(ui: &mut egui::Ui, npc: &NPC) {
    let tip = tip(
        npc.icon(),
        t!(
            "%{name} is opposed to this. If you implement it, your relationship will worsen by -[i]%{icon}[/i].",
            name = t!(&npc.name),
            icon = icons::RELATIONSHIP,
        ),
    );
    add_tip(tip, ui.add(npc_icon(&npc, false)));
}

fn npc_supporter(ui: &mut egui::Ui, npc: &NPC) {
    let tip = tip(
        npc.icon(),
        t!(
            "%{name} supports this. If you implement it, your relationship will improve by +[i]%{icon}[/i].",
            name = t!(&npc.name),
            icon = icons::RELATIONSHIP,
        ),
    );
    add_tip(tip, ui.add(npc_icon(&npc, true)));
}
