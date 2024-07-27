mod conditions;
mod effects;

use base64::prelude::*;
pub use conditions::Conditions;
pub use effects::Effects;
use hes_engine::{
    flavor::{Image, ImageData},
    kinds::{
        ByproductMap,
        FeedstockMap,
        OutputMap,
        ResourceMap,
    },
    Collection,
    HasId,
    Id,
};
use js_sys::Uint8Array;
use leptos::*;
use leptos_use::{on_click_outside, use_element_hover};
use num::Num;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, File};

/// This provides an easier way to create a slice,
/// i.e. a tuple of `(Signal<T>, SignalSetter<T>)`
/// from another slice, without requiring an
/// intermediary `RwSignal<T>`.
#[macro_export]
macro_rules! subsignal {
    ($signal:ident.$($field:tt).+) => {{
        let read = $signal.0;
        let write = $signal.1;
        let reader = Signal::derive(move || {
            with!(|read| read.$($field).+.clone())
        });
        let writer = SignalSetter::map(move |val| {
            let mut data = read.get();
            data.$($field).+ = val;
            write.set(data);
        });
        (reader, writer)
    }};

    ($signal:ident[$index:tt]) => {{
        let read = $signal.0;
        let write = $signal.1;
        let reader = Signal::derive(move || {
            with!(|read| read[$index].clone())
        });
        let writer = SignalSetter::map(move |val| {
            let mut data = read.get();
            data[$index] = val;
            write.set(data);
        });
        (reader, writer)
    }};
}

/// Conveniently create a slice from an enum variant.
#[macro_export]
macro_rules! enum_slice {
    (|$write_signal:ident| $enum:ident::$variant:ident($($before:ident,)* [ $arg:ident ] $(, $after:ident)*)) => {
        (
            Signal::derive(move || $arg),
            SignalSetter::map(move |$arg| $write_signal.set($enum::$variant($($before,)* $arg $(, $after)*)))
        )
    };
}

#[component]
pub fn TextInput(
    signal: (Signal<String>, SignalSetter<String>),
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] help: String,
    #[prop(into, optional)] inline: bool,
) -> impl IntoView {
    let (read, write) = signal;

    view! {
        <div class="input-group" class:inline={inline}>
            <div class="text-group-inner">
                <label>{label}</label>
                <input
                    class="text-input"
                    value=read.get_untracked()
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        write.set(value);
                    } />
            </div>
            <div class="input-help">{help}</div>
        </div>
    }
}

trait NumberError {
    fn error_desc() -> &'static str;
}
impl NumberError for f32 {
    fn error_desc() -> &'static str {
        "Must be a valid number."
    }
}
impl NumberError for usize {
    fn error_desc() -> &'static str {
        "Must be a valid positive number."
    }
}

#[component]
pub fn NumericInput<
    T: Num
        + Clone
        + Copy
        + std::str::FromStr
        + std::fmt::Display
        + IntoAttribute
        + IntoView
        + NumberError
        + 'static,
