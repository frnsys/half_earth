use serde::de::DeserializeOwned;
use serde_json::{Value, from_value};

pub trait Saveable {
    fn save(&self) -> Value;
    fn load(&mut self, state: Value);
}

pub fn coerce<T: DeserializeOwned>(value: &Value) -> T {
    from_value(value.clone()).unwrap()
}
