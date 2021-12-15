use serde::Serialize;
use crate::kinds::{OutputMap, ByproductMap};
use crate::regions::{Region, Income};

#[derive(Default, Serialize, Clone)]
pub struct World {
    pub year: usize,
    pub base_outlook: f32,
    pub temp_outlook: f32,
    pub extinction_rate: f32,
    pub temperature: f32,     // global temp anomaly, C
    pub precipitation: f32,   // global precip avg
    pub sea_level_rise: f32,  // meters
    pub water_stress: f32,    // 0-100%
    pub co2_emissions: f32,
    pub ch4_emissions: f32,
    pub n2o_emissions: f32,
    pub temperature_modifier: f32,
    pub regions: Vec<Region>,
    pub byproduct_mods: ByproductMap<f32>,
    pub population_growth_modifier: f32,
    pub sea_level_rise_modifier: f32,  // meters
}

impl World {
    pub fn population(&self) -> f32 {
        self.regions.iter().map(|r| r.population).sum()
    }

    pub fn lic_population(&self) -> f32 {
        self.regions.iter().map(|r| r.lic_population()).sum()
    }

    pub fn update_pop(&mut self) {
        for region in &mut self.regions {
            region.update_pop(self.year as f32, 1. + self.population_growth_modifier);
        }
    }

    pub fn outlook(&self) -> f32 {
        let region_outlook = self.regions.iter().map(|r| r.outlook).sum::<f32>()/self.regions.len() as f32;
        self.base_outlook + region_outlook
    }

    pub fn develop_regions(&mut self) {
        for region in &mut self.regions {
            region.develop();
        }
    }

    pub fn update_outlook(&mut self) {
        for region in &mut self.regions {
            region.update_outlook();
        }
    }

    pub fn update_temp_outlook(&mut self, temp_change: f32) {
        let outlook_change = temp_change * 5. * self.temperature.powf(2.);
        self.temp_outlook += outlook_change;
        self.base_outlook += outlook_change;
        for region in &mut self.regions {
            region.outlook += outlook_change * 0.4;
        }
    }

    pub fn habitability(&self) -> f32 {
        self.regions.iter().map(|r| r.habitability()).sum::<f32>()/self.regions.len() as f32
    }

    pub fn emissions(&self) -> f32 {
        self.co2_emissions + (self.n2o_emissions * 298.) + (self.ch4_emissions * 36.)
    }

    pub fn demand(&self) -> OutputMap<f32> {
        self.regions.iter().fold(outputs!(), |mut acc, region| {
            acc += region.demand();
            acc
        })
    }

    pub fn change_population(&mut self, amount: f32) {
        let amount_per_region = amount/self.regions.len() as f32;
        for region in &mut self.regions {
            region.population += amount_per_region;
        }
    }

    pub fn sea_level_rise_rate(&self) -> f32{
        // Meters
        // 0.0005mm per year per deg warming
        // Chosen to roughly hit 1.5m-1.6m rise by 2100 in the BAU scenario
        (0.0065 * self.temperature) + self.sea_level_rise_modifier
    }

    pub fn update_sea_level_rise(&mut self) {
        self.sea_level_rise += self.sea_level_rise_rate();
    }

    pub fn income_level(&self) -> f32 {
        self.regions.iter().map(|r| match r.income {
            Income::Low => 0.,
            Income::LowerMiddle => 1.,
            Income::UpperMiddle => 2.,
            Income::High => 3.,
        } + r.development).sum::<f32>()/self.regions.len() as f32
    }
}
