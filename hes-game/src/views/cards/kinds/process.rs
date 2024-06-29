use super::super::card::*;
use crate::{
    display::{
        format,
        intensity::{self, Variable},
        text::AsText,
        Impact, OutputKind, Var,
    },
    i18n,
    icons::{self, HasIcon},
    state::GameState,
    state_with, t,
    views::{cards::Image, parts::IntensityIcon, tip, HasTip},
};
use hes_engine::{kinds::Feedstock, production::Process};
use leptos::*;

#[derive(Clone)]
struct Card {
    process: Process,
    description: String,
}

fn describe_estimate(estimate: f32) -> String {
    if estimate == 0. {
        t!("This feedstock is depleted, so this process is stopped. You should reallocate its points to other processes.")
    } else if estimate.is_finite() {
        t!("At current usage rates the estimated supply is expected to last {years} years.", years = estimate)
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
pub fn ProcessCard(card: Signal<Card>) -> impl IntoView {
    // TODO card-image
    let image = Image {
        path: "foo".into(),
        attribution: "foo".into(),
    };

    let is_new = move || {
        // return !state.viewed.includes(this.ref_id);
        false
    };
    let name = move || card.with(|c| t!(&c.process.name));
    let output_icon = move || card.with(|c| c.process.output.icon());
    let output_name = move || card.with(|c| t!(&c.process.output.title()));
    let feedstock_name = move || card.with(|c| t!(&c.process.feedstock.0.title()));

    let feedstock_estimate = state_with!(|state, ui, card| {
        let feedstock = card.process.feedstock.0;
        match feedstock {
            Feedstock::Soil | Feedstock::Other => None,
            _ => {
                let estimate = state.feedstocks[feedstock] / state.consumed_feedstocks[feedstock];
                Some(estimate.round())
            }
        }
    });
    let feedstock_estimate_desc = move || {
        let estimate = feedstock_estimate();
        estimate.map(describe_estimate).unwrap_or_default()
    };

    let feedstock_level = move || {
        let estimate = feedstock_estimate();
        estimate.map(describe_stocks).unwrap_or("high")
    };

    let max_share = state_with!(|state, ui, card| { state.process_max_share(&card.process) });

    let produced = state_with!(|state, ui, card| {
        let process = &card.process;
        let base_amount = state.produced_by_process[process.id];
        let mut amount = format::output(base_amount, process.output);
        if amount > 0. {
            amount = amount.max(1.);
        }
        let gtco2eq = process.byproducts.gtco2eq();
        let mut emissions = gtco2eq * base_amount;
        if emissions > 0. {
            emissions = emissions.max(1.);
        }
        (amount, emissions)
    });
    let output_tip = move || {
        card.with(|c| {
            let output = c.process.output;
            let (amount, emissions) = produced();
            tip(output.icon(),
                t!("This process currently produces {amount}<img src='{outputIcon}'> and {emissions}<img src='{emissionsIcon}'> per year.", emissions = emissions, amount = amount, emissionsIcon = icons::EMISSIONS, outputIcon = output.icon()))
        })
    };
    let change_tip = move || {
        card.with(|c| {
            let output = t!(c.process.output.lower());
            let mix_percent = c.process.mix_share * 5;
            tip(
                icons::MIX_TOKEN,
                t!(
                    "This process currently makes up {mixPercent}% of {output} production.",
                    output = output,
                    mixPercent = mix_percent
                ),
            )
        })
    };

    let class = move || {
        if is_new() {
            "is_new"
        } else {
            ""
        }
    };

    let changed_mix_share = 1000; // TODO
    let alert_tip = move || {
        card.with(|c| {
            let mix_share = c.process.mix_share;
            let max_share = max_share();
            tip(
                icons::ALERT,
                t!("Because of resource availability this process can only make up to {maxPercent}% of production. {suggestion}",
                    maxPercent = max_share * 5,
                    suggestion = if mix_share > max_share || changed_mix_share > max_share {
                        t!("You should reallocate its points to other processes.")
                    } else {
                        "".into()
                    }
                ))
        })
    };

    view! {
        <Card color="#ffffff" class=class.into_signal()>
            <Header slot>
                <div>{output_name}</div>
                <Show when=is_new>
                    <img class="new-card-icon" src="/public/assets/new.svg"/>
                </Show>
                <HasTip tip=output_tip.into_signal()>
                    <div>
                        {move || produced().0} <img src=output_icon/>
                        {move || produced().1} <img src=icons::EMISSIONS/>
                    </div>
                </HasTip>
            </Header>

            <Name slot>{name}</Name>

            <Figure slot>
                <img class="card-image" src=&image.path/>
                <Show when=move || max_share() < 20>
                    <HasTip tip=alert_tip.into_signal()>
                        <div class="process-limit-alert">
                            <img src=icons::ALERT/>
                        </div>
                    </HasTip>
                </Show>
            </Figure>

            <Body slot>
                <div class="space-even"></div>
            </Body>
        </Card>
    }
}
