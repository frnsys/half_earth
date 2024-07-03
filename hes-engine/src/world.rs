use crate::events::Event;
use crate::industries::Industry;
use crate::kinds::{
    ByproductMap, FeedstockMap, Output, OutputMap, ResourceMap,
};
use crate::production::Process;
use crate::projects::Project;
use crate::regions::{Income, Region};
use serde::{Deserialize, Serialize};

/// The `World` represents a game configuration,
/// defining the world's parameters as well
/// as the projects, processes, regions, and industries.
#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct World {
    pub year: usize,
    pub base_outlook: f32,
    pub extinction_rate: f32,
    pub temperature: f32, // global temp anomaly, C
    pub sea_level_rise: f32, // meters
    pub regions: Vec<Region>,
    pub events: Vec<Event>,
    pub industries: Vec<Industry>,
    pub projects: Vec<Project>,
    pub processes: Vec<Process>,
    pub feedstock_reserves: FeedstockMap,
    pub starting_resources: ResourceMap,
    pub output_demand: [OutputMap; 4],
    pub water_by_income: [f32; 4],
    pub materials_by_income: [f32; 4],
    pub income_pop_coefs: [[f32; 4]; 4],
}

impl World {
    pub fn population(&self) -> f32 {
        self.regions.iter().map(|r| r.population).sum()
    }

    /// Contribution to extinction rate from the tgav
    pub fn tgav_extinction_rate(&self) -> f32 {
        self.temperature.powf(2.)
    }

    /// Contribution to extinction rate from sea level rise
    pub fn slr_extinction_rate(&self) -> f32 {
        self.sea_level_rise.powf(2.)
    }

    pub fn lic_population(&self) -> f32 {
        self.regions
            .iter()
            .map(|r| {
                r.lic_population(&self.materials_by_income)
            })
            .sum()
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
                if !(degrow
                    && region.income == Income::UpperMiddle)
                {
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

    pub fn update_outlook(
        &mut self,
        wretched_ally: bool,
        consumerist_ally: bool,
    ) {
        for region in &mut self.regions {
            region.update_outlook(
                wretched_ally,
                consumerist_ally,
            );
        }
    }

    pub fn habitability(&self) -> f32 {
        self.regions
            .iter()
            .map(|r| r.habitability())
            .sum::<f32>()
            / self.regions.len() as f32
    }

    pub fn demand(&self) -> OutputMap {
        self.regions.iter().fold(
            outputs!(),
            |mut acc, region| {
                acc += region.demand(&self.output_demand);
                acc
            },
        )
    }

    pub fn demand_by_income_levels(
        &self,
        output: Output,
    ) -> [f32; 4] {
        self.output_demand
            .iter()
            .map(|demand| demand[output])
            .collect::<Vec<_>>()
            .try_into()
            .expect("Mapping from same size arrays")
    }

    pub fn change_population(&mut self, amount: f32) {
        let amount_per_region =
            amount / self.regions.len() as f32;
        for region in &mut self.regions {
            region.population += amount_per_region;
        }
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
