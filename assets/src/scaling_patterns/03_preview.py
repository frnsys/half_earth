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
dims = (320, 160, 4)


for k, pattern in patterns.items():
    scaled = pattern['w'] * (tgav + 15) + pattern['b']
    val_range = (scaled.min(), scaled.max())
    norm = [(v - val_range[0])/(val_range[1] - val_range[0]) for v in scaled.flatten()]
    img_data = np.array([list(colors[k](v)) for v in norm]).reshape(dims)
    img = Image.fromarray((img_data*255).astype('uint8'), 'RGBA')
    fname = 'out/preview_{}.png'.format(k)
    img.rotate(90, expand=1).resize((dims[0]*scale, dims[1]*scale), Image.NEAREST).save(fname)
    print('Preview saved to', fname)