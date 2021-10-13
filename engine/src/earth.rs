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
