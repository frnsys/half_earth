#[macro_use]
pub mod kinds;
mod npcs;
mod utils;
mod projects;
mod world;
mod industries;
mod consts;
mod regions;
mod content;
mod state;
pub mod game;
pub mod events;
pub mod surface;
pub mod production;

// When the `wee_alloc` feature is enabled,
// use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
