import math
from PIL import Image

icons = ['wildfires', 'flood', 'resistance', 'power', 'heatwave']

for icon in icons:
    src_img = Image.open('out/{}.png'.format(icon))
    size = src_img.size

    img = Image.new('RGBA', src_img.size)
    scaled_size = (142, 142)
    icon_img = src_img.resize(scaled_size)
    img.paste(icon_img, (0, 0), icon_img)
    img.paste(icon_img,
            (size[0] - scaled_size[0]-1,
             size[1] - scaled_size[1]-1), icon_img)
    img.save('out/{}__2.png'.format(icon))

    img = Image.new('RGBA', src_img.size)
    scaled_size = (124, 124)
    padding = 8
    icon_img = src_img.resize(scaled_size)
    img.paste(icon_img,
            (0, size[1]-scaled_size[1]-padding), icon_img)
    img.paste(icon_img,
            (size[0]-scaled_size[0], size[1]-scaled_size[1]-padding), icon_img)
    img.paste(icon_img,
            (round(size[0]/2-scaled_size[0]/2), padding), icon_img)
    img.save('out/{}__3.png'.format(icon))