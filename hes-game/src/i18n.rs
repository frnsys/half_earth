use gloo_net::http::Request;
use leptos::*;
use leptos_router::*;
use leptos_use::{
    storage::use_local_storage, use_intl_number_format, use_window, utils::FromToStringCodec,
    NumberStyle, UseIntlNumberFormatOptions, UseIntlNumberFormatReturn,
};
use std::collections::HashMap;

const DEFAULT_LANGUAGE: &str = "en";
pub const AVAILABLE_LANGUAGES: &[&str] = &[
    "en", "pt", "pt-br", "pt-pt", "es", "de-de", "jp", "fr-fr", "th",
];

pub struct Language {
    pub locale: String,
    phrases: Option<HashMap<String, String>>,
    number_fmt: UseIntlNumberFormatReturn,
    percent_fmt: UseIntlNumberFormatReturn,
}
impl Language {
    fn new(phrases: Option<HashMap<String, String>>, locale: &str) -> Self {
        Language {
            locale: locale.to_string(),
            phrases,
            number_fmt: use_intl_number_format(
                UseIntlNumberFormatOptions::default().locale(locale),
            ),
            percent_fmt: use_intl_number_format(
                UseIntlNumberFormatOptions::default()
                    .locale(locale)
                    .style(NumberStyle::Percent)
                    .minimum_fraction_digits(2)
                    .maximum_fraction_digits(2),
            ),
        }
    }
}

#[macro_export]
macro_rules! t {
    ($text:expr) => {{
        crate::i18n::t($text)
    }};
    ($text:expr, $($key:ident: $val:expr),* $(,)?) => {{
        use crate::i18n::t;
        let mut result = t($text);
        $(
            let pattern = concat!("{", stringify!($key), "}");
            result = result.replace(pattern, &$val.to_string());
        )*
        result
    }};
}

pub fn t(s: &str) -> String {
    let lang = expect_context::<RwSignal<Language>>();
    lang.with(|lang| match &lang.phrases {
        None => s.to_string(),
        Some(phrases) => phrases
            .get(s)
            .map(|s| s.to_string())
            .unwrap_or(s.to_string()),
    })
}
pub fn num_fmt() -> impl Fn(f32) -> String {
    let lang = expect_context::<RwSignal<Language>>();
    move |v: f32| lang.with(|lang| lang.number_fmt.format(v)).get()
}

pub fn per_fmt() -> impl Fn(f32) -> String {
    let lang = expect_context::<RwSignal<Language>>();
    move |v: f32| lang.with(|lang| lang.percent_fmt.format(v)).get()
}

#[derive(Clone, Params, PartialEq)]
struct QueryParams {
    lang: Option<String>,
}

/// In order of priority:
/// - Url search/query param
/// - Local storage
/// - Navigator language
/// - Default
pub fn get_preferred_language() -> String {
    let params = use_query::<QueryParams>();
    if let Some(lang) = params.with_untracked(|params| params.clone().ok().and_then(|q| q.lang)) {
        return lang;
    }

    let (storage, _, _) = use_local_storage::<String, FromToStringCodec>("hes.language");
    let lang = storage.get_untracked();
    if !lang.is_empty() {
        return lang;
    }

    let window = use_window();
    if let Some(navigator) = window.navigator() {
        if let Some(lang) = navigator.language() {
            return lang;
        }
    }

    DEFAULT_LANGUAGE.to_string()
}

pub async fn load_language(mut lang: &str) -> anyhow::Result<()> {
    if !AVAILABLE_LANGUAGES.contains(&lang) {
        lang = DEFAULT_LANGUAGE;
    }

    let phrases = if lang == DEFAULT_LANGUAGE {
        None
    } else {
        let url = format!("/public/assets/lang/{lang}.json");
        let resp = Request::get(&url).send().await?;
        let phrases: HashMap<String, String> = resp.json().await?;
        Some(phrases)
    };

    let language = Language::new(phrases, &lang);
    if let Some(ctx) = use_context::<RwSignal<Language>>() {
        ctx.set(language);
    } else {
        provide_context(create_rw_signal(language));
    }
    Ok(())
}
