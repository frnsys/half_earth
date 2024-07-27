use super::super::{
    card::{Body, Card},
    FactorsCard as FactorsCardData,
};
use crate::{t, views::factors::FactorsList};
use leptos::*;

#[component]
pub fn FactorsCard(
    #[prop(into)] factors: Signal<FactorsCardData>,
) -> impl IntoView {
    view! {
        <Card class="factors">
            <Body slot>
                <FactorsList factors/>
            </Body>
        </Card>
        <div class="factors-note">
            {t!("Only direct impacts are shown.")}
        </div>
    }
}
