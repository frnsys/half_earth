use crate::display::{
    intensity::{self, N_PIPS},
    Var,
};
use leptos::*;

#[component]
pub fn IntensityBar(
    #[prop(into)] intensity: MaybeSignal<usize>,
    #[prop(optional)] invert: bool,
) -> impl IntoView {
    let color = move || intensity::color(intensity.get(), invert);
    let colors = move || {
        (0..N_PIPS).map(move |i| {
            if i < intensity.get() {
                Some(color())
            } else {
                None
            }
        })
    };

    view! {
        <div class="intensity-pips">
            <For
                each=move || colors().enumerate()
                key=|(i, _)| *i
                children=move |(_, color)| {
                    if let Some(color) = color {
                        view! {
                            <div
                                class="intensity-pip"
                                style:background=color
                                style:boxShadow=format!("0px 0px 7px {color}")
                            ></div>
                        }
                    } else {
                        view! { <div class="intensity-pip"></div> }
                    }
                }
            />

        </div>
    }
}

#[component]
pub fn IntensityIcon(
    #[prop(into)] intensity: MaybeSignal<usize>,
    #[prop(into)] icon: MaybeSignal<&'static str>,
    #[prop(optional)] invert: bool,
) -> impl IntoView {
    view! {
        <div class="card-icon intensity-icon">
            <img class="pip-icon" src=icon/>
            <IntensityBar intensity=intensity invert=invert/>
        </div>
    }
}
