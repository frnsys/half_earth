use crate::player::Player;
use crate::events::EventPool;
use crate::production::CellGrid;
use crate::earth::{Earth, Region};
use crate::kinds::{OutputMap, ResourceMap, ByproductMap};

#[derive(Default)]
pub struct State<'a> {
    player: Player,
    events: EventPool,
    resources: CellGrid,
    demand: OutputMap<f32>,
    stocks: ResourceMap<f32>,
    byproducts: ByproductMap<f32>,
    earth: Earth,
    regions: Vec<Region<'a>>,
}


impl State<'_> {
    pub fn step(&mut self) {
        // TODO
        // - update cells
        // - refresh resources
        // - extract resources, add stocks from previous step
        // - calculate demand for this step
        // - create production orders for all sectors
        // - merge in production orders from all projects
        // - run production function
        // - compare required resources against planned resources; then adjust resources
        // - for stockable resources, save stock for next step

        // check if projects finished/stalled/etc, and apply or wtihdraw effects
        // Run events, apply effects
    }
}

// TODO figure out how this interfaces with Javascript
