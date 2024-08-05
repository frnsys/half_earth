use js_sys::Promise;
use leptos::*;
use serde_wasm_bindgen::to_value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/public/js/dist/tgav.js")]
extern "C" {
    type Temperature;

    #[wasm_bindgen(constructor)]
    fn new(start_year: usize) -> Temperature;

    #[wasm_bindgen(method, js_name = addEmissions)]
    fn add_emissions(this: &Temperature, emissions: JsValue);

    #[wasm_bindgen(method, js_name = updateTemperature)]
    fn calc_tgav(this: &Temperature) -> Promise;
}

pub struct HectorRef {
    inner: Rc<RefCell<Temperature>>,
}
impl Clone for HectorRef {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl HectorRef {
    pub fn new(start_year: usize) -> Self {
        let hector = Temperature::new(start_year);
        HectorRef {
            inner: Rc::new(RefCell::new(hector)),
        }
    }

    pub fn add_emissions(
        &self,
        emissions: HashMap<&'static str, f64>,
    ) {
        let emissions = to_value(&emissions).unwrap();
        self.inner.borrow().add_emissions(emissions);
    }

    pub async fn calc_tgav(&self) -> f64 {
        let promise = self.inner.borrow().calc_tgav();
        let future = JsFuture::from(promise);
        let result = future.await.unwrap();
        result.as_f64().unwrap()
    }
}
