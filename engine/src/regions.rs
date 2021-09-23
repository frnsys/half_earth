use std::collections::HashSet;
use super::kinds::{Resource, ResourceMap};

const STATUS_CHANGE_STEPS: u8 = 3;

#[derive(Debug, PartialEq)]
pub enum Status {
    Available,
    Building(u8),
    Active,
    Decommissioning(u8)
}

pub type CellIdx = usize;

#[derive(Debug)]
pub struct Cell {
    pub status: Status,

    // Resources available in this cell, and in what quantity
    pub resources: ResourceMap<f32>,
}

pub struct CellGrid<const N: usize> {
    cells: [Cell; N],
    index: ResourceMap<HashSet<CellIdx>>
}

impl<const N: usize> CellGrid<N> {
    fn new(cells: [Cell; N]) -> CellGrid<N> {
        let mut index: ResourceMap<HashSet<CellIdx>> = resources!();
        for (i, cell) in cells.iter().enumerate() {
            for (k, v) in cell.resources.items() {
                if v > 0. {
                    index[k].insert(i);
                }
            }
        }

        CellGrid {
            cells, index
        }
    }

    pub fn resources_for_cells(&self, idxs: &[CellIdx]) -> ResourceMap<f32> {
        idxs.iter().filter_map(|idx| {
            let cell = &self.cells[*idx];
            match cell.status {
                Status::Active => Some(cell.resources),
                _ => None
            }
        }).fold(resources!(), |acc, res| {
            acc + res
        })
    }

    fn planned_resources_for_cells(&self, idxs: &[CellIdx]) -> ResourceMap<f32> {
        idxs.iter().filter_map(|idx| {
            let cell = &self.cells[*idx];
            match cell.status {
                Status::Active|Status::Building(_) => Some(cell.resources),
                _ => None
            }
        }).fold(resources!(), |acc, res| {
            acc + res
        })
    }

    fn find_cells_for_resources(&self, resources: &ResourceMap<f32>) -> HashSet<CellIdx> {
        let mut sets: Vec<HashSet<CellIdx>> = resources.items().iter().filter_map(|(k, v)| {
            if *v > 0. {
                Some(self.index[*k].clone())
            } else {
                None
            }
        }).collect();
        if sets.len() == 0 {
            HashSet::default()
        } else {
            // Ok to unwrap, checked that there's at least one item
            let mut intersection = sets.pop().unwrap();
            for other in sets {
                intersection.retain(|e| other.contains(e));
            }
            intersection
        }
    }

    fn cells_by_potential(&self, idxs: &Vec<CellIdx>, needed: &ResourceMap<f32>) -> Vec<(CellIdx, f32)> {
        let total_need: f32 = needed.values().iter().sum();
        let weights: ResourceMap<f32> = *needed/total_need;
        let mut cells: Vec<(CellIdx, f32)> = idxs.iter().filter_map(|idx| {
            let cell = &self.cells[*idx];
            // TODO could also minize resources that *aren't* needed
            let potential = cell.resources.items().iter().fold(0., |acc, (k, v)| {
                      // How much of the resource is available,
                      // up to how much is actually needed (to minimize excess/waste)
                acc + v.min(needed[*k]) * weights[*k]
                                          // Weighted by how much it's needed
            });
            Some((*idx, potential))
        }).collect();

        // Highest to lowest potential
        cells.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        cells
    }

    // Claim new cells that are most suited to providing the required resources
    pub fn expand_resources(&mut self, deficit: &ResourceMap<f32>, n_expansions: usize) -> Vec<CellIdx> {
        let idxs = self.find_cells_for_resources(&deficit).iter().cloned()
            .filter(|idx| self.cells[*idx].status == Status::Available).collect();
        let cells = self.cells_by_potential(&idxs, deficit);

        let n_expands = cells.len().min(n_expansions);
        cells[..n_expands].iter().map(|(idx, _)| {
            self.cells[*idx].status = Status::Building(0);
            *idx
        }).collect()
    }

    // The assumption here is that resource contraction should happen as quickly as possible, i.e.
    // get rid of the highest surplus resource capacity cells first, i.e. get rid of as few cells
    // as possible. The downside is that it frees up land the slowest too (unless land is the
    // surplus resource; so maybe that's fine since we explicitly track land as a resource?)
    pub fn contract_resources(&mut self, idxs: &Vec<CellIdx>, surplus: &ResourceMap<f32>, transition_speed: f32) -> Vec<CellIdx> {
        // TODO This feels like it can be cleaner/simplified
        let to_reduce: Vec<Resource> = surplus.keys().iter().filter(|k| surplus[**k] > 0.).cloned().collect();
        let mut reduce_by = *surplus * transition_speed;
        let cells = self.cells_by_potential(idxs, surplus);

        let mut to_keep = Vec::with_capacity(cells.len());
        for (idx, _) in cells.iter() {
            if to_reduce.iter().any(|k| reduce_by[*k] <= 0.) {
                to_keep.push(*idx);
            } else {
                reduce_by -= self.cells[*idx].resources;
            }
        }
        to_keep
    }

    // TODO test
    pub fn deduct_resources(&mut self, idxs: &Vec<CellIdx>, resources: &ResourceMap<f32>) {
    }

