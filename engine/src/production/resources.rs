use crate::kinds::{Resource, ResourceMap};

// TODO should this not be a cell grid and more amorphous? Just based on where resources
// are in the world world?
// TODO shoudl this also be where biome emissions/etc are held?

const STATUS_CHANGE_STEPS: u8 = 3;
const BASE_YIELD_RATE: f32 = 0.001;
const MAX_EXPLOITATION: u8 = 10;

pub type CellIdx = usize;

#[derive(Debug, PartialEq)]
pub enum Status {
    Available,
    Occupied,
    Developing(u8),
    Active(u8),
    Decommissioning(u8)
}

#[derive(Debug)]
pub struct Cell {
    pub status: Status,

    // Currently exploited resource
    pub resource: Option<Resource>,

    // Amount of resources theorically available in this cell
    pub limits: ResourceMap<f32>,

    // Amount of resources actually available in this cell,
    // i.e. reflects how much has been extracted
    pub resources: ResourceMap<f32>,
}

#[derive(Default)]
pub struct CellGrid {
    cells: Vec<Cell>,
    index: ResourceMap<Vec<CellIdx>>,
    active: Vec<CellIdx>,

    // How fast each resource replenishes
    //  rate == 0.0 means non-renewable
    //  rate == 0.5 means 50% of the total is replenished per step
    //  rate == 1.0 means 100% of the total is replenished per step
    refresh_rates: ResourceMap<f32>
}

fn yielded_resources(exploitation_level: u8) -> f32 {
    (exploitation_level as f32) * BASE_YIELD_RATE
}

impl CellGrid {
    fn new(cells: Vec<Cell>, refresh_rates: ResourceMap<f32>) -> CellGrid {
        // Build resource index
        let mut active = Vec::new();
        let mut index: ResourceMap<Vec<CellIdx>> = resources!();
        for (idx, cell) in cells.iter().enumerate() {
            for (k, v) in cell.resources.items() {
                if *v > 0. {
                    index[k].push(idx);
                }
            }
            match cell.status {
                Status::Active(_)|Status::Developing(_)|Status::Decommissioning(_)  => {
                    active.push(idx);
                },
                _ => ()
            }
        }

        CellGrid {
            cells, index,
            active, refresh_rates
        }
    }

    /// Tallies up how many resources are available for immediate use.
    pub fn resources(&self) -> ResourceMap<f32> {
        let mut resources = resources!();
        for idx in &self.active {
            let cell = &self.cells[*idx];
            if let Some(r) = cell.resource {
                match cell.status {
                    Status::Active(i) => {
                        let amount = cell.resources[r] * yielded_resources(i);
                        resources[r] += amount;
                    }
                    _ => ()
                };
            }
        }
        resources
    }

    /// Tallies up how many resources will be available for use after
    /// resource is developed.
    pub fn planned_resources(&self) -> ResourceMap<f32> {
        let mut resources = resources!();
        for idx in &self.active {
            let cell = &self.cells[*idx];
            if let Some(r) = cell.resource {
                match cell.status {
                    Status::Active(i) => {
                        let amount = cell.resources[r] * yielded_resources(i);
                        resources[r] += amount;
                    },
                    Status::Developing(_) => {
                        let amount = cell.resources[r] * yielded_resources(1);
                        resources[r] += amount;
                    },
                    _ => ()
                };
            }
        }
        resources
    }

    /// Extracts (deducts) resources available for immediate use.
    pub fn extract_resources(&mut self) -> ResourceMap<f32> {
        let mut resources = resources!();
        for idx in &self.active {
            let cell = &mut self.cells[*idx];
            if let Some(r) = cell.resource {
                match cell.status {
                    Status::Active(i) => {
                        let amount = f32::min(cell.limits[r] * yielded_resources(i), cell.resources[r]);
                        resources[r] += amount;
                        cell.resources[r] -= amount;
                    }
                    _ => ()
                };
            }
        }
        resources
    }

    /// Replenish (renewable) resources in cells.
    pub fn refresh_resources(&mut self) {
        for cell in &mut self.cells {
            for (k, limit) in cell.limits.items() {
                cell.resources[k] = f32::min(
                    cell.resources[k] + (self.refresh_rates[k] * limit), *limit)
            }
        }
    }

