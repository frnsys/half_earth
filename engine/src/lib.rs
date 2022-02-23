#[macro_use]
pub mod kinds;
mod save;
mod npcs;
mod utils;
mod world;
mod industries;
mod regions;
pub mod content;
pub mod core;
pub mod projects;
pub mod consts;
pub mod state;
pub mod game;
pub mod events;
pub mod surface;
pub mod production;

// When the `wee_alloc` feature is enabled,
// use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
