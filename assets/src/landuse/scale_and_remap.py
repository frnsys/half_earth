"""
IGBP Land Use Classification
<https://lpdaac.usgs.gov/documents/101/MCD12_User_Guide_V6.pdf>
1 Evergreen Needleleaf Forests
2 Evergreen Broadleaf Forests
3 Deciduous Needleleaf Forests
4 Deciduous Broadleaf Forests
5 Mixed Forests
6 Closed Shrublands
7 Open Shrublands
8 Woody Savannas
9 Savannas
10 Grasslands
11 Permanent Wetlands
12 Croplands
13 Urban and Built-up Lands
14 Cropland/Natural Vegetation Mosaics
15 Permanent Snow and Ice
16 Barren
17 Water Bodies
255 Unclassified
"""

# import json
# import struct
# import msgpack
import rasterio
import subprocess
import numpy as np
import matplotlib.pyplot as plt
from rasterio.enums import Resampling
from rasterio.plot import show, reshape_as_image
from PIL import Image

mosaic_tif = 'mosaic.tif'
target_width = 3840
target_height = target_width/2

sparse_remap = {
    1: 0,
    2: 0,
    3: 0,
    4: 0,
    5: 0,
    6: 0,
    7: 0,
    8: 0,
    9: 0,
    10: 0,
    11: 0,
    12: 0,
    13: 1,
    14: 0,
    15: 0,
    16: 0,
    17: 0,
    0: 0
}

def to_image(data, outpath, colormap=None, normalize=True):
    data = reshape_as_image(data)
    if data.dtype == np.uint8:
        fac = 1.
        alpha = 255.
    else:
        fac = 255.
        alpha = 1.

    # Normalize
    if normalize:
        mn = data.min(axis=(0, 1), keepdims=True)
        mx = data.max(axis=(0, 1), keepdims=True)
        data = (data - mn)/(mx - mn)
        fac = 255.
        alpha = 1.

    # Grayscale
    if data.shape[-1] == 1:
        # Use colormap, if specified
        if colormap is not None:
            cm = plt.get_cmap(colormap)
            data = cm(data)

        # Otherwise, just convert to RGB
        else:
            data = np.concatenate((data,)*3, axis=-1)

    # Convert to RGBA
    if data.shape[-1] == 3:
        data = np.dstack((data, np.full(data.shape[:2], alpha)))

    ndata = (data*fac).astype('uint8')
    im = Image.fromarray(ndata, 'RGBA')
    im.save(outpath)
    return im


def remap(dataset, mapping, outpath):
    # Remap land classification labels
    # so we only have cropland and urban/built-up development
    data = dataset.read()
    for key, val in mapping.items():
        data[data == key] = val

    # Save remapped geotiff
    kwargs = dataset.meta.copy()
    kwargs.update(meta)
    with rasterio.open(outpath, 'w', **kwargs) as dst:
        dst.write(data)

    return rasterio.open(outpath)



if __name__ == '__main__':
    with rasterio.open(mosaic_tif) as dataset:
        # print(dataset.width, dataset.height)
        # assert dataset.width/dataset.height == target_width/target_height

        scale = target_width/dataset.width

        # See <https://rasterio.readthedocs.io/en/latest/api/rasterio.enums.html#rasterio.enums.Resampling>
        # for resampling methods
        data = dataset.read(
            out_shape=(
                dataset.count,
                int(dataset.height * scale),
                int(dataset.width * scale)
            ),
            resampling=Resampling.nearest
        )

        # scale image transform
        transform = dataset.transform * dataset.transform.scale(
            (dataset.width / data.shape[-1]),
            (dataset.height / data.shape[-2])
        )

        meta = {
            'height': dataset.height * scale,
            'width': dataset.width * scale,
            'transform': transform
        }

        # Save scaled geotiff
        outname = '/tmp/mosaic_scaled.tif'
        kwargs = dataset.meta.copy()
        kwargs.update(meta)
        with rasterio.open(outname, 'w', **kwargs) as dst:
            dst.write(data)

        # Reopen scaled geotiff
        dataset = rasterio.open(outname)
        im = to_image(dataset.read(), '/tmp/mosaic_scaled.png')

        # Remap water to white (which is unclassified, which is mostly if not entirely water)
        remapped_dataset = remap(dataset, {17: 255}, '/tmp/grid_landuse.tif')
        im = to_image(remapped_dataset.read(), '/tmp/grid_landuse.png', normalize=False)
        subprocess.run(['pngquant', '/tmp/grid_landuse.png', '--speed', '1', '--force', '-o', 'out/grid_landuse.png'])

        # Compression experiments...see dev notes
        # raw_labels = remapped_dataset.read()[0]
        # grid = {
        #     'size': im.size,
        #     'labels': raw_labels[::-1, :].flatten().tolist(),
        # }
        # np.save('out/grid_landuse.npy', raw_labels)
        # with open('out/grid_landuse.json', 'w') as f:
        #     json.dump(grid, f)
        # with open('out/grid_landuse.msgpack', 'wb') as f: # about 1/3 of the size
        #     msgpack.pack(grid, f)

        # vals = raw_labels[::-1, :].flatten().tolist()
        # compressed = b''
        # seq = []
        # for v in vals:
        #     if not seq or seq[-1] == v:
        #         seq.append(v)
        #     else:
        #         # 1 byte
        #         compressed += struct.pack('1B', seq[-1])
        #         # uint (4 bytes)
        #         compressed += struct.pack('1I', len(seq))
        #         seq = []
        # if seq:
        #     # 1 byte
        #     compressed += struct.pack('1B', seq[-1])
        #     # uint (4 bytes)
        #     compressed += struct.pack('1I', len(seq))
        # with open('out/grid_landuse.raw.compressed', 'wb') as f:
        #     f.write(compressed)

        # Remap land classification labels
        # so we only have cropland and urban/built-up development
        # remapped_dataset = remap(dataset, sparse_remap, 'out/grid_landuse_human.tif')
        # im = to_image(remapped_dataset.read(), 'out/grid_landuse_human.png', normalize=False)

        # show(dataset, cmap='viridis')