    /// Adjusts resources available for production by expanding (intensifying extraction
    /// at already-developed cells or starting development in new cells) or by contracting
    /// (reducing extraction, halting development, or decommissioning).
    pub fn adjust_resources(&mut self, changes: &mut ResourceMap<f32>) {
        let mut existing: ResourceMap<Vec<(CellIdx, f32)>> = resources!();
        for idx in &self.active {
            let cell = &self.cells[*idx];
            if let Some(r) = cell.resource {
                let should_expand = changes[r] > 0.;
                let should_contract = changes[r] < 0.;
                let base_yield = cell.resources[r] * yielded_resources(1);
                match cell.status {
                    // Adjust exploitation of existing resources.
                    // Prefer this over developing/decommissioning
                    Status::Active(i) => {
                        // Expand underexploited
                        if should_expand {
                            if i < MAX_EXPLOITATION {
                                let cur_yield = cell.resources[r] * yielded_resources(i);
                                let max_yield = cell.resources[r] * yielded_resources(MAX_EXPLOITATION);
                                let expansion_potential = max_yield - cur_yield;
                                existing[r].push((*idx, expansion_potential));
                            }

                        // Contract overexploited
                        } else if should_contract {
                            existing[r].push((*idx, base_yield));
                        }
                    },

                    // Halt development of new resources
                    Status::Developing(i) => {
                        if should_contract {
                            existing[r].push((*idx, base_yield/(i as f32)));
                        }
                    },

                    // Halt decommissioning of existing resources.
                    Status::Decommissioning(i) => {
                        if should_expand {
                            existing[r].push((*idx, base_yield/(i as f32)));
                        }
                    },

                    _ => ()
                }
            }
        }

        for (r, change) in changes.items_mut() {
            // Expand
            if *change > 0. {
                // First expand exploitation in existing places
                if !existing[r].is_empty() {
                    // Sort by highest expansion potential
                    existing[r].sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    for (idx, _) in &existing[r] {
                        let cell = &mut self.cells[*idx];
                        let yield_change = cell.resources[r] * yielded_resources(1);
                        match cell.status {
                            // Increase exploitation rate by 1 tick
                            Status::Active(i) => {
                                cell.status = Status::Active(i+1);
                                *change -= yield_change;
                            },

                            // Reverse decommissioning
                            Status::Decommissioning(i) => {
                                cell.status = Status::Developing(i);
                                *change -= yield_change;
                            },
                            _ => ()
                        }
                        if *change <= 0. {
                            break;
                        }
                    }
                }

                // If still more change is needed, expand to new areas
                if *change > 0. {
                    // Get undeveloped cells for this resource
                    let mut cands: Vec<(CellIdx, f32)> = self.index[r].iter()
                        .filter_map(|idx| {
                            let cell = &self.cells[*idx];
                            // TODO handle Status::Occupied cells
                            if cell.status == Status::Available {
                                Some((*idx, cell.resources[r]))
                            } else {
                                None
                            }
                        }).collect();

                    cands.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    for (idx, _) in cands {
                        let cell = &mut self.cells[idx];

                        // Start developing the resource
                        cell.status = Status::Developing(0);
                        cell.resource = Some(r);

                        self.active.push(idx);

                        let base_yield = cell.resources[r] * yielded_resources(1);
                        *change -= base_yield;

                        if *change <= 0. {
                            break;
                        }
                    }
                }

            // Contract
            } else if *change < 0. {
                // Sort by highest contraction potential
                existing[r].sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                for (idx, _) in &existing[r] {
                    let cell = &mut self.cells[*idx];
                    let yield_change = cell.resources[r] * yielded_resources(1);
                    match cell.status {
                        // Decrease exploitation rate by 1 tick,
                        // or start decommissioning
                        Status::Active(i) => {
                            if i > 1 {
                                cell.status = Status::Active(i-1);
                            } else {
                                cell.status = Status::Decommissioning(STATUS_CHANGE_STEPS);
                            }
                            *change += yield_change;
                        },

                        // Reverse development
                        Status::Developing(i) => {
                            cell.status = Status::Decommissioning(i);
                            *change += yield_change;
                        },

                        _ => ()
                    }
                    if *change >= 0. {
                        break;
                    }
                }
            }
        }
    }

