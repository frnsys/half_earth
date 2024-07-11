mod card;
mod cards;
mod kinds;
mod mini;

use crate::vars::{Impact, Var};

pub use cards::{CardFocusArea, Cards};
pub use kinds::*;
pub use mini::*;

#[derive(Clone)]
pub struct FactorsCard {
    pub icon: &'static str,
    pub kind: Var,
    pub total: f32,
    pub current: Option<String>,
}
