mod calculate;

use crate::{
    consts,
    icons::{self, HasIcon},
    t,
    vars::Var,
    views::{cards::FactorsCard, intensity::IntensityIcon},
    with_state,
};
pub use calculate::{rank, Factor};
use leptos::*;

pub use calculate::factors_card;

#[component]
pub fn FactorsList(
    #[prop(into)] factors: Signal<FactorsCard>,
) -> impl IntoView {
    let relation = move || {
        factors.with(|factors| {
            let relation = match factors.kind {
                Var::Emissions => "makes",
                Var::Biodiversity => "causes",
                _ => "uses",
            };
            t!(relation)
        })
    };
    let icon = move || factors.with(|facs| facs.icon);
    let cur_name =
        move || factors.with(|facs| facs.current.clone());
    let total_label = move || {
        factors.with(|factors| {
            let max_value = match factors.kind {
                Var::Biodiversity => Some(consts::MAX_BIODIVERSITY),
                Var::Contentedness => Some(consts::MAX_CONTENTEDNESS),
                _ => None,
            };
            let total = factors.total_formatted();
            if let Some(max_value) = max_value {
                view! {
                    <div>
                        {total} <span class="type-total">/ {max_value}</span>
                        <img src=icon/>
                    </div>
                }
            } else {
                view! { <div>{total} <img src=icon/></div> }
            }
        })
    };

    let relevant_factors =
        with_state!(|_state, ui, factors| {
            ui.factors[factors.kind]
                .iter()
                .filter(|user| match user {
                    Factor::Industry { produced, .. }
                    | Factor::Process { produced, .. } => {
                        *produced != 0.
                    }
                    _ => true,
                })
                .cloned()
                .collect::<Vec<_>>()
        });

    view! {
        <div class="factors--users">
            <div class="factors--total">
                <div>{t!("Total")} :</div>
                {total_label}
            </div>
            <For
                each=relevant_factors
                key=|user: &Factor| user.name().to_string()
                children=move |user| {
                    let highlight = cur_name() == Some(user.name().to_string());
                    let name = user.name().to_string();
                    view! {
                        <div class="factors--user" class:highlight=highlight>
                            <div>
                                <div>{t!(& name)}</div>
                            </div>
                            <div>
                                <FactorLine
                                    factor=user
                                    relation=relation.into_signal()
                                    icon=icon.into_signal()
                                />
                            </div>
                        </div>
                    }
                }
            />

        </div>
    }
}

#[component]
fn FactorLine(
    factor: Factor,
    relation: Signal<String>,
    icon: Signal<&'static str>,
) -> impl IntoView {
    match factor {
        Factor::Region {
            intensity, display, ..
        } => view! {
            <IntensityIcon
                icon=icons::WEALTH
                intensity=move || intensity
                max_pips=4
            />
            <div class="factors--usage">{display} <img src=icon/></div>
        }
        .into_view(),
        Factor::Project {
            display, ..
        } => view! {
            <div class="factors--usage factors--usage-solo">
                {display} <img src=icon/>
            </div>
        }
        .into_view(),
        Factor::Event {
            display, amount, ..
        } => {
            let display = display.clone().unwrap_or_else(|| amount.to_string());
            view! {
                <div class="factors--usage factors--usage-solo">
                    {display} <img src=icon/>
                </div>
            }
            .into_view()
        }
        Factor::Process {
            intensity,
            display_produced,
            display,
            output,
            ..
        } => view! {
            <IntensityIcon
                icon=icon
                intensity=move || intensity
                max_pips=4
            />
            <div class="factors--usage">
                {display_produced} <img src=output.icon()/>
                <span class="factor-relation">{relation}</span> {display}
                <img src=icon/>
            </div>
        }
        .into_view(),
        Factor::Industry {
            intensity, display, ..
        } => view! {
            <IntensityIcon
                icon=icon
                intensity=move || intensity
                max_pips=4
            />
            <div class="factors--usage">{display} <img src=icon/></div>
        }
        .into_view(),
    }
}
