use xxhash_rust::xxh3::xxh3_64;

include!(concat!(env!("OUT_DIR"), "/locales.rs"));

pub struct Backend;
impl rust_i18n::Backend for Backend {
    fn available_locales(&self) -> Vec<&str> {
        LOCALES.keys().map(|v| *v).collect()
    }

    fn translate(
        &self,
        locale: &str,
        key: &str,
    ) -> Option<&str> {
        if locale == "en" {
            None
        } else {
            let key = xxh3_64(key.as_bytes());
            LOCALES
                .get(locale)
                .and_then(|trs| trs.get(&key))
                .map(|v| *v)
        }
    }
}
