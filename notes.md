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

## 7/19: Integrating the [Hector](https://github.com/JGCRI/hector) model

Would be really nice if it's possible to run Hector in the browser rather than setting it up as a web service.

One possibility is writing Rust bindings for hector, then compiling all the Rust code into a single WASM binary. I looked into [cxx](https://github.com/dtolnay/cxx) to help creating the bindings and referenced the [pyhector](https://github.com/openclimatedata/pyhector) code for the C++ wrapping into the Hector library (specifically the [`include/`](https://github.com/openclimatedata/pyhector/tree/master/include) and [`src/`](https://github.com/openclimatedata/pyhector/tree/master/src) directories and [setup.py](https://github.com/openclimatedata/pyhector/blob/master/setup.py)). I've only skimmed the Hector code but it looks like it uses the file I/O quite a bit and I worry that it will not properly work with WebAssembly ([based on the info here](https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html)). I'm also not sure about the compatibility of Boost with WASM. And in general I'm not certain of if Rust with a FFI compiles to WASM. Based on the comments [here](https://www.reddit.com/r/rust/comments/i8snc5/compiling_rust_library_with_c_dependencies_to_wasm/) and [here](https://www.reddit.com/r/rust/comments/8bnco7/including_external_c_library_in_web_assembly_rust/), it might work if the C code is also compiled directly to WASM. I think `cxx` might do this as part of [building with Cargo](https://cxx.rs/build/cargo.html) to the WASM target?

Update 7/20: It does not look like Rust + C combined will compile to WASM in most cases, see <https://github.com/rustwasm/team/issues/291>. I tried a minimal example with `cxx` and couldn't compile the Rust/C to WASM with `wasm-pack`, it looks like some headers couldn't be found:

```
running: "clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-Wall" "-Wextra" "-o" "/home/ftseng/work/half_earth/hector-rs/target/wasm32-unknown-unknown/release/build/cxx-542f93539571d0d9/out/src/cxx.o" "-c" "src/cxx.cc"
cargo:warning=In file included from src/cxx.cc:1:
cargo:warning=src/../include/cxx.h:2:10: fatal error: 'algorithm' file not found
cargo:warning=#include <algorithm>
cargo:warning=         ^~~~~~~~~~~
cargo:warning=1 error generated.
exit status: 1
```


Alternatively, perhaps Hector can be directly compiled to WASM (e.g. via [Emscripten](https://developer.mozilla.org/en-US/docs/WebAssembly/C_to_wasm), see [this question about compiling Boost with Emscripten](https://stackoverflow.com/questions/15724357/using-boost-with-emscripten)). In this case Javascript would be a glue between the rest of the Rust-WASM code and Hector. Ideally Hector would just be part of the Rust-WASM code, especially if other parts of the Rust code will be communicating with Hector frequently. And again, here with Emscripten I don't know if file system access complicates things (I'm assuming it does).

For the sake of time and feasibility it makes the most sense to use `pyhector` and set up a web service that game clients query for model output. It makes the infrastructure more complicated than just serving static assets. At a later point we can revisit integrating Hector directly into the frontend--especially because we're not totally sure it's even the model we want to use!

For one thing we want spatial/gridded version of Hector's output. As far as I can tell, Hector's output is not spatialized--just global averages. However [`hectorui`](https://github.com/JGCRI/hectorui/) can output gridded maps using Hector's output ([see the maps here](https://jgcri.shinyapps.io/HectorUI/)). [This code](https://github.com/JGCRI/hectorui/blob/229865a96f1676c65f9994ba1d8a453a32e65fd3/inst/shinyApp/output.r#L154) is how they do it--according to Drew, they're using output from more sophisticated models that describe variations from global averages (called "scaling patterns", as best as I can tell), which lets them translate the global average into cell values. The [`fldgen::pscl_apply`](https://rdrr.io/github/JGCRI/fldgen/man/pscl_apply.html) function does the actual translation to the cells. The patterns themselves [are available in the repo](https://github.com/JGCRI/hectorui/tree/main/inst/shinyApp/www/maps),. So we should just be able to port that code to Python and read the map files (`.rds` format) using [`rpy2`](https://github.com/rpy2/rpy2/). Strangely `hectorui` lets you view maps from 2000-2100 but the patterns are only for 2006-2100?

Different scaling patterns have different resolutions ("tas" is temperature, "pr" is precipitation).

- `GFDL-ESM2G` (tas) and `MRI-ESM1` (tas & pr): There are 51,200 cells so I'm pretty sure the maps are 320x160, with latitude ranging from -90 to 90 and longitude from 0 to 360, so each cell should be 1.125x1.125 (lon x lat) but in the latitude direction the coordinates don't exactly start and end on -90/90, but start/end on -89.14152/89.14152...and the lat size is inconsistent across cells. I don't know if this is something related to projections or not; the `hectorui` code doesn't seem to reference any projections.
    - `GFDL-ESM2G` (pr): has 12,960 cells, which also doesn't give round dimensions if assuming a 2:1 aspect ratio. Not sure why it's different than its temperature version.
- `CanESM2` (tas, pr): 128x64
- `CESM1-BGC` (tas & pr): has 55,296 cells, which assuming an aspect ratio of 2:1 doesn't give round dimensions
- `MPI-ESM-LR` (tas & pr): 192x96
- `MIROC-ESM` (tas & pr): 128x64

It seems like for higher resolution we should stick with the `MRI-ESM1` scaling patterns, though maybe there are other factors to consider. For comparison, the biome/land use labels are 480x240, though we can also scale them to 320x160 to simplify things.

_7/21 update_: Managed to get Hector compiling to WASM with Emscripten (see <https://github.com/frnsys/hector-wasm>), and it's just as fast as `pyhector`. I should look more closely at memory usage and file sizes though.

## 7/22: Trying to lower file sizes/load times

This is a little early in the process but I want to get a feel for where things are at and what I need to keep in mind moving forward.

For now only looking at the `dist/main.js` resulting from the production build command (`npm run build`). The `.map` files are large but only loaded when using dev tools.

Right now `dist/main.js` comes out at about 1.8MB. Commenting out imports to or that rely on `three.js` reduces this by ~500KB (from what I looked at I thought the version of `three.js` I'm using should allow for tree-shaking which should reduce this, but doesn't look like it is. Might be missing something...). Commenting out imports to or that rely on Vue reduces this by another ~300KB. The remaining 1MB is almost entirely from `hector-wasm` (~200KB of that is from the scenario import; that can probably be reduced). I think `three.js` and `hector-wasm` will be the only large dependencies; hopefully most of the other game logic can be implemented in Rust and so have a relatively small footprint.

This puts three.js's minified size at 613.2kb: <https://bundlephobia.com/package/three@0.131.3>

For `three.js`, one workaround might be this: <https://gist.github.com/drcmda/974f84240a329fa8a9ce04bbdaffc04d>, i.e. creating a proxy `three.js` file to manually export only the parts that are needed.

The inclusion of `src/globe/worker.js` creates a couple additional `.js` files (72KB total), and since it's what interacts with the Rust code also brings in that corresponding `.wasm` file (24KB).

In practice, `hector-wasm` will only be used by the worker, which spreads out the size a bit (920KB for `dist/main.js` and the worker scripts go up to 800KB total).

Adding async code to `src/globe/worker.js` caused some problems with Babel (`regeneratorRuntime is not defined`, probably because Babel wasn't processing that file) and I realized that we might not need Babel at all since [ES6 is supported by ~95% of browsers](https://caniuse.com/?search=es6). Removing Babel brought `dist/main.js` down to ~830KB, so almost another 100KB saved.

Using the `three.js` exports approach brought it down to ~715KB, so another ~115KB saved. Not as much as I'd hoped but still not bad.

For the `hector-wasm` integration I changed it so that the config and scenario data are loaded separately as async requests.

Total size for `dist/main.js` and the worker files (including the `.wasm`) is 1.532MB, original size for all these was 1.896MB, so overall savings of about 364KB.

After reconfiguring `hector-wasm` to have its `.wasm` file separate from the Javascript, the total file size of the workers went down to 112KB (so down 688KB). The total file size across `dist/main.js` and the workers is 828KB. A big chunk of those savings are from moving the `.wasm` file out of the JS--the total size of the `.wasm` files are now 464KB. So the total file size is still 1.292MB, but that was another 240KB savings.
