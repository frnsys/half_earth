use crate::vars::Impact;
use hes_engine::*;

pub trait DisplayValue {
    fn display(&self) -> String;
}

impl DisplayValue for Emissions {
    fn display(&self) -> String {
        emissions(self.as_gtco2eq())
    }
}

pub fn format_number(val: f32) -> String {
    if val >= 1e9 {
        format!("{:.1}b", val / 1e9)
    } else if val >= 1e6 {
        format!("{:.1}m", val / 1e6)
    } else if val >= 1e3 {
        format!("{:.1}k", val / 1e3)
    } else {
        format!("{:.1}", val)
    }
}

pub fn rounded(value: f32) -> String {
    if value == 0. {
        value.to_string()
    } else {
        let value = value.round();
        if value >= 0. && value < 1. {
            "<1".into()
        } else {
            value.to_string()
        }
    }
}

pub fn temp(tgav: f32) -> String {
    format!("{:+.1}C", tgav)
}

pub fn emissions(emissions_gt: f32) -> String {
    format!("{:.1}Gt", emissions_gt)
}

// From kWh to TWh
pub fn twh(amount: f32) -> f32 {
    (amount * 1e-9).round()
}

pub fn pwh(amount: f32) -> f32 {
    (amount * 1e-12).round()
}

/// Per 20000 Tcals
pub fn to_calorie_units(amount: f32) -> f32 {
    amount * (1e-9 / 2e4)
}

/// Per PWh
pub fn to_energy_units(amount: f32) -> f32 {
    amount * 1e-12
}

pub fn output(amount: f32, output: Output) -> f32 {
    match output {
        Output::Fuel | Output::Electricity => {
            to_energy_units(amount)
        }
        Output::PlantCalories | Output::AnimalCalories => {
            to_calorie_units(amount)
        }
    }
    .round_to(1)
}

pub fn resource(
    amount: f32,
    resource: Resource,
    available_resources: ResourceMap,
) -> f32 {
    let scale = match resource {
        Resource::Water => 100. / available_resources.water, // percent of available water
        Resource::Land => 100. / available_resources.land, // percent of habitable land
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

pub fn format_resource(
    amount: f32,
    res: Resource,
    available_resources: ResourceMap,
) -> String {
    let amount = resource(amount, res, available_resources);
    match res {
        Resource::Water | Resource::Land => {
            format!("{}%", amount)
        }
        _ => amount.to_string(),
    }
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

pub fn land_use_percent(m2: f32, available: f32) -> f32 {
    m2 / available * 100.
}

pub fn water_use_percent(l: f32, available: f32) -> f32 {
    l / available * 100.
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
    if percent.abs() < 1. && percent.abs() > 0. {
        if p < 0. {
            "-<1".to_string()
        } else {
            "<1".to_string()
        }
    } else if round {
        let percent = percent.round();
        if percent == 0. {
            "0".into()
        } else {
            format!("{:.0}", percent.round())
        }
    } else {
        format!("{:.1}", percent)
    }
}

pub fn signed_percent(p: f32, round: bool) -> String {
    let s = percent(p, round);
    if p > 0. {
        format!("+{s}")
    } else {
        s
    }
}

fn is_small(val: f32) -> bool {
    val < 1. && val > 0.
}

pub fn format_impact(
    impact: Impact,
    val: f32,
    available_resources: ResourceMap,
) -> String {
    match impact {
        Impact::Land => {
            if is_small(val) {
                "<1%".into()
            } else {
                format!(
                    "{}%",
                    percent(
                        land_use_percent(
                            val,
                            available_resources.land
                        ) / 100.,
                        true
                    )
                )
            }
        }
        Impact::Emissions => {
            let val = val * 1e-15;
            if is_small(val) {
                "<1Gt".into()
            } else {
                format!("{:.1}Gt", val)
            }
        }
        Impact::Water => {
            if is_small(val) {
                "<1%".into()
            } else {
                format!(
                    "{}%",
                    percent(
                        water_use_percent(
                            val,
                            available_resources.water
                        ) / 100.,
                        true
                    )
                )
            }
        }
        Impact::Biodiversity => {
            if is_small(val) {
                "<1".into()
            } else {
                format!("{:.0}", val)
            }
        }
        Impact::Energy => {
            let val = val * 1e-9;
            if is_small(val) {
                "<1TWh".into()
            } else {
                format!("{:.1}TWh", val * 1e-9)
            }
        }
        Impact::Fuel => {
            let val = val * 1e-9;
            if is_small(val) {
                "<1TWh".into()
            } else {
                format!("{:.1}TWh", val * 1e-9)
            }
        }
        Impact::Electricity => {
            let val = val * 1e-9;
            if is_small(val) {
                "<1TWh".into()
            } else {
                format!("{:.1}TWh", val * 1e-9)
            }
        }
    }
}

pub trait FloatExt {
    fn round_to(&self, precision: i32) -> f32;
}
impl FloatExt for f32 {
    fn round_to(&self, precision: i32) -> f32 {
        let factor = 10_f32.powi(precision);
        let abs_number = self.abs();
        let rounded = f32::round(abs_number * factor) / factor;
        if *self < 0.0 {
            -rounded
        } else {
            rounded
        }
    }
}
