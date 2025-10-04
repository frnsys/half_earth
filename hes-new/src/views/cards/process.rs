use std::{borrow::Cow, collections::BTreeMap};

use crate::{
    consts,
    display::{
        self,
        AsText,
        DisplayEffect,
        FloatExt,
        HasIcon,
        emissions,
        factors::factors_card,
        icon_from_slug,
        icons,
        intensity::{self, render_intensity_bar},
    },
    image,
    state::{PlanChange, StateExt},
    vars::{Impact, Var},
    views::{
        Tip,
        cards::CardState,
        events::{active_effects, render_effects},
        parts::flex_justified,
        tip,
        tips::add_tip,
    },
};

use super::AsCard;
use egui::{Color32, Margin};
use egui_taffy::{Tui, TuiBuilderLogic, taffy, tui};
use hes_engine::{
    Effect as EngineEffect,
    Feedstock,
    Flag,
    Group,
    Id,
    Process,
    Project,
    ProjectType,
};
use rust_i18n::t;

fn describe_estimate(estimate: f32) -> Cow<'static, str> {
    if estimate == 0. {
        t!(
            "This feedstock is depleted, so this process is stopped. You should reallocate its points to other processes."
        )
    } else if estimate.is_finite() {
        t!(
            "At current usage rates the estimated supply is expected to last %{years} years.",
            years = estimate
        )
    } else {
        t!(
            "At current usage rates the estimated supply is expected to last indefinitely."
        )
    }
}

fn describe_stocks(estimate: f32) -> &'static str {
    if estimate < 20. {
        "low"
    } else if estimate < 50. {
        "mid"
    } else if estimate < 80. {
        "high"
    } else {
        "very-high"
    }
}

impl AsCard for Process {
    fn bg_color(&self) -> egui::Color32 {
        egui::Color32::from_gray(20)
    }

    fn fg_color(&self) -> Color32 {
        egui::Color32::WHITE
    }

    fn header(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let output_icon = self.output.icon();
        let output_name = t!(&self.output.title());

        let produced_by_process = ctx
            .state
            .produced
            .by_process
            .get(&self.id)
            .unwrap_or(&0.);

        let (produced, emissions) = {
            let base_amount = *produced_by_process;
            let mut amount =
                display::output(base_amount, self.output);
            if amount > 0. {
                amount = amount.max(1.);
            }
            let gtco2eq = self.byproducts.gtco2eq();
            let mut emissions = gtco2eq * base_amount;
            if emissions > 0. {
                emissions = emissions.max(1.);
            }
            (amount, emissions.round_to(1))
        };
        let output_tip = {
            tip(
                self.output.icon(),
                t!(
                    "This process currently produces %{amount}[i]%{outputIcon}[/i] and %{emissions}[i]%{emissionsIcon}[/i] per year.",
                    emissions = emissions,
                    amount = produced,
                    emissionsIcon = icons::EMISSIONS,
                    outputIcon = self.output.icon()
                ),
            )
        };

        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(4, 6))
            .show(ui, |ui| {
                flex_justified(ui, &self.name, |tui| {
                    tui.style(taffy::Style {
                        flex_grow: 1.,
                        ..Default::default()
                    })
                    .label(output_name);

                    tui.ui(|ui| {
                        add_tip(
                            output_tip,
                            ui.horizontal_centered(|ui| {
                                ui.label(produced.to_string());
                                // ui.image(icon_from_slug(
                                //     output_icon,
                                // ));
                                ui.label(emissions.to_string());
                                // ui.image(icon_from_slug(
                                //     icons::EMISSIONS,
                                // ));
                            })
                            .response,
                        );
                    });
                });
            });

