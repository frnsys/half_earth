#[macro_use]
pub mod kinds;
pub mod events;
pub mod flavor;
pub mod industries;
pub mod npcs;
pub mod production;
pub mod projects;
pub mod regions;
pub mod state;
pub mod surface;
mod util;
pub mod world;
pub use projects::Type as ProjectType;
pub use state::State;
pub use util::*;
