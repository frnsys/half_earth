"""
Use this to generate the grid of biome labels to use
in the globe.
"""

import json
import numpy as np
import geopandas as gpd
from PIL import Image, ImageDraw
from shapely import affinity
from tqdm import tqdm

BIOMES = [
       'Tundra',
       'Tropical & Subtropical Moist Broadleaf Forests',
       'Mediterranean Forests, Woodlands & Scrub',
       'Deserts & Xeric Shrublands',
       'Temperate Grasslands, Savannas & Shrublands',
       'Boreal Forests/Taiga',
       'Temperate Conifer Forests',
       'Temperate Broadleaf & Mixed Forests',
       'Montane Grasslands & Shrublands',
       'Mangroves',
       'Flooded Grasslands & Savannas',
       'Tropical & Subtropical Grasslands, Savannas & Shrublands',
       'Tropical & Subtropical Dry Broadleaf Forests',
       'Tropical & Subtropical Coniferous Forests',
       'N/A' # Rock and Ice
]

bar_height_px = 100
width_px = 1280
height_px = int(width_px/2)

# epsg:4326
width, height = 360, 180
scale = width_px/width
assert width/height == width_px/height_px

im = Image.new('L', (width_px, height_px), color=255)
draw = ImageDraw.Draw(im)

gdf = gpd.read_file('Ecoregions2017/Ecoregions2017.shp')
for name, group in tqdm(gdf.groupby('BIOME_NAME')):
    idx = BIOMES.index(name)
    for _, item in group.iterrows():
        geotype = item['geometry'].type
        if geotype == 'MultiPolygon':
            polys = list(item['geometry'])
        elif geotype == 'Polygon':
            polys = [item['geometry']]
        else:
            raise Exception(f'Unhandled type: {geotype}')
        for poly in polys:
            pts = zip(*poly.exterior.xy)
            draw.polygon([
                (width_px/2 + x*scale, height_px/2 + y*scale)
                for x, y in pts
            ], fill=idx)

im.save('out/grid_biomes.png')

# Not as compact as PNG, see dev notes
# raw_labels = np.asarray(im) # 255 is water, everything else is an index for BIOMES
# with open('out/grid_biomes.json', 'w') as f:
#     json.dump({
#         'size': im.size,
#         'labels': raw_labels.flatten().tolist(),
#     }, f)