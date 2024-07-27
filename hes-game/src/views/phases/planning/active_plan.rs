use crate::{icons, state, t, views::cards::MiniProject};
use leptos::*;

#[component]
pub fn ActivePlan(
    #[prop(into)] close: Callback<()>,
    #[prop(into)] add: Callback<()>,
) -> impl IntoView {
    let projects = state!(world.projects.clone());
    let active_projects = move || {
        projects
            .get()
            .iter()
            .filter(|p| p.is_online() || p.is_building())
            .cloned()
            .map(create_rw_signal)
            .collect::<Vec<_>>()
    };

    view! {
        <div class="planning--page active-plan">
            <div class="planning--page-tabs">
                <div on:click=move |_| close.call(())>{t!("Back")}</div>
            </div>
            <div class="plan--changes">
                <div class="plan--change">
                    <div
                        class="plan--add-change minicard"
                        on:click=move |_| add.call(())
                    >
                        <div>
                            <img src=icons::ADD/>
                            <div class="plan--action">{t!("Add")}</div>
                        </div>
                    </div>
                </div>
                <For
                    each=active_projects
                    key=|project| project.get().id
                    children=move |project| {
                        view! {
                            <div class="plan--change">
                                <MiniProject project/>
                                <div class="plan--change-name">
                                    {t!(& project.get().name)}
                                </div>
                            </div>
                        }
                    }
                />

            </div>
        </div>
    }
}