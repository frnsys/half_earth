use xxhash_rust::xxh3::xxh3_64;

// This file is compiled from the translation files at build time.
include!(concat!(env!("OUT_DIR"), "/locales.rs"));

pub struct Backend;
impl rust_i18n::Backend for Backend {
    fn available_locales(&self) -> Vec<&str> {
        LOCALES.keys().copied().collect()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        // Assume the key is already in English so no need to translate it.
        if locale == "en" {
            None
        } else {
            let key = xxh3_64(key.as_bytes());
            LOCALES.get(locale).and_then(|trs| trs.get(&key)).copied()
        }
    }
}
