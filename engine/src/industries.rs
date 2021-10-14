use crate::kinds::{ResourceMap, ByproductMap};

pub struct Industry {
    pub name: &'static str,
    pub resources: ResourceMap<f32>,
    pub byproducts: ByproductMap<f32>,
}
