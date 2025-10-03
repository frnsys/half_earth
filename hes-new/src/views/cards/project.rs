use std::collections::BTreeMap;

use crate::{
    consts,
    display::{
        AsText,
        DisplayEffect,
        HasIcon,
        icon_from_slug,
        icons,
    },
    image,
    state::PlanChange,
    views::{
        Tip,
        cards::CardState,
        events::{active_effects, render_effects},
        parts::{flavor_image, flex_justified},
        tip,
        tips::add_tip,
    },
};

use super::AsCard;
use egui::{Color32, Margin};
use egui_taffy::{Tui, TuiBuilderLogic, taffy, tui};
use hes_engine::{
    Effect as EngineEffect,
    Flag,
    Group,
    Id,
    Project,
    ProjectType,
};
use rust_i18n::t;

impl AsCard for Project {
    fn bg_color(&self) -> egui::Color32 {
        let (bg, _) = card_color(&self.group);
        bg
    }

    fn header(&self, ui: &mut egui::Ui, ctx: &CardState) {
        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(4, 6))
            .show(ui, |ui| {
                flex_justified(ui, &self.name, |tui| {
                    let group = t!(self.group.to_string());
                    tui.style(taffy::Style {
                        flex_grow: 1.,
                        ..Default::default()
                    })
                    .label(
                        egui::RichText::new(group).monospace(),
                    );

                    tui.ui(|ui| {
                        if self.is_online() {
                            let has_levels =
                                !self.upgrades.is_empty();
                            if has_levels {
                                let level = self.level + 1;
                                ui.label(format!(
                                    "{} {level}",
                                    t!("Level")
                                ));
                            } else {
                                ui.image(icons::CHECK_BLK);
                                ui.label(t!("Completed"));
                            }
                        } else {
                            let remaining_cost = remaining_cost(
                                self,
                                ctx.plan_changes,
                            );
                            let is_countdown = self.kind
                                != ProjectType::Policy
                                || self.is_building();
                            add_tip(
                                cost_tip(
                                    &self.kind,
                                    &remaining_cost,
                                ),
                                ui.horizontal_centered(|ui| {
                                    // if is_countdown {
                                    //     ui.image(icon_from_slug(icons::TIME));
                                    // }
                                    ui.label(remaining_cost);
                                    // if self.kind == ProjectType::Policy {
                                    //     ui.image(icon_from_slug(
                                    //         icons::POLITICAL_CAPITAL,
                                    //     ));
                                    // }
                                })
                                .response,
                            );
                        }
                    });
                });
            });

        let is_new = !ctx.viewed.contains(&self.id);
        // if is_new {
        //     let new_icon = image!("new.svg");
        //     ui.image(new_icon);
        // }

