#[macro_use]
mod kinds;
mod game;
mod utils;
mod projects;
mod world;
mod events;
mod industries;
mod production;
mod consts;
mod regions;
mod content;
pub mod surface;

// When the `wee_alloc` feature is enabled,
// use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