>(
    signal: (Signal<T>, SignalSetter<T>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
    #[prop(into, optional)] inline: bool,
) -> impl IntoView {
    let (read, write) = signal;
    let maybe_val = create_rw_signal(Ok(read.get_untracked()));

    let help = store_value(help);

    view! {
        <div class="input-group numeric-group tooltip-parent" class:inline={inline}>
            <div class="numeric-group-inner">
                <label>{label}</label>
                <input
                    class="numeric-input"
                    inputmode="decimal"
                    value=read.get_untracked()
                    on:change=move |ev| {
                        let res = event_target_value(&ev).parse::<T>();
                        if let Ok(value) = &res {
                            write.set(*value);
                        }
                        maybe_val.set(res);
                    } />
            </div>
            <Show when=move || with!(|maybe_val| maybe_val.is_err())>
                <div class="input-error">{T::error_desc()}</div>
            </Show>
            {move || {
                 (!help.get_value().is_empty()).then(|| {
                     view! {
                         <div class="tooltip">{help.get_value()}</div>
                     }
                 })
            }}
        </div>
    }
}

#[component]
pub fn PercentInput(
    signal: (Signal<f32>, SignalSetter<f32>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
    #[prop(into, optional)] inline: bool,
) -> impl IntoView {
    let (read, write) = signal;
    let maybe_val = create_rw_signal(Ok(read.get_untracked()));

    view! {
        <div class="input-group numeric-group" class:inline={inline}>
            <div class="numeric-group-inner">
                <label>{label}</label>
                <div class="input-suffixed">
                    <input
                        class="numeric-input"
                        inputmode="decimal"
                        value=read.get_untracked() * 100.
                        on:input=move |ev| {
                            let res = event_target_value(&ev).parse::<f32>();
                            if let Ok(value) = &res {
                                write.set(*value/100.);
                            }
                            maybe_val.set(res);
                        } />
                    <div class="input-suffix">%</div>
                </div>
            </div>
            <Show when=move || with!(|maybe_val| maybe_val.is_err())>
                <div class="input-error">Must be a number.</div>
            </Show>
            <div class="input-help">{help}</div>
        </div>
    }
}

#[component]
pub fn OptionalNumericInput<
    T: Num
        + Clone
        + Copy
        + Default
        + std::str::FromStr
        + std::fmt::Display
        + IntoAttribute
        + IntoView
        + NumberError
        + 'static,
>(
    signal: (Signal<Option<T>>, SignalSetter<Option<T>>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
) -> impl IntoView {
    let (read, write) = signal;
    let maybe_val = create_rw_signal(read.get_untracked());
    let value = create_rw_signal::<T>(
        read.get_untracked().unwrap_or_else(T::default),
    );

    view! {
        <div class="input-group option-group">
            <ToggleInput
                label=label
                help=help
                signal=create_slice(maybe_val,
                    move |opt| opt.is_some(),
                    move |opt, val| {
                        if val {
                            *opt = Some(value.get());
                        } else {
                            *opt = None;
                        }
                    })
                inner=move || {
                    view! {
                        <Show when=move || with!(|maybe_val| maybe_val.is_some())>
                            <NumericInput
                                label=""
                                help=""
                                signal=create_slice(maybe_val,
                                    move |opt| opt.clone().unwrap(),
                                    move |opt, val| {
                                        opt.insert(val);
                                        value.set(val);
                                    }) />
                        </Show>
                }} />
        </div>
    }
}

#[component]
pub fn MultiNumericInput<const N: usize>(
    signal: (Signal<[f32; N]>, SignalSetter<[f32; N]>),
    sublabels: [&'static str; N],
    #[prop(into)] label: String,
    #[prop(into)] help: String,
) -> impl IntoView {
    let inputs: Vec<_> = (0..N)
        .map(|i| {
            view! {
                <NumericInput
                    label=sublabels[i]
                    help=""
                    signal=subsignal!(signal[i]) />
            }
        })
        .collect();

    let help = store_value(help);

    view! {
        <div class="map-group">
            <h2 class="tooltip-parent">
                {label}
                {move || {
                     (!help.get_value().is_empty()).then(|| {
                         view! {
                             <div class="tooltip">{help.get_value()}</div>
                         }
                     })
                }}
            </h2>
            <div class="map-inputs">
                {inputs}
            </div>
        </div>
    }
}

#[component]
pub fn ResourceMapInput(
    signal: (Signal<ResourceMap>, SignalSetter<ResourceMap>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
) -> impl IntoView {
    let map = signal;
    let help = store_value(help);

    view! {
        <div class="map-group resources-group">
            <h2 class="tooltip-parent">
                {label}
                {move || {
                     (!help.get_value().is_empty()).then(|| {
                         view! {
                             <div class="tooltip">{help.get_value()}</div>
                         }
                     })
                }}
            </h2>
            <div class="map-inputs">
                <NumericInput
                    label="Land"
                    help="Land in square meters (m2)."
                    signal=subsignal!(map.land)
                    />
                <NumericInput
                    label="Water"
                    help="Water in liters (L)."
                    signal=subsignal!(map.water)
                    />
                <NumericInput
                    label="Electricity"
                    help="Electricity in kilowatt-hours (kWh)."
                    signal=subsignal!(map.electricity)
                    />
                <NumericInput
                    label="Fuel"
                    help="Fuel in kilowatt-hours (kWh)."
                    signal=subsignal!(map.fuel)
                    />
            </div>
        </div>
    }
}

#[component]
pub fn ByproductMapInput(
    signal: (Signal<ByproductMap>, SignalSetter<ByproductMap>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
) -> impl IntoView {
    let map = signal;
    let help = store_value(help);

    view! {
        <div class="map-group byproducts-group">
            <h2 class="tooltip-parent">
                {label}
                {move || {
                     (!help.get_value().is_empty()).then(|| {
                         view! {
                             <div class="tooltip">{help.get_value()}</div>
                         }
                     })
                }}
            </h2>
            <div class="map-inputs">
                <NumericInput
                    label="CO2"
                    help="CO2 in grams."
                    signal=subsignal!(map.co2)
                    />
                <NumericInput
                    label="CH4"
                    help="CH4 (methane) in grams."
                    signal=subsignal!(map.ch4)
                    />
                <NumericInput
                    label="N2O"
                    help="N2O (nitrous oxide) in grams."
                    signal=subsignal!(map.n2o)
                    />
                <NumericInput
                    label="Biodiversity"
                    help=r#"Effects on biodiversity, in "pressure"; e.g. -1 pressure means +1 to the extinction rate."#
                    signal=subsignal!(map.biodiversity)
                    />
            </div>
        </div>
    }
}

#[component]
pub fn OutputMapInput(
    signal: (Signal<OutputMap>, SignalSetter<OutputMap>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
) -> impl IntoView {
    let map = signal;
    let help = store_value(help);

    view! {
        <div class="map-group output-group">
            <h2 class="tooltip-parent">
                {label}
                {move || {
                     (!help.get_value().is_empty()).then(|| {
                         view! {
                             <div class="tooltip">{help.get_value()}</div>
                         }
                     })
                }}
            </h2>
            <div class="map-inputs">
                <NumericInput
                    label="Fuel"
                    help="Fuel in kilowatt-hours (kWh)."
                    signal=subsignal!(map.fuel)
                    />
                <NumericInput
                    label="Electricity"
                    help="Electricity in kilowatt-hours (kWh)."
                    signal=subsignal!(map.electricity)
                    />
                <NumericInput
                    label="Plant Calories"
                    help="Plant calories in kilocalories (kcal)."
                    signal=subsignal!(map.plant_calories)
                    />
                <NumericInput
                    label="Animal Calories"
                    help="Animal calories in kilocalories (kcal)."
                    signal=subsignal!(map.animal_calories)
                    />
            </div>
        </div>
    }
}

#[component]
pub fn FeedstockMapInput(
    signal: (Signal<FeedstockMap>, SignalSetter<FeedstockMap>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
) -> impl IntoView {
    let map = signal;
    let help = store_value(help);

    view! {
        <div class="map-group feedstocks-group">
            <h2 class="tooltip-parent">
                {label}
                {move || {
                     (!help.get_value().is_empty()).then(|| {
                         view! {
                             <div class="tooltip">{help.get_value()}</div>
                         }
                     })
                }}
            </h2>
            <div class="map-inputs">
                <NumericInput
                    label="Coal"
                    help="Coal in grams (g)."
                    signal=subsignal!(map.coal)
                    />
                <NumericInput
                    label="Oil"
                    help="Oil in liters (L)."
                    signal=subsignal!(map.oil)
                    />
                <NumericInput
                    label="Natural Gas"
                    help="Natural Gas in liters (L)"
                    signal=subsignal!(map.natural_gas)
                    />
                <NumericInput
                    label="Thorium"
                    help="Thorium in grams (g)."
                    signal=subsignal!(map.thorium)
                    />
                <NumericInput
                    label="Uranium"
                    help="Uranium in grams (g)."
                    signal=subsignal!(map.uranium)
                    />
                <NumericInput
                    label="Lithium"
                    help="Lithium in grams (g)."
                    signal=subsignal!(map.lithium)
                    />
            </div>
        </div>
    }
}

#[component]
pub fn EnumInput<
    E: IntoEnumIterator
        + Debug
        + Clone
        + Copy
        + FromStr
        + Display
        + Into<&'static str>
        + PartialEq
        + 'static,
>(
    signal: (Signal<E>, SignalSetter<E>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
    #[prop(into, optional)] tooltip: bool,
) -> impl IntoView
where
    <E as FromStr>::Err: Debug,
{
    let (read, write) = signal;

    let opts = move || {
        let current = read.get_untracked();
        E::iter()
            .map(|var| {
                let label: &'static str = var.into();
                view! {
                    <option selected={var == current} value=label>{var.to_string()}</option>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="input-group enum-select tooltip-parent">
            <div class="enum-select-inner">
                <label>{label}</label>
                  <select
                    on:change=move |ev| {
                      let new_value = event_target_value(&ev);
                      write.set(new_value.parse().unwrap());
                    }
                  >
                    {opts}
                  </select>
            </div>
            <div class:input-help=!tooltip class:tooltip=tooltip>{help}</div>
      </div>
    }
}

pub trait Describe {
    fn describe(&self) -> &'static str;
}

#[component]
pub fn MultiEnumInput<
    E: IntoEnumIterator
        + Debug
        + Clone
        + Copy
        + FromStr
        + Display
        + Into<&'static str>
        + PartialEq
        + Describe
        + 'static,
>(
    signal: (Signal<Vec<E>>, SignalSetter<Vec<E>>),
    #[prop(into)] label: String,
    #[prop(into)] help: String,
) -> impl IntoView
where
    <E as FromStr>::Err: Debug,
{
    let (read, write) = signal;

    let opts = move || {
        let current = read.get();
        E::iter()
            .map(|var| {
                let label: &'static str = var.into();
                view! {
                    <div
                        class="multi-select-opt tooltip-parent"
                        class:selected={current.contains(&var)}
                        on:click=move |_| {
                            let mut current = read.get();
                            if current.contains(&var) {
                                current.retain(|v| v != &var);
                            } else {
                                current.push(var);
                            }
                            write.set(current);
                        }
                    >
                        {var.to_string()}
                        <div class="tooltip">{var.describe()}</div>
                    </div>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="input-group multi-select-group">
            <label>{label}</label>
            <div class="input-help">{help}</div>
            <div class="multi-select-opts">
                {opts}
            </div>
      </div>
    }
}

pub struct Ref<T: ?Sized> {
    id: Id,
    label: String,
    marker: std::marker::PhantomData<T>,
}

impl<T: ?Sized> HasId for Ref<T> {
    fn id(&self) -> &Id {
        &self.id
    }
}

pub trait AsRef {
    fn as_ref(&self) -> Ref<Self>;
}
impl<T: ?Sized + HasId + Display + 'static> AsRef for T {
    fn as_ref(&self) -> Ref<T> {
        Ref {
            id: *self.id(),
            label: self.to_string(),
            marker: std::marker::PhantomData,
        }
    }
}

#[component]
pub fn MultiEntitySelect<T: AsRef + 'static>(
    signal: (Signal<Vec<Id>>, SignalSetter<Vec<Id>>),
    opts: Signal<Collection<Ref<T>>>,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] help: String,
) -> impl IntoView {
    let (current, write) = signal;

    let opts = move || {
        with!(|current, opts| opts
            .iter()
            .map(|opt| {
                let id = *opt.id();
                let label = opt.label.clone();
                let mut selected = current.clone();
                view! {
                    <div
                        class="multi-select-opt"
                        class:selected={current.contains(&id)}
                        on:click=move |_| {
                            if selected.contains(&id) {
                                selected.retain(|v| v != &id);
                            } else {
                                selected.push(id);
                            }
                            write.set(selected.clone());
                        }
                    >
                        {label}
                    </div>
                }
            })
            .collect::<Vec<_>>())
    };

    view! {
        <div class="input-group multi-select-group">
            <label>{label}</label>
            <div class="input-help">{help}</div>
            <div class="multi-select-opts">
                {opts}
            </div>
      </div>
    }
}

#[component]
pub fn EntityPicker<T: AsRef + 'static>(
    signal: (Signal<Id>, SignalSetter<Id>),
    opts: Signal<Collection<Ref<T>>>,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] help: String,
) -> impl IntoView {
    let (read, write) = signal;

    let local = create_rw_signal(read.get_untracked());

    // Initialize the filter string to match the selected entity,
    // if a matching one exists.
    let initial = opts.with_untracked(|opts| {
        opts.try_get(&read.get_untracked())
            .map(|v| v.label.clone())
            .unwrap_or_default()
    });
    let filter = create_rw_signal(initial);

    // Does an entity with a matching id exist in the collection?
    let is_valid = move || {
        with!(|local, opts| opts.try_get(&local).is_some())
    };
    let selected = move || {
        with!(|local, opts| opts
            .try_get(&local)
            .map(|v| v.label.clone())
            .unwrap_or("(None)".into()))
    };

    // Only show the results when the filter input is focused.
    let focused = create_rw_signal(false);
    let results = move || {
        with!(|filter, opts| opts
            .iter()
            .filter(|opt| opt
                .label
                .to_lowercase()
                .contains(&filter.to_lowercase()))
            .map(|opt| {
                let id = *opt.id();
                let label = opt.label.clone();
                view! {
                    <div class="picker-opt" on:click=move |_| {
                        local.set(id);
                    }>{label}</div>
                }
            })
            .collect::<Vec<_>>())
    };

    let ref_input = create_node_ref::<html::Input>();
    create_effect(move |_| {
        if focused.get() {
            if let Some(ref_input) = ref_input.get() {
                ref_input.on_mount(|input| {
                    input.focus();
                    input.select();
                });
            }
        }
    });

    let target = create_node_ref::<html::Div>();
    on_click_outside(target, move |_| {
        if focused.get() {
            focused.set(false);
            write.set(local.get());
        }
    });

    view! {
        <div class="input-group picker-group" ref=target>
            <div class="picker-group-header">
                <label>{label}</label>
                <div class="picker-selected" on:click=move |_| {
                    focused.set(true);
                }>{selected}</div>
            </div>
            <div class="input-help">{help}</div>
            <Show when=move || !is_valid()>
                <div class="input-error">"The selected entity doesn't exist."</div>
            </Show>
            <Show when=move || focused.get()>
                <div class="picker-filter">
                    <input type="text"
                        ref=ref_input
                        placeholder="Search"
                        value={filter.get_untracked()}
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            filter.set(value);
                        }
                    />
                    <div class="picker-results">
                        {results}
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn ImageInput(
    signal: (Signal<Image>, SignalSetter<Image>),
) -> impl IntoView {
    let (read, write) = signal;
    let help = "Images will be bundled with your exported world, so it's recommended that you make sure they aren't too big.";

    let image_src = move || match read.get().data {
        ImageData::File(fname) => {
            format!("/public/images/{fname}",)
        }
        ImageData::Data { bytes, mime } => format!(
            "data:{mime};charset=utf-8;base64,{}",
            BASE64_STANDARD.encode(bytes)
        ),
    };

    view! {
        <div class="image-input">
            <img src={image_src} />
            <TextInput label="Attribution"
                inline=true
                signal=subsignal!(signal.attribution) />
            <input
                type="file"
                multiple=false
                accept="image/png, image/gif, image/jpeg, image/webp"
                on:input=move |ev| {
                    let files = ev.target().unwrap()
                        .unchecked_ref::<web_sys::HtmlInputElement>()
                        .files().unwrap();
                    if let Some(file) = files.get(0) {
                        let mime = file.type_();
                        spawn_local(async move {
                            let bytes = read_file(file).await;
                            let mut image = read.get();
                            image.data = ImageData::Data {
                                bytes,
                                mime,
                            };
                            write.set(image);
                        })
                    }
                }
            />
            <div class="input-help">{help}</div>
        </div>
    }
}

#[component]
pub fn OptionalImageInput(
    signal: (
        Signal<Option<Image>>,
        SignalSetter<Option<Image>>,
    ),
) -> impl IntoView {
    let (read, write) = signal;
    let value = create_rw_signal(
        read.get_untracked().unwrap_or_else(Image::default),
    );

    view! {
        <div class="input-group option-group">
            <ToggleInput
                label="Include Image"
                help=""
                signal=(
                    Signal::derive(move || with!(|read| read.is_some())),
                    SignalSetter::map(move |enable| {
                        let value = if enable {
                            Some(value.get())
                        } else {
                            None
                        };
                        write.set(value);
                    })) />
            <Show when=move || with!(|read| read.is_some())
                fallback=move || view! { <div class="image-placeholder" /> }>
                <ImageInput
                    signal=(
                        Signal::derive(move || read.get().unwrap()),
                        SignalSetter::map(move |image: Image| {
                            let mut opt = read.get();
                            opt.insert(image.clone());
                            value.set(image);
                            write.set(opt);
                        })
                    ) />
            </Show>
        </div>
    }
}

async fn read_file(file: File) -> Vec<u8> {
    let blob: &Blob = file.as_ref();
    let array_buffer_promise = blob.array_buffer();
    let js_array_buffer =
        JsFuture::from(array_buffer_promise).await.unwrap();
    let array_buffer = js_array_buffer
        .dyn_into::<js_sys::ArrayBuffer>()
        .unwrap();
    let uint8_array = Uint8Array::new(&array_buffer);
    uint8_array.to_vec()
}

#[component]
pub fn ToggleInput(
    signal: (Signal<bool>, SignalSetter<bool>),
    #[prop(into, optional)] inner: ViewFn,
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] help: String,
    #[prop(into, optional)] tooltip: bool,
    #[prop(into, optional)] icons: Option<(
        &'static str,
        &'static str,
    )>,
) -> impl IntoView {
    let (read, write) = signal;
    let display = move || {
        let toggled = read.get();
        icons.map_or_else(
            || {
                if toggled {
                    format!("✓ {label}")
                } else {
                    format!("✗ {label}")
                }
            },
            |(on, off)| {
                if toggled {
                    on.to_string()
                } else {
                    off.to_string()
                }
            },
        )
    };

    view! {
        <div class="input-group checkbox-group tooltip-parent">
            <div class="checkbox-inner">
                <label on:click=move |_| {
                    write.set(!read.get());
                }>
                    {display}
                </label>
                {inner.run()}
            </div>
            <div class:input-help=!tooltip class:tooltip=tooltip>{help}</div>
        </div>
    }
}

#[component]
pub fn TextArea(
    signal: (Signal<String>, SignalSetter<String>),
    #[prop(into, optional)] label: String,
    #[prop(into, optional)] help: String,
) -> impl IntoView {
    let (read, write) = signal;
    view! {
        <div class="input-group text-area-group">
            <label>{label}</label>
            <div class="input-help">{help}</div>
            <textarea
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    write.set(value);
                }>{read.get_untracked()}</textarea>
        </div>
    }
}