        let is_new = !ctx.viewed.contains(&self.id);
        // if is_new {
        //     let new_icon = image!("new.svg");
        //     ui.image(new_icon);
        // }
    }

    fn figure(&self, ui: &mut egui::Ui, ctx: &CardState) {
        // TODO
        // let image = self.flavor.image.src();
        // ui.image(image);

        let max_share = ctx.state.process_max_share(&self.id);

        if max_share < 20 {
            let changed_mix_share = {
                if let Some(change) = ctx.process_mix_changes
                    [self.output]
                    .get(&self.id)
                {
                    self.mix_share as isize + change
                } else {
                    self.mix_share as isize
                }
            };

            let alert_tip = {
                let mix_share = self.mix_share;
                tip(
                    icons::ALERT,
                    t!(
                        "Because of resource availability this process can only make up to %{maxPercent}% of production. %{suggestion}",
                        maxPercent = max_share * 5,
                        suggestion = if mix_share > max_share
                            || changed_mix_share
                                > max_share as isize
                        {
                            t!(
                                "You should reallocate its points to other processes."
                            )
                        } else {
                            "".into()
                        }
                    ),
                )
            };
            // add_tip(
            //     alert_tip,
            //     ui.image(icon_from_slug(icons::ALERT)),
            // );
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
            ui.label(&self.name);
        });
    }

    fn body(&self, ui: &mut egui::Ui, ctx: &CardState) {
        // TODO dedupe
        let max_share = ctx.state.process_max_share(&self.id);
        let changed_mix_share = {
            if let Some(change) = ctx.process_mix_changes
                [self.output]
                .get(&self.id)
            {
                self.mix_share as isize + change
            } else {
                self.mix_share as isize
            }
        };

        let process_excess = {
            self.mix_share > max_share
                || changed_mix_share > max_share as isize
        };
        if process_excess {
            let excess_tip = tip(
                icons::ALERT,
                t!(
                    "This process can't produce this much because of feedstock or other limits. You should reallocate its points to other processes."
                ),
            );
            // add_tip(
            //     excess_tip,
            //     ui.image(icon_from_slug(icons::ALERT)),
            // );
        }

        let change_tip = {
            let output = t!(self.output.lower());
            let mix_percent = self.mix_share * 5;
            tip(
                icons::MIX_TOKEN,
                t!(
                    "This process currently makes up %{mixPercent}% of %{output} production.",
                    output = output,
                    mixPercent = mix_percent
                ),
            )
        };

        // TODO dedupe
        let feedstocks = &ctx.state.feedstocks;
        let feedstock_estimate = {
            let feedstock = self.feedstock.0;
            match feedstock {
                Feedstock::Soil | Feedstock::Other => None,
                _ => {
                    let estimate =
                        feedstocks.until_exhaustion(feedstock);
                    Some(estimate.round())
                }
            }
        };

        // TODO use this
        let depleted = feedstock_estimate == Some(0.);

        let change = {
            if let Some(change) = ctx.process_mix_changes
                [self.output]
                .get(&self.id)
            {
                *change
            } else {
                0
            }
        };
        let has_change = change != 0;
        let mix_share_percent = self.mix_share * 5;

        let is_shrink =
            (self.mix_share as isize) > changed_mix_share;
        let is_grow =
            (self.mix_share as isize) < changed_mix_share;
        let changed_mix_share_percent = changed_mix_share * 5;

        add_tip(
            change_tip,
            ui.horizontal_centered(|ui| {
                ui.label(format!("{mix_share_percent}%"));
                if has_change {
                    // ui.image(icon_from_slug(
                    //     icons::ARROW_RIGHT,
                    // ));
                    if is_shrink {
                        // TODO
                    } else if is_grow {
                        // TODO
                    }
                    ui.label(format!(
                        "{changed_mix_share_percent}%"
                    ));
                }
            })
            .response,
        );

        ui.horizontal_centered(|ui| {
            let land_intensity =  {
                let usage = self.adj_resources().land;
                let int = intensity::impact_intensity(
                    usage,
                    Impact::Land,
                    self.output.into(),
                );
                let percent = ctx.state.land_use_percent();
                let tip = tip(icons::LAND, t!("Land: They're not making anymore of it. You're using %{percent} of land.", percent=percent))
                    .card(factors_card(Some(self.name.clone()), Var::Land, ctx.state));
                (tip, icons::LAND, int)
            };
            let water_intensity =  {
                let usage = self.adj_resources().water;
                let int = intensity::impact_intensity(
                    usage,
                    Impact::Water,
                    self.output.into(),
                );
                let percent = ctx.state.water_use_percent();
                let tip = tip(icons::WATER, t!("Water: The giver of life. You're using %{percent} of water resources.", percent=percent))
                    .card(factors_card(Some(self.name.clone()), Var::Water, ctx.state));
                (tip, icons::WATER, int)
            };
            let energy_intensity =  {
                let usage = self.adj_resources().energy();
                let int = intensity::impact_intensity(
                    usage,
                    Impact::Energy,
                    self.output.into(),
                );
                let amount = ctx.state.energy_twh();
                let tip = tip(icons::ENERGY, t!("Energy: The fundamental mover. You're using %{amount}TWh of energy.", amount=amount))
                    .card(factors_card(Some(self.name.clone()), Var::Energy, ctx.state));
                (tip, icons::ENERGY, int)
            };
            let emissions_intensity =  {
                let usage = self.adj_byproducts().co2eq();
                let int = intensity::impact_intensity(
                    usage,
                    Impact::Emissions,
                    self.output.into(),
                );
                let amount = ctx.state.emissions.as_gtco2eq();
                let tip = tip(icons::EMISSIONS, t!("Emissions: A shroud around the earth. You're emitting %{amount} gigatonnes per year.", amount=amount))
                    .card(factors_card(Some(self.name.clone()), Var::Emissions, ctx.state));
                (tip, icons::EMISSIONS, int)
            };
            let biodiversity_intensity =  {
                let usage = self.extinction_rate(
                    ctx.state.world.starting_resources.land,
                );
                let int = intensity::impact_intensity(
                    usage,
                    Impact::Biodiversity,
                    self.output.into(),
                );
                let amount = ctx.state.world.extinction_rate;
                let tip = tip(icons::EXTINCTION_RATE, t!("Biodiversity: The co-inhabitants of the planet. The current biodiversity threat index is %{amount}.", amount=amount))
                    .card(factors_card(Some(self.name.clone()), Var::Biodiversity, ctx.state));
                (tip, icons::EXTINCTION_RATE, int)
            };

            for (tip, icon, intensity) in [land_intensity, water_intensity, energy_intensity, emissions_intensity, biodiversity_intensity] {
                add_tip(tip, ui.vertical_centered(|ui| {
                    // ui.image(icon_from_slug(
                    //         icon,
                    // ));
                    // render_intensity_bar(
                    //     ui,
                    //     intensity,
                    //     false,
                    // );
                }).response);
            }
        });
    }

    fn top_back(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let desc = t!(&self.flavor.description);
        ui.label(desc);
    }

    fn bottom_back(&self, ui: &mut egui::Ui, ctx: &CardState) {
        let feedstocks = &ctx.state.feedstocks;
        let feedstock_estimate = {
            let feedstock = self.feedstock.0;
            match feedstock {
                Feedstock::Soil | Feedstock::Other => None,
                _ => {
                    let estimate =
                        feedstocks.until_exhaustion(feedstock);
                    Some(estimate.round())
                }
            }
        };

        let is_halted =
            feedstock_estimate.is_some_and(|est| est == 0.);
        let almost_halted =
            feedstock_estimate.is_some_and(|est| est < 0.);
        if almost_halted {
            // ui.image(icon_from_slug(icons::ALERT));
        } else if is_halted {
            // ui.image(icon_from_slug(icons::HALTED));
        }

        let has_feedstock =
            self.feedstock.0 != Feedstock::Other;
        if has_feedstock {
            let icon = self.feedstock.0.icon();
            let desc = feedstock_estimate
                .map(describe_estimate)
                .unwrap_or_default();
            let tip = tip(
                icon,
                t!(
                    "This process uses %{feedstockName}. %{feedstockEstimateDesc}",
                    feedstockName =
                        t!(self.feedstock.0.lower()),
                    feedstockEstimateDesc = desc
                ),
            );
            // add_tip(tip, ui.image(icon_from_slug(icon)));
        }

        if feedstock_estimate.is_some() {
            // TODO feedstock level display
            let level = feedstock_estimate
                .map(describe_stocks)
                .unwrap_or("high");
        }

        for feat in &self.features {
            let tip = tip(feat.icon(), t!(feat.title()));
            // add_tip(tip, ui.image(icon_from_slug(feat.icon())));
        }

        let image_attrib = &self.flavor.image.attribution;
        ui.label(format!("{} {image_attrib}", t!("Image:")));
    }
}

fn render_mix_bar(
    ui: &mut egui::Ui,
    mix_share: usize,
    max_share: usize,
    changed_mix_share: isize,
    is_depleted: bool,
) {
    let process_mix_tip = tip(
        icons::MIX_TOKEN,
        if max_share < 20 {
            t!(
                "Because of resource availability this process can only make up to %{maxPercent}% of production.",
                maxPercent = max_share * 5
            )
        } else {
            t!(
                "There is currently no limit on this process' mix share."
            )
        },
    );

    add_tip(
        process_mix_tip,
        ui.vertical(|ui| {
            for i in 1..=20 {
                let disabled = i > max_share;
                let active = i <= mix_share;
                let grow = i > mix_share
                    && (i as isize <= changed_mix_share);
                let shrink = i <= mix_share
                    && (i as isize > changed_mix_share);
                let excess = (i <= mix_share
                    || (i as isize <= changed_mix_share))
                    && i > max_share;

                // TODO
                // view! {
                //     <div
                //         class="process-mix-cell"
                //         class:active=active
                //         class:depleted=is_depleted
                //         class:shrink=shrink
                //         class:grow=grow
                //         class:excess=excess
                //         class:disabled=disabled
                //     ></div>
                // }
            }
        })
        .response,
    );
}
