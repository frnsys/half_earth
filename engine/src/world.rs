use crate::kinds::{ByproductMap, FeedstockMap, OutputMap, ResourceMap};
use crate::regions::{Income, Region};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub year: usize,
    pub base_outlook: f32,
    pub temp_outlook: f32,
    pub shortages_outlook: f32,
    pub extinction_rate: f32,
    pub temperature: f32,    // global temp anomaly, C
    pub precipitation: f32,  // global precip avg
    pub sea_level_rise: f32, // meters
    pub water_stress: f32,   // 0-100%
    pub co2_emissions: f32,
    pub ch4_emissions: f32,
    pub n2o_emissions: f32,
    pub temperature_modifier: f32,
    pub byproduct_mods: ByproductMap,
    pub population_growth_modifier: f32,
    pub sea_level_rise_modifier: f32, // meters
    //
    #[wasm_bindgen(skip)]
    pub regions: Vec<Region>,
}

#[wasm_bindgen]
impl WorldState {
    #[wasm_bindgen]
    pub fn population(&self) -> f32 {
        self.regions.iter().map(|r| r.population).sum()
    }

    #[wasm_bindgen]
    pub fn outlook(&self) -> f32 {
        let region_outlook =
            self.regions.iter().map(|r| r.outlook).sum::<f32>() / self.regions.len() as f32;
        self.base_outlook - self.shortages_outlook + region_outlook
    }

    #[wasm_bindgen]
    pub fn total_emissions_gt(&self) -> f32 {
        self.emissions() * 1e-15
    }

    #[wasm_bindgen]
    pub fn sea_level_rise_rate(&self) -> f32 {
        // Meters
        // Chosen to roughly hit 1.4m-1.6m rise by 2100 in the BAU scenario
        (0.0025 * self.temperature.powf(1.5)) + self.sea_level_rise_modifier
    }

    /// Contribution to extinction rate from the tgav
    #[wasm_bindgen]
    pub fn tgav_extinction_rate(&self) -> f32 {
        self.temperature.powf(2.)
    }

    /// Contribution to extinction rate from sea level rise
    #[wasm_bindgen]
    pub fn slr_extinction_rate(&self) -> f32 {
        self.sea_level_rise.powf(2.)
    }
}

impl WorldState {
    pub fn lic_population(&self, materials_by_income: &[f32; 4]) -> f32 {
        self.regions
            .iter()
            .map(|r| r.lic_population(materials_by_income))
            .sum()
    }

    pub fn update_pop(&mut self, income_pop_coefs: &[[f32; 4]; 4]) {
        for region in &mut self.regions {
            region.update_pop(
                self.year as f32,
                1. + self.population_growth_modifier,
                income_pop_coefs,
            );
        }
    }

    /// Returns a vec of ids of regions that "leveled up"
    /// and a vec of ids of regions that "leveled down"
    pub fn develop_regions(
        &mut self,
        stop: bool,
        fast: bool,
        degrow: bool,
    ) -> (Vec<usize>, Vec<usize>) {
        let mut up = vec![];
        let mut down = vec![];

        let speed = if fast { 1.25 } else { 1. };
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
        self.regions.iter().map(|r| r.habitability()).sum::<f32>() / self.regions.len() as f32
    }

    pub fn emissions(&self) -> f32 {
        self.co2_emissions + (self.n2o_emissions * 298.) + (self.ch4_emissions * 36.)
    }

    pub fn demand(&self, output_demand: &[OutputMap; 4]) -> OutputMap {
        self.regions.iter().fold(outputs!(), |mut acc, region| {
            acc += region.demand(output_demand);
            acc
        })
    }

    pub fn change_population(&mut self, amount: f32) {
        let amount_per_region = amount / self.regions.len() as f32;
        for region in &mut self.regions {
            region.population += amount_per_region;
        }
    }

    pub fn update_sea_level_rise(&mut self) {
        self.sea_level_rise += self.sea_level_rise_rate();
    }

    pub fn base_extinction_rate(&self) -> f32 {
        self.tgav_extinction_rate() + self.slr_extinction_rate() - self.byproduct_mods.biodiversity
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

#[wasm_bindgen]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct World {
    pub feedstock_reserves: FeedstockMap,
    pub starting_resources: ResourceMap,

    #[wasm_bindgen(skip)]
    pub output_demand: [OutputMap; 4],

    #[wasm_bindgen(skip)]
    pub water_by_income: [f32; 4],

    #[wasm_bindgen(skip)]
    pub materials_by_income: [f32; 4],

    #[wasm_bindgen(skip)]
    pub income_pop_coefs: [[f32; 4]; 4],
}

impl World {
    pub fn data() -> Self {
        World {
            feedstock_reserves: FeedstockMap {
                oil: 824182962000000.0,
                coal: 1274000000000000000.0,
                uranium: 7988600000000.0,
                lithium: 80000000.0,
                natural_gas: 719100000000000000.0,
                soil: 100000000000000000000.0,
                other: 0.0,
                thorium: 5805982300000.0,
            },
            starting_resources: ResourceMap {
                fuel: 141397300000000.0,
                electricity: 26936000000000.0,
                land: 104000000000000.0,
                water: 45500000000000000.0,
            },
            output_demand: [
                OutputMap {
                    fuel: 84.823,
                    electricity: 12.647,
                    animal_calories: 62258.01225379132,
                    plant_calories: 781214.6569453699,
                },
                OutputMap {
                    fuel: 478.748,
                    electricity: 80.881867,
                    animal_calories: 101446.22307270509,
                    plant_calories: 835119.0398573277,
                },
                OutputMap {
                    fuel: 1842.78168,
                    electricity: 368.96432,
                    animal_calories: 235324.57700012674,
                    plant_calories: 883168.4828934266,
                },
                OutputMap {
                    fuel: 4111.4561,
                    electricity: 750.5439,
                    animal_calories: 320351.0835388199,
                    plant_calories: 900747.3812494389,
                },
            ],
            water_by_income: [2040.4095, 4552.624175, 5839.79276, 11648.18348],
            materials_by_income: [2.253141687346895, 4.3768, 15.430, 25.9541],
            income_pop_coefs: [
                [
                    -137.09104615549052,
                    0.20175900568502633,
                    -9.881496778856683e-05,
                    1.6107860430297862e-08,
                ],
                [
                    -31.645089275035065,
                    0.049053282850800455,
                    -2.5144480355319464e-05,
                    4.267315072570761e-09,
                ],
                [
                    -73.97073712590549,
                    0.110304344757395,
                    -5.471537682118361e-05,
                    9.02938905851344e-09,
                ],
                [
                    193.7774375120351,
                    -0.27776781135845113,
                    0.00013271413986610546,
                    -2.113553227448261e-08,
                ],
            ],
        }
    }
}
