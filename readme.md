# Misc notes

- Full screen plane: <https://stackoverflow.com/questions/63872740/three-js-scaling-a-plane-to-full-screen>
- Hex coordinates
    - <https://gamedev.stackexchange.com/questions/61032/how-do-i-get-pixel-to-hex-coordinates-on-an-array-based-hex-map>
    - <https://www.redblobgames.com/grids/hexagons/>
    - <http://www-cs-students.stanford.edu/~amitp/Articles/GridToHex.html>
    - <http://3dmdesign.com/development/hexmap-coordinates-the-easy-way>
    - <https://math.stackexchange.com/questions/247587/calculation-of-hexagon-coordinates-given-row-and-column>
- Force landscape on mobile: <https://stackoverflow.com/questions/14360581/force-landscape-orientation-mode>
- Hexasphere: important note: impossible to have a sphere fully covered by hexagons, there have to be some pentagons (see <https://en.wikipedia.org/wiki/Goldberg_polyhedron>)
    - <https://github.com/arscan/hexasphere.js>, seems like it runs chunky?
    - <https://shaderfrog.com/app/view/2977?view=shader>, runs quite smooth
    - <https://www.shadertoy.com/view/NdBSz1> this one might be better
    - <https://stackoverflow.com/questions/46777626/mathematically-producing-sphere-shaped-hexagonal-grid>
    - <https://github.com/search?q=hexasphere>

---

# Dev notes

## 6/30: Packing land use labels

Trying to figure out the best (lightest) way to load all the land label data. Two motivations: minimize network traffic and see if we can reasonably load higher-resolution data. Was using JSON for convenience, but it has a lot of excess because we really only need to represent values from 0-255 (since the labels are basically sparse grayscale images). For example, with the land use labels at 2560x1280, the JSON representation is 15MB (!), messagepack is 5.3MB, numpy (`.npy`) is 3.2MB, which as far as I can tell is just using `struct.pack`--when I use that (`struct.pack('xB', vals)`, where `x = 2560*1280`, i.e. the number of values (`len(vals)`), and `B` indicates we're using 1byte per value) I also get 3.2MB. I then made a naive compressed format which counts sequences of identical values such that `AAAAAABBB` is encoded as `A6B3`. The `A`s and `B`s are still encoded as 1 byte unsigned ints, and the counts are encoded as 4 byte unsigned ints (since they can be higher than 255). This looked like:

```
# where vals is a list of the labels
compressed = b''
seq = []
for v in vals:
    if not seq or seq[-1] == v:
        seq.append(v)
    else:
        # 1 byte
        compressed += struct.pack('1B', seq[-1])
        # uint (4 bytes)
        compressed += struct.pack('1I', len(seq))
        seq = []
if seq:
    # 1 byte
    compressed += struct.pack('1B', seq[-1])
    # uint (4 bytes)
    compressed += struct.pack('1I', len(seq))
with open('output/grid_landuse.raw.compressed', 'wb') as f:
    f.write(compressed)
```

This gets the file size down to 822KB. The PNG version however is only 406KB, and if you use `pngquant` to further compress it (`pngquant grid_landuse.png -o test.png`) it goes down to 219KB.

The only problem is I struggled to load the PNG in JS such that I could access the raw pixel values. I tried this:

```
function imageToUint8Array(src) {
  let canvas = document.createElement('canvas');
  let ctx = canvas.getContext('2d');
  return loadImage(src).then((image) => {
    ctx.width = image.width;
    ctx.height = image.height;
    ctx.drawImage(image, 0, 0);
    // Data is flat RGBA values
    // since the images we're working with is grayscale,
    // we only need the first of every four values
    let imageData = ctx.getImageData(0, 0, ctx.width, ctx.height);
    let labels = [];
    for (let i=0; i<imageData.data.length; i=i+4) {
      labels.push(imageData.data[i]);
    }
    return {
      labels,
      dims: [image.width, image.height]
    };
  });
}
```

which does give access to the pixel values of the rendered canvas but there's some stuff happening in the browser such that the values you get aren't exactly the same as the original image (see <https://stackoverflow.com/a/4464512>). [This JS PNG decoder](https://github.com/photopea/UPNG.js) decodes the raw PNG data directly with JS and returns the correct values, so I'm using that.