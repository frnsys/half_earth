#[macro_use]
pub mod kinds;
pub mod events;
pub mod game;
pub mod industries;
pub mod npcs;
pub mod production;
pub mod projects;
pub mod regions;
pub mod state;
pub mod surface;
pub mod world;

pub mod flavor;

use projects::years_for_points;

pub use game::Game;
pub use projects::Type as ProjectType;

pub fn years_remaining(
    progress: f32,
    points: usize,
    cost: usize,
) -> usize {
    let remaining = 1. - progress;
    let progress_per_year = 1. / years_for_points(points, cost);
    (remaining / progress_per_year).round() as usize
}
