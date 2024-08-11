use std::path::PathBuf;

use hes_engine::World;
use leptos::*;
use leptos_toaster::*;
use leptos_use::{
    on_click_outside,
    use_document,
    use_event_listener,
};

use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob,
    File,
    FileReader,
    HtmlAnchorElement,
    HtmlInputElement,
    Url,
};

use crate::{files, validate::validate};

pub async fn pick_and_load_file() -> Option<(String, String)> {
    let document = window().document().unwrap();

    // Create an input element of type 'file'
    let input: HtmlInputElement = document
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    input.set_type("file");
    input.set_accept(".world");

    // Add the input element to the DOM (hidden)
    input.style().set_property("display", "none").unwrap();
    document.body().unwrap().append_child(&input).unwrap();

    // Create a promise and its resolution/rejection callbacks
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let resolve = resolve.clone();
        let value = input.clone();
        let closure = Closure::wrap(Box::new(move || {
            let files = value.files();
            if let Some(files) = files {
                if files.length() > 0 {
                    let file = files.get(0).unwrap(); // Take the first file (for simplicity)
                    resolve
                        .call1(&JsValue::NULL, &file)
                        .unwrap();
                } else {
                    resolve
                        .call1(&JsValue::NULL, &JsValue::NULL)
                        .unwrap();
                }
            }
        })
            as Box<dyn FnMut()>);

        input.set_onchange(Some(
            closure.as_ref().unchecked_ref(),
        ));
        closure.forget();
    });

    // Trigger the file input click to open the file picker dialog
    input.click();

    // Await the user's file selection
    let result = JsFuture::from(promise).await.unwrap();

    // Clean up: remove the input element from the DOM
    input.remove();

    // Return the file content or None if no file was selected
    if result.is_null() {
        return None;
    }

    let file = result.dyn_into::<File>().unwrap();
    let file_content = read_file_as_text(&file).await;

    Some((file.name(), file_content))
}

async fn read_file_as_text(file: &File) -> String {
    let reader = FileReader::new().unwrap();

    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let reader_clone = reader.clone();
        let resolve_clone = resolve.clone();

        let closure = Closure::wrap(Box::new(move || {
            let result = reader_clone.result().unwrap();
            resolve_clone
                .call1(&JsValue::NULL, &result)
                .unwrap();
        })
            as Box<dyn FnMut()>);

        reader.set_onloadend(Some(
            closure.as_ref().unchecked_ref(),
        ));
        closure.forget();

        reader.read_as_text(file).unwrap();
    });

    let result = JsFuture::from(promise).await.unwrap();

    result.as_string().unwrap()
}

async fn download(data: &str, filename: &str) {
    let parts = js_sys::Array::of1(&JsValue::from_str(&data));
    let blob = Blob::new_with_str_sequence_and_options(
        &parts,
        web_sys::BlobPropertyBag::new().type_("text/plain"),
    )
    .expect("Failed to create Blob");
    let url = Url::create_object_url_with_blob(&blob)
        .expect("Failed to create object URL");

    let document = window()
        .document()
        .expect("Should have a document on window");
    let a: HtmlAnchorElement = document
        .create_element("a")
        .expect("Failed to create anchor element")
        .dyn_into()
        .expect("Failed to cast to HtmlAnchorElement");

    a.set_href(&url);
    a.set_download(filename);

    document
        .body()
        .expect("Document should have a body")
        .append_child(&a)
        .expect("Failed to append anchor to body");
    a.click();

    a.remove();
    Url::revoke_object_url(&url)
        .expect("Failed to revoke object URL");
}

async fn export(
    default_name: &str,
    world: World,
) -> Result<Option<String>, Error> {
    let errors = validate(&world);
    if !errors.is_empty() {
        return Err(Error::Validation(errors));
    }

    let result = window()
        .prompt_with_message_and_default(
            "Name Your World",
            default_name,
        )
        .unwrap();

    if let Some(name) = result {
        let data =
            serde_json::to_string_pretty(&world).unwrap();
        let name = format!("{name}.world");
        download(&data, &name).await;
        Ok(Some(name))
    } else {
        Ok(None)
    }
}

async fn confirm_lose_changes() -> bool {
    let msg = "Any unsaved changes to the current world will be lost. Continue?";
    crate::confirm(msg).await
}

enum Error {
    Validation(Vec<String>),
    IO(String),
}

#[component]
pub fn WorldsMenu(world: RwSignal<World>) -> impl IntoView {
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

    let _ = use_event_listener(
        use_document(),
        ev::keydown,
        move |ev| {
            if ev.key() == "s" && ev.ctrl_key() {
                ev.prevent_default();
                match files::save_session(&world.get()) {
                    Ok(_) => {
                        notice_toast(
                            "Session saved".into(),
                            "Don't forget to export your world!"
                            .into(),
                        );
                    }
                    Err(err) => {
                        notice_toast(
                            "Failed to save session:".into(),
                            err.to_string(),
                        );
                    }
                }
            }
        },
    );

    let default_name = format!(
        "world-{}",
        (js_sys::Math::random() * usize::MAX as f64).round()
            as usize
    );
    let last_name = create_rw_signal(default_name);

    let load_action = create_action(move |_: &()| async move {
        if let Some((name, data)) = pick_and_load_file().await {
            if let Ok(w) = serde_json::from_str::<World>(&data)
            {
                let as_path = PathBuf::from(name);
                if let Some(name) = as_path.file_stem() {
                    last_name.set(
                        name.to_string_lossy().to_string(),
                    );
                }
                return Some(w);
            } else {
                error_toast(Error::IO("Failed to parse world from file. Are you sure it's valid?".to_string()));
            }
        } else {
            error_toast(Error::IO(
                "Failed to read file.".to_string(),
            ));
        }
        None
    });
    let value = load_action.value();
    create_effect(move |_| {
        if let Some(w) = value.get().flatten() {
            world.set(w);
        }
    });

    view! {
        <div class="worlds-menu" ref=target>
            <div class="worlds-menu-open"
                on:click=move |_| {
                    update!(|open| *open = !*open);
            }
            >"≡"</div><span class="world-name">{last_name}</span>
            <div class="worlds-menu-inner" class:hidden=move || !open.get() >
                <div on:click=move |_| {
                    spawn_local(async move {
                        if confirm_lose_changes().await {
                            world.set(World::default());
                        }
                    });
                    open.set(false);
                }>"New"</div>

                <div on:click=move |_| {
                    load_action.dispatch(());
                    open.set(false);
                }>"Import"</div>

                 <div on:click=move |_| {
                     spawn_local(async move {
                         match export(&last_name.get_untracked(), world.get_untracked()).await {
                             Ok(None) => (),
                             Ok(Some(name)) => {
                                 let as_path = PathBuf::from(name.clone());
                                 if let Some(name) = as_path.file_stem() {
                                     last_name.set(
                                         name.to_string_lossy().to_string(),
                                     );
                                 }
                                 notice_toast(name, "Successfully exported.".into());
                             }
                             Err(err) => error_toast(err),
                         }
                     });
                     open.set(false);
                 }>"Export"</div>
            </div>
        </div>
    }
}
