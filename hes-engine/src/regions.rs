use std::fmt::Display;

use crate::{
    events::RegionFlag,
    flavor::RegionFlavor,
    kinds::*,
    HasId,
    Id,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

// 40 years per level
const DEVELOP_SPEED: f32 = 1. / 40.;

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
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
    pub fn develop(
        &mut self,
        speed: f32,
        stop: bool,
        degrow: bool,
    ) -> (usize, usize) {
        let start = self.income.level();
        if degrow && self.income == Income::High {
            self.develop_by(-1.);
        } else if !stop && self.income != Income::High {
            if !(degrow && self.income == Income::UpperMiddle) {
                self.develop_by(speed);
            }
        }
        let end = self.income.level();
        (start, end)
    }

    pub fn habitability(&self) -> f32 {
        // Factors:
        // - [X] regional temp
        // - [ ] precip TODO
        // - [ ] sea_level_rise TODO
        // - [X] number of negative events
        self.base_habitability
            - (f32::max(0., self.temp_hi - 35.).powf(2.) * 10.)
    }

    pub fn set_income_level(&mut self, level: usize) {
        self.income = level.into();
    }

    pub fn adjusted_income(&self) -> f32 {
        self.income.level() as f32 + self.development
    }

    pub fn demand_level(
        &self,
        output: &Output,
        output_demand: &[OutputDemand; 4],
    ) -> usize {
        let demand =
            self.demand(output_demand) / self.population;
        if let Some(idx) = output_demand
            .iter()
            .position(|m| m.of(*output) >= demand[*output])
        {
            idx + 1
        } else {
            output_demand.len() + 1
        }
    }

    pub fn demand_levels(
        &self,
        output_demand: &[OutputDemand; 4],
    ) -> OutputMap {
        let mut demand_levels: OutputMap = outputs!();
        for (k, v) in demand_levels.items_mut() {
            *v = self.demand_level(&k, output_demand) as f32;
        }
        demand_levels
    }

    pub fn update_pop(
        &mut self,
        year: f32,
        modifier: f32,
        income_pop_coefs: &[[f32; 4]; 4],
    ) {
        let coefs = income_pop_coefs[self.income.level()];
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

    fn develop_by(&mut self, modifier: f32) {
        self.development += DEVELOP_SPEED * modifier;
        if self.development >= 1.0 {
            self.development = 0.;
            self.income = self.income.next();
        } else if self.development < 0. {
            self.development = 1. - self.development;
            self.income = self.income.prev();
        }
    }

    pub fn demand(
        &self,
        output_demand: &[OutputDemand; 4],
    ) -> OutputMap {
        let mut demand = outputs!();
        let idx = self.income.level();
        if idx < 3 {
            let upper_demand = output_demand[idx + 1].total();
            for (k, v_a) in output_demand[idx].total().items() {
                let v_b = upper_demand[k];
                let v = (v_b - v_a) * self.development + v_a;
                demand[k] = v * self.population;
            }
        } else {
            for (k, v) in output_demand[idx].total().items() {
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
        let idx = self.income.level();
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
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
pub enum Income {
    #[default]
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
impl Income {
    pub fn next(&self) -> Self {
        match self {
            Income::Low => Income::LowerMiddle,
            Income::LowerMiddle => Income::UpperMiddle,
            Income::UpperMiddle => Income::High,
            Income::High => Income::High,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Income::Low => Income::Low,
            Income::LowerMiddle => Income::Low,
            Income::UpperMiddle => Income::LowerMiddle,
            Income::High => Income::UpperMiddle,
        }
    }

    pub fn level(&self) -> usize {
        match self {
            Income::Low => 0,
            Income::LowerMiddle => 1,
            Income::UpperMiddle => 2,
            Income::High => 3,
        }
    }
}
impl From<usize> for Income {
    fn from(value: usize) -> Self {
        match value {
            0 => Income::Low,
            1 => Income::LowerMiddle,
            2 => Income::UpperMiddle,
            _ => Income::High,
        }
    }
}

#[derive(
    Default,
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
    #[default]
    Tropic,
    Subtropic,
    Temperate,
    Frigid,
}
