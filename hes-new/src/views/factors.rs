use rust_i18n::t;

use crate::{
    consts,
    display::{
        self,
        HasIcon,
        factors::Factor,
        icon_from_slug,
        icons,
        intensity::render_intensity_bar_with_pips,
    },
    state::FACTORS,
    vars::Var,
};

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

pub fn render_factors_list(
    ui: &mut egui::Ui,
    factors: FactorsCard,
) {
    let relation = {
        let relation = match factors.kind {
            Var::Emissions => "makes",
            Var::Biodiversity => "causes",
            _ => "uses",
        };
        t!(relation)
    };

    let cur_name = factors.current.as_ref();

    ui.horizontal_centered(|ui| {
        ui.label(format!("{} :", t!("Total")));

        let max_value = match factors.kind {
            Var::Biodiversity => Some(consts::MAX_BIODIVERSITY),
            Var::Contentedness => {
                Some(consts::MAX_CONTENTEDNESS)
            }
            _ => None,
        };
        let total = factors.total_formatted();
        if let Some(max_value) = max_value {
            ui.label(format!("{total}/{max_value}"));
        } else {
            ui.label(total);
        }
        ui.image(icon_from_slug(factors.icon));
    });

    let ranked = FACTORS.read();
    let ranked =
        ranked[factors.kind].iter().filter(|user| match user {
            Factor::Industry { produced, .. }
            | Factor::Process { produced, .. } => {
                *produced != 0.
            }
            _ => true,
        });

    for user in ranked {
        let highlight =
            cur_name.is_some_and(|name| name == user.name()); // TODO use to highlight user
        let name = user.name();
        ui.label(t!(name));
        render_factor_line(ui, user, &relation, factors.icon);
    }
}

fn render_factor_line(
    ui: &mut egui::Ui,
    factor: &Factor,
    relation: &str,
    icon: &'static str,
) {
    match factor {
        Factor::Region {
            intensity, display, ..
        } => {
            ui.horizontal_centered(|ui| {
                ui.image(icon_from_slug(icons::WEALTH));
                render_intensity_bar_with_pips(
                    ui, *intensity, false, 4,
                );
                ui.label(display);
                ui.image(icon_from_slug(icon));
            });
        }
        Factor::Project { display, .. } => {
            ui.horizontal_centered(|ui| {
                ui.label(display);
                ui.image(icon_from_slug(icon));
            });
        }
        Factor::Event {
            display, amount, ..
        } => {
            let display = display
                .clone()
                .unwrap_or_else(|| amount.to_string());
            ui.horizontal_centered(|ui| {
                ui.label(display);
                ui.image(icon_from_slug(icon));
            });
        }
        Factor::Process {
            intensity,
            display_produced,
            display,
            output,
            ..
        } => {
            ui.horizontal_centered(|ui| {
                ui.image(icon_from_slug(icon));
                render_intensity_bar_with_pips(
                    ui, *intensity, false, 4,
                );
                ui.label(display_produced);
                ui.image(icon_from_slug(output.icon()));
                ui.label(relation);
                ui.label(display);
                ui.image(icon_from_slug(icon));
            });
        }
        Factor::Industry {
            intensity, display, ..
        } => {
            ui.horizontal_centered(|ui| {
                ui.image(icon_from_slug(icon));
                render_intensity_bar_with_pips(
                    ui, *intensity, false, 4,
                );
                ui.label(display);
                ui.image(icon_from_slug(icon));
            });
        }
    }
}
