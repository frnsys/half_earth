use super::kinds::SectorMap;

struct World {
    emissions: i32,       // GtCO2eq
    atmospheric_ghg: f32, // ppm
    biodiversity: i32,    // species index? <http://www.coastalwiki.org/wiki/Measurements_of_biodiversity>
    temperature: f32,     // global temp avg, C
    precipitation: f32,   // global precip avg
    sea_level_rise: f32,  // meters
    ozone_health: f32 // TODO ??
}

struct Region<'a> {
    name: &'a str,
    population: u32,

    // Per capita demand for each sector
    demand: SectorMap<f32>,

    health: f32,
    safety: f32,
    outlook: f32,

    // Catch-all for non-baseline-survival satisfaction,
    // like agency, community
    satiety: f32,
}

impl Region<'_> {
    // Simple mean
    fn contentedness(&self) -> f32 {
        (self.health + self.safety + self.outlook + self.satiety)/4.
    }

    fn habitability(&self) -> f32 {
        todo!()
    }
}
