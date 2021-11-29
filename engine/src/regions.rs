use crate::consts;
use crate::kinds::{Output, OutputMap};
use serde::Serialize;

#[cfg(not(feature = "static_development"))]
const DEVELOP_SPEED: f32 = 0.003;

#[cfg(feature = "static_development")]
const DEVELOP_SPEED: f32 = 0.0;

#[derive(Serialize, Clone)]
pub struct Region {
    pub id: usize,

    pub name: &'static str,
    pub population: f32,
    pub seceded: bool,

    pub income: Income,
    pub development: f32,
    pub flags: Vec<String>,

    /// How hopeful are people in the region about the future?
    pub outlook: f32,

    /// Base habitability encapsulates
    /// other factors that influence habitability.
    /// E.g. negative events such as hurricanes should subtract
    /// from this value
    pub base_habitability: f32,
}

impl Region {
    pub fn habitability(&self) -> f32 {
        // TODO Factors:
        // - regional temp, precip, sea_level_rise
        // - number of negative events
        self.base_habitability
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

    pub fn demand_level(&self, output: &Output) -> usize {
        let demand = self.demand();
        if let Some(idx) = consts::OUTPUT_DEMAND.iter().position(|m| m[*output] >= demand[*output]) {
            idx + 1
        } else {
            consts::OUTPUT_DEMAND.len() + 1
        }
    }

    pub fn update_pop(&mut self, year: f32) {
        if !cfg!(feature = "static_population") {
            self.population *= 1. + consts::income_pop_change(year, &self.income);
        }
    }

    pub fn develop(&mut self) {
        if self.income != Income::High {
            self.development += DEVELOP_SPEED;
            if self.development >= 1.0 {
                let next_income = match self.income {
                    Income::Low => Income::LowerMiddle,
                    Income::LowerMiddle => Income::UpperMiddle,
                    Income::UpperMiddle => Income::High,
                    Income::High => Income::High,
                };
                self.development = 0.;
                self.income = next_income;
            }
        }
    }

    pub fn demand(&self) -> OutputMap<f32> {
        let mut demand = outputs!();
        let idx = self.income_level();
        if idx < 3 {
            let upper_demand = consts::OUTPUT_DEMAND[idx+1];
            for (k, v_a) in consts::OUTPUT_DEMAND[idx].items() {
                let v_b = upper_demand[k];
                let v = (v_b - v_a) * self.development + v_a;
                demand[k] = v * self.population;
            }
        } else {
            for (k, v) in consts::OUTPUT_DEMAND[idx].items() {
                demand[k] = v * self.population;
            }
        }
        demand
    }

    /// Low-income capita population;
    /// i.e. equivalent population with the same
    /// aggregate consumption but each individual
    /// consumes at a low-income level
    pub fn lic_population(&self) -> f32 {
        let idx = match self.income {
            Income::Low => 0,
            Income::LowerMiddle => 1,
            Income::UpperMiddle => 2,
            Income::High => 3,
        };
        let per_capita_demand = if idx < 3 {
            let upper_demand = consts::MATERIALS_BY_INCOME[idx+1];
            let demand = consts::MATERIALS_BY_INCOME[idx];
            (upper_demand - demand) * self.development + demand
        } else {
            consts::MATERIALS_BY_INCOME[idx]
        };
        self.population * per_capita_demand/consts::MATERIALS_BY_INCOME[0]
    }
}

#[derive(PartialEq, Serialize, Clone)]
pub enum Income {
    Low,
    LowerMiddle,
    UpperMiddle,
    High
}
