mod calculate;

use crate::{vars::Var, views::intensity};
use calculate::N_PIPS;
use leptos::*;

pub use calculate::*;

#[component]
pub fn IntensityBar(
    #[prop(into)] intensity: Signal<usize>,
    #[prop(optional)] invert: bool,
    #[prop(optional, default=N_PIPS)] max_pips: usize,
) -> impl IntoView {
    let color =
        move || intensity::color(intensity.get(), invert);
    let colors = move || {
        (0..max_pips).map(move |i| {
            if i < intensity.get() {
                Some(color())
            } else {
                None
            }
        })
    };

    let pips = move || {
        colors()
            .map(|color| {
                if let Some(color) = color {
                    view! {
                        <div
                            class="intensity-pip"
                            style:background=color
                            style:box-shadow=format!("0px 0px 7px {color}")
                        ></div>
                    }
                } else {
                    view! { <div class="intensity-pip"></div> }
                }
            })
            .collect::<Vec<_>>()
    };

    view! { <div class="intensity-pips">{pips}</div> }
}

#[component]
pub fn IntensityIcon(
    #[prop(into)] intensity: Signal<usize>,
    #[prop(into)] icon: MaybeSignal<&'static str>,
    #[prop(optional)] invert: bool,
    #[prop(optional, default=N_PIPS)] max_pips: usize,
) -> impl IntoView {
    view! {
        <div class="card-icon intensity-icon">
            <img class="pip-icon" src=icon/>
            <IntensityBar intensity=intensity invert=invert max_pips/>
        </div>
    }
}
