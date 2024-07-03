use crate::{
    consts,
    icons::{self, HasIcon},
    t,
    util::{scale_text, to_ws_el},
    views::cards::kinds::project::card_color,
    views::phases::ProjectScanner,
};

use super::super::project::ProjectCard;
use super::*;
use hes_engine::{kinds::Output, projects::Project};
use leptos::*;

#[component]
pub fn MiniProject(#[prop(into)] project: Signal<Project>) -> impl IntoView {
    let image = move || {
        project
            .with(|project| format!("url(/public/assets/content/{})", project.flavor.image.fname))
    };
    let icon = move || project.with(|project| project.kind.icon());
    let is_building = move || project.with(|project| project.is_building());
    let is_finished = move || project.with(|project| project.is_active());
    let progress = move || project.with(|project| format!("{}%", project.progress * 100.));
    let border = move || {
        project.with(|project| {
            let (bg, _) = card_color(&project.group);
            format!("5px solid {bg}")
        })
    };
    let projects = move || vec![project.get()];

    view! {
        <MiniCard class="label" border=border.into_signal()>
            <Body slot>
                <div
                    class="minicard-background"
                    style:background-image=image
                ></div>
                <div style:z-index=1>
                    <img src=icon/>
                    <Show when=is_building>
                        <div class="project-progress">
                            <div
                                class="project-progress-fill"
                                style:width=progress
                            ></div>
                        </div>
                    </Show>
                    <Show when=is_finished>
                        <div class="project-check">
                            <img src=icons::CHECK/>
                        </div>
                    </Show>
                </div>

            </Body>
            <Expanded slot>
                <ProjectScanner
                    projects=projects.into_signal()
                    on_change=|_| {}
                />
            </Expanded>
        </MiniCard>
    }
}
