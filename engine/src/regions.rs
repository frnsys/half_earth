use crate::consts;
use crate::kinds::{Output, OutputMap};
use serde::ser::{Serialize, Serializer, SerializeStruct};

const DEVELOP_SPEED: f32 = 0.003;

#[derive(Clone)]
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

    /// Local temperature and precipitation
    pub temp_lo: f32,
    pub temp_hi: f32,
    pub precip_lo: f32,
    pub precip_hi: f32,

    pub pattern_idxs: Vec<usize>,
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

    pub fn demand_level(&self, output: &Output) -> usize {
        let demand = self.demand()/self.population;
        if let Some(idx) = consts::OUTPUT_DEMAND.iter().position(|m| m[*output] >= demand[*output]) {
            idx + 1
        } else {
            consts::OUTPUT_DEMAND.len() + 1
        }
    }

    pub fn demand_levels(&self) -> OutputMap<usize> {
        let mut demand_levels: OutputMap<usize> = outputs!();
        for k in demand_levels.keys() {
            demand_levels[k] = self.demand_level(&k);
        }
        demand_levels
    }

    pub fn update_pop(&mut self, year: f32, modifier: f32) {
        self.population *= 1. + (consts::income_pop_change(year, &self.income) * modifier);
    }

    // Outlook slowly rebounds over time
    pub fn update_outlook(&mut self) {
        self.outlook += 0.1;
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

impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_struct("Region", 15)?;
        seq.serialize_field("id", &self.id)?;
        seq.serialize_field("name", &self.name)?;
        seq.serialize_field("population", &self.population)?;
        seq.serialize_field("seceded", &self.seceded)?;
        seq.serialize_field("outlook", &self.outlook)?;
        seq.serialize_field("income", &self.income)?;
        seq.serialize_field("income_level", &self.income_level())?;
        seq.serialize_field("development", &self.development)?;
        seq.serialize_field("habitability", &self.habitability())?;
        seq.serialize_field("demand", &self.demand())?;
        seq.serialize_field("demand_levels", &self.demand_levels())?;
        seq.serialize_field("temp_lo", &self.temp_lo)?;
        seq.serialize_field("temp_hi", &self.temp_hi)?;
        seq.serialize_field("precip_lo", &self.precip_lo)?;
        seq.serialize_field("precip_hi", &self.precip_hi)?;
        seq.end()
    }
}

#[derive(PartialEq, serde::Serialize, Clone)]
pub enum Income {
    Low,
    LowerMiddle,
    UpperMiddle,
    High
}
