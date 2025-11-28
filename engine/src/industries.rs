use crate::{
    Collection, HasId, Id, byproducts,
    flavor::IndustryFlavor,
    kinds::{Byproduct, ByproductMap, Resource, ResourceMap},
    resources,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Industry {
    pub id: Id,
    pub name: String,
    pub resources: ResourceMap,
    pub byproducts: ByproductMap,
    pub resource_modifiers: ResourceMap,
    pub byproduct_modifiers: ByproductMap,
    pub demand_modifier: f32,
    pub flavor: IndustryFlavor,
    pub notes: String,
}

impl Display for Industry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        self.demand_modifier.max(0.) * lic_pop
    }

    pub fn total_demand_for_resource(&self, lic_pop: f32, resource: Resource) -> f32 {
        self.demand(lic_pop) * self.adj_resources()[resource]
    }

    pub fn total_for_byproduct(&self, lic_pop: f32, byproduct: Byproduct) -> f32 {
        self.demand(lic_pop) * self.adj_byproducts()[byproduct]
    }

    // NOTE: For byproducts and resources we assume that less is better,
    // which influences how we calculate modified values:
    //
    // - The multiplier (modifier) value has a minimum of 0%, i.e. it can
    //   never "flip" the sign of a value.
    // - The multiplier only applies to positive values, i.e. it can only
    //   shrink positive impacts.
    //
    // Consider the scenario where an industry has negative carbon emissions,
    // but you also have effects that reduce carbon emissions by a percentage.
    // So if you have `-1` emissions and a 25% reduction to emissions, we don't
    // want it to then "reduce" the negative emissions to `-0.75`.
    //
    // Alternatively, if we had managed to get a total reduction of over 100%,
    // i.e. leading to a negative multiplier (say 125% reduction so -0.25 multiplier),
    // this multiplier would flip our negative emissions back to positive.
    //
    // A better way to do this would be to have the:
    //
    // - The unmodified base value
    // - The absolute change from effects
    // - The multiplier from effects
    //
    // And do `(base * multiplier) - change`, but that is probably too big
    // a change for this point in the game's development.
    //
    // In reality I don't think there is a "right" answer here as it's
    // sensitive to the order of effects, e.g. subtract an absolute amount,
    // then apply a multiplier, then add an amount, etc, which is different
    // than what we do here which is aggregate absolute changes and multipliers
    // into single values.
    pub fn total_byproducts(&self, lic_pop: f32) -> ByproductMap {
        self.adj_byproducts() * self.demand(lic_pop)
    }

    pub fn adj_resources(&self) -> ResourceMap {
        (self.resources * (self.resource_modifiers + 1.).max(0.)).or_negative(self.resources)
    }

    pub fn adj_byproducts(&self) -> ByproductMap {
        (self.byproducts * (self.byproduct_modifiers + 1.).max(0.)).or_negative(self.byproducts)
    }

    pub fn extinction_rate(&self, starting_land: f32) -> f32 {
        let pressure = self.adj_byproducts().biodiversity;
        let land = self.adj_resources().land;
        (pressure / 1e4 + land / starting_land) * 100.
    }
}

impl Collection<Industry> {
    pub fn resource_demand(&self, lic_pop: f32) -> ResourceMap {
        self.iter().fold(resources!(), |acc, ind| {
            acc + ind.adj_resources() * ind.demand(lic_pop)
        })
    }

    pub fn byproducts(&self, lic_pop: f32) -> ByproductMap {
        self.iter().fold(byproducts!(), |acc, ind| {
            acc + ind.adj_byproducts() * ind.demand(lic_pop)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_resource_demand() {
        let lic_pop = 100.;
        let mut industry = Industry::new();
        industry.resources.water = 2.;
        industry.demand_modifier = 1.;
        industry.resource_modifiers.water = 0.;

        let demand = industry.total_demand_for_resource(lic_pop, Resource::Water);
        assert_eq!(demand, 200.);

        industry.resource_modifiers.water = 1.;
        let demand = industry.total_demand_for_resource(lic_pop, Resource::Water);
        assert_eq!(demand, 400.);

        industry.demand_modifier = 2.;
        let demand = industry.total_demand_for_resource(lic_pop, Resource::Water);
        assert_eq!(demand, 800.);
    }

    #[test]
    fn test_total_byproduct() {
        let lic_pop = 100.;
        let mut industry = Industry::new();
        industry.byproducts.co2 = 2.;
        industry.demand_modifier = 1.;
        industry.byproduct_modifiers.co2 = 0.;

        let emissions = industry.total_for_byproduct(lic_pop, Byproduct::Co2);
        assert_eq!(emissions, 200.);

        industry.byproduct_modifiers.co2 = 1.;
        let emissions = industry.total_for_byproduct(lic_pop, Byproduct::Co2);
        assert_eq!(emissions, 400.);

        industry.demand_modifier = 2.;
        let emissions = industry.total_for_byproduct(lic_pop, Byproduct::Co2);
        assert_eq!(emissions, 800.);
    }

    #[test]
    fn test_adj_byproducts() {
        let mut industry = Industry::new();
        industry.byproducts.co2 = -2.;
        industry.byproduct_modifiers.co2 = 0.;

        let bps = industry.adj_byproducts();
        assert_eq!(bps.co2, -2.);

        // Modifier should not apply to negative value.
        industry.byproduct_modifiers.co2 = 1.;
        let bps = industry.adj_byproducts();
        assert_eq!(bps.co2, -2.);

        // Modifier should apply to positive value.
        industry.byproducts.co2 = 1.;
        industry.byproduct_modifiers.co2 = 1.;
        let bps = industry.adj_byproducts();
        assert_eq!(bps.co2, 2.);

        // Negative modifier should apply to positive value.
        industry.byproducts.co2 = 1.;
        industry.byproduct_modifiers.co2 = -0.5;
        let bps = industry.adj_byproducts();
        assert_eq!(bps.co2, 0.5);

        // Negative modifier should not apply to negative value.
        industry.byproducts.co2 = -1.;
        industry.byproduct_modifiers.co2 = -0.5;
        let bps = industry.adj_byproducts();
        assert_eq!(bps.co2, -1.);

        // Negative modifiers over 100% should reduce positive value to negative.
        industry.byproducts.co2 = 1.;
        industry.byproduct_modifiers.co2 = -1.5;
        let bps = industry.adj_byproducts();
        assert_eq!(bps.co2, 0.);

        // Negative modifiers over 100% should not apply to negative value.
        industry.byproducts.co2 = -1.;
        industry.byproduct_modifiers.co2 = -1.5;
        let bps = industry.adj_byproducts();
        assert_eq!(bps.co2, -1.);
    }
}
