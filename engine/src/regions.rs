use crate::consts;
use crate::kinds::OutputMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct Region {
    pub id: usize,

    pub name: &'static str,
    pub population: f32,
    pub seceded: bool,

    pub income: Income,

    /// Public health
    pub health: f32,

    /// How hopeful are people in the region about the future?
    pub outlook: f32,

    /// Base habitability encapsulates
    /// other factors that influence habitability.
    /// E.g. negative events such as hurricanes should subtract
    /// from this value
    pub base_habitability: f32,

    /// Base contentedness encapsulates
    /// other factors that influence contentedness,
    /// e.g. permanent penalties or bonuses to it.
    pub base_contentedness: f32,
}

impl Region {
    // Simple mean
    pub fn contentedness(&self) -> f32 {
        self.base_contentedness + (self.health + self.outlook)/2.
    }

    pub fn habitability(&self) -> f32 {
        // TODO Factors:
        // - regional temp, precip, sea_level_rise, health, safety,
        // - number of negative events
        self.base_habitability + self.health
    }

    pub fn pop_change(&mut self) {
        self.population = consts::income_pop_change(self.population, &self.income);
    }

    pub fn demand(&self) -> OutputMap<f32> {
        let mut demand = outputs!();
        let idx = match self.income {
            Income::Low => 0,
            Income::LowerMiddle => 1,
            Income::UpperMiddle => 2,
            Income::High => 3,
        };
        for (k, v) in consts::OUTPUT_DEMAND[idx].items() {
            demand[k] += v * self.population;
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
        self.population * consts::MATERIALS_BY_INCOME[idx]/consts::MATERIALS_BY_INCOME[0]
    }
}

#[derive(Serialize)]
pub enum Income {
    Low,
    LowerMiddle,
    UpperMiddle,
    High
}
