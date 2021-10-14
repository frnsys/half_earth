#[derive(Default)]
pub struct World {
    pub year: usize,
    pub population: f32,      // Total population
    pub contentedness: f32,   // Mean contentedness
    pub health: f32,          // Mean health
    pub outlook: f32,         // Mean oultook

    pub extinction_rate: f32,
    pub temperature: f32,     // global temp anomaly, C
    pub precipitation: f32,   // global precip avg
    pub sea_level_rise: f32,  // meters
    pub water_stress: f32,    // 0-100%
}

impl World {
    pub fn population(&self) -> f32 {
    }
}
