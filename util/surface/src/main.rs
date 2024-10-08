use hes_engine::World;
use rgb::ComponentBytes;
use std::collections::HashMap;

include!("../assets/scaling_patterns/out/scale_patterns.in");
include!("../assets/biome_lookup/out/biome_lookup.in");

type BiomeLabel = u8;

const STRIDE: usize = 3; // For r,g,b

// Set the radius to 1 if you need to debug
// the underlying biome labels with no effect
const RADIUS: usize = 3;
const INTENSITY: f32 = 25.;

pub const BASE_TEMP: f32 = 15.;

// Technically should be u8
// but we need larger numbers,
// which we later divide down to fit u8
type BigColor = (usize, usize, usize);
type Color = (u8, u8, u8);

// Biome colors
const COLORS: [Color; 11] = [
    (21, 120, 194),  // Water Bodies
    (200, 247, 142), // Croplands
    (201, 225, 244), // Tundra
    (106, 196, 106), // Temperate grassland/desert
    (234, 171, 68),  // Subtropical desert
    (185, 232, 118), // Tropical seasonal forest/savanna
    (10, 120, 70),   // Boreal forest
    (27, 114, 24),   // Temperate seasonal forest
    (127, 171, 98),  // Woodland/shrubland
    (55, 172, 81),   // Temperate rain forest
    (26, 176, 59),   // Tropical rain forest
];

#[derive(Clone)]
pub struct EarthSurface {
    width: usize,
    height: usize,
    scale: usize,
    biomes: Vec<BiomeLabel>,
    biome_lookup: Vec<BiomeLabel>,
    intensities: Vec<(BigColor, usize)>,
    pub pixels: Vec<u8>,
}

