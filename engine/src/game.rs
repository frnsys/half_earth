use rand::rngs::StdRng;
use crate::player::Player;
use crate::events::EventPool;
use crate::projects::{Project, Status};
use crate::earth::{Earth, Region};
use crate::production::{Sector, CellGrid, ProductionOrder, produce, calculate_required};
use crate::kinds::{Output, OutputMap, ResourceMap, ByproductMap};

#[derive(Default)]
pub struct State<'a> {
    earth: Earth,
    player: Player,
    resources: CellGrid,
    stocks: ResourceMap<f32>,
    byproducts: ByproductMap<f32>,
    regions: Vec<Region<'a>>,
    projects: Vec<Project>,
    sectors: Vec<Sector>,
}


impl State<'_> {
    // Event pool is kept outside of the state to avoid borrowing conflicts,
    // since `EventPool::roll` takes `self` as an argument
    pub fn step(&mut self, event_pool: &mut EventPool, rng: &mut StdRng) {
        self.resources.update_cells();
        self.resources.refresh_resources();
        self.stocks += self.resources.extract_resources();

        // Aggregate demand across regions
        let demand = self.regions.iter().fold(outputs!(), |mut acc, region| {
            acc += region.demand * (region.population as f32);
            acc
        });

        // Generate production orders based on current process mixes and demand
        let mut orders: Vec<ProductionOrder> = self.sectors.iter()
            .map(|s| s.production_orders(&demand)).flatten().collect();

        // Merge in production orders for projects
        for project in &self.projects {
            match project.production_order() {
                Some(order) => orders.push(order),
                _ => ()
            }
        }

        // Run production function
        let (produced_by_type, consumed, byproducts) = produce(&orders, &self.stocks);
        self.byproducts += byproducts;
        self.stocks -= consumed;

        // Calculate production shorfalls
        let mut amount_by_type = outputs!();
        for (k, amounts) in produced_by_type.items() {
            amount_by_type[k] = amounts.iter().sum();
        }
        let shortfalls = demand - amount_by_type;

        // Get resource deficit/surplus
        let required = calculate_required(&orders);

        // Weigh resources by scarcity
        let resource_weights = required / self.stocks;

        // Update mix according to resource scarcity
        let orders_by_sector: Vec<Vec<ProductionOrder>> = self.sectors.iter().map(|s| s.production_orders(&demand)).collect();
        for (sector, orders) in self.sectors.iter_mut().zip(orders_by_sector) {
            sector.update_mix(&orders, &demand, &resource_weights);
        }

        // Identify resource shortages/surpluses
        let planned = self.resources.planned_resources();
        let mut gaps = required - planned;

        // Adjust resource extraction
        self.resources.adjust_resources(&mut gaps);

        // New effects to apply are gathered here.
        // (Mostly to avoid borrowing conflicts)
        let mut effects = Vec::new();

        // Apply points to projects
        let in_the_works = self.projects.iter_mut().filter(|p| p.production_order().is_some());
        for (project, points) in in_the_works.zip(&produced_by_type[Output::Project]) {
            let maintenance = project.status == Status::Active || project.status == Status::Finished;
            project.apply_points(*points);
            match project.status {
                Status::Active|Status::Finished => {
                    if !maintenance {
                        effects.extend_from_slice(&project.effects);
                    }
                },
                Status::Stalled => {
                    // TODO withdraw effect
                },
                _ => ()
            }
        }

        // Roll for events and collect effects
        let events = event_pool.roll(self, rng);
        for event in events {
            effects.extend_from_slice(&event.effects);
        }

        for effect in &effects {
            effect.apply(self);
        }
    }
}
