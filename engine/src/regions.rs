use crate::kinds::{Output, OutputMap};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

// 40 years per level
const DEVELOP_SPEED: f32 = 1. / 40.;

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: usize,

    #[wasm_bindgen(skip)]
    pub name: String,

    pub population: f32,
    pub seceded: bool,

    pub income: Income,
    pub development: f32,

    #[wasm_bindgen(skip)]
    pub flags: Vec<String>,

    /// How hopeful are people in the region about the future?
    pub outlook: f32,

    /// Base habitability encapsulates
    /// other factors that influence habitability.
    /// E.g. negative events such as hurricanes should subtract
    /// from this value
    pub base_habitability: f32,

    /// Local temperature and precipitation
    pub temp_lo: f32,
    pub temp_hi: f32,
    pub precip_lo: f32,
    pub precip_hi: f32,
    pub latitude: Latitude,

    #[wasm_bindgen(skip)]
    pub pattern_idxs: Vec<usize>,
}

#[wasm_bindgen]
impl Region {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Region {
    pub fn habitability(&self) -> f32 {
        // Factors:
        // - [X] regional temp
        // - [ ] precip TODO
        // - [ ] sea_level_rise TODO
        // - [X] number of negative events
        self.base_habitability - (f32::max(0., self.temp_hi - 35.).powf(2.) * 10.)
    }

    pub fn income_level(&self) -> usize {
        match self.income {
            Income::Low => 0,
            Income::LowerMiddle => 1,
            Income::UpperMiddle => 2,
            Income::High => 3,
        }
    }

    pub fn adjusted_income(&self) -> f32 {
        let income = self.income_level() as f32;
        income + self.development
    }

    pub fn demand_level(&self, output: &Output, output_demand: &[OutputMap; 4]) -> usize {
        let demand = self.demand(output_demand) / self.population;
        if let Some(idx) = output_demand
            .iter()
            .position(|m| m[*output] >= demand[*output])
        {
            idx + 1
        } else {
            output_demand.len() + 1
        }
    }

    pub fn demand_levels(&self, output_demand: &[OutputMap; 4]) -> OutputMap {
        let mut demand_levels: OutputMap = outputs!();
        for k in demand_levels.keys() {
            demand_levels[k] = self.demand_level(&k, output_demand) as f32;
        }
        demand_levels
    }

    pub fn update_pop(&mut self, year: f32, modifier: f32, income_pop_coefs: &[[f32; 4]; 4]) {
        let coefs = match &self.income {
            Income::Low => income_pop_coefs[0],
            Income::LowerMiddle => income_pop_coefs[1],
            Income::UpperMiddle => income_pop_coefs[2],
            Income::High => income_pop_coefs[3],
        };
        let change = coefs[0]
            + (coefs[1] * year)
            + (coefs[2] * year.powf(2.0))
            + (coefs[3] * year.powf(3.0));
        self.population *= 1. + (change * modifier);
    }

    // Outlook slowly rebounds over time
    pub fn update_outlook(&mut self, wretched_ally: bool, consumerist_ally: bool) {
        let buffed = match self.income {
            Income::Low => wretched_ally,
            Income::LowerMiddle => wretched_ally,
            Income::UpperMiddle => consumerist_ally,
            Income::High => consumerist_ally,
        };
        self.outlook += if buffed { 0.3 } else { 0.1 };
        self.outlook = f32::min(10., self.outlook);
    }

    pub fn develop(&mut self, modifier: f32) {
        self.development += DEVELOP_SPEED * modifier;
        if self.development >= 1.0 {
            let next_income = match self.income {
                Income::Low => Income::LowerMiddle,
                Income::LowerMiddle => Income::UpperMiddle,
                Income::UpperMiddle => Income::High,
                Income::High => Income::High,
            };
            self.development = 0.;
            self.income = next_income;
        } else if self.development < 0. {
            let next_income = match self.income {
                Income::Low => Income::Low,
                Income::LowerMiddle => Income::Low,
                Income::UpperMiddle => Income::LowerMiddle,
                Income::High => Income::UpperMiddle,
            };
            self.development = 1. - self.development;
            self.income = next_income;
        }
    }

    pub fn demand(&self, output_demand: &[OutputMap; 4]) -> OutputMap {
        let mut demand = outputs!();
        let idx = self.income_level();
        if idx < 3 {
            let upper_demand = output_demand[idx + 1];
            for (k, v_a) in output_demand[idx].items() {
                let v_b = upper_demand[k];
                let v = (v_b - v_a) * self.development + v_a;
                demand[k] = v * self.population;
            }
        } else {
            for (k, v) in output_demand[idx].items() {
                demand[k] = v * self.population;
            }
        }
        demand
    }

    /// Low-income capita population;
    /// i.e. equivalent population with the same
    /// aggregate consumption but each individual
    /// consumes at a low-income level
    pub fn lic_population(&self, materials_by_income: &[f32; 4]) -> f32 {
        let idx = match self.income {
            Income::Low => 0,
            Income::LowerMiddle => 1,
            Income::UpperMiddle => 2,
            Income::High => 3,
        };
        let per_capita_demand = if idx < 3 {
            let upper_demand = materials_by_income[idx + 1];
            let demand = materials_by_income[idx];
            (upper_demand - demand) * self.development + demand
        } else {
            materials_by_income[idx]
        };
        self.population * per_capita_demand / materials_by_income[0]
    }
}

// impl Serialize for Region {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_struct("Region", 15)?; // TODO derived fields
//         seq.serialize_field("id", &self.id)?;
//         seq.serialize_field("name", &self.name)?;
//         seq.serialize_field("population", &self.population)?;
//         seq.serialize_field("seceded", &self.seceded)?;
//         seq.serialize_field("outlook", &self.outlook)?;
//         seq.serialize_field("income", &self.income)?;
//         seq.serialize_field("income_level", &self.income_level())?;
//         seq.serialize_field("development", &self.development)?;
//         seq.serialize_field("habitability", &self.habitability())?;
//         seq.serialize_field("demand", &self.demand())?;
//         seq.serialize_field("demand_levels", &self.demand_levels())?;
//         seq.serialize_field("temp_lo", &self.temp_lo)?;
//         seq.serialize_field("temp_hi", &self.temp_hi)?;
//         seq.serialize_field("precip_lo", &self.precip_lo)?;
//         seq.serialize_field("precip_hi", &self.precip_hi)?;
//         seq.end()
//     }
// }

#[wasm_bindgen]
#[derive(PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Income {
    Low,
    LowerMiddle,
    UpperMiddle,
    High,
}

#[wasm_bindgen]
#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Latitude {
    Tropic,
    Subtropic,
    Temperate,
    Frigid,
}
