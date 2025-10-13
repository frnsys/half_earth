use std::collections::HashMap;
pub type EmissionsData =
    HashMap<String, HashMap<String, Vec<f64>>>;

#[cfg(not(target_arch = "wasm32"))]
mod tgav {
    use std::collections::HashMap;

    use hector::{emissions::get_emissions, run_hector};

    const DEFAULT_SCENARIO: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/hector/rcp26.default_emissions.json"
    ));

    pub struct Climate {
        start_year: usize,
        emissions: super::EmissionsData,
        default_emissions: HashMap<&'static str, f64>,
    }
    impl Climate {
        pub fn new(start_year: usize) -> Self {
            Self {
                start_year,
                emissions: get_emissions(start_year),
                default_emissions: serde_json::from_str(
                    DEFAULT_SCENARIO,
                )
                .expect("default emissions are valid"),
            }
        }

        pub fn set_emissions_data(
            &mut self,
            data: super::EmissionsData,
        ) {
            self.emissions = data;
        }

        pub fn emissions_data(&self) -> &super::EmissionsData {
            &self.emissions
        }

        pub fn add_emissions(
            &mut self,
            emissions: HashMap<&'static str, f64>,
        ) {
            for (k, val) in self.default_emissions.iter() {
                let val = emissions.get(k).unwrap_or(val);

                let mut found = false;
                'outer: for (_, hm) in self.emissions.iter_mut()
                {
                    for (k_, vals) in hm.iter_mut() {
                        if k_ == k {
                            vals.push(*val);
                            found = true;
                            break 'outer;
                        }
                    }
                }

                if !found {
                    panic!("{k} not found");
                }
            }
        }

        pub fn calc_tgav(&self) -> f64 {
            let end_year = self.start_year
                + self.emissions["simpleNbox"]["ffi_emissions"]
                    .len();
            let tgav = unsafe {
                run_hector(end_year, &self.emissions)
            };
            tgav
        }
    }
}
pub use tgav::*;

#[cfg(target_arch = "wasm32")]
mod tgav {
    use js_sys::Promise;
    use leptos::*;
    use serde::ser::Serialize;
    use serde_wasm_bindgen::Serializer;
    use std::{cell::RefCell, collections::HashMap, rc::Rc};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    #[wasm_bindgen(module = "/public/js/dist/tgav.js")]
    extern "C" {
        type Temperature;

        #[wasm_bindgen(constructor)]
        fn new(start_year: usize) -> Temperature;

        #[wasm_bindgen(method, js_name = addEmissions)]
        fn add_emissions(
            this: &Temperature,
            emissions: JsValue,
        );

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
            let serializer = Serializer::new()
                .serialize_maps_as_objects(true);
            let emissions =
                emissions.serialize(&serializer).unwrap();
            self.inner.borrow().add_emissions(emissions);
        }

        pub async fn calc_tgav(&self) -> f64 {
            let promise = self.inner.borrow().calc_tgav();
            let future = JsFuture::from(promise);
            let result = future.await.unwrap();
            result.as_f64().unwrap()
        }
    }
}
