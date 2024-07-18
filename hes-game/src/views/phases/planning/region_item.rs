use crate::{
    display::{self, AsText},
    icons::{self, HasIcon},
    t,
    views::{
        intensity::{self, IntensityIcon},
        tip,
        HasTip,
        Tip,
    },
    with_state,
};
use hes_engine::{kinds::Output, regions::Region};
use leptos::*;

fn temp_tip() -> Tip {
    tip(
        icons::TEMPERATURE,
        t!("This region's current temperature range."),
    )
}

fn precip_tip() -> Tip {
    tip(
        icons::PRECIPITATION,
        t!("This region's current precipitation range."),
    )
}

fn devel_tip() -> Tip {
    tip(
        icons::DEVELOPMENT,
        t!("This region's progress to the next income level."),
    )
}

fn cont_tip() -> Tip {
    tip(
        icons::CONTENTEDNESS,
        t!("This region's contentedness."),
    )
}

fn hab_tip() -> Tip {
    tip(
        icons::HABITABILITY,
        t!("This region's habitability. Natural disasters and hotter temperatures lower habitability."),
    )
}

fn inc_tip(income: &str) -> Tip {
    tip(
        icons::WEALTH,
        t!("This region has {incomeName} living standards. Higher living standards mean higher material footprints.", incomeName: income),
    )
}

fn demand_tip(
    output: &Output,
    demand: f32,
    percent: String,
) -> Tip {
    let demand = if demand < 1. {
        "<1".to_string()
    } else {
        demand.to_string()
    };
    let icon = output.icon();
    let msg = t!("This region's per-capita demand level for {output}. The total regions's demand is {demand}<img src='{icon}' />. This makes up {percent} of total demand for {output}.",
        output: t!(output.lower()),
        icon: icon,
        demand: demand,
        percent: percent);
    tip(icon, msg)
}

#[component]
pub fn RegionItem(
    #[prop(into)] region: Signal<Region>,
) -> impl IntoView {
    let events = with_state!(|_state, ui, region| {
        ui.annual_region_events.get(&region.id).cloned()
    });

    let contentedness = move || {
        region.with(|region| {
            intensity::scale(
                region.outlook,
                intensity::Variable::Outlook,
            )
        })
    };
    let habitability = move || {
        region.with(|region| {
            intensity::scale(
                region.habitability(),
                intensity::Variable::Habitability,
            )
        })
    };
    let income_tip = move || {
        region.with(|region| {
            let name = t!(region.income.lower());
            inc_tip(&name)
        })
    };
    let income_level =
        move || region.with(|region| region.income_level() + 1);
    let seceded = move || region.with(|region| region.seceded);
    let temp_range =
        move || region.with(|region| region.temp_range());
    let precip_range =
        move || region.with(|region| region.precip_range());
    let devel_bar = move || {
        region.with(move |region| {
            let is_max_level = region.is_max_income();
            let development = region.development;
            let width = move || {
                format!(
                    "{}%",
                    if is_max_level {
                        100.
                    } else {
                        development * 100.
                    }
                )
            };

            view! {
                <div class:max-level=is_max_level>
                    <span>{t!("Development Progress")}: </span>
                    <Show
                        when=move || !is_max_level
                        fallback=move || {
                            view! { <span>{t!("Max Level")}</span> }
                        }
                    >

                        <div class="minibar">
                            <div class="minibar-fill" style:width=width></div>
                        </div>
                    </Show>
                </div>
            }
        })
    };

    let events_display = move || {
        events()
            .unwrap_or_default()
            .iter()
            .map(|ev| {
                let icon = ev.icon.clone();
                view! { <img class="pip-icon" src=icon/> }
            })
            .collect::<Vec<_>>()
    };
    let demand_display = with_state!(|state, _ui, region| {
        region.demand(&state.world.output_demand).items().map(
            |(k, demand)| {
                let per_capita_demand =
                    demand / region.population;
                let int = intensity::output_intensity(
                    per_capita_demand,
                    k,
                );
                let per = display::demand_percent(
                    demand,
                    state.output_demand[k],
                    true,
                );
                let amount = display::output(demand, k);
                (k, int, per, amount)
            },
        )
    });

    let image = move || {
        region.with(|region| {
            format!(
                "/assets/content/images/{}",
                region.flavor.image.fname
            )
        })
    };

    let demand_bars = move || {
        demand_display()
            .into_iter()
            .map(|(key, int, per, amount)| {
                let tip = demand_tip(&key, amount, per);
                view! {
                    <HasTip tip>
                        <IntensityIcon
                            icon=key.icon()
                            intensity=move || int
                            max_pips=4
                        />
                    </HasTip>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="region-item">
            <div class="region-item--info cell">
                <img src=image/>
                <Show when=seceded>
                    <div class="seceded-label">{t!("Seceded")}</div>
                </Show>
                <div>
                    <HasTip tip=temp_tip.into_signal()>
                        <div class="region-stat">
                            <img src=icons::TEMPERATURE/>
                            {temp_range}
                        </div>
                    </HasTip>
                    <HasTip tip=precip_tip.into_signal()>
                        <div class="region-stat">
                            <img src=icons::PRECIPITATION/>
                            {precip_range}
                        </div>
                    </HasTip>
                </div>

                <HasTip tip=devel_tip.into_signal()>{devel_bar}</HasTip>

                <div class="region-disasters">
                    <div>{t!("Recent Disasters")}</div>
                    <div>{events_display}</div>
                </div>
            </div>
            <div class="region-item--intensities cell">
                <HasTip tip=hab_tip.into_signal()>
                    <IntensityIcon
                        icon=icons::HABITABILITY
                        intensity=habitability
                        invert=true
                        max_pips=4
                    />
                </HasTip>
                <HasTip tip=cont_tip.into_signal()>
                    <IntensityIcon
                        icon=icons::CONTENTEDNESS
                        intensity=contentedness
                        invert=true
                        max_pips=4
                    />

                </HasTip>
                <HasTip tip=income_tip.into_signal()>
                    <IntensityIcon
                        icon=icons::WEALTH
                        intensity=income_level
                        invert=true
                        max_pips=4
                    />
                </HasTip>
                {demand_bars}
            </div>
        </div>
    }
}
