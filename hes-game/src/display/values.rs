use crate::{
    state::{STARTING_LAND, STARTING_WATER},
    vars::Impact,
};
use hes_engine::kinds::*;

pub fn format_number(val: f32) -> String {
    if (val >= 1e9) {
        format!("{:.1}b", val / 1e9)
    } else if (val >= 1e6) {
        format!("{:.1}m", val / 1e6)
    } else if (val >= 1e3) {
        format!("{:.1}k", val / 1e3)
    } else {
        format!("{:.1}", val)
    }
}

pub fn emissions(emissions_gt: f32) -> String {
    format!("{:.1}Gt", emissions_gt)
}

pub fn twh(amount: f32) -> f32 {
    (amount * 1e-9).round()
}

pub fn output(amount: f32, output: Output) -> f32 {
    let scale = match output {
        Output::Fuel => 1e-9 / 1e3, // per 1000 TWh
        Output::Electricity => 1e-9 / 1e3, // per 1000 TWh
        Output::PlantCalories => 1e-9 / 2e4, // per 20000 Tcals
        Output::AnimalCalories => 1e-9 / 2e4, // per 20000 Tcals
    };
    (amount * scale).round()
}

pub fn resource(amount: f32, resource: Resource) -> f32 {
    let total_land =
        STARTING_LAND.read().expect("Can read shared value");
    let scale = match resource {
        Resource::Water => 1e-12 / 50., // per 50 km3
        Resource::Land => 100. / *total_land, // percent of habitable land
        other => {
            if let Some(o) = other.as_output() {
                return output(amount, o);
            } else {
                panic!(
                    "No formatting defined for {:?}",
                    resource
                );
            }
        }
    };
    (amount * scale).round()
}

pub fn outputs(outputs: &OutputMap) -> OutputMap {
    OutputMap {
        fuel: output(outputs.fuel, Output::Fuel),
        electricity: output(
            outputs.electricity,
            Output::Electricity,
        ),
        plant_calories: output(
            outputs.plant_calories,
            Output::PlantCalories,
        ),
        animal_calories: output(
            outputs.animal_calories,
            Output::AnimalCalories,
        ),
    }
}

pub fn land_use_percent(m2: f32) -> f32 {
    let total_land =
        STARTING_LAND.read().expect("Can read shared value");
    m2 / *total_land * 100.
}

pub fn water_use_percent(l: f32) -> f32 {
    let total_water =
        STARTING_WATER.read().expect("Can read shared value");
    l / *total_water * 100.
}

pub fn demand_percent(
    demand: f32,
    total_demand: f32,
    round: bool,
) -> String {
    let mut total = total_demand;
    if total == 0. {
        total = 1.;
    }
    percent(demand / total, round)
}

pub fn percent(p: f32, round: bool) -> String {
    let percent = p * 100.;
    if percent < 1. && percent > 0. {
        "<1%".to_string()
    } else if round {
        format!("{:.0}%", percent.round())
    } else {
        format!("{:.1}%", percent)
    }
}

pub fn signed_percent(p: f32, round: bool) -> String {
    let percent = p * 100.;
    if percent < 1. && percent > 0. {
        "<1%".to_string()
    } else if round {
        format!("{:.0}%", percent.round())
    } else {
        format!("{:.1}%", percent)
    }
}

pub fn format_impact(impact: Impact, val: f32) -> String {
    match impact {
        Impact::Land => {
            percent(land_use_percent(val) / 100., true)
        }
        Impact::Emissions => format!("{:.1}Gt", val * 1e-15),
        Impact::Water => {
            percent(water_use_percent(val) / 100., true)
        }
        Impact::Biodiversity => format!("{:.0}", val),
        Impact::Energy => format!("{:.1}TWh", val * 1e-9),
        Impact::Fuel => format!("{:.1}TWh", val * 1e-9),
        Impact::Electricity => format!("{:.1}TWh", val * 1e-9),
    }
}
