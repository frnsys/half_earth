use crate::kinds::{ResourceMap, ByproductMap};

pub struct Industry {
    name: &'static str,
    resources: ResourceMap<f32>,
    byproducts: ByproductMap<f32>,
}
