mod card;
mod cards;
mod kinds;
mod mini;

use crate::{display, vars::Var};

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
                    display::percent(self.total, true)
                )
            }
            Var::Energy => format!("{:.1}TWh", self.total),
            Var::Electricity => format!("{:.1}TWh", self.total),
            Var::Fuel => format!("{:.1}TWh", self.total),
            Var::PlantCalories => {
                format!("{:.1}Tcals", self.total)
            }
            Var::AnimalCalories => {
                format!("{:.1}Tcals", self.total)
            }
            Var::Contentedness => {
                format!("{}", self.total)
            }
        }
    }
}
