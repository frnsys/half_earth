import math
import numpy as np
import matplotlib.pyplot as plt
from PIL import Image

tgav = 2.55
patterns = {
    'temp': np.load('data/npz/tas_Amon_MRI-ESM1_esmrcp85_r1i1p1_200601-210012_pattern.npz'),
    'precip': np.load('data/npz/pr_Amon_MRI-ESM1_rcp85_r1i1p1_200601-210012_pattern.npz')
}
colors = {
    'temp': plt.get_cmap('RdYlBu_r'),
    'precip': plt.get_cmap('Purples')
}

scale = 3
dims = (160, 320, 4)

applied = {}
for k, pattern in patterns.items():
    scaled = pattern['w'] * (tgav + 15) + pattern['b']
    scaled = scaled.flatten()

    val_range = (scaled.min(), scaled.max())
    norm = [(v - val_range[0])/(val_range[1] - val_range[0]) for v in scaled]
    img_data = np.array([list(colors[k](v)) for v in norm]).reshape(dims)
    img = Image.fromarray((img_data*255).astype('uint8'), 'RGBA')
    fname = 'out/preview_{}.png'.format(k)
    img.resize((dims[1]*scale, dims[0]*scale), Image.NEAREST).save(fname)
    applied[k] = scaled
    print('Preview saved to', fname)

def clamp(v, mn, mx):
    return min(max(v, mn), mx)

# From `../biome_lookup/out/biome_lookup.in`
BIOME_SIZE = 40
BIOME_TEMP_MIN = -15.631337573638135
BIOME_TEMP_MAX = 29.984603468913555
BIOME_TEMP_STEP = 1.1696395139115818
BIOME_PRECIP_MIN = 0.0
BIOME_PRECIP_MAX = 444.15407527147266
BIOME_PRECIP_STEP = 11.388566032601863

biome_labels = np.array(Image.open('../landuse/out/grid_landuse.png')).flatten()
biome_lookup = np.array(Image.open('../biome_lookup/out/biomes.png')).flatten()
biome_colors = [
  (21,120,194),  # Water Bodies
  (200,247,142), # Croplands
  (201,225,244), # Tundra
  (106,196,106), # Temperate grassland/desert
  (234,171,68),  # Subtropical desert
  (185,232,118), # Tropical seasonal forest/savanna
  (10,120,70),   # Boreal forest
  (27,114,24),   # Temperate seasonal forest
  (127,171,98),  # Woodland/shrubland
  (55,172,81),   # Temperate rain forest
  (26,176,59),   # Tropical rain forest
]

img_data = {
    'biomes_lookup': [], # Biomes, post-Whittaker lookup
    'biomes_all': [],    # Biomes, post-Whittaker lookup, ignoring water
    'biomes_base': [],   # Base biomes, with no processing
    'biomes_temp': [],   # Clamped biome temp data
    'biomes_precip': [], # Clamped biome precip data
}

for label, temp, precip in zip(biome_labels, applied['temp'], applied['precip']):
    biome = label
    img_data['biomes_base'].append(biome_colors[biome])

    # In kg/m2/s, convert to cm/year
    # 1 kg/m2/s = 1 mm/s
    # 31536000 seconds per year, which yields mm/year
    precip_cm_year = precip * 31536000. / 10.

    temp_ = clamp(temp, BIOME_TEMP_MIN, BIOME_TEMP_MAX)
    precip_ = clamp(precip_cm_year, BIOME_PRECIP_MIN, BIOME_PRECIP_MAX)

    x = math.floor((temp_ - BIOME_TEMP_MIN) / BIOME_TEMP_STEP)
    y = math.floor((precip_ - BIOME_PRECIP_MIN) / BIOME_PRECIP_STEP)
    idx = y * BIOME_SIZE + x
    biome = biome_lookup[idx]
    color = biome_colors[biome]
    img_data['biomes_all'].append(color)

    if label != 0: # Skip water
        img_data['biomes'].append(color)
    else:
        color = biome_colors[label]
        img_data['biomes'].append(color)

    norm = (temp_ - BIOME_TEMP_MIN)/(BIOME_TEMP_MAX - BIOME_TEMP_MIN)
    r, g, b, _ = colors['temp'](norm)
    img_data['biomes_temp'].append((r*255,g*255,b*255))

    norm = (precip_ - BIOME_PRECIP_MIN)/(BIOME_PRECIP_MAX - BIOME_PRECIP_MIN)
    r, g, b, _ = colors['precip'](norm)
    img_data['biomes_precip'].append((r*255,g*255,b*255))


dims = (160, 320, 3)
for k in img_data.keys():
    data = np.array(img_data[k]).reshape(dims)
    img = Image.fromarray(data.astype('uint8'), 'RGB')
    fname = 'out/preview_{}.png'.format(k)
    img.resize((dims[1]*scale, dims[0]*scale), Image.NEAREST).save(fname)
    print('Preview saved to', fname)