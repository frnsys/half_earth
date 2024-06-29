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
use hes_engine::regions::Region;
use leptos::*;

#[derive(Clone)]
struct Card {
    region: Region,
    description: String,
}

#[component]
pub fn RegionCard(card: Signal<Card>) -> impl IntoView {
    // TODO card-image
    let image = Image {
        path: "foo".into(),
        attribution: "foo".into(),
    };

    let contentedness = move || {
        let outlook = card.with(|c| c.region.outlook);
        intensity::scale(outlook, Variable::Outlook)
    };
    let demand = state_with!(|state, ui, card| {
        let total_demand = state.output_demand;
        let per_capita_demand = state.world.output_demand;
        let demand = card.region.demand(&per_capita_demand);
        let pop = card.region.population;
        demand.items().map(|(output, demand)| {
            let region_per_capita_demand = demand / pop;
            let intensity = intensity::output_intensity(region_per_capita_demand, output);
            let percent = format::demand_percent(demand, total_demand[output], false);
            let fmted = format::output(demand, output);
            (output, fmted, percent, intensity)
        })
    });
    let habitability = move || {
        let habitability = card.with(|c| c.region.habitability());
        intensity::scale(habitability, Variable::Habitability)
    };
    let income_name = move || {
        let income = card.with(|c| c.region.income.lower());
        t!(income)
    };
    let income_level = move || card.with(|c| c.region.income_level() + 1);
    let name = move || {
        let name = card.with(|c| c.region.name.clone());
        t!(&name)
    };
    let population = move || {
        let pop = card.with(|c| c.region.population);
        i18n::num_fmt()(pop)
    };
    let seceded = move || card.with(|c| c.region.seceded);
    let temp_range = move || card.with(|c| c.region.temp_range());
    let precip_range = move || card.with(|c| c.region.precip_range());

    view! {
        <Card class="region">
            <Header slot>
                <div>{name}</div>
                <div>{population} <img src=icons::POPULATION/></div>
            </Header>

            <Figure slot>
                <img class="card-image" src=&image.path/>
                <div class="card-tack-ur">
                    <HasTip tip=tip(
                        icons::WARMING,
                        t!("This region's current temperature range."),
                    )>
                        <div class="region-stat">
                            <img src=icons::WARMING/>
                            {temp_range}
                        </div>
                    </HasTip>
                    <br/>
                    <HasTip tip=tip(
                        icons::PRECIPTATION,
                        t!("This region's current precipitation range."),
                    )>
                        <div
                            class="region-stat"
                            v-tip="{icon: 'precipitation', text: ''}"
                        >
                            <img src=icons::PRECIPTATION/>
                            {precip_range}
                        </div>
                    </HasTip>
                </div>
                <Show when=seceded>
                    <div class="card-tack-cb">Seceded</div>
                </Show>
            </Figure>

            <Body slot>
                <div class="space-even">
                    <HasTip tip=tip(
                        icons::WEALTH,
                        t!(
                            "This region has {incomeName} living standards. Higher living standards mean higher material footprints.",
                            incomeName = income_name(),
                        ),
                    )>
                        <IntensityIcon
                            icon=icons::WEALTH
                            intensity=income_level.into_signal()
                            invert=true
                        />
                    </HasTip>
                    <HasTip tip=tip(
                        icons::HABITABILITY,
                        t!("This region's habitability."),
                    )>
                        <IntensityIcon
                            icon=icons::HABITABILITY
                            intensity=habitability.into_signal()
                            invert=true
                        />
                    </HasTip>
                    <HasTip tip=tip(
                        icons::CONTENTEDNESS,
                        t!("This region's contentedness."),
                    )>
                        <IntensityIcon
                            icon=icons::CONTENTEDNESS
                            intensity=contentedness.into_signal()
                            invert=true
                        />
                    </HasTip>
                    <For
                        each=demand
                        key=|(output, _, _, _)| output.clone()
                        children=move |(output, demand, percent, intensity)| {
                            let tip = tip(
                                output.icon(),
                                t!(
                                    "This region's per-capita demand level for {output}. The total regions's demand is {demand}. This makes up {demandPercent} of total demand for {output}.",
                                    output = output.lower(), demand = demand, demandPercent =
                                    percent,
                                ),
                            );
                            view! {
                                <HasTip tip>
                                    <IntensityIcon icon=output.icon() intensity=intensity/>
                                </HasTip>
                            }
                        }
                    />

                </div>
            </Body>
        </Card>
    }
}
