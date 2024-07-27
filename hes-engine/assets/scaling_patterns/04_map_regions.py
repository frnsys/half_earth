"""
This creates a mapping of regions->indices in the scaling patterns,
so we know which cells refer to a given region.
"""
import json
from PIL import Image
from collections import defaultdict

colors = {
    '#C86500': 'Northern America',
    '#C80000': 'Western Africa',
    '#00C800': 'Western Europe',
    '#00ACC8': 'Western Asia',
    '#2E00C8': 'Eastern Asia',
    '#C8C700': 'Southern America',
    '#BB00C8': 'Central Asia',
    '#5600C8': 'Northern Europe',
    '#6DC800': 'Eastern Africa',
    '#C89600': 'South-eastern Asia',
    '#C80079': 'Central America',
    '#00C865': 'Northern Africa',
    '#1700C8': 'Southern Africa',
    '#FF45F9': 'Eastern Europe',
    '#4555FF': 'Australasia',
    '#B374FF': 'Central Africa',
    '#74FFFF': 'Southern Europe',
    '#FFC174': 'Southern Asia',
    '#A1FF74': 'Oceania',
    '#FFFF74': 'Caribbean'
}

assert len(colors) == 20

def rgb_to_hex(rgb):
    return '%02x%02x%02x' % rgb

grid_dim = (320, 160)

if __name__ == '__main__':
    world = Image.open('data/un_geoscheme_colored.png').convert('RGBA')
    width, height = world.size

    grid = Image.new(mode='RGBA', size=world.size)
    cell_width = int(width/grid_dim[0])
    cell_height = int(height/grid_dim[1])

    labels = defaultdict(list)

    for i in range(grid_dim[0]):
        x_ = i * cell_width
        for j in range(grid_dim[1]):
            y_ = j * cell_height
            pixels = defaultdict(int)
            for a in range(cell_width):
                for b in range(cell_height):
                    x = x_ + a
                    y = y_ + b
                    px = world.getpixel((x, y))
                    pixels[px] += 1
            max_px = max(pixels.keys(), key=lambda k: pixels[k])
            hex_px = '#{}'.format(rgb_to_hex(max_px[:3])).upper()
            p = pixels[max_px]/sum(pixels.values())
            if hex_px in colors and p >= 0.5:
                idx = i + j * grid_dim[0]
                region = colors[hex_px]
                labels[region].append(idx)

    for region, idxs in labels.items():
        assert len(idxs) > 0

    with open('out/regions_pscl.json', 'w') as f:
        json.dump(labels, f)