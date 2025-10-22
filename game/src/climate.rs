//! Two different interfaces to Hector are provided here: one for native builds,
//! where C++ FFI is ok, and one for web/WASM builds where it isn't. For WASM builds
//! Hector is compiled to it's own WASM binary and called through Javascript.
//! That JS glue code is from the old web version of the game, I kept it mostly intact.
//!
//! The actual `tgav` calculation method is a bit weird because it has to be async for web.
//! So on native you really just call it once and get the temperature anomaly immediately.
//! On web you have to poll it, calling `tgav` each frame until `Some` is returned.

#[cfg(not(target_arch = "wasm32"))]
mod tgav {
    use std::collections::HashMap;

    use hector::{emissions::get_emissions, run_hector};

    pub type EmissionsData = HashMap<String, HashMap<String, Vec<f64>>>;

    const DEFAULT_SCENARIO: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/hector/rcp26.default_emissions.json"
    ));

    pub struct Climate {
        emissions: EmissionsData,
        default_emissions: HashMap<&'static str, f64>,
    }
    impl Climate {
        pub fn new(start_year: usize) -> Self {
            Self {
                emissions: get_emissions(start_year),
                default_emissions: serde_json::from_str(DEFAULT_SCENARIO)
                    .expect("default emissions are valid"),
            }
        }

        pub fn set_emissions_data(&mut self, data: EmissionsData) {
            self.emissions = data;
        }

        pub fn emissions_data(&self) -> EmissionsData {
            self.emissions.clone()
        }

        pub fn add_emissions(&mut self, emissions: HashMap<&'static str, f64>) {
            for (k, val) in self.default_emissions.iter() {
                let val = emissions.get(k).unwrap_or(val);

                let mut found = false;
                'outer: for (_, hm) in self.emissions.iter_mut() {
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

        pub fn tgav(&self, year: usize) -> Option<f32> {
            let tgav = unsafe { run_hector(year, &self.emissions) };
            Some(tgav as f32)
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod tgav {
    use poll_promise::Promise;
    use serde::ser::Serialize;
    use serde_wasm_bindgen::Serializer;
    use std::{cell::RefCell, collections::HashMap, rc::Rc};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    pub type EmissionsData = HashMap<String, Vec<f64>>;

    #[wasm_bindgen(module = "/assets/js/dist/tgav.js")]
    extern "C" {
        type Temperature;

        #[wasm_bindgen(constructor)]
        fn new(start_year: usize) -> Temperature;

        #[wasm_bindgen(method, js_name = addEmissions)]
        fn add_emissions(this: &Temperature, emissions: JsValue);

        #[wasm_bindgen(method, js_name = setEmissions)]
        fn set_emissions(this: &Temperature, emissions: JsValue);

        #[wasm_bindgen(method, js_name = getEmissions)]
        fn get_emissions(this: &Temperature) -> JsValue;

        #[wasm_bindgen(method, js_name = updateTemperature)]
        fn calc_tgav(this: &Temperature) -> wasm_bindgen_futures::js_sys::Promise;
    }

    pub struct Climate {
        inner: Rc<RefCell<Temperature>>,
        task: Option<(usize, Promise<f32>)>,
    }
    impl Climate {
        pub fn new(start_year: usize) -> Self {
            let hector = Temperature::new(start_year);
            Climate {
                inner: Rc::new(RefCell::new(hector)),
                task: None,
            }
        }

        pub fn set_emissions_data(&mut self, data: EmissionsData) {
            let serializer = Serializer::new().serialize_maps_as_objects(true);
            let emissions = data.serialize(&serializer).unwrap();
            self.inner.borrow().set_emissions(emissions);
        }

        pub fn emissions_data(&self) -> EmissionsData {
            let data = self.inner.borrow().get_emissions();
            serde_wasm_bindgen::from_value(data).unwrap()
        }

        pub fn add_emissions(&self, emissions: HashMap<&'static str, f64>) {
            let serializer = Serializer::new().serialize_maps_as_objects(true);
            let emissions = emissions.serialize(&serializer).unwrap();
            self.inner.borrow().add_emissions(emissions);
        }

        pub fn tgav(&mut self, year: usize) -> Option<f32> {
            match &self.task {
                Some((y, prom)) if *y == year => prom.ready().cloned(),
                _ => {
                    let inner = self.inner.clone();
                    self.task = Some((
                        year,
                        Promise::spawn_local(async move {
                            let promise = inner.borrow().calc_tgav();
                            let future = JsFuture::from(promise);
                            let result = future.await.unwrap();
                            result.as_f64().unwrap() as f32
                        }),
                    ));
                    None
                }
            }
        }
    }
}

pub use tgav::*;
