use super::super::card::*;
use crate::{
    display::{self, AsText},
    i18n,
    icons::{self, HasIcon},
    t,
    util::ImageExt,
    views::{
        intensity::{self, IntensityIcon, Variable},
        tip,
        HasTip,
    },
    with_state,
};
use hes_engine::regions::Region;
use leptos::*;

#[component]
pub fn RegionCard(region: Signal<Region>) -> impl IntoView {
    let contentedness = move || {
        let outlook = region.with(|region| region.outlook);
        intensity::scale(outlook, Variable::Outlook)
    };
    let demand = with_state!(|state, _ui, region| {
        let total_demand = state.demand_for_outputs();
        let per_capita_demand = state.world.output_demand;
        let demand = region.demand(&per_capita_demand);
        let pop = region.population;
        demand.items().map(|(output, demand)| {
            let region_per_capita_demand = demand / pop;
            let intensity = intensity::output_intensity(
                region_per_capita_demand,
                output,
            );
            let percent = display::demand_percent(
                demand,
                total_demand[output],
                false,
            );
            let fmted = display::output(demand, output);
            (output, fmted, percent, intensity)
        })
    });
    let habitability = move || {
        let habitability =
            region.with(|region| region.habitability());
        intensity::scale(habitability, Variable::Habitability)
    };
    let income_name = move || {
        let income =
            region.with(|region| region.income.lower());
        t!(income)
    };
    let income_level =
        move || region.with(|region| region.income_level() + 1);
    let name = move || {
        let name = region.with(|region| region.name.clone());
        t!(&name)
    };
    let population = move || {
        let pop = region.with(|region| region.population);
        i18n::num_fmt()(pop)
    };
    let seceded = move || region.with(|region| region.seceded);
    let temp_range =
        move || region.with(|region| region.temp_range());
    let precip_range =
        move || region.with(|region| region.precip_range());

    let image =
        move || with!(|region| region.flavor.image.src());

    view! {
        <Card class="region">
            <Header slot>
                <div>{name}</div>
                <div>{population} <img src=icons::POPULATION/></div>
            </Header>

            <Figure slot>
                <img class="card-image" src=image/>
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
                        icons::PRECIPITATION,
                        t!("This region's current precipitation range."),
                    )>
                        <div
                            class="region-stat"
                            v-tip="{icon: 'precipitation', text: ''}"
                        >
                            <img src=icons::PRECIPITATION/>
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
                            incomeName : income_name(),
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
                                    output : output.lower(), demand : demand, demandPercent :
                                    percent,
                                ),
                            );
                            let (int, _) = create_signal(intensity);
                            view! {
                                <HasTip tip>
                                    <IntensityIcon icon=output.icon() intensity=int/>
                                </HasTip>
                            }
                        }
                    />

                </div>
            </Body>
        </Card>
    }
}