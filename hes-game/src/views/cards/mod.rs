mod card;
mod card_focus_area;
mod cards;
mod draggable;
mod kinds;

use crate::display::{Impact, Var};

use super::parts::IntensityBar;
pub use cards::Cards;
pub use card_focus_area::CardFocusArea;
pub use draggable::{Draggable, DragRect};

// pub trait Card {
// }

#[derive(Clone)]
pub struct FactorsCard {
    pub text: String,
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
