use gloo_net::http::Request;
use leptos::*;
use leptos_router::*;
use leptos_use::{
    storage::use_local_storage,
    use_intl_number_format,
    use_window,
    NumberStyle,
    UseIntlNumberFormatOptions,
    UseIntlNumberFormatReturn,
};
use std::{collections::BTreeMap, rc::Rc};

use crate::state::Settings;

const DEFAULT_LANGUAGE: &str = "en";
pub const AVAILABLE_LANGUAGES: &[&str] = &[
    "en", "pt", "pt-br", "pt-pt", "es", "de-de", "jp", "fr-fr",
    "th",
];

pub struct Language {
    pub locale: &'static str,
    phrases: Option<BTreeMap<String, String>>,
    number_fmt: UseIntlNumberFormatReturn,
    percent_fmt: UseIntlNumberFormatReturn,
}
impl Language {
    fn new(
        phrases: Option<BTreeMap<String, String>>,
        locale: &'static str,
    ) -> Self {
        Language {
            locale,
            phrases,
            number_fmt: use_intl_number_format(
                UseIntlNumberFormatOptions::default()
                    .locale(locale),
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
    let lang = expect_context::<Rc<Language>>();
    match &lang.phrases {
        None => s.to_string(),
        Some(phrases) => phrases
            .get(s)
            .map(|s| s.to_string())
            .unwrap_or(s.to_string()),
    }
}
pub fn num_fmt() -> impl Fn(f32) -> String {
    let lang = expect_context::<Rc<Language>>();
    move |v: f32| lang.number_fmt.format(v).get_untracked()
}

pub fn per_fmt() -> impl Fn(f32) -> String {
    let lang = expect_context::<Rc<Language>>();
    move |v: f32| lang.percent_fmt.format(v).get_untracked()
}

#[derive(Clone, Params, PartialEq)]
struct QueryParams {
    lang: Option<String>,
}

/// First check for exact match,
/// otherwise search for matches at start of the string.
fn get_language_match(target: &str) -> &'static str {
    let exact_match = AVAILABLE_LANGUAGES
        .iter()
        .find(|lang| *lang == &target);

    if let Some(lang) = exact_match {
        lang
    } else {
        AVAILABLE_LANGUAGES
            .iter()
            .find(|lang| lang.starts_with(target))
            .unwrap_or(&"en")
    }
}

/// In order of priority:
/// - Url search/query param
/// - Local storage
/// - Navigator language
/// - Default
pub fn get_preferred_language() -> &'static str {
    let params = use_query::<QueryParams>();
    if let Some(lang) = params.with_untracked(|params| {
        params.as_ref().ok().and_then(|q| q.lang.clone())
    }) {
        return get_language_match(&lang);
    }

    let (settings, _) = Settings::rw();
    let settings = settings.get_untracked();
    if !settings.language.is_empty() {
        return get_language_match(&settings.language);
    }

    let window = use_window();
    if let Some(navigator) = window.navigator() {
        if let Some(lang) = navigator.language() {
            return get_language_match(&lang);
        }
    }

    DEFAULT_LANGUAGE
}

pub async fn load_language(
    mut lang: &'static str,
) -> anyhow::Result<()> {
    if !AVAILABLE_LANGUAGES.contains(&lang) {
        lang = DEFAULT_LANGUAGE;
    }

    let phrases = if lang == DEFAULT_LANGUAGE {
        None
    } else {
        let url = format!("/assets/lang/{lang}.json");
        let resp = Request::get(&url).send().await?;
        let phrases: BTreeMap<String, String> =
            resp.json().await?;
        Some(phrases)
    };

    let language = Language::new(phrases, &lang);
    provide_context(Rc::new(language));
    Ok(())
}
