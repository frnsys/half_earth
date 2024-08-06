use crate::{
    display::FloatExt,
    icons,
    memo,
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
use hes_engine::State;
use leptos::*;
use std::time::Duration;

#[component]
pub fn Hud() -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();

    let (show_menu, set_show_menu) = create_signal(false);

    let year = memo!(game.world.year);
    let pc = memo!(game.political_capital.max(0));
    let outlook = memo!(game.outlook());
    let emissions = memo!(game.emissions.as_gtco2eq());
    let extinction = memo!(game.world.extinction_rate);
    let temperature = memo!(game.world.temperature);

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
        tracing::debug!("HUD extinction called");
        intensity::scale(
            extinction.get(),
            intensity::Variable::Extinction,
        )
    };
    let warming = move || {
        tracing::debug!("HUD warming called");
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
        tracing::debug!("HUD warming tip called");
        tip(
            icons::WARMING,
            t!(r#"The current global temperature anomaly is +{anomaly}°C. The higher this is, the more unpredictable the climate becomes. <b class="tip-goal">Your goal is to get this below 1°C.</b>"#, anomaly: temperature.get()),
        )
    };

    let biodiversity_tip = move || {
        tracing::debug!("HUD biodiversity tip called");
        let tip_text = t!(
            r#"The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse. <b class="tip-goal">Your goal is to get this to below 20.</b>"#
        );
        crate::views::tip(icons::EXTINCTION_RATE, tip_text)
            .card(with!(|game| factors_card(
                None,
                Var::Biodiversity,
                &game,
            )))
    };

    let emissions_tip = move || {
        tracing::debug!("HUD emissions tip called");
        let tip_text = t!(r#"Current annual emissions are {emissions} gigatonnes. <b class="tip-goal">Your goal is to get this to below 0.</b>"#, emissions: emissions.get().round_to(1));
        crate::views::tip(icons::EMISSIONS, tip_text).card(
            with!(|game| factors_card(
                None,
                Var::Emissions,
                &game,
            )),
        )
    };

    let contentedness_tip = move || {
        tracing::debug!("HUD contentedness tip called");
        let tip_text = t!(
            r#"How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions. <b class="tip-warn">If this goes below 0 you will be removed from power.</b>"#
        );
        crate::views::tip(icons::CONTENTEDNESS, tip_text).card(
            with!(|game| factors_card(
                None,
                Var::Contentedness,
                &game,
            )),
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
