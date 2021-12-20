use crate::consts;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::kinds::{ResourceMap, ByproductMap};

#[derive(Clone)]
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

    pub fn extinction_rate(&self) -> f32 {
        let pressure = self.adj_byproducts().biodiversity;
        let land = self.adj_resources().land;
        (pressure/1e4 + land/consts::STARTING_RESOURCES.land) * 100.
    }
}

impl Serialize for Industry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_struct("Industry", 5)?;
        seq.serialize_field("id", &self.id)?;
        seq.serialize_field("name", &self.name)?;
        seq.serialize_field("resources", &self.adj_resources())?;
        seq.serialize_field("byproducts", &self.adj_byproducts())?;
        seq.serialize_field("extinction_rate", &self.extinction_rate())?;
        seq.end()
    }
}
