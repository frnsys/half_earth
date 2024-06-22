This produces an approximation of Whittaker's biome classification, which classifies biomes according to a given average global temperature and precipitation (see image below).

The following biomes are used:

- Tundra
- Temperate grassland/desert
- Subtropical desert
- Tropical seasonal forest/savanna
- Boreal forest
- Temperate seasonal forest
- Woodland/shrubland
- Temperate rain forest
- Tropical rain forest

## Usage

1. Run `01_convert.py` to generate the approximation. The results are:
    - A grayscale PNG (`out/biomes.png`) where each pixel is a biome label (where 0 = Tundra, 1 = Temperate grassland/desert, etc).
    - Include files for the Rust code (`out/biome_lookup.in`) which describe the min, max, and step sizes for the `x` (temperature) and `y` (precipitation) axes for translating a given temperature and precipitation to a 1d index for the PNG.

![](biomes.png)

If you want to preview how this lookup is applied, see `../scaling_patterns/03_preview.py` (the `preview_biomes_lookup.png` output).