    /// Update cell development/decommissioning progress.
    pub fn update_cells(&mut self) {
        for idx in &self.active {
            let cell = &mut self.cells[*idx];
            match cell.status {
                Status::Developing(step) => {
                    if step >= STATUS_CHANGE_STEPS {
                        cell.status = Status::Active(1);
                    } else {
                        cell.status = Status::Developing(step+1);
                    }
                },
                Status::Decommissioning(step) => {
                    if step <= 1 {
                        cell.status = Status::Available;
                        cell.resource = None;
                    } else {
                        cell.status = Status::Decommissioning(step-1);
                    }
                },
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;

    fn gen_cell_grid() -> CellGrid {
        CellGrid::new(vec![Cell {
            status: Status::Available,
            resource: None,
            limits: resources!(sun: 0.2, wind: 0.2, water: 0.5),
            resources: resources!(sun: 0.2, wind: 0.2, water: 0.5)
        }, Cell {
            status: Status::Active(1),
            resource: Some(Resource::Sun),
            limits: resources!(sun: 0.3, wind: 0.3, water: 0.2),
            resources: resources!(sun: 0.3, wind: 0.3, water: 0.2)
        }, Cell {
            status: Status::Developing(1),
            resource: Some(Resource::Water),
            limits: resources!(sun: 0.3, wind: 0.3, water: 0.6),
            resources: resources!(sun: 0.3, wind: 0.3, water: 0.6)
        }, Cell {
            status: Status::Decommissioning(2),
            resource: Some(Resource::Wind),
            limits: resources!(sun: 0.3, wind: 0.3, water: 0.8),
            resources: resources!(sun: 0.3, wind: 0.3, water: 0.8)
        }, Cell {
            status: Status::Available,
            resource: None,
            limits: resources!(sun: 0.3, wind: 0.5, water: 0.2),
            resources: resources!(sun: 0.3, wind: 0.5, water: 0.2)
        }], resources!(
            sun: 1.0,
            wind: 1.0,
            water: 0.01))
    }

    #[test]
    fn test_resources() {
        let grid = gen_cell_grid();

        // Only active extracted resources should
        // be non-zero
        let resources = grid.resources();
        assert_eq!(resources, resources!(
            sun: 0.3 * yielded_resources(1)
        ));
    }

    #[test]
    fn test_planned_resources() {
        let grid = gen_cell_grid();

        // Only active and planned extracted resources
        // should be non-zero
        let resources = grid.planned_resources();
        assert_eq!(resources, resources!(
            sun: 0.3 * yielded_resources(1),
            water: 0.6 * yielded_resources(1)
        ));
    }

    #[test]
    fn test_extract_resources() {
        let mut grid = gen_cell_grid();

        // Only active extracted resources should be applied
        let resources = grid.extract_resources();
        assert_eq!(resources, resources!(
            sun: 0.3 * yielded_resources(1)
        ));
        assert_approx_eq!(f32, grid.cells[1].resources.sun, 0.3 - (0.3 * yielded_resources(1)));

        // Additional extraction should happen as a percent of the cell's resource limit,
        // *not* the current remaining amount (otherwise we get 10% of 10% of 10% etc)
        let _resources = grid.extract_resources();
        assert_approx_eq!(f32, grid.cells[1].resources.sun, 0.3 - (0.3 * yielded_resources(2)));

        // Amount extracted should never be more than what's left,
        // and remaining resource amount should never be negative
        grid.cells[1].resources.sun = 0.3;
        grid.cells[1].status = Status::Active(255);
        for _ in 0..20 {
            let remaining = grid.cells[1].resources.sun;
            let resources = grid.extract_resources();
            assert!(resources.sun <= remaining);
        }
        assert_eq!(grid.cells[1].resources.sun, 0.);
    }

    #[test]
    fn test_refresh_resources() {
        let mut grid = gen_cell_grid();

        grid.cells[0].status = Status::Active(10);
        grid.cells[0].resource = Some(Resource::Uranium);
        grid.cells[0].resources[Resource::Uranium] = 1.;

        grid.cells[2].status = Status::Active(1);
        grid.cells[2].resource = Some(Resource::Water);

        let _resources = grid.extract_resources();
        assert_eq!(grid.cells[1].resources.sun, 0.3 - (0.3 * yielded_resources(1)));

        let remaining_water = grid.cells[2].resources.water;
        let _resources = grid.refresh_resources();

        // Sun should be fully refreshed
        assert_eq!(grid.cells[1].resources.sun, 0.3);

        // Water should be somewhat refreshed
        assert!(grid.cells[2].resources.water > remaining_water);

        // Uranium should not be refreshed
        assert!(grid.cells[0].resources.uranium < 1.);
    }

    #[test]
    fn test_adjust_resources() {
        let mut grid = CellGrid::new(vec![Cell {
            status: Status::Active(1),
            resource: Some(Resource::Water),
            limits: resources!(sun: 0.2, wind: 0.2, water: 0.5),
            resources: resources!(sun: 0.2, wind: 0.2, water: 0.5)
        }, Cell {
            status: Status::Active(1),
            resource: Some(Resource::Sun),
            limits: resources!(sun: 0.3, wind: 0.3, water: 0.2),
            resources: resources!(sun: 0.3, wind: 0.3, water: 0.2)
        }, Cell {
            status: Status::Developing(1),
            resource: Some(Resource::Sun),
            limits: resources!(sun: 0.3, wind: 0.3, water: 0.6),
            resources: resources!(sun: 0.3, wind: 0.3, water: 0.6)
        }, Cell {
            status: Status::Decommissioning(2),
            resource: Some(Resource::Wind),
            limits: resources!(sun: 0.3, wind: 0.3, water: 0.8),
            resources: resources!(sun: 0.3, wind: 0.3, water: 0.8)
        }, Cell {
            status: Status::Available,
            resource: None,
            limits: resources!(sun: 0.3, wind: 0.5, water: 0.2),
            resources: resources!(sun: 0.3, wind: 0.5, water: 0.2)
        }], resources!(
            sun: 1.0,
            wind: 1.0,
            water: 0.01));

        assert_eq!(grid.active, vec![0, 1, 2, 3]);

        let mut changes = resources!(
            water: 1.0,
            wind: 1.0,
            sun: -1.0
        );
        grid.adjust_resources(&mut changes);

        // Expand capacity for wind
        assert_eq!(grid.cells[3].status, Status::Developing(2)); // Reverse decommissioning
        assert_eq!(grid.cells[4].status, Status::Developing(0)); // Develop new resource
        assert!(changes.wind < 1.0);

        // Expand capacity for water
        assert_eq!(grid.cells[0].status, Status::Active(2)); // Increase exploitation
        assert!(changes.water < 1.0);

        // Contract capacity for sun
        assert_eq!(grid.cells[1].status, Status::Decommissioning(3)); // Decommission active
        assert_eq!(grid.cells[2].status, Status::Decommissioning(1)); // Reverse development
        assert!(changes.sun > -1.0);

        // Should now include the last cell since it's being developed
        assert_eq!(grid.active, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_update_cells() {
        let mut grid = gen_cell_grid();
        grid.update_cells();

        // Unchanged
        assert_eq!(grid.cells[0].status, Status::Available);
        assert_eq!(grid.cells[1].status, Status::Active(1));
        assert_eq!(grid.cells[4].status, Status::Available);

        // Changed
        assert_eq!(grid.cells[2].status, Status::Developing(2));
        assert_eq!(grid.cells[3].status, Status::Decommissioning(1));

        grid.update_cells();

        // Unchanged
        assert_eq!(grid.cells[0].status, Status::Available);
        assert_eq!(grid.cells[1].status, Status::Active(1));
        assert_eq!(grid.cells[4].status, Status::Available);

        // Changed
        assert_eq!(grid.cells[2].status, Status::Developing(3));
        assert_eq!(grid.cells[3].status, Status::Available);

        grid.update_cells();

        // Unchanged
        assert_eq!(grid.cells[0].status, Status::Available);
        assert_eq!(grid.cells[1].status, Status::Active(1));
        assert_eq!(grid.cells[3].status, Status::Available);
        assert_eq!(grid.cells[4].status, Status::Available);

        // Changed
        assert_eq!(grid.cells[2].status, Status::Active(1));
    }
}
