use crate::kinds::{ByproductMap, ResourceMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Industry {
    pub id: usize,
    pub name: String,
    pub resources: ResourceMap,
    pub byproducts: ByproductMap,
    pub resource_modifiers: ResourceMap,
    pub byproduct_modifiers: ByproductMap,
    pub demand_modifier: f32,
}

impl Industry {
    pub fn adj_resources(&self) -> ResourceMap {
        self.resources * (self.resource_modifiers + 1.)
    }

    pub fn adj_byproducts(&self) -> ByproductMap {
        self.byproducts * (self.byproduct_modifiers + 1.)
    }

    pub fn extinction_rate(&self, starting_land: f32) -> f32 {
        let pressure = self.adj_byproducts().biodiversity;
        let land = self.adj_resources().land;
        (pressure / 1e4 + land / starting_land) * 100.
    }
}

// impl Serialize for Industry {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_struct("Industry", 5)?; // TODO derived fields
//         seq.serialize_field("id", &self.id)?;
//         seq.serialize_field("name", &self.name)?;
//         seq.serialize_field("resources", &self.adj_resources())?;
//         seq.serialize_field("byproducts", &self.adj_byproducts())?;
//         seq.serialize_field("extinction_rate", &self.extinction_rate())?;
//         seq.end()
//     }
// }
