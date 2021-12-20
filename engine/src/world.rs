use crate::kinds::{OutputMap, ByproductMap};
use crate::regions::{Region, Income};
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Default, Clone)]
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

    pub fn update_tgav(&mut self, tgav: f32) {
        self.temperature = tgav + self.temperature_modifier;
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

    pub fn update_sea_level_rise(&mut self) {
        self.sea_level_rise += self.sea_level_rise_rate();
    }

    pub fn sea_level_rise_rate(&self) -> f32 {
        // Meters
        // Chosen to roughly hit 1.4m-1.6m rise by 2100 in the BAU scenario
        (0.0025 * self.temperature.powf(1.5)) + self.sea_level_rise_modifier
    }

    /// Contribution to extinction rate from the tgav
    pub fn tgav_extinction_rate(&self) -> f32 {
        self.temperature.powf(2.)
    }

    /// Contribution to extinction rate from sea level rise
    pub fn slr_extinction_rate(&self) -> f32 {
        self.sea_level_rise.powf(2.)
    }

    pub fn base_extinction_rate(&self) -> f32 {
        self.tgav_extinction_rate() + self.slr_extinction_rate()
            - self.byproduct_mods.biodiversity
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

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let total_emissions = (self.co2_emissions + (self.n2o_emissions * 298.) + (self.ch4_emissions * 36.)) * 1e-15;

        let mut seq = serializer.serialize_struct("World", 17)?;
        seq.serialize_field("year", &self.year)?;
        seq.serialize_field("contentedness", &self.outlook())?;
        seq.serialize_field("temp_outlook", &self.temp_outlook)?;
        seq.serialize_field("extinction_rate", &self.extinction_rate)?;
        seq.serialize_field("tgav_extinction_rate", &self.tgav_extinction_rate())?;
        seq.serialize_field("slr_extinction_rate", &self.slr_extinction_rate())?;
        seq.serialize_field("temperature", &self.temperature)?;
        seq.serialize_field("precipitation", &self.precipitation)?;
        seq.serialize_field("sea_level_rise", &self.sea_level_rise)?;
        seq.serialize_field("sea_level_rise_rate", &self.sea_level_rise_rate())?;
        seq.serialize_field("water_stress", &self.water_stress)?;
        seq.serialize_field("emissions", &total_emissions)?;
        seq.serialize_field("co2_emissions", &self.co2_emissions)?;
        seq.serialize_field("ch4_emissions", &self.ch4_emissions)?;
        seq.serialize_field("n2o_emissions", &self.n2o_emissions)?;
        seq.serialize_field("regions", &self.regions)?;
        seq.serialize_field("population", &self.population())?;
        seq.end()
    }
}