impl EarthSurface {
    pub fn new(
        biomes: Vec<BiomeLabel>,
        width: usize,
        height: usize,
        scale: usize,
        lookup: Vec<BiomeLabel>,
    ) -> EarthSurface {
        let mut pixels: Vec<u8> = biomes_to_pixels(&biomes);
        pixels = nearest_neighbor_scale(
            &pixels, width, height, scale,
        );
        let intensities =
            compute_intensities(&pixels).collect();

        // Assert they have the same number of values
        // (assumes they are the same aspect ratio)
        assert!(biomes.len() == TEMP_PATTERN_W.len());
        assert!(biomes.len() == TEMP_PATTERN_B.len());

        let w = width * scale;
        let h = height * scale;
        EarthSurface {
            biomes,
            pixels,
            scale,
            intensities,
            width: w,
            height: h,
            biome_lookup: lookup,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn update_surface(&mut self) {
        oil_paint_effect(
            &mut self.pixels,
            &self.intensities,
            self.width,
            self.height,
        );
    }

    pub fn update_biomes(&mut self, tgav: f32) {
        // Above we assert that TEMP_PATTERN_W, TEMP_PATTERN_B, and tgav are all the same size,
        // so no scaling necessary.
        // Add 15 to tgav to get actual temperature (this is what `hectorui` does).
        let global_temp = BASE_TEMP + tgav;
        for (idx, ((temp, precip), biome)) in apply_pscl(
            &TEMP_PATTERN_W,
            &TEMP_PATTERN_B,
            global_temp,
        )
        .zip(apply_pscl(
            &PRECIP_PATTERN_W,
            &PRECIP_PATTERN_B,
            global_temp,
        ))
        .zip(self.biomes.iter_mut())
        .enumerate()
        {
            // In kg/m2/s, convert to cm/year
            // 1 kg/m2/s = 1 mm/s
            // 31536000 seconds per year, which yields mm/year
            let precip_cm_year = precip * 31536000. / 10.;
            let label = biome_for_temp(
                biome,
                temp,
                precip_cm_year,
                &self.biome_lookup,
            );
            if *biome != label {
                *biome = label;
                let color = color_for_biome(label);
                let r = color.0 as usize;
                let g = color.1 as usize;
                let b = color.2 as usize;

                // Update intensities
                // Then you can run `update_surface()` to update the surface pixels
                let intensity = compute_intensity(r, g, b);
                for i in scaled_px_indices(
                    idx,
                    self.width / self.scale,
                    self.scale,
                ) {
                    self.intensities[i..i + self.scale]
                        .fill(((r, g, b), intensity));
                }
            }
        }
    }
}

// The biome changing logic
fn biome_for_temp(
    biome: &mut BiomeLabel,
    temp: f32,
    precip: f32,
    lookup: &[BiomeLabel],
) -> BiomeLabel {
    match *biome {
        0 => 0, // Water
        1 => 1, // Cropland,
        _ => {
            // Clamp to known range
            let temp_ =
                temp.clamp(BIOME_TEMP_MIN, BIOME_TEMP_MAX);
            let precip_ = precip
                .clamp(BIOME_PRECIP_MIN, BIOME_PRECIP_MAX);

            let x = ((temp_ - BIOME_TEMP_MIN) / BIOME_TEMP_STEP)
                .floor() as usize;
            let y = ((precip_ - BIOME_PRECIP_MIN)
                / BIOME_PRECIP_STEP)
                .floor() as usize;
            let idx = y * BIOME_SIZE + x;
            lookup[idx]
        }
    }
}

fn scale_idx(idx: usize, width: usize, scale: usize) -> usize {
    let scaled_width = width * scale;
    let x = (idx % width) * scale;
    let y = (idx / width) * scale;
    (y * scaled_width) + x
}

fn scaled_px_indices(
    idx: usize,
    width: usize,
    scale: usize,
) -> impl Iterator<Item = usize> {
    let scaled_idx = scale_idx(idx, width, scale);
    (0..scale).map(move |i| scaled_idx + (i * width * scale))
}

fn color_for_biome(label: u8) -> Color {
    COLORS[label as usize]
}

// Convert biome labels to RGB
fn biomes_to_pixels(biomes: &[u8]) -> Vec<u8> {
    let mut pixels: Vec<u8> =
        Vec::with_capacity(biomes.len() * STRIDE);
    for label in biomes {
        let (r, g, b) = color_for_biome(*label);
        pixels.push(r);
        pixels.push(g);
        pixels.push(b);
    }
    pixels
}

fn nearest_neighbor_scale(
    img: &[u8],
    width: usize,
    height: usize,
    scale: usize,
) -> Vec<u8> {
    let new_width = width * scale;
    let new_height = height * scale;
    let mut result: Vec<u8> =
        Vec::with_capacity(new_width * new_height * STRIDE);

    for i in 0..new_height {
        let i_ = i / scale;
        for j in 0..new_width {
            let j_ = j / scale;
            let idx_ = (i_ * width + j_) * STRIDE;
            result.push(img[idx_]);
            result.push(img[idx_ + 1]);
            result.push(img[idx_ + 2]);
        }
    }
    result
}

// Compute pixel intensities, for applying the oil paint effect
pub fn compute_intensities<'a>(
    img: &'a [u8],
) -> impl Iterator<Item = (BigColor, usize)> + 'a {
    img.chunks_exact(3).map(|rgb| {
        let r = rgb[0] as usize;
        let g = rgb[1] as usize;
        let b = rgb[2] as usize;
        ((r, g, b), compute_intensity(r, g, b))
    })
}

fn compute_intensity(r: usize, g: usize, b: usize) -> usize {
    let avg = (r + g + b) as f32 / 3.;
    ((avg * INTENSITY) / 255.).round() as usize
}

