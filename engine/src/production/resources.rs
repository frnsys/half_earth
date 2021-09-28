use std::collections::HashSet;
use crate::kinds::{Resource, ResourceMap};

const STATUS_CHANGE_STEPS: u8 = 3;
const BASE_YIELD_RATE: f32 = 0.001;
const MAX_EXPLOITATION: u8 = 10;

pub type CellIdx = usize;

#[derive(Debug, PartialEq)]
pub enum Status {
    Available,
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
    pub limit: ResourceMap<f32>,

    // Amount of resources actually available in this cell,
    // i.e. reflects how much has been extracted
    pub resources: ResourceMap<f32>,
}

pub struct CellGrid<const N: usize> {
    cells: [Cell; N],
    index: ResourceMap<HashSet<CellIdx>>,

    // How fast each resource replenishes
    //  rate == 0.0 means non-renewable
    //  rate == 0.5 means 50% of the total is replenished per step
    //  rate == 1.0 means 100% of the total is replenished per step
    refresh_rates: ResourceMap<f32>
}

fn yielded_resources(exploitation_level: u8) -> f32 {
    (exploitation_level as f32) * BASE_YIELD_RATE
}

impl<const N: usize> CellGrid<N> {
    fn new(cells: [Cell; N], refresh_rates: ResourceMap<f32>) -> CellGrid<N> {
        // Build resource index
        let mut index: ResourceMap<HashSet<CellIdx>> = resources!();
        for (i, cell) in cells.iter().enumerate() {
            for (k, v) in cell.resources.items() {
                if *v > 0. {
                    index[k].insert(i);
                }
            }
        }

        CellGrid {
            cells, index,
            refresh_rates
        }
    }

