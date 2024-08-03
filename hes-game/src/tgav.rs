use itertools::Itertools;
use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Deserialize)]
struct HectorShim {
    lag: usize,
    degree: usize,
    intercept: f32,
    coefficients: Vec<f32>,
    co2_hist: Vec<f32>,
    n2o_hist: Vec<f32>,
    ch4_hist: Vec<f32>,
}

static HECTOR: LazyLock<HectorShim> = LazyLock::new(|| {
    serde_json::from_str(include_str!(
        "../../hes-hector/params.json"
    ))
    .unwrap()
});

fn take_last(from: &[f32]) -> f32 {
    from[from.len() - 1]
}
fn take_lagged(from: &[f32], lag: usize) -> f32 {
    from[from.len() - 1 - lag]
}

fn polynomial_features(
    input: &[f32],
    degree: usize,
) -> Vec<f32> {
    let mut features = vec![1.0];
    for d in 1..=degree {
        for combination in
            (0..input.len()).combinations_with_replacement(d)
        {
            let mut product = 1.0;
            for &index in &combination {
                product *= input[index];
            }
            features.push(product);
        }
    }
    features
}

/// Calculate new avg global temp.
/// Emissions are three-tuples of `(CO2, CH4, N2O)`.
pub fn compute_tgav(
    annual_emissions: &[(f32, f32, f32)],
) -> f32 {
    let lag = HECTOR.lag;
    let mut co2_hist = HECTOR.co2_hist.clone();
    let mut n2o_hist = HECTOR.n2o_hist.clone();
    let mut ch4_hist = HECTOR.ch4_hist.clone();

    co2_hist.extend(annual_emissions.iter().map(
        |(v, _, _)| {
            v * 12. / 44. * 1e-15 // Pg C/y
        },
    ));
    n2o_hist.extend(annual_emissions.iter().map(
        |(_, _, v)| {
            v * 1e-12 // Tg/y
        },
    ));
    ch4_hist.extend(annual_emissions.iter().map(
        |(_, v, _)| {
            v * 1e-12 // Tg/y
        },
    ));

    leptos::logging::log!("{:?}", co2_hist);
    leptos::logging::log!("{:?}", n2o_hist);
    leptos::logging::log!("{:?}", ch4_hist);

    let input = vec![
        take_last(&co2_hist),
        take_last(&n2o_hist),
        take_last(&ch4_hist),
        take_lagged(&co2_hist, lag),
        take_lagged(&n2o_hist, lag),
        take_lagged(&ch4_hist, lag),
    ];
    leptos::logging::log!("{:?}", input);
    let coefs = &HECTOR.coefficients;
    let features = polynomial_features(&input, HECTOR.degree);
    assert_eq!(features.len(), coefs.len());

    let mut tgav = HECTOR.intercept;
    for i in 0..features.len() {
        tgav += features[i] * coefs[i];
    }
    tracing::debug!("Calculated tgav: {tgav}.");
    tgav
}