// Ported from <https://codepen.io/loktar00/pen/Fhzot>
pub fn oil_paint_effect(
    pixels: &mut [u8],
    intensities: &[(BigColor, usize)],
    width: usize,
    height: usize,
) {
    // For each pixel, get the most common intensity value of the neighbors in radius
    let mut pixel_intensity_count: Vec<(usize, BigColor)> =
        vec![(0, (0, 0, 0)); INTENSITY as usize + 1];
    for idx in 0..intensities.len() {
        pixel_intensity_count.fill((0, (0, 0, 0)));

        // Find intensities of nearest pixels within radius.
        let x = idx % width;
        let y = idx / width;
        let up_span = y.min(RADIUS); // rows to traverse up from idx
        let down_span = (height - y - 1).min(RADIUS); // rows to traverse down from idx
        let left_span = x.min(RADIUS); // rows to traverse left from idx
        let right_span = (width - x - 1).min(RADIUS); // rows to traverse right from idx
        let y_span = up_span + down_span + 1; // rows to traverse up and down, including idx
        let start_idx = idx - (up_span * width);

        for i in 0..y_span {
            let midpoint = start_idx + i * width;
            for (rgb, intensity_val) in &intensities
                [midpoint - left_span..midpoint + right_span]
            {
                let count =
                    &mut pixel_intensity_count[*intensity_val];

                count.0 += 1;
                count.1 .0 += rgb.0;
                count.1 .1 += rgb.1;
                count.1 .2 += rgb.2;
            }
        }

        // Max intensity value
        let top = pixel_intensity_count.iter().fold(
            (0, (0, 0, 0)),
            |acc, count| {
                if count.0 > acc.0 {
                    *count
                } else {
                    acc
                }
            },
        );

        let i = idx * STRIDE;
        pixels[i] = !!(top.1 .0 / top.0) as u8; // r
        pixels[i + 1] = !!(top.1 .1 / top.0) as u8; // g
        pixels[i + 2] = !!(top.1 .2 / top.0) as u8; // b
    }
}

/*
Applies tgav from Hector over a scaling pattern,
to spatialize temperatures to a grid.
This approach is what `hectorui` uses.

Jason Evanoff, Chris Vernon, Stephanie Pennington, & Robert Link. (2021, May 13). JGCRI/hectorui: v1.2.0 PNNL web feature (Version v1.2.0). Zenodo. http://doi.org/10.5281/zenodo.4758524

Ported from:
- <https://rdrr.io/github/JGCRI/fldgen/man/apply_pscl.html>
- <https://rdrr.io/github/JGCRI/fldgen/src/R/meanfield.R>

The original `apply_pscl` takes a vector for `tgav`, where each
value is the temperature anomaly for one year. We only need to
calculate one year at a time, so for simplicity this takes a single
value for `tgav`.

Important note: If using `temperature.Tgav` from Hector,
add 15 to it (the base temperature) before passing it here.
This is what they do in `hectorui`.
*/
pub fn apply_pscl<'a>(
    pscl_w: &'a [f32],
    pscl_b: &'a [f32],
    tgav: f32,
) -> impl Iterator<Item = f32> + 'a {
    pscl_w
        .iter()
        .zip(pscl_b)
        .map(move |(w_i, b_i)| w_i * tgav + b_i)
}

fn prepare_surface() -> EarthSurface {
    // A grayscale image where each value
    // indicates the label of that pixel
    let label_data =
        include_bytes!("../assets/surface/landuse.png");

    // A grayscale image that maps
    // temp (x-axis) and precip (y-axis)
    // to a biome label.
    let lookup_data =
        include_bytes!("../assets/surface/biomes.png");

    // Note that `decode32` always decodes to RGBA,
    // so we need to extract just the first value of each pixel.
    let biome_labels = lodepng::decode32(label_data).unwrap();
    let biome_lookup = lodepng::decode32(lookup_data).unwrap();

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
}

fn round_to(value: f32, precision: i32) -> f32 {
    let factor = 10_f32.powi(precision);
    let abs_number = value.abs();
    let rounded = f32::round(abs_number * factor) / factor;
    if value < 0.0 {
        -rounded
    } else {
        rounded
    }
}

