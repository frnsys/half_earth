use std::{collections::BTreeMap, sync::LazyLock};

use crate::{
    events::Event,
    industries::Industry,
    kinds::{FeedstockMap, Output, OutputMap, ResourceMap},
    outputs,
    production::Process,
    projects::Project,
    regions::{Income, Region},
    round_to,
    Collection,
    Id,
    OutputDemand,
};
use serde::{Deserialize, Serialize};

pub static CLIMATES: LazyLock<BTreeMap<String, Vec<[f32; 4]>>> =
    LazyLock::new(|| {
        let data = include_str!("../assets/climates.json");
        serde_json::from_str(data).unwrap()
    });

/// The `World` represents a game configuration,
/// defining the world's parameters as well
/// as the projects, processes, regions, and industries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct World {
    pub year: usize,

    pub base_outlook: f32,
    pub temp_outlook: f32,

    pub extinction_rate: f32,
    pub temperature: f32, // global temp anomaly, C
    pub sea_level_rise: f32, // meters
    pub temperature_modifier: f32,
    pub population_growth_modifier: f32,
    pub sea_level_rise_modifier: f32, // meters
    pub precipitation: f32,           // global precip avg

    pub regions: Collection<Region>,
    pub per_capita_demand: [OutputDemand; 4],
    pub water_by_income: [f32; 4],
    pub materials_by_income: [f32; 4],
    pub income_pop_coefs: [[f32; 4]; 4],

    pub industries: Collection<Industry>,
    pub projects: Collection<Project>,
    pub processes: Collection<Process>,
    pub project_lockers: BTreeMap<Id, Id>,
    pub events: Collection<Event>,

    pub feedstock_reserves: FeedstockMap,
    pub starting_resources: ResourceMap,
}

impl Default for World {
    fn default() -> Self {
        serde_json::from_str(include_str!(
            "../assets/DEFAULT.world"
        ))
        .unwrap()
    }
}

impl World {
    pub fn update_climate(&mut self, tgav: f32) -> f32 {
        let prev_temp = self.temperature;
        self.temperature = tgav + self.temperature_modifier;
        let temp_change = prev_temp - self.temperature;
        self.regions.update_climates(tgav);
        self.sea_level_rise += self.sea_level_rise_rate();
        temp_change
    }

    pub fn update_outlook(
        &mut self,
        temp_change: f32,
        wretched_ally: bool,
        consumerist_ally: bool,
    ) {
        let temp_outlook =
            temp_change * 6. * self.temperature.powf(2.);
        let region_outlook_change = temp_outlook * 0.4;
        for region in self.regions.iter_mut() {
            region.outlook += region_outlook_change;
        }
        self.base_outlook += temp_outlook;
        self.temp_outlook += temp_outlook;
        self.regions
            .update_outlook(wretched_ally, consumerist_ally);
    }

    pub fn outlook(&self) -> f32 {
        self.base_outlook + self.regions.outlook()
    }

    pub fn update_populations(&mut self) {
        for region in self.regions.iter_mut() {
            region.update_pop(
                self.year as f32,
                1. + self.population_growth_modifier,
                &self.income_pop_coefs,
            );
        }
    }

    pub fn update_extinction_rate(
        &mut self,
        produced_by_process: &BTreeMap<Id, f32>,
    ) {
        let base = self.tgav_extinction_rate()
            + self.slr_extinction_rate();
        let lic_pop = self.lic_population();
        let from_processes =
            self.processes.iter().fold(0., |acc, p| {
                let amount = produced_by_process
                    .get(&p.id)
                    .unwrap_or(&0.);
                let contrib = p.extinction_rate(
                    self.starting_resources.land,
                ) * amount;
                acc + contrib
            });
        let from_industries =
            self.industries.iter().fold(0., |acc, ind| {
                acc + ind.extinction_rate(
                    self.starting_resources.land,
                ) * ind.demand(lic_pop)
            });
        let rate = base + from_industries + from_processes;
        self.extinction_rate = rate;
    }

    /// Contribution to extinction rate from the tgav
    pub fn tgav_extinction_rate(&self) -> f32 {
        self.temperature.powf(2.)
    }

    /// Contribution to extinction rate from sea level rise
    pub fn slr_extinction_rate(&self) -> f32 {
        self.sea_level_rise.powf(2.)
    }

    pub fn sea_level_rise_rate(&self) -> f32 {
        // Meters
        // Chosen to roughly hit 1.4m-1.6m rise by 2100 in the BAU scenario
        (0.0025 * self.temperature.powf(1.5))
            + self.sea_level_rise_modifier
    }

    pub fn lic_population(&self) -> f32 {
        self.regions
            .iter()
            .map(|r| {
                r.lic_population(&self.materials_by_income)
            })
            .sum()
    }

    pub fn region_demand(&self) -> OutputMap {
        self.regions.iter().fold(
            outputs!(),
            |mut acc, region| {
                acc += region.demand(&self.per_capita_demand);
                acc
            },
        )
    }

    pub fn demand_by_income_levels(
        &self,
        output: Output,
    ) -> [f32; 4] {
        self.per_capita_demand
            .iter()
            .map(|demand| demand.of(output))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Mapping from same size arrays")
    }
}

impl Collection<Region> {
    /// Returns a vec of ids of regions that "leveled up"
    /// and a vec of ids of regions that "leveled down"
    pub fn develop(
        &mut self,
        stop: bool,
        fast: bool,
        degrow: bool,
    ) -> (Vec<Id>, Vec<Id>) {
        let mut up = vec![];
        let mut down = vec![];

        let speed = if fast { 1.25 } else { 1. };
        for region in self.iter_mut() {
            let (start, end) =
                region.develop(speed, stop, degrow);
            if end < start {
                down.push(region.id);
            } else if end > start {
                up.push(region.id);
            }
        }
        (up, down)
    }

    pub fn population(&self) -> f32 {
        self.iter().map(|r| r.population).sum()
    }

    fn update_outlook(
        &mut self,
        wretched_ally: bool,
        consumerist_ally: bool,
    ) {
        for region in self.iter_mut() {
            region.update_outlook(
                wretched_ally,
                consumerist_ally,
            );
        }
    }

    fn update_climates(&mut self, temp: f32) {
        // Max range is -2 to 14.9.
        let temp = temp.max(-2.).min(14.9);
        let tgav = round_to(temp, 1);
        let key = format!("{tgav:.1}");
        if let Some(climates) = CLIMATES.get(&key) {
            for (region, vals) in self.iter_mut().zip(climates)
            {
                region.temp_lo = vals[0];
                region.temp_hi = vals[1];
                region.precip_lo = vals[2];
                region.precip_hi = vals[3];
            }
        }
    }

    /// Mean outlook of all regions.
    pub fn outlook(&self) -> f32 {
        self.iter().map(|r| r.outlook).sum::<f32>()
            / self.len() as f32
    }

    /// Mean habitability of all regions.
    pub fn habitability(&self) -> f32 {
        self.iter().map(|r| r.habitability()).sum::<f32>()
            / self.len() as f32
    }

    /// Mean income level of all regions.
    pub fn income_level(&self) -> f32 {
        self.iter().map(|r| match r.income {
            Income::Low => 0.,
            Income::LowerMiddle => 1.,
            Income::UpperMiddle => 2.,
            Income::High => 3.,
        } + r.development).sum::<f32>()/self.len() as f32
    }

    /// Equally distribute an amount of population change
    /// across all regions.
    pub fn change_population(&mut self, amount: f32) {
        let amount_per_region = amount / self.len() as f32;
        for region in self.iter_mut() {
            region.population += amount_per_region;
        }
    }
}
