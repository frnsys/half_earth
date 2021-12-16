use crate::consts;
use wasm_bindgen::prelude::*;
use crate::projects::years_for_points;

/// Contribution to extinction rate from the tgav
#[wasm_bindgen]
pub fn tgav_extinction_rate(tgav: f32) -> f32 {
    tgav.powf(2.)
}

/// Contribution to extinction rate from sea level rise
#[wasm_bindgen]
pub fn slr_extinction_rate(slr: f32) -> f32 {
    slr.powf(2.)
}

/// Contribution to extinction rate from a single process
#[wasm_bindgen]
pub fn process_extinction_rate(biodiversity_pressure: f32, land: f32, produced: f32) -> f32 {
    (biodiversity_pressure/1e4 + land/consts::STARTING_RESOURCES.land) * produced * 100.
}

#[wasm_bindgen]
pub fn sea_level_rise_rate(tgav: f32, modifier: f32) -> f32 {
    // Meters
    // 0.0005mm per year per deg warming
    // Chosen to roughly hit 1.5m-1.6m rise by 2100 in the BAU scenario
    (0.0065 * tgav) + modifier
}

#[wasm_bindgen]
pub fn years_remaining(progress: f32, points: usize, cost: usize) -> usize {
    let remaining = 1. - progress;
    let progress_per_year = 1./years_for_points(points, cost);
    (remaining/progress_per_year).round() as usize
}
