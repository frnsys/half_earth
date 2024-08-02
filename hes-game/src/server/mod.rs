pub mod sharing;

use crate::consts;
use hes_engine::surface::EarthSurface;
use leptos::*;
use rgb::ComponentBytes;
use std::sync::LazyLock;

/// Calculate new avg global temp.
/// Only compute up to the current year,
/// so the last returned tgav is the current tgav
/// Emissions are three-tuples of `(CO2, CH4, N2O)`.
pub fn compute_tgav(
    year: usize,
    annual_emissions: &[(f64, f64, f64)],
) -> f64 {
    use hector::{
        emissions::{get_emissions, START_YEAR},
        run_hector,
    };

    logging::log!("Computing TGAV...");

    let mut ffi_arr = vec![];
    let mut ch4_arr = vec![];
    let mut n2o_arr = vec![];
    for (co2, ch4, n2o) in annual_emissions.iter().cloned() {
        // Set an upper cap to the amount of emissions we pass to hector,
        // because very large numbers end up breaking it.
        let total_gtco2eq =
            (co2 + (ch4 * 36.) + (n2o * 298.)) * 1e-15;
        let emissions_factor =
            consts::MAX_EMISSIONS as f64 / total_gtco2eq;

        // Hector separates out FFI and LUC emissions
        // but we lump them together
        // Units: <https://github.com/JGCRI/hector/wiki/Hector-Units>
        let ffi = co2 * 12. / 44. * 1e-15 * emissions_factor; // Pg C/y
        let ch4 = ch4 * 1e-12 * emissions_factor; // Tg/y
        let n2o = n2o * 1e-12 * emissions_factor; // Tg/y

        ffi_arr.push(ffi);
        ch4_arr.push(ch4);
        n2o_arr.push(n2o);
    }

    let mut emissions = get_emissions(year);
    let nbox = emissions.get_mut("simpleNbox").unwrap();
    let n_years = nbox["ffi_emissions"].len();
    let idx = n_years - annual_emissions.len();
    nbox.get_mut("ffi_emissions")
        .unwrap()
        .splice(idx.., ffi_arr);

    emissions
        .get_mut("CH4")
        .unwrap()
        .get_mut("CH4_emissions")
        .unwrap()
        .splice(idx.., ch4_arr);
    emissions
        .get_mut("N2O")
        .unwrap()
        .get_mut("N2O_emissions")
        .unwrap()
        .splice(idx.., n2o_arr);

    let end_year = START_YEAR + n_years;

    logging::log!("> Running hector...");
    let tgav = unsafe { run_hector(end_year, &emissions) };
    logging::log!("> TGAV calculated: {tgav}.");
    tgav
}

pub static STARTING_SURFACE: LazyLock<EarthSurface> =
    LazyLock::new(|| {
        // A grayscale image where each value
        // indicates the label of that pixel
        let label_data = include_bytes!(
            "../../public/assets/surface/landuse.png"
        );

        // A grayscale image that maps
        // temp (x-axis) and precip (y-axis)
        // to a biome label.
        let lookup_data = include_bytes!(
            "../../public/assets/surface/biomes.png"
        );

        // Note that `decode32` always decodes to RGBA,
        // so we need to extract just the first value of each pixel.
        let biome_labels =
            lodepng::decode32(label_data).unwrap();
        let biome_lookup =
            lodepng::decode32(lookup_data).unwrap();

        let width = biome_labels.width;
        let height = biome_labels.height;

        // Extract just the first value of each RGBA pixel.
        let biome_labels: Vec<_> = biome_labels
            .buffer
            .as_bytes()
            .iter()
            .step_by(4)
            .copied()
            .collect();
        let biome_lookup: Vec<_> = biome_lookup
            .buffer
            .as_bytes()
            .iter()
            .step_by(4)
            .copied()
            .collect();

        let scale = 4;
        EarthSurface::new(
            biome_labels,
            width,
            height,
            scale,
            biome_lookup,
        )
    });
