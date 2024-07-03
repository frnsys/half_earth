use crate::{
    consts,
    icons::{self, HasIcon},
    t,
    util::{scale_text, to_ws_el},
};

use super::super::process::ProcessCard;
use super::*;
use hes_engine::{kinds::Output, production::Process};
use leptos::*;

#[component]
pub fn MiniProcess(#[prop(into)] process: Signal<Process>) -> impl IntoView {
    let image = move || {
        process
            .with(|process| format!("url(/public/assets/content/{})", process.flavor.image.fname))
    };
    let icon = move || process.with(|process| process.output.icon());
    let label = move || {
        process.with(|process| match process.output {
            Output::Electricity => t!("electricity"),
            Output::Fuel => t!("fuel"),
            Output::PlantCalories => t!("crops"),
            Output::AnimalCalories => t!("livestock"),
        })
    };

    view! {
        <div class="miniprocess--wrapper">
            <MiniCard class="label">
                <Body slot>
                    <div
                        class="minicard-background"
                        style:background-image=image
                    ></div>
                    <div style:z-index=1>
                        <img class="minicard-process-icon" src=icon/>
                    </div>

                </Body>
                <Expanded slot>
                    <ProcessCard process/>
                </Expanded>
            </MiniCard>
            <small class="process--label">{label}</small>
        </div>
    }
}
