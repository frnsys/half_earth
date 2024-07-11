use super::super::card::*;
use crate::{
    display::{self, AsText},
    icons::{self, HasIcon},
    state,
    t,
    vars::Impact,
    views::{tip, HasTip},
};
use hes_engine::industries::Industry;
use leptos::*;

#[component]
pub fn IndustryCard(
    #[prop(into)] industry: Signal<Industry>,
) -> impl IntoView {
    let lic_pop = state!(world.lic_population());
    let demand = move || {
        industry.with(move |ind| ind.demand(lic_pop.get()))
    };
    let name = move || industry.with(|c| t!(&c.name));
    let total_resources = move || {
        industry.with(|ind| ind.adj_resources() * demand())
    };
    let empty = move || total_resources().sum() == 0.;
    let emissions = move || {
        industry.with(|ind| {
            (ind.adj_byproducts() * demand()).co2eq()
        })
    };
    let resources_demand = state!(resources_demand);
    let body_view = move || {
        if empty() {
            t!("This industry is not yet significant.")
                .into_view()
        } else {
            view! {
                <For
                    each=move || total_resources().items()
                    key=|(key, _)| key.clone()
                    children=move |(key, val)| {
                        let percent = display::demand_percent(
                            val,
                            resources_demand.get()[key],
                            false,
                        );
                        let tip = tip(
                            key.icon(),
                            t!(
                                "This industry's demand for {output}. This makes up {percent} of total demand for {output}.",
                                output : key.lower(), percent : percent,
                            ),
                        );
                        view! {
                            <HasTip tip>
                                <div>
                                    <div class="card-icon">
                                        <img src=key.icon()/>
                                        {total_resources()[key]}
                                    </div>
                                </div>
                            </HasTip>
                        }
                    }
                />

                <Show when=move || emissions() != 0.>
                    <HasTip tip=tip(
                        icons::EMISSIONS,
                        t!("This industry's non-energy CO2eq emissions."),
                    )>
                        <div class="card-icon">
                            <img src=icons::EMISSIONS/>
                            {move || {
                                let e = emissions();
                                if e < 1. {
                                    "<1".to_string()
                                } else {
                                    display::format_impact(Impact::Emissions, e)
                                }
                            }}

                        </div>
                    </HasTip>
                </Show>
            }.into_view()
        }
    };

    let image_url = move || {
        industry.with(|ind| {
            format!(
                "/public/assets/content/images/{}",
                ind.flavor.image.fname
            )
        })
    };
    let image_attrib = move || {
        industry
            .with(|ind| ind.flavor.image.attribution.clone())
    };

    // TODO?
    // let description = move || industry.with(|ind| t!(&ind.flavor.description));
    let description = "";

    view! {
        <Card
            class="industry"
            color="#000000"
            background="palevioletred"
        >
            <Header slot>
                <div>{t!("Sector")}</div>
            </Header>
            <Figure slot>
                <img class="card-image" src=image_url/>
            </Figure>
            <Name slot>
                <div>{name}</div>
            </Name>
            <TopBack slot>
                <p class="card-desc">{description}</p>
            </TopBack>
            <BottomBack slot>
                <div class="card-image-attribution">
                    {t!("Image:")} {image_attrib}
                </div>
            </BottomBack>
            <Body slot>
                <div class="space-even"></div>
            </Body>
        </Card>
    }
}
