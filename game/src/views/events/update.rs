use hes_engine::{KindMap, Project, Region, State, Update as EngineUpdate, World};
use hes_images::flavor_image;
use rust_i18n::t;

use super::{AsEventView, Dialogue, EventDetails};
use crate::{
    display::{
        AsText, HasIcon, active_effects, icons,
        intensity::{self, intensity_bar},
        render_effects,
    },
    tips::{add_tip, tip},
};

impl AsEventView for EngineUpdate {
    fn details<'a>(&'a self, state: &'a State) -> EventDetails<'a> {
        let title = title_for_update(self);
        let (image, attrib) = image_for_update(self, &state.world);

        let name = match self {
            EngineUpdate::Project { id } | EngineUpdate::Policy { id } => {
                let proj = &state.world.projects[id];
                &proj.name
            }
            EngineUpdate::Region { id, .. } => {
                let region = &state.world.regions[id];
                &region.name
            }
        };
        EventDetails {
            title,
            name,
            image: Some(image),
            attrib: Some(attrib),
            effects: None,
        }
    }

    fn dialogue(&self, state: &State) -> Option<super::dialogue::Dialogue> {
        match self {
            EngineUpdate::Policy { id } | EngineUpdate::Project { id } => {
                let proj = &state.world.projects[id];
                proj.active_outcome.and_then(|id| {
                    let d = proj.flavor.outcomes.get(id);
                    d.map(|d| Dialogue::new(d.clone(), None, None, None))
                })
            }
            _ => None,
        }
    }

    fn show_card(&self) -> bool {
        true
    }

    fn render_extras(&self, ui: &mut egui::Ui, state: &State) {
        match self {
            EngineUpdate::Project { id } | EngineUpdate::Policy { id } => {
                let proj = &state.world.projects[id];
                render_project_outcomes(ui, proj, state);
            }
            EngineUpdate::Region { id, up } => {
                let region = &state.world.regions[id];
                render_region_outcomes(ui, region, *up, &state.world);
            }
        }
    }
}

fn title_for_update(update: &EngineUpdate) -> &'static str {
    match update {
        EngineUpdate::Project { .. } => "Project Completed",
        EngineUpdate::Policy { .. } => "Policy Outcome",
        EngineUpdate::Region { up: true, .. } => "Region Developed",
        EngineUpdate::Region { up: false, .. } => "Region Contracted",
    }
}

fn image_for_update<'a>(update: &'a EngineUpdate, world: &'a World) -> (egui::Image<'a>, &'a str) {
    match update {
        EngineUpdate::Project { id } | EngineUpdate::Policy { id } => {
            let proj = &world.projects[id];
            let attrib = proj.flavor.image.attribution.as_str();
            (flavor_image(&proj.flavor.image), attrib)
        }
        EngineUpdate::Region { id, .. } => {
            let region = &world.regions[id];
            let attrib = region.flavor.image.attribution.as_str();
            (flavor_image(&region.flavor.image), attrib)
        }
    }
}

fn render_project_outcomes(ui: &mut egui::Ui, proj: &Project, state: &State) {
    let effects = active_effects(proj);
    render_effects(ui, state, &effects);
}

fn render_region_outcomes(ui: &mut egui::Ui, region: &Region, up: bool, world: &World) {
    let per_capita_demand = &world.per_capita_demand;

    let prev_tip = tip(icons::WEALTH, t!("This region's previous income level."));
    let next_tip = tip(icons::WEALTH, t!("This region's new income level."));

    let (next_income, prev_income, body) = if up {
        let next = region.income.level();
        let prev = next - 1;
        let body = t!(
            "This region's income level has increased to [b]%{income}[/b]. Demand for [i]%{iconElec}[/i]electricity, [i]%{iconFuel}[/i]fuel, [i]%{iconPCals}[/i]plant and [i]%{iconACals}[/i]animal-based food has been updated.",
            income = region.income.lower(),
            iconFuel = icons::FUEL,
            iconElec = icons::ELECTRICITY,
            iconPCals = icons::PLANT_CALORIES,
            iconACals = icons::ANIMAL_CALORIES
        );
        (next, prev, body)
    } else {
        let next = region.income.level();
        let prev = next + 1;
        let body = t!(
            "This region's income level has contracted to [b]%{income}[/b]. Demand for [i]%{iconElec}[/i]electricity, [i]%{iconFuel}[/i]fuel, [i]%{iconPCals}[/i]plant and [i]%{iconACals}[/i]animal-based food has been updated.",
            income = region.income.lower(),
            iconFuel = icons::FUEL,
            iconElec = icons::ELECTRICITY,
            iconPCals = icons::PLANT_CALORIES,
            iconACals = icons::ANIMAL_CALORIES
        );
        (next, prev, body)
    };

    let mut prev_region = region.clone();
    prev_region.set_income_level(prev_income);

    let demand = region.demand(per_capita_demand);
    let prev_demand = prev_region.demand(per_capita_demand);
    let pop = region.population;

    ui.label(body);

    ui.horizontal(|ui| {
        add_tip(
            prev_tip,
            ui.horizontal(|ui| {
                ui.add(icons::WEALTH.size(18.));
                ui.add(intensity_bar(prev_income + 1).invert());
            })
            .response,
        );
        ui.add(icons::ARROW_RIGHT_LIGHT.size(18.));
        add_tip(
            next_tip,
            ui.horizontal(|ui| {
                ui.add(icons::WEALTH.size(18.));
                ui.add(intensity_bar(next_income + 1).invert());
            })
            .response,
        );
    });

    for (output, demand) in demand.items() {
        let region_per_capita_demand = demand / pop;
        let intensity = intensity::output_intensity(region_per_capita_demand, output);
        let prev_region_per_capita_demand = prev_demand[output] / pop;
        let prev_intensity = intensity::output_intensity(prev_region_per_capita_demand, output);

        let prev_tip = tip(
            output.icon(),
            t!(
                "This region's previous demand for %{output}.",
                output = output.lower()
            ),
        );
        let next_tip = tip(
            output.icon(),
            t!(
                "This region's new demand for %{output}.",
                output = output.lower()
            ),
        );

        ui.horizontal(|ui| {
            add_tip(
                prev_tip,
                ui.horizontal(|ui| {
                    ui.add(output.icon().size(18.));
                    ui.add(intensity_bar(prev_intensity));
                })
                .response,
            );
            ui.add(icons::ARROW_RIGHT_LIGHT.size(18.));
            add_tip(
                next_tip,
                ui.horizontal(|ui| {
                    ui.add(output.icon().size(18.));
                    ui.add(intensity_bar(intensity));
                })
                .response,
            );
        });
    }
}
