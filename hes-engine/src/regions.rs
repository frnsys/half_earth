use std::fmt::Display;

use crate::{
    events::RegionFlag,
    flavor::RegionFlavor,
    kinds::{Output, OutputMap},
    HasId,
    Id,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

// 40 years per level
const DEVELOP_SPEED: f32 = 1. / 40.;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Region {
    pub id: Id,

    pub name: String,

    pub population: f32,
    pub seceded: bool,

    pub income: Income,
    pub development: f32,

    pub flags: Vec<RegionFlag>,

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

    pub flavor: RegionFlavor,
    pub pattern_idxs: Vec<usize>,
}

impl HasId for Region {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl Region {
    pub fn habitability(&self) -> f32 {
        // Factors:
        // - [X] regional temp
        // - [ ] precip TODO
        // - [ ] sea_level_rise TODO
        // - [X] number of negative events
        self.base_habitability
            - (f32::max(0., self.temp_hi - 35.).powf(2.) * 10.)
    }

    pub fn income_level(&self) -> usize {
        match self.income {
            Income::Low => 0,
            Income::LowerMiddle => 1,
            Income::UpperMiddle => 2,
            Income::High => 3,
        }
    }

    pub fn set_income_level(&mut self, level: usize) {
        self.income = match level {
            0 => Income::Low,
            1 => Income::LowerMiddle,
            2 => Income::UpperMiddle,
            _ => Income::High,
        };
    }

    pub fn adjusted_income(&self) -> f32 {
        let income = self.income_level() as f32;
        income + self.development
    }

    pub fn demand_level(
        &self,
        output: &Output,
        output_demand: &[OutputMap; 4],
    ) -> usize {
        let demand =
            self.demand(output_demand) / self.population;
        if let Some(idx) = output_demand
            .iter()
            .position(|m| m[*output] >= demand[*output])
        {
            idx + 1
        } else {
            output_demand.len() + 1
        }
    }

    pub fn demand_levels(
        &self,
        output_demand: &[OutputMap; 4],
    ) -> OutputMap {
        let mut demand_levels: OutputMap = outputs!();
        for k in demand_levels.keys() {
            demand_levels[k] =
                self.demand_level(&k, output_demand) as f32;
        }
        demand_levels
    }

    pub fn update_pop(
        &mut self,
        year: f32,
        modifier: f32,
        income_pop_coefs: &[[f32; 4]; 4],
    ) {
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
    pub fn update_outlook(
        &mut self,
        wretched_ally: bool,
        consumerist_ally: bool,
    ) {
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

    pub fn demand(
        &self,
        output_demand: &[OutputMap; 4],
    ) -> OutputMap {
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
    pub fn lic_population(
        &self,
        materials_by_income: &[f32; 4],
    ) -> f32 {
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
        self.population * per_capita_demand
            / materials_by_income[0]
    }

    pub fn temp_range(&self) -> String {
        format!(
            "{}-{}°C",
            self.temp_lo.round(),
            self.temp_hi.round()
        )
    }

    pub fn precip_range(&self) -> String {
        format!(
            "{}-{}cm/yr",
            self.precip_lo.round(),
            self.precip_hi.round()
        )
    }

    pub fn is_max_income(&self) -> bool {
        self.income == Income::High
    }
}

#[derive(
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    EnumIter,
    Debug,
    EnumString,
    IntoStaticStr,
)]
pub enum Income {
    Low,
    LowerMiddle,
    UpperMiddle,
    High,
}
impl Display for Income {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Income::Low => "Low",
                Income::High => "High",
                Income::LowerMiddle => "Lower-Middle",
                Income::UpperMiddle => "Upper-Middle",
            }
        )
    }
}

#[derive(
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    EnumIter,
    IntoStaticStr,
    EnumString,
    Display,
)]
pub enum Latitude {
    Tropic,
    Subtropic,
    Temperate,
    Frigid,
}
