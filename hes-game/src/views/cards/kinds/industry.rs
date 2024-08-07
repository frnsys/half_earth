use super::super::card::*;
use crate::{
    display::{self, AsText},
    icons::{self, HasIcon},
    memo,
    t,
    util::ImageExt,
    vars::Impact,
    views::{tip, HasTip},
};
use hes_engine::{Industry, KindMap, State};
use leptos::*;

#[component]
pub fn IndustryCard(
    #[prop(into)] industry: Signal<Industry>,
) -> impl IntoView {
    let game = expect_context::<RwSignal<State>>();

    let lic_pop = memo!(game.world.lic_population());
    let available_resources = memo!(game.resources.available);
    let demand = move || {
        with!(|industry| industry.demand(lic_pop.get()))
    };
    let name = move || with!(|industry| t!(&industry.name));
    let total_resources = move || {
        with!(|industry| industry.adj_resources() * demand())
    };
    let empty = move || total_resources().sum() == 0.;
    let emissions = move || {
        with!(|industry| {
            (industry.adj_byproducts() * demand()).co2eq()
        })
    };
    let resources_demand = memo!(game.resource_demand.total());

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
                        let formatted = display::format_resource(val, key, available_resources.get());
                        let percent = display::demand_percent(
                            val,
                            resources_demand.get()[key],
                            false,
                        );
                        let tip = tip(
                            key.icon(),
                            t!(
                                "This industry's demand for {output}. This makes up {percent}% of total demand for {output}.",
                                output : key.lower(), percent : percent,
                            ),
                        );
                        view! {
                            <HasTip tip>
                                <div>
                                    <div class="card-icon">
                                        <img src=key.icon()/>
                                        {formatted}
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
                                    display::format_impact(Impact::Emissions, e, available_resources.get())
                                }
                            }}

                        </div>
                    </HasTip>
                </Show>
            }.into_view()
        }
    };

    let image_url =
        move || with!(|industry| industry.flavor.image.src());
    let image_attrib = move || {
        with!(|industry| industry
            .flavor
            .image
            .attribution
            .clone())
    };

    let description = move || {
        with!(|industry| t!(&industry.flavor.description))
    };

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
                    {t!("Image:")}" "{image_attrib}
                </div>
            </BottomBack>
            <Body slot>
                <div class="space-even">{body_view}</div>
            </Body>
        </Card>
    }
}
