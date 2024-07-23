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

        fn find_element_with_id(
            nodelist: web_sys::NodeList,
            id: &str) -> Option<web_sys::HtmlElement> {
            use wasm_bindgen::JsCast;
            (0..nodelist.length())
                .filter_map(|i| nodelist.item(i))
                .filter_map(|node| {
                    node.dyn_into::<web_sys::HtmlElement>().ok()
                })
                .find(|el| el.id() == id)
        }

        #[component]
        pub fn $name(world: RwSignal<World>) -> impl IntoView {
            const PER_PAGE: usize = 20;
            let index_open = create_rw_signal(false);
            let index = move || {
                with!(|world| world
                    .$field
                    .iter()
                    .map(|item| (item.id, item.name.clone()))
                    .collect::<Vec<_>>())
            };
            let go_to = create_rw_signal::<Option<hes_engine::Id>>(None);
            create_effect(move |_| {
                let id = go_to.get();
                if let Some(id) = id {
                    let target = document().get_element_by_id(&id.to_string());
                    target.unwrap().scroll_into_view();
                    go_to.set_untracked(None);
                }
            });

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

            let list_el = create_node_ref::<html::Div>();
            let _ = use_infinite_scroll_with_options(
                list_el,
                move |_| async move {
                    update!(|max_idx| {
                        *max_idx += PER_PAGE;
                        *max_idx = (*max_idx).min(total());
                    });
                },
                UseInfiniteScrollOptions::default().distance(50.0),
            );

            view! {
                <div class="scroll-index">
                    <div class="scroll-index-open"
                        on:click=move |_| {
                            update!(|index_open| *index_open = !*index_open);
                        }
                      >"≡"</div>
                    <ul class="scroll-index-list" class:hidden=move || !index_open.get()>
                        <For each=index
                            key=|(id, _)| *id
                            children=move |(id, name)| {
                                view! {
                                    <li><a on:click=move |_| {
                                        let idx = with!(|world| world
                                            .$field
                                            .iter()
                                            .position(|item| item.id == id)
                                            .unwrap());
                                        let new_max_idx = max_idx.get().max(idx + 1);
                                        max_idx.set(new_max_idx);
                                        go_to.set(Some(id));
                                    }>{name}</a></li>
                                }
                            } />
                    </ul>
                </div>

                <div ref=list_el class="scroll-list">
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
                                    <div class="remove-item"
                                        title="Ctrl-click to remove without confirmation."
                                        on:click=move |ev| {
                                            let msg = "Are you sure you want to delete this?";
                                            if ev.ctrl_key() || window().confirm_with_message(msg).unwrap() {
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
