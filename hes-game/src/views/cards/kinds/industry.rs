use super::super::card::*;
use crate::{
    display::{format, text::AsText, Impact},
    icons::{self, HasIcon},
    state,
    state::GameState,
    state_with, t,
    views::{cards::Image, tip, HasTip},
};
use hes_engine::industries::Industry;
use leptos::*;

#[derive(Clone)]
struct Card {
    industry: Industry,
    description: String,
}

#[component]
pub fn IndustryCard(card: Signal<Card>) -> impl IntoView {
    // TODO card-image
    let image = Image {
        path: "foo".into(),
        attribution: "foo".into(),
    };

    let demand = state_with!(|state, ui, card| {
        let lic_pop = state.world.lic_population();
        card.industry.demand(lic_pop)
    });
    let name = move || card.with(|c| t!(&c.industry.name));
    let total_resources = move || card.with(|c| c.industry.adj_resources() * demand());
    let empty = move || total_resources().sum() == 0.;
    let emissions = move || card.with(|c| (c.industry.adj_byproducts() * demand()).co2eq());
    let body_view = state!(|state, ui| {
        let demand = state.resources_demand;
        if empty() {
            t!("This industry is not yet significant.").into_view()
        } else {
            view! {
                <For
                    each=move || total_resources().items()
                    key=|(key, _)| key.clone()
                    children=move |(key, val)| {
                        let percent = format::demand_percent(
                            val,
                            demand[key],
                            false,
                        );
                        let tip = tip(
                            key.icon(),
                            t!(
                                "This industry's demand for {output}. This makes up {percent} of total demand for {output}.",
                                output = key.lower(), percent = percent,
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
                                    format::format_impact(Impact::Emissions, e)
                                }
                            }}

                        </div>
                    </HasTip>
                </Show>
            }.into_view()
        }
    });

    let description = move || card.with(|card| t!(&card.description));

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
                <img class="card-image" src=&image.path/>
            </Figure>
            <Name slot>
                <div>{name}</div>
            </Name>
            <TopBack slot>
                <p class="card-desc">{description}</p>
            </TopBack>
            <BottomBack slot>
                <div class="card-image-attribution">
                    {t!("Image:")} {&image.attribution}
                </div>
            </BottomBack>
            <Body slot>
                <div class="space-even"></div>
            </Body>
        </Card>
    }
}
