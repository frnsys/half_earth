use crate::{state, state::GameState};
use hes_engine::kinds::Output;
use leptos::*;

use super::{Impact, OutputKind};

const BASE_WORLD_OUTLOOK: f32 = 20.;
const BASE_REGIONAL_OUTLOOK: f32 = 10.;
const BASE_REGIONAL_HABITABILITY: f32 = 10.;

pub enum Variable {
    Outlook,
    Extinction,
    Habitability,
    WorldOutlook,
    Warming,
}

fn impact_stops(key: Impact, kind: OutputKind) -> [f32; 4] {
    match key {
        Impact::Land => match kind {
            OutputKind::Energy => [0., 0.001, 0.01, 0.1],
            OutputKind::Calories => [0., 0.001, 0.002, 0.01],
        },
        Impact::Energy => match kind {
            OutputKind::Energy => [0., 0.001, 0.01, 0.1], // TODO EROI
            OutputKind::Calories => [0., 0.00015, 0.0005, 0.001],
        },
        Impact::Water => match kind {
            OutputKind::Energy => [0., 1., 2., 5.],
            OutputKind::Calories => [0., 1., 2., 3.],
        },
        Impact::Emissions => match kind {
            OutputKind::Energy => [-2000., 0., 200., 800.],
            OutputKind::Calories => [-1., 0., 0.5, 1.],
        },
        Impact::Biodiversity => match kind {
            OutputKind::Energy => [0., 1e-15, 1e-14, 1.5e-14],
            OutputKind::Calories => [0., 1e-16, 1e-15, 1e-14],
        },
        Impact::Electricity | Impact::Fuel => state!(|state, ui| {
            let output = key.as_output().expect("Checked they're valid outputs");
            state.world.demand_by_income_levels(output)
        })(),
    }
}

pub fn impact_intensity(val: f32, key: Impact, kind: OutputKind) -> usize {
    let stops = impact_stops(key, kind);
    stops
        .windows(2)
        .filter(|stops| val >= stops[0] && val < stops[1])
        .count()
}

fn output_stops(key: Output) -> [f32; 4] {
    state!(|state, ui| state.world.demand_by_income_levels(key))()
}

pub fn output_intensity(val: f32, key: Output) -> usize {
    let stops = output_stops(key);
    stops
        .windows(2)
        .filter(|stops| val >= stops[0] && val < stops[1])
        .count()
}

pub const N_PIPS: usize = 5;

pub fn color(mut intensity: usize, invert: bool) -> &'static str {
    if invert {
        intensity = N_PIPS - intensity;
    }
    if intensity <= 1 {
        "#2FE863"
    } else if intensity == 2 {
        "#FBC011"
    } else if intensity == 3 {
        "#f28435"
    } else {
        "#EF3838"
    }
}

pub fn describe(intensity: usize) -> &'static str {
    match intensity {
        0 => "Very Low",
        1 => "Low",
        2 => "Moderate",
        3 => "High",
        _ => "Very High",
    }
}

pub fn scale(val: f32, key: Variable) -> usize {
    let val = match key {
        Variable::Outlook => (val / BASE_REGIONAL_OUTLOOK * 4.).round().max(1.),
        Variable::Extinction => (val / 100. * 4.).round().max(0.),
        Variable::Habitability => (val / BASE_REGIONAL_HABITABILITY * 4.).round().max(0.),
        Variable::WorldOutlook => (val / (BASE_REGIONAL_OUTLOOK + BASE_WORLD_OUTLOOK) * 4.)
            .round()
            .max(1.),
        Variable::Warming => val.floor() + 1.,
    };
    val as usize
}
