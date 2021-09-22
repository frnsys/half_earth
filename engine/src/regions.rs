use super::kinds::{Resource, Sector, SectorMap, ResourceMap, ByproductMap};

pub struct Cell {
    // Cells can have up to 3 overlapping uses
    // TODO are the cell's resources just evenly split between them?
    pub users: [Option<(Sector, u8)>; 3],

    // Resources available in this cell, and in what quantity
    pub resources: ResourceMap<f32>,
}

struct CellGrid<const N: usize> {
    width: usize,
    height: usize,
    cells: [Cell; N]
}
