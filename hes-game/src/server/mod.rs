pub mod sharing;

use hes_engine::surface::EarthSurface;
use rgb::ComponentBytes;
use std::sync::LazyLock;

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
