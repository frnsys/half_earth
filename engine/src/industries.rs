use serde::Serialize;
use crate::kinds::{ResourceMap, ByproductMap};

#[derive(Serialize)]
pub struct Industry {
    pub name: &'static str,
    pub resources: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
    pub demand_modifier: f32,
}
