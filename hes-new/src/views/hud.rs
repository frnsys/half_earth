use egui::Button;
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use hes_engine::State;
use rust_i18n::t;

use crate::{
    display::{
        FloatExt,
        factors::factors_card,
        icon_from_slug,
        icons,
        intensity::{self, render_intensity_bar},
    },
    vars::Var,
    views::{tip, tips::add_tip},
};

pub enum HudAction {
    OpenMenu,
}

pub fn render_hud(
    ui: &mut egui::Ui,
    state: &mut State,
) -> Option<HudAction> {
    let year = state.world.year;
    let pc = state.political_capital.max(0);
    let outlook = state.outlook();
    let emissions = state.emissions.as_gtco2eq();
    let extinction = state.world.extinction_rate;
    let temperature = state.world.temperature;

    let pc_danger = pc <= 20;
    let unhappy = outlook < 0.;
    let emissions_up = emissions >= 0.;
    let contentedness = intensity::scale(
        outlook,
        intensity::Variable::WorldOutlook,
    );
    let extinction = intensity::scale(
        extinction,
        intensity::Variable::Extinction,
    );
    let warming = intensity::scale(
        temperature,
        intensity::Variable::Warming,
    );

    let pc_tip = tip(
        icons::POLITICAL_CAPITAL,
        t!(
            r#"How much political capital you have. Political capital is what you spend to implement your plans. [w]If you run out you'll be pushed out of government.[/w]"#
        ),
    );

    let warming_tip = tip(
        icons::WARMING,
        t!(
            r#"The current global temperature anomaly is +%{anomaly}°C. The higher this is, the more unpredictable the climate becomes. [g]Your goal is to get this below 1°C.[/g]"#,
            anomaly = temperature.round_to(2)
        ),
    );

    let biodiversity_tip = {
        let tip_text = t!(
            r#"The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse. [g]Your goal is to get this to below 20.[/g]"#
        );
        tip(icons::EXTINCTION_RATE, tip_text)
            .card(factors_card(None, Var::Biodiversity, state))
    };

    let emissions_tip = {
        let tip_text = t!(
            r#"Current annual emissions are %{emissions} gigatonnes. [g]Your goal is to get this to below 0.[/g]"#,
            emissions = emissions.round_to(1)
        );
        tip(icons::EMISSIONS, tip_text).card(factors_card(
            None,
            Var::Emissions,
            state,
        ))
    };

    let contentedness_tip = {
        let tip_text = t!(
            r#"How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions. [w]If this goes below 0 you will be removed from power.[/w]"#
        );
        tip(icons::CONTENTEDNESS, tip_text).card(factors_card(
            None,
            Var::Contentedness,
            state,
        ))
    };

    tui(ui, ui.id().with("top-bar"))
        .reserve_available_width()
        .style(taffy::Style {
            flex_grow: 1.,
            flex_direction: taffy::FlexDirection::Row,
            min_size: taffy::Size {
                width: taffy::prelude::percent(1.),
                height: taffy::prelude::auto(),
            },
            align_items: Some(taffy::AlignItems::Center),
            justify_content: Some(
                taffy::JustifyContent::SpaceBetween,
            ),
            ..Default::default()
        })
        .show(|tui| {
            tui.ui(|ui| {
                ui.set_min_width(50.);
                ui.label(year.to_string());
            });

            tui.ui(|ui| {
                ui.horizontal(|ui| {
                    add_tip(
                        pc_tip,
                        ui.horizontal(|ui| {
                            // <div class:warnPc=pc_danger> // TODO
                            ui.image(icon_from_slug(
                                icons::HUD_POLITICAL_CAPITAL,
                            ));
                            ui.label(pc.to_string());
                        })
                        .response,
                    );

                    add_tip(
                        biodiversity_tip,
                        ui.horizontal(|ui| {
                            ui.image(icon_from_slug(
                                icons::HUD_EXTINCTION_RATE,
                            ));
                            render_intensity_bar(
                                ui, extinction, false,
                            );
                        })
                        .response,
                    );

                    add_tip(
                        contentedness_tip,
                        ui.horizontal(|ui| {
                            // <div class:bad=unhappy>
                            ui.image(icon_from_slug(
                                icons::HUD_CONTENTEDNESS,
                            ));
                            render_intensity_bar(
                                ui,
                                contentedness,
                                true,
                            );
                        })
                        .response,
                    );

                    add_tip(
                        warming_tip,
                        ui.horizontal(|ui| {
                            ui.image(icon_from_slug(
                                icons::HUD_WARMING,
                            ));
                            render_intensity_bar(
                                ui, warming, false,
                            );
                        })
                        .response,
                    );

                    add_tip(
                        emissions_tip,
                        ui.horizontal(|ui| {
                            ui.image(icon_from_slug(
                                icons::HUD_EMISSIONS,
                            ));
                            let sym = if emissions_up {
                                "↑"
                            } else {
                                "↓"
                            };
                            ui.label(sym);
                        })
                        .response,
                    );
                });
            });

            let button = Button::image_and_text(
                icon_from_slug(icons::SETTINGS),
                t!("Menu"),
            )
            .wrap_mode(egui::TextWrapMode::Extend);
            let resp = tui.ui_add(button);
            if resp.clicked() {
                return Some(HudAction::OpenMenu);
            }

            None
        })
}
