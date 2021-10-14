use crate::kinds::OutputMap;
use crate::consts::{OUTPUT_DEMAND, income_pop_change};

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
        (self.health + self.outlook)/2.
    }

    pub fn habitability(&self) -> f32 {
        // TODO Factors:
        // - regional temp, precip, sea_level_rise, health, safety,
        // - number of negative events
        todo!()
    }

    pub fn pop_change(&mut self) {
        self.population = income_pop_change(self.population, &self.income);
    }

    pub fn demand(&self) -> OutputMap<f32> {
        let mut demand = outputs!();
        let idx = match self.income {
            Income::Low => 0,
            Income::LowerMiddle => 1,
            Income::UpperMiddle => 2,
            Income::High => 3,
        };
        for (k, v) in OUTPUT_DEMAND.items() {
            demand[k] += v[idx] * self.population;
        }
        demand
    }
}

pub enum Income {
    Low,
    LowerMiddle,
    UpperMiddle,
    High
}