fn main() {
    let world = World::default();
    let mut climates: HashMap<String, Vec<[f32; 4]>> =
        HashMap::default();
    let mut surface = prepare_surface();
    for i in -2..15 {
        for j in 0..10 {
            let tgav = round_to(i as f32 + j as f32 / 10., 1);
            surface.update_biomes(tgav);
            surface.update_surface();

            lodepng::encode24_file(
                format!(
                    "hes-game/public/assets/surface/for_temp/{tgav:.1}.png"
                ),
                &surface.pixels,
                surface.width,
                surface.height,
            )
            .unwrap();

            let temps: Vec<f32> = apply_pscl(
                &TEMP_PATTERN_W,
                &TEMP_PATTERN_B,
                BASE_TEMP + tgav,
            )
            .collect();
            let precips: Vec<f32> = apply_pscl(
                &PRECIP_PATTERN_W,
                &PRECIP_PATTERN_B,
                BASE_TEMP + tgav,
            )
            .collect();

            let mut climate = vec![];
            for region in world.regions.iter() {
                // We assert when generating the pattern idxs that they are not empty
                let local_temps: Vec<f32> = region
                    .pattern_idxs
                    .iter()
                    .map(|idx| &temps[*idx])
                    .cloned()
                    .collect();
                let local_precips: Vec<f32> = region
                    .pattern_idxs
                    .iter()
                    .map(|idx| &precips[*idx])
                    .cloned()
                    .collect();
                let temp_lo = local_temps
                    .iter()
                    .fold(f32::INFINITY, |a, &b| a.min(b));
                let temp_hi = local_temps
                    .iter()
                    .fold(-f32::INFINITY, |a, &b| a.max(b));

                // In kg/m2/s, convert to cm/year
                // 1 kg/m2/s = 1 mm/s
                // 31536000 seconds per year, which yields mm/year
                let mut precip_lo = local_precips
                    .iter()
                    .fold(f32::INFINITY, |a, &b| a.min(b));
                let mut precip_hi = local_precips
                    .iter()
                    .fold(-f32::INFINITY, |a, &b| a.max(b));
                precip_lo *= 31536000. / 10.;
                precip_hi *= 31536000. / 10.;

                climate.push([
                    round_to(temp_lo, 1),
                    round_to(temp_hi, 1),
                    round_to(precip_lo, 1),
                    round_to(precip_hi, 1),
                ]);
            }

            let key = format!("{tgav:.1}");
            climates.insert(key, climate);
        }
    }

    let path = "hes-engine/assets/climates.json";
    let data = serde_json::to_string(&climates).unwrap();
    std::fs::write(&path, data).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_apply_pscl() {
        let pscl_w: [f32; 6] = [0., 1., 0., 0.5, 1.0, 0.];
        let pscl_b: [f32; 6] = [-1., 1., 0., 0., 0.5, 0.5];
        let tgav = 8.;
        let expected = vec![-1., 9., 0., 4., 8.5, 0.5];
        let map: Vec<f32> =
            apply_pscl(&pscl_w, &pscl_b, tgav).collect();

        assert!(map.len() == expected.len());
        assert!(map.iter().zip(expected).all(
            |(x1, x2)| approx_eq!(f32, *x1, x2, epsilon = 1e-8)
        ))
    }

    #[test]
    fn test_nearest_neighbor_scale() {
        let img: [u8; 18] = [
            0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5,
            5,
        ];
        let width = 3;
        let height = 2;
        let scale = 2;
        let expected: [u8; 72] = [
            0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
            2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2,
            2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5, 5,
            5, 5, 5, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5,
            5, 5, 5, 5,
        ];
        let scaled =
            nearest_neighbor_scale(&img, width, height, scale);
        // println!("{:?}", scaled);

        assert!(scaled.len() == expected.len());
        assert!(scaled
            .iter()
            .zip(expected)
            .all(|(x1, x2)| *x1 == x2));
    }

    #[test]
    fn test_scale_idx() {
        let mut scale = 2;
        let mut width = 3;
        assert!(scale_idx(0, width, scale) == 0);
        assert!(scale_idx(1, width, scale) == 2);
        assert!(scale_idx(5, width, scale) == 16);
        assert!(scale_idx(7, width, scale) == 26);

        scale = 4;
        width = 3;
        assert!(scale_idx(0, width, scale) == 0);
        assert!(scale_idx(1, width, scale) == 4);
        assert!(scale_idx(2, width, scale) == 8);
        assert!(scale_idx(3, width, scale) == 48);
        assert!(scale_idx(4, width, scale) == 52);
        assert!(scale_idx(5, width, scale) == 56);
    }

    #[test]
    fn test_scaled_indices() {
        let scale = 3;
        let width = 2;
        let mut scaled_image = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];

        let idx = 3;
        for i in scaled_px_indices(idx, width, scale) {
            assert!(&scaled_image[i..i + scale] == &[1, 1, 1]);
            scaled_image[i..i + scale].fill(2);
        }

        let expected_image = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2,
            2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        assert!(expected_image == scaled_image);
    }
}
