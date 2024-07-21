mod inputs;
mod tabs;

use hes_engine::{
    kinds::{FeedstockMap, OutputMap, ResourceMap},
    npcs::NPC,
    regions::Income,
    world::World,
    Collection,
    Game,
};
use inputs::{AsRef, Ref};
use leptos::*;
use paste::paste;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use strum::{Display, EnumIter, IntoEnumIterator};
use tabs::*;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, Display, EnumIter, PartialEq)]
enum Tab {
    World,
    Regions,
    Industries,
    Processes,
    Projects,
    Events,
}

#[component]
pub fn App() -> impl IntoView {
    let tab = create_rw_signal(Tab::World);

    // TODO load provided world
    // TODO list user worlds
    let world = create_rw_signal(World::default());
    let npcs = NPC::load();

    provide_context(Signal::derive(move || {
        npcs.clone()
            .iter()
            .map(|item| item.as_ref())
            .collect::<Collection<Ref<_>>>()
    }));

    provide_context(Signal::derive(move || {
        with!(|world| world
            .processes
            .iter()
            .map(|item| item.as_ref())
            .collect::<Collection<Ref<_>>>())
    }));
    provide_context(Signal::derive(move || {
        with!(|world| world
            .events
            .iter()
            .map(|item| item.as_ref())
            .collect::<Collection<Ref<_>>>())
    }));
    provide_context(Signal::derive(move || {
        with!(|world| world
            .projects
            .iter()
            .map(|item| item.as_ref())
            .collect::<Collection<Ref<_>>>())
    }));
    provide_context(Signal::derive(move || {
        with!(|world| world
            .industries
            .iter()
            .map(|item| item.as_ref())
            .collect::<Collection<Ref<_>>>())
    }));

    let tabs = move || {
        Tab::iter()
            .map(|t| {
                let name = t.to_string();
                view! {
                    <div class="tab"
                        class:selected=move || { tab.get() == t }
                        on:click=move |_| {
                            tab.set(t)
                        }>{name}</div>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <main>
            <div id="tabs">{tabs}</div>
            {move || {
                 match tab.get() {
                     Tab::World => view! { <World world / > }.into_view(),
                     Tab::Regions => view! { <Regions world / > }.into_view(),
                     Tab::Industries => view! { <Industries world / > }.into_view(),
                     Tab::Processes => view! { <Processes world / > }.into_view(),
                     Tab::Projects => view! { <Projects world / > }.into_view(),
                     Tab::Events => view! { <Events world / > }.into_view(),
                 }
             }
            }
        </main>
    }
}
