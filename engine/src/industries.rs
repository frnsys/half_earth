use serde::Serialize;
use crate::kinds::{ResourceMap, ByproductMap};

#[derive(Serialize, Clone)]
pub struct Industry {
    pub id: usize,
    pub name: &'static str,
    pub resources: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub demand_modifier: f32,
    pub resource_modifiers: ResourceMap<f32>,
    pub byproduct_modifiers: ByproductMap<f32>,
}

impl Industry {
    pub fn adj_resources(&self) -> ResourceMap<f32> {
        self.resources * (self.resource_modifiers + 1.)
    }

    pub fn adj_byproducts(&self) -> ByproductMap<f32> {
        self.byproducts * (self.byproduct_modifiers + 1.)
    }
}