        // let barcode = image!("barcode.png");
        // ui.image(barcode);
    }

    fn figure(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let parliament_suspended = ctx
            .state
            .flags
            .contains(&Flag::ParliamentSuspended);
        let player_seats = ctx.state.npcs.coalition_seats();
        let majority_satisfied = if parliament_suspended {
            true
        } else {
            player_seats as f32 > self.required_majority
        };
        let warn_majority =
            self.required_majority > 0. && !majority_satisfied;

        if warn_majority {
            // ui.image(icon_from_slug(icons::WARNING));
            ui.label(t!(
                    "Because of opposition, this requires a majority in parliament."
            ));
        }

        let image = flavor_image(&self.flavor.image);
        ui.add(image);

        let has_points = self.kind != ProjectType::Policy
            && self.is_building();
        if has_points {
            for i in 1..consts::MAX_POINTS {
                let tip = tip(
                    self.kind.icon(),
                    t!(
                        "%{points} %{kind} points are allocated to this project",
                        points = self.points,
                        kind = self.kind.lower()
                    ),
                );
                let empty = i >= self.points;
                let icon = self.kind.icon();
                // add_tip(
                //     tip,
                //     if empty {
                //         // TODO
                //         ui.image(icon_from_slug(icon))
                //     } else {
                //         ui.image(icon_from_slug(icon))
                //     },
                // );
            }
        }

        let npcs = &ctx.state.npcs;
        let opposers = self
            .opposers
            .iter()
            .map(|id| npcs[id].clone())
            .filter(|npc| !npc.locked)
            .collect::<Vec<_>>();
        let supporters = self
            .supporters
            .iter()
            .map(|id| npcs[id].clone())
            .filter(|npc| !npc.locked)
            .collect::<Vec<_>>();
        let has_opposers = !opposers.is_empty();
        let has_supporters = !supporters.is_empty();

        if has_opposers {
            for npc in opposers {
                let tip = tip(
                    npc.icon(),
                    t!(
                        "%{name} is opposed to this. If you implement it, your relationship will worsen by -[i]%{icon}[/i].",
                        name = t!(&npc.name),
                        icon = icons::RELATIONSHIP,
                    ),
                );
                // add_tip(tip, ui.image(npc.icon()));
            }
        }
        if has_supporters {
            for npc in supporters {
                let tip = tip(
                    npc.icon(),
                    t!(
                        "%{name} supports this. If you implement it, your relationship will improve by +[i]%{icon}[/i].",
                        name = t!(&npc.name),
                        icon = icons::RELATIONSHIP,
                    ),
                );
                // add_tip(tip, ui.image(npc.icon()));
            }
        }
    }

    fn name(&self, ui: &mut egui::Ui, ctx: &CardState) {
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new(&self.name).heading());
        });
    }

    fn body(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let passed = self.kind == ProjectType::Policy
            && (self.is_building() || self.is_online());
        if passed {
            let stamp = image!("stamp.svg");
            ui.image(stamp);
        }

        let visible_effect = |d: &DisplayEffect| -> bool {
            !matches!(
                d.effect,
                EngineEffect::ProjectRequest(..)
                    | EngineEffect::ProcessRequest(..)
            )
        };

        let effects = active_effects(self)
            .into_iter()
            .filter(visible_effect)
            .collect::<Vec<_>>();

        render_effects(ui, ctx.state, &effects);

        let is_building = self.is_building();
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
                if let Some(changes) =
                    ctx.plan_changes.get(&self.id)
                {
                    if changes.downgrades > 0 {
                        cost = 0;
                    }
                }
                let effects: Vec<DisplayEffect> = upgrade
                    .effects
                    .iter()
                    .map(|e| e.into())
                    .collect();
                Some((cost, effects))
            }
        };
        let is_upgrading =
            ctx.queued_upgrades.get(&self.id) == Some(&true);

        if is_active && let Some((cost, effects)) = next_upgrade
        {
            if is_upgrading {
                ui.label(t!(
                    "Upgrading in one planning cycle."
                ));
            } else {
                ui.horizontal_centered(|ui| {
                    ui.label(t!("Next Level"));
                    ui.label(cost.to_string());
                    ui.image(icons::POLITICAL_CAPITAL);
                });
            }
            render_effects(ui, ctx.state, &effects);
        }

        let can_downgrade =
            self.kind == ProjectType::Policy && self.level > 0;
        let has_downgrade = self.is_active() && can_downgrade;
        if has_downgrade {
            ui.label(t!("Prev Level"));

            if can_downgrade {
                let prev_upgrade = {
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
                        if let Some(upgrade) =
                            self.upgrades.get(idx as usize)
                        {
                            let effects: Vec<DisplayEffect> =
                                upgrade
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
                };
                if let Some((_, effects)) = prev_upgrade {
                    render_effects(ui, ctx.state, &effects);
                }
            }
        }

        if is_building {
            let building_term = match self.kind {
                ProjectType::Research => t!("Researching"),
                ProjectType::Initiative => t!("Building"),
                ProjectType::Policy => t!("Passing"),
            };
            ui.label(building_term);
        }
    }

    fn top_back(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let desc = t!(&self.flavor.description);
        ui.label(desc);
    }

    fn bottom_back(&self, ui: &mut egui::Ui, ctx: &CardState) {
        // TODO dedupe this
        let npcs = &ctx.state.npcs;
        let opposers = self
            .opposers
            .iter()
            .map(|id| npcs[id].clone())
            .filter(|npc| !npc.locked)
            .collect::<Vec<_>>();
        let supporters = self
            .supporters
            .iter()
            .map(|id| npcs[id].clone())
            .filter(|npc| !npc.locked)
            .collect::<Vec<_>>();
        let has_opposers = !opposers.is_empty();
        let has_supporters = !supporters.is_empty();

        if has_opposers || has_supporters {
            ui.label(t!("Political Effects"));
            if has_opposers {
                ui.label(t!("Nay"));
                for npc in opposers {
                    let tip = tip(
                        npc.icon(),
                        t!(
                            "%{name} is opposed to this. If you implement it, your relationship will worsen by -[i]%{icon}[/i].",
                            name = t!(&npc.name),
                            icon = icons::RELATIONSHIP,
                        ),
                    );
                    // add_tip(tip, ui.image(npc.icon()));
                }
            }
            if has_supporters {
                ui.label(t!("Yea"));
                for npc in supporters {
                    let tip = tip(
                        npc.icon(),
                        t!(
                            "%{name} supports this. If you implement it, your relationship will improve by +[i]%{icon}[/i].",
                            name = t!(&npc.name),
                            icon = icons::RELATIONSHIP,
                        ),
                    );
                    // add_tip(tip, ui.image(npc.icon()));
                }
            }
        }

        let image_attrib = &self.flavor.image.attribution;
        ui.label(format!("{} {image_attrib}", t!("Image:")));
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

fn remaining_cost(
    project: &Project,
    plan_changes: &BTreeMap<Id, PlanChange>,
) -> String {
    if project.is_online() {
        0.to_string()
    } else if project.is_building() {
        match project.kind {
            ProjectType::Policy => {
                t!("1 planning cycle left").to_string()
            }
            _ => {
                let years = project.years_remaining();
                t!("%{years} yrs left", years = years)
                    .to_string()
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
                if let Some(changes) =
                    plan_changes.get(&project.id)
                {
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

fn card_color(group: &Group) -> (Color32, Color32) {
    match group {
        Group::Restoration => (
            Color32::from_rgb(0x24, 0x7f, 0x24),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Protection => (
            Color32::from_rgb(0x53, 0xa5, 0x53),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Nuclear => (
            Color32::from_rgb(0xff, 0xa5, 0x00),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Agriculture => (
            Color32::from_rgb(0xf5, 0xde, 0xb3),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Control => (
            Color32::from_rgb(0xd8, 0x35, 0x35),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Population => (
            Color32::from_rgb(0x6b, 0x6b, 0xec),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Food => (
            Color32::from_rgb(0xf3, 0xff, 0x56),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Space => (
            Color32::from_rgb(0x25, 0x04, 0x41),
            Color32::from_rgb(0xd0, 0xc0, 0xe4),
        ),
        Group::Geoengineering => (
            Color32::from_rgb(0x61, 0x68, 0x8b),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Electrification => (
            Color32::from_rgb(0xfc, 0xba, 0x03),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Behavior => (
            Color32::from_rgb(0xb8, 0xad, 0x91),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Limits => (
            Color32::from_rgb(0x4B, 0x5A, 0x85),
            Color32::from_rgb(0xff, 0xff, 0xff),
        ),
        Group::Energy => (
            Color32::from_rgb(0xfe, 0xe9, 0x4a),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Materials => (
            Color32::from_rgb(0x5f, 0x29, 0x29),
            Color32::from_rgb(0xff, 0xff, 0xff),
        ),
        Group::Buildings => (
            Color32::from_rgb(0x8f, 0x7e, 0xa9),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
        Group::Cities => (
            Color32::from_rgb(0x56, 0x6b, 0x6a),
            Color32::from_rgb(0xff, 0xff, 0xff),
        ),
        Group::Other => (
            Color32::from_rgb(0xe0, 0xe0, 0xe0),
            Color32::from_rgb(0x00, 0x00, 0x00),
        ),
    }
}
