mod card;
mod card_focus_area;
mod cards;
mod draggable;
mod kinds;

use crate::display::{Impact, Var};

use super::parts::IntensityBar;
pub use card_focus_area::CardFocusArea;
pub use cards::Cards;
pub use draggable::{DragRect, Draggable};
pub use kinds::*;

// pub trait Card {
// }

#[derive(Clone)]
pub struct FactorsCard {
    pub icon: &'static str,
    pub kind: Var,
    pub total: f32,
    pub current: Option<String>,
}

// TODO
#[derive(Clone)]
pub struct Image {
    pub path: String,
    pub attribution: String,
}
