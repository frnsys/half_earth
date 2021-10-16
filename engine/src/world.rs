use serde::Serialize;
use crate::regions::Region;
use crate::kinds::OutputMap;

#[derive(Default, Serialize)]
pub struct World {
    pub year: usize,
    pub extinction_rate: f32,
    pub temperature: f32,     // global temp anomaly, C
    pub precipitation: f32,   // global precip avg
    pub sea_level_rise: f32,  // meters
    pub water_stress: f32,    // 0-100%
    pub co2_emissions: f32,
    pub ch4_emissions: f32,
    pub n2o_emissions: f32,
    pub regions: Vec<Region>,
}

impl World {
    pub fn population(&self) -> f32 {
        self.regions.iter().map(|r| r.population).sum()
    }

    pub fn lic_population(&self) -> f32 {
        self.regions.iter().map(|r| r.lic_population()).sum()
    }

    pub fn contentedness(&self) -> f32 {
        self.regions.iter().map(|r| r.contentedness()).sum::<f32>()/self.regions.len() as f32
    }

    pub fn health(&self) -> f32 {
        self.regions.iter().map(|r| r.health).sum::<f32>()/self.regions.len() as f32
    }

    pub fn outlook(&self) -> f32 {
        self.regions.iter().map(|r| r.outlook).sum::<f32>()/self.regions.len() as f32
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

    pub fn change_population(&mut self, percent: f32) {
        for region in &mut self.regions {
            region.population *= 1. + percent;
        }
    }

    pub fn change_outlook(&mut self, amount: f32) {
        for region in &mut self.regions {
            region.outlook += amount;
        }
    }

    pub fn change_health(&mut self, amount: f32) {
        for region in &mut self.regions {
            region.health += amount;
        }
    }

    pub fn change_contentedness(&mut self, amount: f32) {
        for region in &mut self.regions {
            region.health += amount/2.;
            region.outlook += amount/2.;
        }
    }

    pub fn change_emissions(&mut self, percent: f32) {
        self.co2_emissions *= 1. + percent;
    }
}
