import numpy as np
from PIL import Image

f = open('pixels.bin', 'r')
a = np.fromfile(f, dtype=np.uint8)
a = a.reshape((640, 1280, 3))
im = Image.fromarray(a, mode='RGB')
im.save('static_surface.png')