    /// Tallies up how many resources are available for immediate use.
    pub fn resources(&self, idxs: &[CellIdx]) -> ResourceMap<f32> {
        let mut resources = resources!();
        for idx in idxs {
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
    fn planned_resources(&self, idxs: &[CellIdx]) -> ResourceMap<f32> {
        let mut resources = resources!();
        for idx in idxs {
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
    pub fn extract_resources(&mut self, idxs: &[CellIdx]) -> ResourceMap<f32> {
        let mut resources = resources!();
        for idx in idxs {
            let cell = &mut self.cells[*idx];
            if let Some(r) = cell.resource {
                match cell.status {
                    Status::Active(i) => {
                        let amount = cell.resources[r] * yielded_resources(i);
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
    pub fn refresh_resources(&mut self, idxs: &[CellIdx]) {
        for idx in idxs {
            let cell = &mut self.cells[*idx];
            for (k, limit) in cell.limit.items() {
                cell.resources[k] = f32::min(
                    cell.resources[k] + (self.refresh_rates[k] * limit), *limit)
            }
        }
    }

    /// Adjusts resources available for production by expanding (intensifying extraction
    /// at already-developed cells or starting development in new cells) or by contracting
    /// (reducing extraction, halting development, or decommissioning).
    pub fn adjust_resources(&mut self, idxs: &mut Vec<CellIdx>, changes: &mut ResourceMap<f32>) {
        // Get underexploited cells for each TODO
        let mut existing: ResourceMap<Vec<(CellIdx, f32)>> = resources!();
        for idx in idxs {
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
                            if i > 0 {
                                cell.status = Status::Active(i-1);
                            } else {
                                cell.status = Status::Decommissioning(3);
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
    fn update_cells(&mut self, idxs: &Vec<CellIdx>) {
        for idx in idxs {
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
                    if step >= STATUS_CHANGE_STEPS {
                        cell.status = Status::Available;
                        cell.resource = None;
                    } else {
                        cell.status = Status::Decommissioning(step+1);
                    }
                },
                _ => ()
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     // TODO test deduct resources
//     // TODO test yield changes
//     // TODO test expand resources increases exploitation first
//     // TODO test contract resources

//     #[test]
//     fn test_cells_for_resources() {
//         let cells = [Cell {
//             status: Status::Available,
//             resources: resources!(water: 1.0, soil: 0.5)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(soil: 2.0, labor: 3.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(soil: 2.0, labor: 1.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(sun: 2.0, labor: 1.0)
//         }];
//         let grid = CellGrid::new(cells);

//         // Should only return cell indices where
//         // soil and labor is present
//         let required = resources!(soil: 0.2, labor: 0.3);
//         let idxs = grid.find_cells_for_resources(&required);

//         let mut expected = HashSet::default();
//         expected.insert(1);
//         expected.insert(2);
//         assert_eq!(idxs, expected);
//     }

//     #[test]
//     fn test_resources_for_sector() {
//         let cells = [Cell {
//             status: Status::Active(1),
//             resources: resources!(water: 1.0, soil: 0.5)
//         }, Cell {
//             status: Status::Developing(0),
//             resources: resources!(soil: 2.0, labor: 3.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(soil: 2.0, labor: 1.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(sun: 2.0, labor: 1.0)
//         }];
//         let grid = CellGrid::new(cells);
//         let idxs = [0, 1];
//         let resources = grid.resources_for_cells(&idxs);
//         let expected = resources!(
//             water: 1.,
//             labor: 0.,
//             soil: 0.5
//         );
//         assert_eq!(resources, expected);
//     }

//     #[test]
//     fn test_planned_resources_for_sector() {
//         let cells = [Cell {
//             status: Status::Active(1),
//             resources: resources!(water: 1.0, soil: 0.5)
//         }, Cell {
//             status: Status::Developing(0),
//             resources: resources!(soil: 2.0, labor: 3.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(soil: 2.0, labor: 1.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(sun: 2.0, labor: 1.0)
//         }];
//         let grid = CellGrid::new(cells);
//         let idxs = [0, 1];
//         let resources = grid.planned_resources_for_cells(&idxs);
//         let expected = resources!(
//             water: 1.,
//             labor: 3.,
//             soil: 2.5
//         );
//         assert_eq!(resources, expected);
//     }

//     #[test]
//     fn test_update_cells() {
//         let cells = [Cell {
//             status: Status::Developing(0),
//             resources: resources!(water: 1.0, soil: 0.5)
//         }, Cell {
//             status: Status::Decommissioning(0),
//             resources: resources!(soil: 2.0, labor: 3.0)
//         }];
//         let mut grid = CellGrid::new(cells);

//         for i in 1..=STATUS_CHANGE_STEPS {
//             grid.update_cells();
//             assert_eq!(grid.cells[0].status, Status::Developing(i));
//             assert_eq!(grid.cells[1].status, Status::Decommissioning(i));
//         }

//         grid.update_cells();
//         assert_eq!(grid.cells[0].status, Status::Active(1));
//         assert_eq!(grid.cells[1].status, Status::Available);
//     }

//     #[test]
//     fn test_expand_resources() {
//         let cells = [Cell {
//             status: Status::Developing(0),
//             resources: resources!(sun: 8.0, soil: 3.5, labor: 5.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(soil: 2.0, labor: 3.0)
//         }, Cell {
//             status: Status::Active(1),
//             resources: resources!(soil: 2.0, labor: 3.0, sun: 2.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(soil: 2.0, labor: 1.0, sun: 1.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(sun: 2.0, labor: 1.5, soil: 4.0)
//         }];
//         let mut grid = CellGrid::new(cells);
//         let deficit = resources!(
//             soil: 2.,
//             labor: 4.
//         );
//         let n_expansions = 2;
//         let idxs = grid.expand_resources(&deficit, n_expansions);
//         let expected = vec![1, 4];
//         assert_eq!(idxs, expected);
//     }

//     #[test]
//     fn test_contract_resources() {
//         let cells = [Cell {
//             status: Status::Developing(0),
//             resources: resources!(sun: 8.0, soil: 3.5, labor: 5.0)
//         }, Cell {
//             status: Status::Available,
//             resources: resources!(soil: 2.0, labor: 3.0)
//         }, Cell {
//             status: Status::Active(1),
//             resources: resources!(soil: 2.0, labor: 3.0, sun: 2.0)
//         }, Cell {
//             status: Status::Active(1),
//             resources: resources!(soil: 2.0, labor: 1.0, sun: 1.0)
//         }, Cell {
//             status: Status::Active(1),
//             resources: resources!(sun: 2.0, labor: 1.5, soil: 4.0)
//         }];
//         let mut grid = CellGrid::new(cells);
//         let surplus = resources!(
//             soil: 1.0,
//             sun: 1.0,
//             labor: 4.0
//         );

//         let idxs = vec![2,3,4];
//         let transition_speed = 0.2;
//         let kept_idxs = grid.contract_resources(&idxs, &surplus, transition_speed);
//         let expected = vec![4, 3];
//         assert_eq!(kept_idxs, expected);
//     }
// }
