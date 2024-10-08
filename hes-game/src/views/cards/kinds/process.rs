use super::super::card::*;
use crate::{
    display::{self, AsText, FloatExt},
    icons::{self, HasIcon},
    memo,
    state::{StateExt, UIState},
    t,
    util::{scale_text, to_ws_el, ImageExt},
    vars::*,
    views::{
        factors::factors_card,
        intensity::{self, IntensityIcon},
        tip,
        HasTip,
    },
};
use hes_engine::{Feedstock, Process, State};
use html::ToHtmlElement;
use leptos::*;

fn describe_estimate(estimate: f32) -> String {
    if estimate == 0. {
        t!("This feedstock is depleted, so this process is stopped. You should reallocate its points to other processes.")
    } else if estimate.is_finite() {
        t!("At current usage rates the estimated supply is expected to last {years} years.", years: estimate)
    } else {
        t!("At current usage rates the estimated supply is expected to last indefinitely.")
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

#[component]
pub fn ProcessCard(
    #[prop(into)] process: Signal<Process>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();
    let ui = expect_context::<RwSignal<UIState>>();

    let viewed = memo!(ui.viewed);
    let is_new = move || {
        with!(|viewed, process| !viewed.contains(&process.id))
    };
    let name = move || with!(|process| t!(&process.name));
    let description = move || {
        with!(|process| t!(&process.flavor.description))
    };
    let output_icon =
        move || with!(|process| process.output.icon());
    let output_name =
        move || with!(|process| t!(&process.output.title()));

    let feedstocks = memo!(game.feedstocks);
    let feedstock_estimate = move || {
        with!(|process, feedstocks| {
            let feedstock = process.feedstock.0;
            match feedstock {
                Feedstock::Soil | Feedstock::Other => None,
                _ => {
                    let estimate =
                        feedstocks.until_exhaustion(feedstock);
                    Some(estimate.round())
                }
            }
        })
    };
    let feedstock_estimate_desc = move || {
        let estimate = feedstock_estimate();
        estimate.map(describe_estimate).unwrap_or_default()
    };
    let feedstock_icon =
        move || with!(|process| process.feedstock.0.icon());

    let feedstock_level = move || {
        let estimate = feedstock_estimate();
        estimate.map(describe_stocks).unwrap_or("high")
    };
    let has_feedstock = move || {
        with!(|process| {
            process.feedstock.0 != Feedstock::Other
        })
    };

    let id = memo!(process.id);
    let produced_by_process = create_memo(move |_| {
        with!(|game| *game
            .produced
            .by_process
            .get(&id.get())
            .unwrap_or(&0.))
    });
    let produced = move || {
        with!(|produced_by_process, process| {
            let base_amount = *produced_by_process;
            let mut amount =
                display::output(base_amount, process.output);
            if amount > 0. {
                amount = amount.max(1.);
            }
            let gtco2eq = process.byproducts.gtco2eq();
            let mut emissions = gtco2eq * base_amount;
            if emissions > 0. {
                emissions = emissions.max(1.);
            }
            (amount, emissions.round_to(1))
        })
    };
    let output_tip = move || {
        with!(|process| {
            let output = process.output;
            let (amount, emissions) = produced();
            tip(
                output.icon(),
                t!("This process currently produces {amount}<img src='{outputIcon}'> and {emissions}<img src='{emissionsIcon}'> per year.",
                    emissions: emissions,
                    amount: amount,
                    emissionsIcon: icons::EMISSIONS,
                    outputIcon: output.icon()),
            )
        })
    };
    let change_tip = move || {
        with!(|process| {
            let output = t!(process.output.lower());
            let mix_percent = process.mix_share * 5;
            tip(
                icons::MIX_TOKEN,
                t!(
                    "This process currently makes up {mixPercent}% of {output} production.",
                    output: output,
                    mixPercent: mix_percent
                ),
            )
        })
    };

    let name_memo = memo!(process.name);
    let name_ref = create_node_ref::<html::Div>();
    create_effect(move |_| {
        name_memo.track();
        if let Some(name_ref) = name_ref.get() {
            scale_text(
                to_ws_el(
                    name_ref
                        .parent_element()
                        .unwrap()
                        .to_leptos_element(),
                ),
                14,
            );
        }
    });

    let class = move || {
        if is_new() {
            "is-new".to_string()
        } else {
            "".to_string()
        }
    };

    let process_mix_changes = memo!(ui.process_mix_changes);
    let changed_mix_share = move || {
        with!(|process_mix_changes, process| {
            if let Some(change) = process_mix_changes
                [process.output]
                .get(&process.id)
            {
                process.mix_share as isize + change
            } else {
                process.mix_share as isize
            }
        })
    };

    let id = memo!(process.id);
    let max_share = memo!(game.process_max_share(&id.get()));
    let alert_tip = move || {
        with!(|max_share, process| {
            let mix_share = process.mix_share;
            tip(
                icons::ALERT,
                t!("Because of resource availability this process can only make up to {maxPercent}% of production. {suggestion}",
                    maxPercent: max_share * 5,
                    suggestion: if mix_share > *max_share || changed_mix_share() > *max_share as isize {
                        t!("You should reallocate its points to other processes.")
                    } else {
                        "".into()
                    }
                ),
            )
        })
    };

    let npcs = memo!(game.npcs);
    let has_opposers = move || {
        with!(|npcs, process| {
            process
                .opposers
                .iter()
                .map(|id| &npcs[id])
                .filter(|npc| !npc.locked)
                .next()
                .is_some()
        })
    };
    let has_supporters = move || {
        with!(|npcs, process| {
            process
                .supporters
                .iter()
                .map(|id| &npcs[id])
                .filter(|npc| !npc.locked)
                .next()
                .is_some()
        })
    };

    let opposers = move || {
        with!(|npcs, process| {
            process.opposers.iter().map(|id| &npcs[id])
                .filter(|npc| !npc.locked)
                .cloned()
                .map(|npc| {
                    let tip = tip(npc.icon(), t!("{name} is opposed to this. If you implement it, your relationship will worsen by -<img src='{icon}' />.",
                            name: t!(&npc.name),
                            icon: icons::RELATIONSHIP,
                            ));
                    view! {
                        <HasTip tip>
                            <img src=npc.icon() />
                        </HasTip>
                    }
            }).collect::<Vec<_>>()
        })
    };
    let supporters = move || {
        with!(|npcs, process| {
            process.supporters.iter().map(|id| &npcs[id])
                .filter(|npc| !npc.locked)
                .cloned()
                .map(|npc| {
                    let tip = tip(npc.icon(), t!("{name} supports this. If you implement it, your relationship will improve by +<img src='{icon}' />.",
                            name: t!(&npc.name),
                            icon: icons::RELATIONSHIP,
                            ));
                    view! {
                        <HasTip tip>
                            <img src=npc.icon() />
                        </HasTip>
                    }
            }).collect::<Vec<_>>()
        })
    };

    let image =
        move || with!(|process| process.flavor.image.src());

    let process_excess = move || {
        with!(|process| {
            let max = max_share.get();
            process.mix_share > max
                || changed_mix_share() > max as isize
        })
    };
    let excess_tip = move || {
        tip(icons::ALERT, t!("This process can't produce this much because of feedstock or other limits. You should reallocate its points to other processes."))
    };

    let change = move || {
        with!(|process| {
            if let Some(change) = process_mix_changes.get()
                [process.output]
                .get(&process.id)
            {
                *change
            } else {
                0
            }
        })
    };
    let has_change = move || change() != 0;
    let mix_share_percent =
        move || with!(|process| process.mix_share * 5);
    let is_shrink = move || {
        with!(|process| {
            (process.mix_share as isize) > changed_mix_share()
        })
    };
    let is_grow = move || {
        with!(|process| {
            (process.mix_share as isize) < changed_mix_share()
        })
    };
    let changed_mix_share_percent =
        move || changed_mix_share() * 5;
    let is_halted = move || {
        feedstock_estimate().is_some_and(|est| est == 0.)
    };
    let almost_halted = move || {
        feedstock_estimate().is_some_and(|est| est < 0.)
    };

    let feedstock_tip = move || {
        with!(|process| {
            tip(
                feedstock_icon(),
                t!("This process uses {feedstockName}. {feedstockEstimateDesc}", feedstockName: t!(process.feedstock.0.lower()), feedstockEstimateDesc: feedstock_estimate_desc()),
            )
        })
    };
    let feedstock_bar_class = move || {
        format!(
            "feedstock-remaining-fill feedstock-remaining-fill--{}",
            feedstock_level()
        )
    };
    let feature_icons = move || {
        with!(|process| {
            process
                .features
                .iter()
                .cloned()
                .map(|feat| {
                    let tip = tip(feat.icon(), t!(feat.title()));
                    view! {
                        <HasTip tip>
                            <img class="process--feature" src=feat.icon()/>
                        </HasTip>
                    }
                })
                .collect::<Vec<_>>()
        })
    };
    let image_attrib = move || {
        with!(|process| {
            process.flavor.image.attribution.clone()
        })
    };
    let process_mix_tip = move || {
        let max_share = max_share.get();
        tip(
            icons::MIX_TOKEN,
            if max_share < 20 {
                t!("Because of resource availability this process can only make up to {maxPercent}% of production.", maxPercent: max_share * 5)
            } else {
                t!("There is currently no limit on this process' mix share.")
            },
        )
    };
    let mix_cells = move || {
        let depleted = feedstock_estimate() == Some(0.);
        let max_share = max_share.get();
        let changed_mix_share = changed_mix_share();
        with!(|process| {
            (1..=20)
                .map(|i| {
                    let disabled = i > max_share;
                    let active = i <= process.mix_share;
                    let grow = i > process.mix_share
                        && (i as isize <= changed_mix_share);
                    let shrink = i <= process.mix_share
                        && (i as isize > changed_mix_share);
                    let excess = (i <= process.mix_share
                        || (i as isize <= changed_mix_share))
                        && i > max_share;
                    view! {
                        <div
                            class="process-mix-cell"
                            class:active=active
                            class:depleted=depleted
                            class:shrink=shrink
                            class:grow=grow
                            class:excess=excess
                            class:disabled=disabled
                        ></div>
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    let land_intensity = move || {
        with!(|game, process| {
            let usage = process.adj_resources().land;
            let int = intensity::impact_intensity(
                usage,
                Impact::Land,
                process.output.into(),
            );
            let percent = game.land_use_percent();
            let tip = tip(icons::LAND, t!("Land: They're not making anymore of it. You're using {percent} of land.", percent: percent))
            .card(factors_card(Some(process.name.clone()), Var::Land, game));
            let (sig, _) = create_signal(int);
            view! {
                <HasTip tip>
                    <IntensityIcon icon=icons::LAND intensity=sig />
                </HasTip>
            }
        })
    };
    let water_intensity = move || {
        with!(|game, process| {
            let usage = process.adj_resources().water;
            let int = intensity::impact_intensity(
                usage,
                Impact::Water,
                process.output.into(),
            );
            let percent = game.water_use_percent();
            let tip = tip(icons::WATER, t!("Water: The giver of life. You're using {percent} of water resources.", percent: percent))
        .card(factors_card(Some(process.name.clone()), Var::Water, game));
            let (sig, _) = create_signal(int);
            view! {
                <HasTip tip>
                    <IntensityIcon icon=icons::WATER intensity=sig />
                </HasTip>
            }
        })
    };
    let energy_intensity = move || {
        with!(|game, process| {
            let usage = process.adj_resources().energy();
            let int = intensity::impact_intensity(
                usage,
                Impact::Energy,
                process.output.into(),
            );
            let amount = game.energy_twh();
            let tip = tip(icons::ENERGY, t!("Energy: The fundamental mover. You're using {amount}TWh of energy.", amount: amount))
            .card(factors_card(Some(process.name.clone()), Var::Energy, game));
            let (sig, _) = create_signal(int);
            view! {
                <HasTip tip>
                    <IntensityIcon icon=icons::ENERGY intensity=sig />
                </HasTip>
            }
        })
    };
    let emissions_intensity = move || {
        with!(|game, process| {
            let usage = process.adj_byproducts().co2eq();
            let int = intensity::impact_intensity(
                usage,
                Impact::Emissions,
                process.output.into(),
            );
            let amount = game.emissions.as_gtco2eq();
            let tip = tip(icons::EMISSIONS, t!("Emissions: A shroud around the earth. You're emitting {amount} gigatonnes per year.", amount: amount))
            .card(factors_card(Some(process.name.clone()), Var::Emissions, game));
            let (sig, _) = create_signal(int);
            view! {
                <HasTip tip>
                    <IntensityIcon icon=icons::EMISSIONS intensity=sig />
                </HasTip>
            }
        })
    };
    let biodiversity_intensity = move || {
        with!(|game, process| {
            let usage = process.extinction_rate(
                game.world.starting_resources.land,
            );
            let int = intensity::impact_intensity(
                usage,
                Impact::Biodiversity,
                process.output.into(),
            );
            let amount = game.world.extinction_rate;
            let tip = tip(icons::EXTINCTION_RATE, t!("Biodiversity: The co-inhabitants of the planet. The current biodiversity threat index is {amount}.", amount: amount))
        .card(factors_card(Some(process.name.clone()), Var::Biodiversity, game));
            let (sig, _) = create_signal(int);
            view! {
                <HasTip tip>
                    <IntensityIcon icon=icons::EXTINCTION_RATE intensity=sig />
                </HasTip>
            }
        })
    };

    view! {
        <Card color="#ffffff" class=class.into_signal()>
            <Header slot>
                <div>{output_name}</div>
                <Show when=is_new>
                    <img class="new-card-icon" src="/assets/new.svg"/>
                </Show>
                <HasTip tip=output_tip.into_signal()>
                    <div>
                        {move || produced().0} <img src=output_icon/>
                        {move || produced().1} <img src=icons::EMISSIONS/>
                    </div>
                </HasTip>
            </Header>

            <Name slot><div ref=name_ref>{name}</div></Name>

            <Figure slot>
                <img class="card-image" src=image/>
                <Show when=move || max_share.get() < 20>
                    <HasTip tip=alert_tip.into_signal()>
                        <div class="process-limit-alert">
                            <img src=icons::ALERT/>
                        </div>
                    </HasTip>
                </Show>
                <Show when=has_opposers>
                    <div class="opposers">{opposers}</div>
                </Show>
                <Show when=has_supporters>
                    <div class="supporters">{supporters}</div>
                </Show>
            </Figure>

            <Body slot>
                <div class="process-mix">
                    <Show when=process_excess>
                        <HasTip tip=excess_tip.into_signal()>
                            <div class="process-excess-alert">
                                <img src=icons::ALERT/>
                            </div>
                        </HasTip>
                    </Show>
                    <HasTip tip=change_tip.into_signal()>
                        <div
                            class="process-mix-percents"
                            class:depleted=move || feedstock_estimate() == Some(0.)
                        >
                            <div class="process-mix-percent" class:before=has_change>
                                {mix_share_percent}
                                %
                            </div>
                            <Show when=has_change>
                                <img src=icons::ARROW_RIGHT/>
                                <div
                                    class="process-mix-percent after"
                                    class:shrink=is_shrink
                                    class:grow=is_grow
                                >
                                    {changed_mix_share_percent}
                                    %
                                </div>
                            </Show>
                        </div>
                    </HasTip>
                </div>
                <div class="process-intensity space-even">
                    {energy_intensity} {water_intensity}
                    {biodiversity_intensity} {land_intensity}
                    {emissions_intensity}
                </div>
            </Body>

            <TopBack slot>
                <p class="card-desc">{description}</p>
            </TopBack>

            <BottomBack slot>
                <div class="process-details">
                    <div>
                        <Show when=is_halted>
                            <Show
                                when=almost_halted
                                fallback=move || {
                                    view! { <img src=icons::HALTED class="alert-icon"/> }
                                }
                            >

                                <img src=icons::ALERT class="alert-icon"/>
                            </Show>
                        </Show>
                        <Show when=has_feedstock>
                            <HasTip tip=feedstock_tip.into_signal()>
                                <img class="process-feedstock" src=feedstock_icon/>
                            </HasTip>
                        </Show>
                        <Show when=move || feedstock_estimate().is_some()>
                            <div class="feedstock-remaining">
                                <div class=feedstock_bar_class></div>
                            </div>
                        </Show>
                    </div>
                    <div>{feature_icons}</div>
                    <div class="card-spacer"></div>
                    <div class="card-image-attribution">
                        {t!("Image:")}" "{image_attrib}
                    </div>

                </div>
            </BottomBack>

            <ProcessMix slot>
                <HasTip tip=process_mix_tip.into_signal()>
                    <div class="process-mix-cells">{mix_cells}</div>
                </HasTip>
            </ProcessMix>
        </Card>
    }
}
