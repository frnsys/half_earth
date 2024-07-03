use super::parts::IntensityBar;
use crate::{display::intensity, icons, state, t};
use leptos::*;
use std::time::Duration;

#[component]
pub fn Hud() -> impl IntoView {
    let (show_menu, set_show_menu) = create_signal(false);

    let year = state!(|game, _| game.world.year);
    let pc_danger = state!(|game, _| game.political_capital <= 20);
    let pc = state!(|game, _| game.political_capital.max(0));
    let unhappy = state!(|game, _| game.outlook() < 0.);
    let emissions_up = state!(|game, _| game.emissions() >= 0.);
    let contentedness =
        state!(|game, _| { intensity::scale(game.outlook(), intensity::Variable::WorldOutlook) });
    let extinction = state!(|game, _| {
        intensity::scale(game.world.extinction_rate, intensity::Variable::Extinction)
    });
    let warming = state!(|game, _| {
        intensity::scale(game.world.temperature, intensity::Variable::Warming)
    });

    // TODO
    // pcTip() {
    //   return {
    //     icon: 'political_capital',
    //     text: t('How much political capital you have. Political capital is what you spend to implement your plans. <b class="tip-warn">If you run out you\'ll be pushed out of government.</b>')
    //   };
    // },
    // warmingTip() {
    //   return {
    //     icon: 'warming',
    //     text: t(`The current global temperature anomaly is +{anomaly}°C. The higher this is, the more unpredictable the climate becomes. <b class="tip-goal">Your goal is to get this below 1°C.</b>`, {anomaly: state.gameState.world.temperature.toFixed(1)})
    //   };
    // },
    // biodiversityTip() {
    //   return factors.tips.biodiversity(
    //     t('The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse. <b class="tip-goal">Your goal is to get this to below 20.</b>'));
    // },
    // contentednessTip() {
    //   return factors.tips.contentedness(
    //     t('How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions. <b class="tip-warn">If this goes below 0 you will be removed from power.</b>'));
    // },
    // emissionsTip() {
    //   return factors.tips.emissions(
    //     t(`Current annual emissions are {emissions} gigatonnes. <b class="tip-goal">Your goal is to get this to below 0.</b>`, {emissions: state.gameState.world.emissions_gt().toFixed(1)}));
    // }
    //

    view! {
        <AnimatedShow
            when=show_menu
            show_class="mean-leave-active"
            hide_class="menu-leave-to"
            hide_delay=Duration::from_millis(1000)
        >
            <div>TODO</div>
        </AnimatedShow>
        <div class="hud">
            <div class="hud-year">
                <div>{year}</div>
            </div>
            <div class="hud-bars">
                <div v-tip="pcTip" class:warnPc=pc_danger>
                    <img src=icons::HUD_POLITICAL_CAPITAL/>
                    {pc}
                </div>
                <div v-tip="biodiversityTip">
                    <img src=icons::HUD_EXTINCTION_RATE/>
                    <IntensityBar intensity=extinction.into_signal()/>
                </div>
                <div class:bad=unhappy v-tip="contentednessTip">
                    <img src=icons::HUD_CONTENTEDNESS/>
                    <IntensityBar
                        intensity=contentedness.into_signal()
                        invert=true
                    />
                </div>
                <div v-tip="warmingTip">
                    <img src=icons::HUD_WARMING/>
                    <IntensityBar intensity=warming.into_signal()/>
                </div>
                <div v-tip="emissionsTip">
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
