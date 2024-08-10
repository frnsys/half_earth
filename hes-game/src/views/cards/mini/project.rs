use crate::{
    icons::{self, HasIcon},
    util::ImageExt,
    views::scanner::{ProjectScanner, ScannerCards},
};

use super::{
    super::{kinds::project_color, *},
    MiniCard,
};
use hes_engine::Project;
use leptos::*;

#[component]
pub fn MiniProject(
    #[prop(into)] project: Signal<Project>,
) -> impl IntoView {
    let image = move || {
        with!(|project| {
            format!("url('{}')", project.flavor.image.src())
        })
    };
    let icon = move || with!(|project| project.kind.icon());
    let is_building =
        move || with!(|project| project.is_building());
    let is_finished =
        move || with!(|project| project.is_active());
    let progress = move || {
        with!(|project| {
            format!("{}%", project.progress * 100.)
        })
    };
    let border = move || {
        with!(|project| {
            let (bg, _) = project_color(&project.group);
            format!("5px solid {bg}")
        })
    };
    let projects = move || vec![project.get()];
    let scanner_cards = move || {
        let scanner = ProjectScanner::new(None);
        view! {
            <ScannerCards
                spec=scanner
                items=projects.into_signal()
            />
        }
    };

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
                {scanner_cards}
            </Expanded>
        </MiniCard>
    }
}
