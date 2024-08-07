use hes_engine::World;
use leptos::*;
use leptos_toaster::*;
use leptos_use::{
    on_click_outside,
    use_document,
    use_event_listener,
};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri_sys::{dialog::MessageDialogBuilder, tauri};

use crate::validate::validate;

fn ensure_extension<P: AsRef<Path>>(
    path: P,
    ext: &str,
) -> PathBuf {
    let path = path.as_ref();
    let mut new_path = path.to_path_buf();

    if let Some(current_ext) =
        path.extension().and_then(|e| e.to_str())
    {
        if current_ext == ext {
            return new_path;
        }
    }

    // Add the extension
    new_path.set_extension(ext);
    new_path
}

async fn confirm_lose_changes(has_world: bool) -> bool {
    if has_world {
        let msg = "Any unsaved changes to the current world will be lost. Continue?";
        MessageDialogBuilder::new().confirm(msg).await.unwrap()
    } else {
        true
    }
}

enum Error {
    Validation(Vec<String>),
    IO(String),
}

async fn save_named(
    world: World,
) -> Result<Option<PathBuf>, Error> {
    let mut dialog =
        tauri_sys::dialog::FileDialogBuilder::new();
    dialog.add_filter("World", &["world"]);
    if let Some(path) = dialog.save().await.unwrap() {
        save(world, &path).await?;
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

async fn save(world: World, path: &Path) -> Result<(), Error> {
    let errors = validate(&world);
    if !errors.is_empty() {
        return Err(Error::Validation(errors));
    }

    let path = ensure_extension(path, "world");
    let data = serde_json::to_string_pretty(&world)
        .map_err(|err| Error::IO(err.to_string()))?;
    let res: Result<(), _> = tauri::invoke(
        "write_file",
        &WriteFile {
            path: path.clone(),
            content: data,
        },
    )
    .await;
    res.map_err(|err| Error::IO(err.to_string()))?;
    Ok(())
}

#[derive(Clone)]
pub enum Status {
    // No world is being edited.
    None,

    // A new, unsaved world is being edited.
    New,

    // An existing world with a known file path is being edited.
    File(PathBuf),
}
impl Status {
    pub fn is_editing(&self) -> bool {
        !matches!(self, Status::None)
    }
}

#[derive(Serialize)]
struct ReadFile {
    path: PathBuf,
}

#[derive(Serialize)]
struct WriteFile {
    path: PathBuf,
    content: String,
}

#[component]
pub fn WorldsMenu(
    status: RwSignal<Status>,
    world: RwSignal<World>,
) -> impl IntoView {
    let toast_context = expect_context::<Toasts>();
    let notice_toast = move |title: String, details: String| {
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
                        <h3>{title}</h3>
                        <div>{details}</div>
                    </div>
                </div>
            },
            Some(toast_id),
            Some(ToastOptions {
                dismissible: true,
                duration: Some(std::time::Duration::from_millis(5000)),
                position: None,
            })
        );
    };

    let error_toast = move |err: Error| {
        let toast_id = ToastId::new();

        let inner = match err {
            Error::Validation(errs) => view! {
                {move || {
                     errs.iter().map(|name| {
                         view!{ <div>{name}</div> }
                     }).collect::<Vec<_>>()
                 }}
            }
            .into_view(),
            Error::IO(err) => err.into_view(),
        };

        toast_context.toast(
            view! {
                <div class="toast">
                    <div class="toast-header">
                        <div class="toast-remove" on:click=move |_| {
                            dismiss_toast(&toast_id);
                        }>"✗"</div>
                    </div>
                    <div class="toast-body">
                        <h3>Error</h3>
                        <div>{inner}</div>
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

    let open = create_rw_signal(false);

    let target = create_node_ref::<html::Div>();
    let _ = on_click_outside(target, move |_| {
        if open.get() {
            open.set(false);
        }
    });

    let has_world = move || status.get().is_editing();
    let editing_file = move || match status.get() {
        Status::File(path) => Some(path.clone()),
        _ => None,
    };

    let _ = use_event_listener(
        use_document(),
        ev::keydown,
        move |ev| {
            if ev.key() == "s" && ev.ctrl_key() {
                if let Some(path) = editing_file() {
                    spawn_local(async move {
                        match save(world.get(), &path).await {
                            Ok(_) => {
                                notice_toast(
                                    path.display().to_string(),
                                    "Successfully saved."
                                        .into(),
                                );
                                status.set(Status::File(
                                    path.clone(),
                                ));
                            }
                            Err(err) => error_toast(err),
                        }
                    });
                }
            }
        },
    );

    view! {
        <div class="worlds-menu" ref=target>
            <div class="worlds-menu-open"
                on:click=move |_| {
                    update!(|open| *open = !*open);
            }
            >"≡"</div>
            <div class="worlds-menu-inner" class:hidden=move || !open.get() >
                <div on:click=move |_| {
                    spawn_local(async move {
                        if confirm_lose_changes(has_world()).await {
                            status.set(Status::New);
                            world.set(World::default());
                        }
                    });
                    open.set(false);
                }>"New"</div>

                <div on:click=move |_| {
                    spawn_local(async move {
                        if confirm_lose_changes(has_world()).await {
                            let mut dialog = tauri_sys::dialog::FileDialogBuilder::new();
                            dialog.add_filter("World", &["world"]);
                            if let Some(path) = dialog.pick_file().await.unwrap() {
                                let res: Result<String, _> = tauri::invoke("read_file",
                                    &ReadFile { path: path.clone() }).await;
                                if let Ok(data) = res
                                {
                                    if let Ok(w) =
                                        serde_json::from_str::<World>(
                                            &data,
                                        )
                                    {
                                        world.set(w);
                                        status.set(Status::File(path.clone()));
                                    } else {
                                        error_toast(Error::IO("Failed to parse world from file. Are you sure it's valid?".to_string()));
                                    }
                                } else {
                                    error_toast(Error::IO("Failed to read file.".to_string()));
                                }
                            }
                        }
                    });
                    open.set(false);
                }>"Open"</div>

                {move || {
                    editing_file().map(|path| {
                         view! {
                             <div on:click=move |_| {
                                 let path = path.clone();
                                 spawn_local(async move {
                                     match save(world.get(), &path).await {
                                         Ok(_) => {
                                             notice_toast(path.display().to_string(), "Successfully saved.".into());
                                             status.set(Status::File(path.clone()));
                                         }
                                         Err(err) => error_toast(err),
                                     }
                                 });
                                 open.set(false);
                             }>"Save"</div>
                         }
                     })
                 }}

                {move || {
                     if has_world() {
                         Some(view! {
                             <div on:click=move |_| {
                                 spawn_local(async move {
                                     match save_named(world.get()).await {
                                         Ok(None) => (),
                                         Ok(Some(path)) => {
                                             // Save to a named file and make it the
                                             // currently-edited file.
                                             notice_toast(path.display().to_string(), "Successfully saved.".into());
                                             status.set(Status::File(path.clone()));
                                         }
                                         Err(err) => error_toast(err),
                                     }
                                 });
                                 open.set(false);
                             }>"Save As"</div>
                         })
                     } else {
                         None
                     }
                 }}

                {move || {
                     if editing_file().is_some() {
                         Some(view! {
                             <div on:click=move |_| {
                                 spawn_local(async move {
                                     match save_named(world.get()).await {
                                         Ok(None) => (),
                                         Ok(Some(path)) => {
                                             // Save to a named file, but don't make it the
                                             // currently-edited file.
                                             notice_toast(path.display().to_string(), "Successfully saved.".into());
                                         }
                                         Err(err) => error_toast(err),
                                     }
                                 });
                                 open.set(false);
                             }>"Save As Copy"</div>
                         })
                     } else {
                         None
                     }
                 }}
            </div>
        </div>
    }
}
