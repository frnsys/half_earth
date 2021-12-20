use wasm_bindgen::prelude::*;
use crate::projects::years_for_points;

#[wasm_bindgen]
pub fn years_remaining(progress: f32, points: usize, cost: usize) -> usize {
    let remaining = 1. - progress;
    let progress_per_year = 1./years_for_points(points, cost);
    (remaining/progress_per_year).round() as usize
}
