mod card;
mod cards;
mod kinds;
mod mini;

use crate::{display, vars::Var};

pub use cards::{CardFocusArea, Cards};
pub use kinds::*;
pub use mini::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FactorsCard {
    pub icon: &'static str,
    pub kind: Var,
    pub total: f32,
    pub current: Option<String>,
}
impl FactorsCard {
    pub fn total_formatted(&self) -> String {
        match self.kind {
            Var::Emissions => display::emissions(self.total),
            Var::Biodiversity => format!("{:.0}", self.total),
            Var::Land => {
                format!(
                    "{}%",
                    display::percent(self.total / 100., true)
                )
            }
            Var::Water => {
                format!(
                    "{}%",
                    display::percent(self.total / 100., true)
                )
            }
            Var::Energy => format!("{}", self.total.round()),
            Var::Electricity => {
                format!("{}", self.total.round())
            }
            Var::Fuel => format!("{}", self.total.round()),
            Var::PlantCalories => {
                format!("{}", self.total.round())
            }
            Var::AnimalCalories => {
                format!("{}", self.total.round())
            }
            Var::Contentedness => {
                format!("{}", self.total)
            }
        }
    }
}
