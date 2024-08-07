mod inputs;
mod tabs;
mod validate;
mod worlds;

use hes_engine::{Collection, World, NPC};
use inputs::{AsRef, Ref};
use leptos::*;
use leptos_toaster::{Toaster, ToasterPosition};
use strum::{Display, EnumIter, IntoEnumIterator};
use tabs::*;
use worlds::{Status, WorldsMenu};

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

    let status = create_rw_signal(Status::None);
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

    let world_loaded =
        move || with!(|status| status.is_editing());
    let menu = move || {
        view! {
            <div id="tabs">
                <WorldsMenu world=world status=status />
                <Show when={world_loaded}>
                    {tabs}
                </Show>
            </div>
        }
    };

    let main = move || {
        if world_loaded() {
            Some(view! {
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
            })
        } else {
            None
        }
    };

    view! {
        <main>
            <Toaster
                position=ToasterPosition::BottomRight
            >
                {menu}
                {main}
            </Toaster>
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
        use leptos_toaster::{Toasts, ToastId, ToastOptions, dismiss_toast};
        use tauri_sys::{dialog::MessageDialogBuilder};

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

            let toast_context = expect_context::<Toasts>();
            let create_toast = move |name: String, refs: Vec<String>| {
                let toast_id = ToastId::new();
                toast_context.toast(
                    view! {
                        <div class="toast">
                            <div class="toast-header">
                                <div class="toast-remove" on:click=move |_| {
                                    dismiss_toast(&toast_id);
                                }>"✗"</div>
                            </div>
                            <div class="toast-body">
                                <h3>{name}" can't be deleted as it's referenced by:"</h3>
                                <div>{move || {
                                    refs.iter().map(|name| {
                                        view!{ <div>{name}</div> }
                                    }).collect::<Vec<_>>()
                                }}</div>
                            </div>
                        </div>
                    },
                    Some(toast_id),
                    Some(ToastOptions {
                        dismissible: true,
                        duration: Some(std::time::Duration::MAX),
                        position: None,
                    })
                );
            };

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
                                    <div class="remove-item tooltip-parent"
                                        title="Ctrl-click to remove without confirmation."
                                        on:click=move |ev| {
                                            spawn_local(async move {
                                                let msg = "Are you sure you want to delete this?";
                                                let name = with!(|world| world.$field.by_idx(i).name.clone());
                                                let refs = with!(|world| crate::validate::find_references(id, crate::validate::RefKind::$single, world));
                                                if !refs.is_empty() {
                                                    create_toast(name, refs);
                                                } else if ev.ctrl_key() || MessageDialogBuilder::new().confirm(msg).await.unwrap() {
                                                    update!(|world| {
                                                        world.$field.remove(&id);
                                                    });
                                                }
                                            });
                                        }>
                                            "🞬 Delete"
                                        </div>
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
