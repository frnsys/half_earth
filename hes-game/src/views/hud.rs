use crate::{
    icons,
    state,
    t,
    vars::Var,
    views::{
        factors::factors_card,
        intensity::{self, IntensityBar},
        menu::Menu,
        tip,
        HasTip,
    },
};
use leptos::*;
use std::time::Duration;

#[component]
pub fn Hud() -> impl IntoView {
    let (show_menu, set_show_menu) = create_signal(false);

    let year = state!(world.year);
    let pc = state!(political_capital.max(0));
    let outlook = state!(outlook());
    let emissions = state!(emissions());
    let extinction = state!(world.extinction_rate);
    let temperature = state!(world.temperature);

    let pc_danger = move || pc.get() <= 20;
    let unhappy = move || outlook.get() < 0.;
    let emissions_up = move || emissions.get() >= 0.;
    let contentedness = move || {
        intensity::scale(
            outlook.get(),
            intensity::Variable::WorldOutlook,
        )
    };
    let extinction = move || {
        intensity::scale(
            extinction.get(),
            intensity::Variable::Extinction,
        )
    };
    let warming = move || {
        intensity::scale(
            temperature.get(),
            intensity::Variable::Warming,
        )
    };

    let pc_tip = move || {
        tip(
            icons::POLITICAL_CAPITAL,
            t!(
                r#"How much political capital you have. Political capital is what you spend to implement your plans. <b class="tip-warn">If you run out you'll be pushed out of government.</b>"#
            ),
        )
    };

    let warming_tip = move || {
        tip(
            icons::WARMING,
            t!(r#"The current global temperature anomaly is +{anomaly}°C. The higher this is, the more unpredictable the climate becomes. <b class="tip-goal">Your goal is to get this below 1°C.</b>"#, anomaly: temperature.get()),
        )
    };

    let state =
        expect_context::<RwSignal<crate::state::GameState>>();
    let biodiversity_tip = move || {
        let tip_text = t!(
            r#"The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse. <b class="tip-goal">Your goal is to get this to below 20.</b>"#
        );
        crate::views::tip(icons::EXTINCTION_RATE, tip_text)
            .card(factors_card(
                None,
                Var::Biodiversity,
                &state.with(|state| state.game.clone()),
            ))
    };

    let emissions_gt = state!(emissions_gt());
    let emissions_tip = move || {
        let tip_text = t!(r#"Current annual emissions are {emissions} gigatonnes. <b class="tip-goal">Your goal is to get this to below 0.</b>"#, emissions: emissions_gt.get());
        crate::views::tip(icons::EMISSIONS, tip_text).card(
            factors_card(
                None,
                Var::Emissions,
                &state.with(|state| state.game.clone()),
            ),
        )
    };

    let contentedness_tip = move || {
        let tip_text = t!(
            r#"How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions. <b class="tip-warn">If this goes below 0 you will be removed from power.</b>"#
        );
        crate::views::tip(icons::CONTENTEDNESS, tip_text).card(
            factors_card(
                None,
                Var::Contentedness,
                &state.with(|state| state.game.clone()),
            ),
        )
    };

    view! {
        <AnimatedShow
            when=show_menu
            show_class="menu-leave-active"
            hide_class="menu-leave-to"
            hide_delay=Duration::from_millis(1000)
        >
            <div>
                <Menu set_open=set_show_menu/>
            </div>
        </AnimatedShow>
        <div class="hud">
            <div class="hud-year">
                <div>{year}</div>
            </div>
            <div class="hud-bars">
                <HasTip tip=pc_tip.into_signal()>
                    <div class:warnPc=pc_danger>
                        <img src=icons::HUD_POLITICAL_CAPITAL/>
                        {pc}
                    </div>
                </HasTip>
                <HasTip tip=biodiversity_tip.into_signal()>
                    <div>
                        <img src=icons::HUD_EXTINCTION_RATE/>
                        <IntensityBar intensity=extinction.into_signal()/>
                    </div>
                </HasTip>
                <HasTip tip=contentedness_tip.into_signal()>
                    <div class:bad=unhappy>
                        <img src=icons::HUD_CONTENTEDNESS/>
                        <IntensityBar
                            intensity=contentedness.into_signal()
                            invert=true
                        />
                    </div>
                </HasTip>
                <HasTip tip=warming_tip.into_signal()>
                    <div>
                        <img src=icons::HUD_WARMING/>
                        <IntensityBar intensity=warming.into_signal()/>
                    </div>
                </HasTip>
                <HasTip tip=emissions_tip.into_signal()>
                    <div>
                        <img src=icons::HUD_EMISSIONS/>
                        <Show
                            when=emissions_up
                            fallback=move || {
                                view! { <span class="emissions-down">"↓"</span> }
                            }
                        >

                            <span class="emissions-up">"↑"</span>
                        </Show>
                    </div>
                </HasTip>
            </div>
            <div
                class="hud-settings"
                on:click=move |_| { set_show_menu.set(true) }
            >
                <img src=icons::SETTINGS/>
                <span>{t!("Menu")}</span>
            </div>
        </div>
    }
}
