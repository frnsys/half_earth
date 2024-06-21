"""
Pixelize Whittaker's biomes categorization
so we can have a faster lookup of (temp and precip)->biome.

Data is from <https://github.com/valentinitnelav/plotbiomes>
"""

import pyreadr
import subprocess
import numpy as np
import matplotlib.pyplot as plt
from shapely.geometry import Polygon, shape
from shapely.strtree import STRtree
from PIL import Image

x_var = 'temp_c'
y_var = 'precp_cm'

# Resolution of the approximation,
# higher is more fine-grained
n_squares_per_side = 40

result = pyreadr.read_r('src/plotbiomes/data/Whittaker_biomes.rda')
biomes = result['Whittaker_biomes']

# Color-coding
biome_names = biomes['biome'].unique()
biome_to_colors = {
    'Tundra': '#C1E1DD',
    'Temperate grassland/desert': '#FCD57A',
    'Subtropical desert': '#DCBB50',
    'Tropical seasonal forest/savanna': '#A09700',
    'Boreal forest': '#A5C790',
    'Temperate seasonal forest': '#97B669',
    'Woodland/shrubland': '#D16E3F',
    'Temperate rain forest': '#75A95E',
    'Tropical rain forest': '#317A22',
}

# Reserve:
# - 0 for water
# - 1 for cropland
biome_to_labels = {
    'Tundra': 2,
    'Temperate grassland/desert': 3,
    'Subtropical desert': 4,
    'Tropical seasonal forest/savanna': 5,
    'Boreal forest': 6,
    'Temperate seasonal forest': 7,
    'Woodland/shrubland': 8,
    'Temperate rain forest': 9,
    'Tropical rain forest': 10,
}
labels_to_biome = {v: k for k, v in biome_to_labels.items()}

# Build the biome temp/precip range polygons
polys = []
patches = []
for label, group in biomes.groupby('biome'):
    pts = group[[x_var, y_var]].values
    poly = Polygon(pts)

    # There is one weird shape,
    # this handles that
    if not poly.is_valid:
        poly = poly.buffer(0)

    poly.biome = label
    polys.append(poly)

    # Plot so we can see how good/bad our pixelation is
    patch,  = plt.fill(*poly.exterior.xy, biome_to_colors[label])
    patches.append((patch, label))

plt.legend(*zip(*patches))

# For quick querying of matching biome
tree = STRtree(polys)

# How to grid the space
x_min = biomes[x_var].min()
x_max = biomes[x_var].max()
y_min = biomes[y_var].min()
y_max = biomes[y_var].max()
x_step = (x_max - x_min)/(n_squares_per_side - 1)
y_step = (y_max - y_min)/(n_squares_per_side - 1)

mapping = []
for y_ in range(n_squares_per_side):
    y = y_min + y_ * y_step
    for x_ in range(n_squares_per_side):
        x = x_min + x_ * x_step
        pts = [(x, y), (x, y+y_step), (x+x_step, y+y_step), (x+x_step, y)]
        rect = Polygon(pts)
        cands = tree.query(rect)
        overlaps = [(cand, cand.intersection(rect).area) for cand in cands]
        overlaps = [o for o in overlaps if o[1] > 0]
        if not overlaps:
            mapping.append(None)
        elif len(overlaps) == 1:
            mapping.append(overlaps[0][0].biome)
        else:
            match = max(overlaps, key=lambda c: c[1])
            mapping.append(match[0].biome)
        if mapping[-1] is not None:
            plt.fill(*rect.exterior.xy, biome_to_colors[mapping[-1]],
                    alpha=0.5,
                    linewidth=0.25, edgecolor='#000000')
plt.xlabel(x_var)
plt.ylabel(y_var)
plt.show()
plt.close()

def lookup_biome(temp, precip):
    # Can either throw:
    # if temp > x_max or temp < x_min or precip > y_max or precip < y_min:
    #     raise Exception('Out of range')
    # Or clamp:
    temp = max(min(temp, x_max), x_min)
    precip = max(min(precip, y_max), y_min)

    x = (temp - x_min) // x_step
    y = (precip - y_min) // y_step
    idx = y * n_squares_per_side + x
    return mapping[int(idx)]

# print(lookup_biome(10, 100))
# print(lookup_biome(29, 0))

# 255 for None
labels = np.array([biome_to_labels[biome] if biome is not None else 255 for biome in mapping])
labels = np.reshape(labels, (n_squares_per_side, n_squares_per_side))


# Extend the biome labels to fill the entire image,
# because otherwise the "empty" (255) values are interpreted
# as water labels.
# Probably a nicer way of doing this...
for row_idx, row in enumerate(labels):
    valid_idxs = np.argwhere(row != 255).flatten()
    for idx, v in enumerate(row):
        if v == 255:
            # Fill with closest non-255 value.
            # First look to closest non-255 value in the same row and use that.
            if len(valid_idxs) > 0:
                closest_idx = min(valid_idxs, key=lambda i: abs(idx-i))
                row[idx] = row[closest_idx]

            # If the entire row is 255 values, then look to the closest non-255
            # value in the same column and use that.
            else:
                valid_col_idxs = np.argwhere(labels[:,idx] != 255).flatten()
                closest_idx = min(valid_col_idxs, key=lambda i: abs(row_idx-i))
                row[idx] = labels[:,idx][closest_idx]

# Show extended version
size = 20
for y, row in enumerate(labels):
    for x, label in enumerate(row):
        pts = [(x, y), (x, y+size), (x+size, y+size), (x+size, y)]
        rect = Polygon(pts)
        color = biome_to_colors[labels_to_biome[label]]
        plt.fill(*rect.exterior.xy, color,
                alpha=0.5,
                linewidth=0.25, edgecolor='#000000')
# plt.show()

im = Image.fromarray(labels.astype('uint8'), 'L')
im.save('out/biomes.png')

with open('out/biome_lookup.in', 'w') as f:
    size_rs = 'const BIOME_SIZE: usize = {};'.format(n_squares_per_side)
    x_min_rs = 'const BIOME_TEMP_MIN: f32 = {};'.format(x_min)
    x_max_rs = 'const BIOME_TEMP_MAX: f32 = {};'.format(x_max)
    x_step_rs = 'const BIOME_TEMP_STEP: f32 = {};'.format(x_step)
    y_min_rs = 'const BIOME_PRECIP_MIN: f32 = {};'.format(y_min)
    y_max_rs = 'const BIOME_PRECIP_MAX: f32 = {};'.format(y_max)
    y_step_rs = 'const BIOME_PRECIP_STEP: f32 = {};'.format(y_step)
    f.write('\n'.join([
        size_rs, x_min_rs, x_max_rs, x_step_rs, y_min_rs, y_max_rs, y_step_rs]))