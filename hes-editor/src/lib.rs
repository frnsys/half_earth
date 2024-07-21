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

#[macro_export]
macro_rules! infinite_list {
    ($name:ident, $single:ident, $field:ident) => {
        use leptos_use::{
            use_infinite_scroll_with_options,
            UseInfiniteScrollOptions,
        };

        #[component]
        pub fn $name(world: RwSignal<World>) -> impl IntoView {
            const PER_PAGE: usize = 20;
            let max_idx = create_rw_signal(PER_PAGE);
            let list = move || {
                with!(|world, max_idx| world
                    .$field
                    .iter()
                    .enumerate()
                    .map(|(i, item)| (i, item.id))
                    .take(*max_idx)
                    .collect::<Vec<_>>())
            };
            let total = move || with!(|world| world.$field.len());

            let el = create_node_ref::<html::Div>();
            let _ = use_infinite_scroll_with_options(
                el,
                move |_| async move {
                    update!(|max_idx| {
                        *max_idx += PER_PAGE;
                        *max_idx = (*max_idx).min(total());
                    });
                },
                UseInfiniteScrollOptions::default().distance(50.0),
            );

            view! {
                <div ref=el class="scroll-list">
                    <div class="insert-item" on:click=move |_| {
                        update!(|world| {
                            world.$field.push_front($single::new());
                        });
                    }>+ New</div>
                    <For each=list
                    key=|(_, id)| *id
                    children=move |(i, id)| {
                        view! {
                            <div class="scroll-list-item">
                                <div class="remove-item" on:click=move |_| {
                                    let msg = "Are you sure you want to delete this?";
                                    if window().confirm_with_message(msg).unwrap() {
                                        update!(|world| {
                                            world.$field.remove(&id);
                                        });
                                    }
                                }>"🞬 Delete"</div>
                                <$single
                                    signal=create_slice(world,
                                        move |world| world.$field.by_idx(i).clone(),
                                        move |world, val| *world.$field.by_idx_mut(i) = val
                                    ) />
                            </div>
                        }
                    } />
                </div>
            }
        }
    }
}
