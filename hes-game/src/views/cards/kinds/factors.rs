use super::super::{card::*, FactorsCard, *, *};
use crate::{t, views::parts::FactorsList};
use leptos::*;

#[component]
pub fn FactorsCard(factors: Signal<FactorsCard>) -> impl IntoView {
    view! {
        <Card class="factors">
            <Body slot>
                <FactorsList factors/>
            </Body>
        </Card>
        <div class="factors-note">{t!("Only direct impacts are shown.")}</div>
    }
}
