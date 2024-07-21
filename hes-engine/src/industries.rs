use crate::{
    flavor::IndustryFlavor,
    kinds::{ByproductMap, ResourceMap},
    HasId,
    Id,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default,
)]
pub struct Industry {
    pub id: Id,
    pub name: String,
    pub resources: ResourceMap,
    pub byproducts: ByproductMap,
    pub resource_modifiers: ResourceMap,
    pub byproduct_modifiers: ByproductMap,
    pub demand_modifier: f32,
    pub flavor: IndustryFlavor,
}

impl Display for Industry {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl HasId for Industry {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl Industry {
    pub fn new() -> Industry {
        Industry {
            id: Id::new_v4(),
            name: "New Industry".into(),
            ..Default::default()
        }
    }

    pub fn demand(&self, lic_pop: f32) -> f32 {
        self.demand_modifier * lic_pop
    }

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