    fn update_cells(&mut self) {
        for cell in &mut self.cells {
            match cell.status {
                Status::Building(step) => {
                    if step >= STATUS_CHANGE_STEPS {
                        cell.status = Status::Active;
                    } else {
                        cell.status = Status::Building(step+1);
                    }
                },
                Status::Decommissioning(step) => {
                    if step >= STATUS_CHANGE_STEPS {
                        cell.status = Status::Available;
                    } else {
                        cell.status = Status::Decommissioning(step+1);
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

    #[test]
    fn test_cells_for_resources() {
        let cells = [Cell {
            status: Status::Available,
            resources: resources!(water: 1.0, soil: 0.5)
        }, Cell {
            status: Status::Available,
            resources: resources!(soil: 2.0, labor: 3.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(soil: 2.0, labor: 1.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(sun: 2.0, labor: 1.0)
        }];
        let grid = CellGrid::new(cells);

        // Should only return cell indices where
        // soil and labor is present
        let required = resources!(soil: 0.2, labor: 0.3);
        let idxs = grid.find_cells_for_resources(&required);

        let mut expected = HashSet::default();
        expected.insert(1);
        expected.insert(2);
        assert_eq!(idxs, expected);
    }

    #[test]
    fn test_resources_for_sector() {
        let cells = [Cell {
            status: Status::Active,
            resources: resources!(water: 1.0, soil: 0.5)
        }, Cell {
            status: Status::Building(0),
            resources: resources!(soil: 2.0, labor: 3.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(soil: 2.0, labor: 1.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(sun: 2.0, labor: 1.0)
        }];
        let grid = CellGrid::new(cells);
        let idxs = [0, 1];
        let resources = grid.resources_for_cells(&idxs);
        let expected = resources!(
            water: 1.,
            labor: 0.,
            soil: 0.5
        );
        assert_eq!(resources, expected);
    }

    #[test]
    fn test_planned_resources_for_sector() {
        let cells = [Cell {
            status: Status::Active,
            resources: resources!(water: 1.0, soil: 0.5)
        }, Cell {
            status: Status::Building(0),
            resources: resources!(soil: 2.0, labor: 3.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(soil: 2.0, labor: 1.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(sun: 2.0, labor: 1.0)
        }];
        let grid = CellGrid::new(cells);
        let idxs = [0, 1];
        let resources = grid.planned_resources_for_cells(&idxs);
        let expected = resources!(
            water: 1.,
            labor: 3.,
            soil: 2.5
        );
        assert_eq!(resources, expected);
    }

    #[test]
    fn test_update_cells() {
        let cells = [Cell {
            status: Status::Building(0),
            resources: resources!(water: 1.0, soil: 0.5)
        }, Cell {
            status: Status::Decommissioning(0),
            resources: resources!(soil: 2.0, labor: 3.0)
        }];
        let mut grid = CellGrid::new(cells);

        for i in 1..=STATUS_CHANGE_STEPS {
            grid.update_cells();
            assert_eq!(grid.cells[0].status, Status::Building(i));
            assert_eq!(grid.cells[1].status, Status::Decommissioning(i));
        }

        grid.update_cells();
        assert_eq!(grid.cells[0].status, Status::Active);
        assert_eq!(grid.cells[1].status, Status::Available);
    }

    #[test]
    fn test_expand_resources() {
        let cells = [Cell {
            status: Status::Building(0),
            resources: resources!(sun: 8.0, soil: 3.5, labor: 5.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(soil: 2.0, labor: 3.0)
        }, Cell {
            status: Status::Active,
            resources: resources!(soil: 2.0, labor: 3.0, sun: 2.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(soil: 2.0, labor: 1.0, sun: 1.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(sun: 2.0, labor: 1.5, soil: 4.0)
        }];
        let mut grid = CellGrid::new(cells);
        let deficit = resources!(
            soil: 2.,
            labor: 4.
        );
        let n_expansions = 2;
        let idxs = grid.expand_resources(&deficit, n_expansions);
        let expected = vec![1, 4];
        assert_eq!(idxs, expected);
    }

    #[test]
    fn test_contract_resources() {
        let cells = [Cell {
            status: Status::Building(0),
            resources: resources!(sun: 8.0, soil: 3.5, labor: 5.0)
        }, Cell {
            status: Status::Available,
            resources: resources!(soil: 2.0, labor: 3.0)
        }, Cell {
            status: Status::Active,
            resources: resources!(soil: 2.0, labor: 3.0, sun: 2.0)
        }, Cell {
            status: Status::Active,
            resources: resources!(soil: 2.0, labor: 1.0, sun: 1.0)
        }, Cell {
            status: Status::Active,
            resources: resources!(sun: 2.0, labor: 1.5, soil: 4.0)
        }];
        let mut grid = CellGrid::new(cells);
        let surplus = resources!(
            soil: 1.0,
            sun: 1.0,
            labor: 4.0
        );

        let idxs = vec![2,3,4];
        let transition_speed = 0.2;
        let kept_idxs = grid.contract_resources(&idxs, &surplus, transition_speed);
        let expected = vec![4, 3];
        assert_eq!(kept_idxs, expected);
    }
}
