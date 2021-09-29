use crate::kinds::OutputMap;

#[derive(Default)]
pub struct Earth {
    emissions: f32,       // GtCO2eq
    atmospheric_ghg: f32, // ppm
    biodiversity: f32,    // species index? <http://www.coastalwiki.org/wiki/Measurements_of_biodiversity>
    temperature: f32,     // global temp avg, C
    precipitation: f32,   // global precip avg
    sea_level_rise: f32,  // meters
    ozone_damage: f32     // % eroded
}

#[derive(Default)]
pub struct Region<'a> {
    pub name: &'a str,
    pub population: u32,

    /// Per-capita demand
    pub demand: OutputMap<f32>,

    /// "Satiety" is a catch-all for
    /// non-baseline-survival satisfaction,
    /// like agency, community
    pub satiety: f32,

    /// Public health
    pub health: f32,

    /// How safe people in the region are/feel
    pub safety: f32,

    /// How hopeful are people in the region about the future?
    pub outlook: f32,

    /// Base habitability encapsulates
    /// other factors that influence habitability.
    /// E.g. negative events such as hurricanes should subtract
    /// from this value
    pub base_habitability: f32,
}

impl Region<'_> {
    // Simple mean
    pub fn contentedness(&self) -> f32 {
        (self.health + self.safety + self.outlook + self.satiety)/4.
    }

    pub fn habitability(&self) -> f32 {
        // Factors:
        // - regional temp, precip, sea_level_rise, health, safety,
        // - number of negative events
        todo!()
    }
}
