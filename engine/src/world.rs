use crate::kinds::{OutputMap, ByproductMap};
use crate::regions::{Region, Income};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::save::{Saveable, coerce};
use serde_json::{json, Value};

#[derive(Default, Clone)]
pub struct World {
    pub year: usize,
    pub base_outlook: f32,
    pub temp_outlook: f32,
    pub shortages_outlook: f32,
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
        self.base_outlook - self.shortages_outlook + region_outlook
    }

    /// Returns a vec of ids of regions that "leveled up"
    /// and a vec of ids of regions that "leveled down"
    pub fn develop_regions(&mut self, stop: bool, fast: bool, degrow: bool) -> (Vec<usize>, Vec<usize>) {
        let mut up = vec![];
        let mut down = vec![];

        let speed = if fast {
            1.25
        } else {
            1.
        };
        for region in &mut self.regions {
            let start = region.income_level();
            if degrow && region.income == Income::High {
                region.develop(-1.);
            } else if !stop && region.income != Income::High {
                if !(degrow && region.income == Income::UpperMiddle) {
                    region.develop(speed);
                }
            }
            let end = region.income_level();
            if end < start {
                down.push(region.id);
            } else if end > start {
                up.push(region.id);
            }
        }
        (up, down)
    }

    pub fn update_outlook(&mut self, wretched_ally: bool, consumerist_ally: bool) {
        for region in &mut self.regions {
            region.update_outlook(wretched_ally, consumerist_ally);
        }
    }

    pub fn update_temp_outlook(&mut self, temp_change: f32) {
        let outlook_change = temp_change * 6. * self.temperature.powf(2.);
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

        let mut seq = serializer.serialize_struct("World", 18)?;
        seq.serialize_field("year", &self.year)?;
        seq.serialize_field("contentedness", &self.outlook())?;
        seq.serialize_field("temp_outlook", &self.temp_outlook)?;
        seq.serialize_field("shortages_outlook", &self.shortages_outlook)?;
        seq.serialize_field("extinction_rate", &self.extinction_rate)?;
        // if self.year >= 2030 {
        //     seq.serialize_field("extinction_rate", &10)?; // For testing win state
        // } else {
        //     seq.serialize_field("extinction_rate", &self.extinction_rate)?;
        // }
        seq.serialize_field("tgav_extinction_rate", &self.tgav_extinction_rate())?;
        seq.serialize_field("slr_extinction_rate", &self.slr_extinction_rate())?;
        seq.serialize_field("temperature", &self.temperature)?;
        // if self.year >= 2030 {
        //     seq.serialize_field("temperature", &1)?; // For testing win state
        // } else {
        //     seq.serialize_field("temperature", &self.temperature)?;
        // }
        seq.serialize_field("precipitation", &self.precipitation)?;
        seq.serialize_field("sea_level_rise", &self.sea_level_rise)?;
        seq.serialize_field("sea_level_rise_rate", &self.sea_level_rise_rate())?;
        seq.serialize_field("water_stress", &self.water_stress)?;
        seq.serialize_field("emissions", &total_emissions)?;
        // if self.year >= 2030 {
        //     seq.serialize_field("emissions", &0)?; // For testing win state
        // } else {
        //     seq.serialize_field("emissions", &total_emissions)?;
        // }
        seq.serialize_field("co2_emissions", &self.co2_emissions)?;
        seq.serialize_field("ch4_emissions", &self.ch4_emissions)?;
        seq.serialize_field("n2o_emissions", &self.n2o_emissions)?;
        seq.serialize_field("regions", &self.regions)?;
        seq.serialize_field("population", &self.population())?;
        seq.end()
    }
}

impl Saveable for World {
    fn save(&self) -> Value {
        json!({
            "year": self.year,
            "base_outlook": self.base_outlook,
            "temp_outlook": self.temp_outlook,
            "shortages_outlook": self.shortages_outlook,
            "extinction_rate": self.extinction_rate,
            "temperature": self.temperature,
            "precipitation": self.precipitation,
            "sea_level_rise": self.sea_level_rise,
            "water_stress": self.water_stress,
            "co2_emissions": self.co2_emissions,
            "ch4_emissions": self.ch4_emissions,
            "n2o_emissions": self.n2o_emissions,
            "temperature_modifier": self.temperature_modifier,
            "regions": self.regions.iter().map(|o| o.save()).collect::<Vec<Value>>(),
            "byproduct_mods": self.byproduct_mods,
            "population_growth_modifier": self.population_growth_modifier,
            "sea_level_rise_modifier": self.sea_level_rise_modifier,
        })
    }

    fn load(&mut self, state: Value) {
        self.year = coerce(&state["year"]);
        self.base_outlook = coerce(&state["base_outlook"]);
        self.temp_outlook = coerce(&state["temp_outlook"]);
        self.shortages_outlook = coerce(&state["shortages_outlook"]);
        self.extinction_rate = coerce(&state["extinction_rate"]);
        self.temperature = coerce(&state["temperature"]);
        self.precipitation = coerce(&state["precipitation"]);
        self.sea_level_rise = coerce(&state["sea_level_rise"]);
        self.water_stress = coerce(&state["water_stress"]);
        self.co2_emissions = coerce(&state["co2_emissions"]);
        self.ch4_emissions = coerce(&state["ch4_emissions"]);
        self.n2o_emissions = coerce(&state["n2o_emissions"]);
        self.temperature_modifier = coerce(&state["temperature_modifier"]);
        self.byproduct_mods = coerce(&state["byproduct_mods"]);
        self.population_growth_modifier = coerce(&state["population_growth_modifier"]);
        self.sea_level_rise_modifier = coerce(&state["sea_level_rise_modifier"]);

        let regions: Vec<Value> = coerce(&state["regions"]);
        for (o, o_s) in self.regions.iter_mut().zip(regions) {
            o.load(o_s);
        }
    }
}
