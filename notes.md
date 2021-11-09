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

## 8/24: Debugging Rust WASM error

When testing biome changes over time, I got this error in the JS console:

```
RuntimeError: unreachable executed 6ed81d2d4eee6340b340.module.wasm:15536:1
```

Soon followed by:

```
Error: recursive use of an object detected which would lead to unsafe aliasing in rust src_earth_worker_js.js:122:11
    __wbindgen_throw http://localhost:8080/dist/src_earth_worker_js.js:122
    <anonymous> http://localhost:8080/dist/6ed81d2d4eee6340b340.module.wasm:20261
    <anonymous> http://localhost:8080/dist/6ed81d2d4eee6340b340.module.wasm:20288
    <anonymous> http://localhost:8080/dist/6ed81d2d4eee6340b340.module.wasm:19229
    <anonymous> http://localhost:8080/dist/6ed81d2d4eee6340b340.module.wasm:18411
    update_surface http://localhost:8080/dist/src_earth_worker_js.js:104
    updateTexture http://localhost:8080/dist/src_earth_worker_js.js:1122
    prepare http://localhost:8080/dist/src_earth_worker_js.js:958
```

For `src_earth_worker_js.js`:

Line 122:
```
function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
```

Line 104:
```
update_surface() {
    _half_earth_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.earthsurface_update_surface(this.ptr);
}
```

Line 1122:
```
updateTexture() {
    this._surface.update_surface();

    // Update the (shared) array buffer
    // so the main thread has it
    this.pixels.set(this._pixels);
  }
```

Line 958:
```
  // Call a method on the wrapped class instance.
  // This can handle async methods as well
  case TYPE.CALL: {
    const {id, key, args} = data;
    let ret = instances[id][key](...args);
    Promise.resolve(ret).then((ret) => {
      postMessage({reqId, resp: ret});
    });
  } break;
}
```

So something in the rust `update_surface` method, which basically just wraps `oil_paint_effect`.

Looked at these threads:
- <https://github.com/rustwasm/wasm-bindgen/issues/1578>
- <https://github.com/rustwasm/wasm-bindgen/issues/2486>

Issue 2486 suggests setting the panic hook, which gave a Rust traceback:

```
panicked at 'index out of bounds: the len is 1600 but the index is 1640', src/surface.rs:145:9
```

The biome lookup table is 40x40 (hence len 1600 when flattened), so this is probably an indexing flub.

Update: yeah I was calculating the biome lookup indices incorrectly. Fixed now

## Designing and implementing core game systems

### 9/22

We've sketched out the events, projects, and policies available to the player, so now I have a better sense of what the core game systems need to support. Diving into the detailed design and implementation of the core game systems. The challenge is designing something that is not overly complicated (and thus more likely to be robust, performant, and scrutable) while also capturing enough detail and richness to support the variety of user-facing mechanics, events, etc that we want to include.

- Robust: unlikely to crash or encounter ambiguous/undefined behavior.
- Performant: needs to be lightweight enough in terms of memory and computation that it will run well in browsers.
- Scrutable: the systems' outcomes and workings should make sense to the player and there should be some level of predictability/consistency.

I initially started with implementing the resource system, with the idea that there would be two tiers of resources: raw and refined. Raw resources are those that are "environmental", things like wind, water, coal, oil, sunlight, etc and refined resources are the processed and combined commodities produced from them, such as food, energy, and materials. At first I tried to add a fair amount of detail, e.g. specific types of materials like concrete and steel rather than "materials" as a broader category. Each refined material has several possible "processes" for producing it (e.g. energy can be produced with coal, or coal with CCS, or PV solar, etc), and processes are differentiated by their raw and/or refined resource requirements (e.g. materials require both minerals, a raw resource, and energy, a refined resource) and byproducts (e.g. coal emits more CO2 than coal with CCS). The player chooses a mix between these processes and then we calculate how much of each resource is produced, based on what raw/refined resources are available.

I spent quite a bit of time on this approach, but in the end it just felt too heavy and overly detailed. The sweet spot for this kind of design should feel like Legos: simplified and blocky but sturdy and expressive.

Part of this is that we want to spare players from as many meticulous management mechanics as possible. This game is meant to be a relatively light commitment, with sessions of no longer than 10-15 minutes. Deciding mixes between production processes is perhaps too much detail. Deciding what processes are on the table, and then having an auto-mixing system, feels more Lego-like.

So I scratched that initial resources work and started working on this auto-mixing system. I first set up resource production as a linear programming problem--given a set of available input resources (now just called "resources") and a mix of production processes, what's the maximum yield of output resources (now called "sectors")? Similarly, I set up another linear programming problem for finding the optimum mix of processes: given available resources, what mix of processes yields the most sector output? The player then just decides what new processes to research and which ones to ban, or sets resource constraints (e.g. no more than 50% of land in use), or tries to influence demand (e.g. hard quotas or education campaigns), and the ideal mix is determined against those constraints. This auto-mixing then represents the people of the world rationally and democratically deciding what processes to use on their own. In general this is maybe more reflective of a planner's experience--not micromanaging every detail but setting high-level targets that others then figure out the specifics of. It's also feels truer to the idea of Gosplant as a planning algorithm.

Because timing is so important to the game--e.g. starting decarbonization now as opposed to 10 years into the future makes a huge difference--changes in process mixes can't happen instantaneously. They take time--the infrastructure of the old processes needs to be properly decommissioned, the infrastructure for the new processes needs to be built, people need new training, etc. So the ideal mix is really a target mix that gradually comes into realization.

One tricky thing here is that sectors have many different features which require special consideration. For example: energy can vary in intermittency and portability. So we need some way to capture that. Right now I have them added as separate fields but I'm not using them yet, nor am I sure how they'll be used.

The part that I spent today working on is the spatial resources system. We need sector activity to be spatially distributed because it's important for visualization and events. For example, if a hurricane hits a certain region, we want to know what sectors are affected (e.g. reduced output). Resources are spatially distributed in the real world, and that is one main factor in how sectors site their activities.

One consideration is the spatial resolution. Elsewhere in the game the world is discretized into a 320x160 grid (51,200 cells). Seems reasonable to use that here too. It's pretty low resolution but I'm worried about slow performance at higher resolutions. The current approach is that each cell has a set of resources (hopefully we can use resource survey data to initialize these values) and sectors can lay claim to the resources in a cell. Because it's so low resolution I set it so that each cell has three "user slots", i.e. each sector can lay claim to one or more slots for a cell. This roughly triples the resolution without increasing the number of cells.

Then the question was: how do sectors decide what cells to claim? The simplest approach is that they compute the production potential of a cell (how many units output can they produce given that cell's resources) and choose the cells that maximize that production potential. We don't want to search over all 51,200 cells, so instead we have an index mapping resources to cell indices, and we take the set intersection of all of the required resources' cells to drastically reduce the search space.

After some more thought, I think the three user slots idea just complicates things. In practice, if sectors are greedily claiming the cells/slots that are most productive for them, then I anticipate that all user slots of a given cell will tend to be claimed by a single sector anyway. So additional complexity with nothing to show for it. If there's just one sector per cell then I don't have to deal with arrays (e.g. instead of `users: [Sector; 3]` I can just do `user: Sector` as a field on `Cell`). Also, one user per cell might still capture enough detail to be Lego-like, but we'll have to see in playtesting.

The other detail here is that, like mix transition, resource claiming is not instantaneous. Resource extraction infrastructure needs to be built-up or decommissioned, so there is a delay before a resource claim registers as available resources for production.

Other dynamics I hope to eventually capture: new discovery of resources, resource depletion, and maybe seasonal variation?

### 9/23

The sectors included:

```
Agriculture,
Materials,
Transportation,
Energy,
Environment,
Health,
Housing,
Water
```

I think `Housing`, `Transportation`, and `Environment` should be moved. `Housing` and `Transportation` because they are both services/goods that must be consumed where they are produced, which means the need to be sited not only where there are the necessary resources but where the consumers are. The resource claiming system above doesn't handle that and I think it will be too much to add it in. Their outputs also contribute to stocks more than being consumed in the proper sense. It should also be relatively easy to just incorporate them as part of the general per-capita-resource-intensity which is meant to be a catch-all for everything that isn't explicitly represented as a sector.

`Environment` should probably be removed because there isn't really a "product" that's being produced so it doesn't quite match everything else. What I saw going into it feels more like specific "projects" like SRM or rewilding.

`Health` should maybe also be removed--I'm not sure how that's measured, and it seems more easily captured by per-capita-resource-intensity. We also haven't really come up with any health-related policies, so it's not really within the scope of the game.

`Water` could also be removed because it's not "produced" in the same way as agriculture and materials and energy. This can be part of the separate per-capita resource intensity as well.

In fact, that extra per-capita resource intensity should be added here as "Other".

The other thing that is a little complicated is that here I'm generally using `Agriculture` to refer to food production, but of course it's also what produces many materials (e.g. cotton) and energy (e.g. biofuels).

```
Agriculture,
Materials,
Energy,
Other
```

Maybe these shouldn't really be called "sectors" anymore but something else. "Refined resources", like before?

With all those other sectors reduced, I feel there's more space to add in detail to the parts that remain. We could do something like:

```
Meat
NonMeat (need better name)
PortableEnergy
ImmobileEnergy
EnergyStorage
```

Here each sector would have an set of output types (rather than just one), e.g. food/agriculture would have meat calories and plant calories.

Something I'm not sure about is at what level resources should be managed by. Originally I had them globally--developed resources were dumped into a pool and the planner allocated them in a way to minimize production deficits (against demand) across all sectors. I'm worried that the planner could produce skewed outputs in times of resource deficits--making a lot of one sector's output because it has a lower resource requirement than another equally important sector. Like if we wanted to make sandwiches and salads and salads have a lower resource intensity overall. If there's not enough to fill salad demand, the only salads get made because you can make more salads than sandwiches for the same amount of resources. One way around this is to optimize the product of demand fulfillment, e.g. if we want 100 sandwiches and 100 salads, and `n_sandwich` and `n_salad` are the amount of sandwiches and salads the planner producers, we want to maximize `n_sandwiches/100 * n_salads/100`. However, planner can only optimize linear equations, so it can't solve this.

I changed it so that resources are managed on a per-sector level; this way e.g. materials aren't produced at the sacrifice of food. A lot of hand-waving is going on here. In reality many industries have overlapping resource requirements. The example most relevant here is biofuel (energy) and food (agriculture). So in theory land that is currently allocated to the food sector could go to the energy sector. But with this approach that land is "locked in" to the food sector until it goes through the resource transition process outlined above (i.e. decommissioning then building up again). In some cases the resources are fungible to the point where changing its use is just a matter of shipping it to a different factory, so this transition could happen relatively quickly across sectors. If one month there's a sudden shortfall of cars, we can redirect steel from other industries there without needing to (re)build whole facilities to do so. From this perspective, dumping everything into a shared resource pool and optimizing across all sectors (the previous approach) looks appealing.

However, I hope that the sectors are divided here such that this resource overlap is minimized. The car steel example would probably just all be mixed together into the `Materials` or `Other` sector; it all takes place in a black box and we leave it to the player's imagination that these things work out as they should. The food/biofuel example is still there, but you can't change what you're growing on a dime (i.e. a 6 month delay in changing what you're growing is plausible), so the transition delay actually works there.

The other possibility is to track resources on a per-process basis, rather than per-sector. This means we don't even need linear programming for resource allocation; the amount produced by a process is straightforward (`resource requirements per output/resources available`). What does get tricky is the hand off of resources between processes. Say we are spinning down coal power for coal power with CCS. These are two separate processes, so coal power w/o CCS would have to decommission its existing resources so that coal power w/ CCS could then (re)develop them. In this case this wouldn't really happen--each take the same coal input, so the whole decommissioning/building is totally unnecessary. While the CCS systems would take time to be installed, that time is already represented in the mix transition delay (see above). Maybe this is ok--like the biofuel/food example, perhaps this additional delay still feels plausible. The per-sector approach avoids this because the coal is claimed across the whole sector and is automatically redirected to coal power w/ CCS process by the planner. The per-sector approach should also roughly respect the process mix shares because each process can only match a portion of the sector's aggregate demand equivalent to its mix share.

So between per-sector and per-process, I'm not totally sure. I will probably start with per-sector just because it's simpler.

Other things that are hand-wavy:

- Ignoring some spatial relationships, like shipping/logistics in how resources are transported. These impacts and resource requirements could maybe be factored into the `Other` category. Alternatively, we could compute production at a regional level, calculate the surpluses and deficits for each region, figure out what needs to go where, and use that to calculate shipping energy requirements.
- Exclusive rights to a resource cell by a single process. As noted above this is mostly an issue because of the low resolution of the resource grid. It's very likely that resources in one cell are useful to multiple sectors at once, in a non-overlapping way (e.g. coal useful to the energy sector and lumber useful to the materials sector in one cell). However, only one sector can claim those resources at a time. Sectors might be broad enough that this might not be a problem in most cases.
    - Alternatively, resources are claimed not at the cell level but at the cell-resource level. Maybe this could work?

Hopefully even with all these simplifications, in the aggregate it feels plausible to the player and communicates what we want to show with the game.

Coming back to the question of more detailed sectors, e.g. representing energy intermittency/portability...

Intermittency could maybe just be captured by capacity factor? Like if PV solar outputs 1MWh per X resources, and it has a capacity factor of 25%, then it in fact only produces 0.25MWh per X resources? Is that already factored into PV solar resource intensity estimates? How do we represent energy storage (batteries, hydro, EV networks, etc)? These aren't easily added as energy production processes because the amount of energy they "produce" depends on how much energy other processes are actually producing. Maybe it's ok to cheat and treat batteries as if they do "produce" energy. Or we just take for granted that storage systems are built as part of separate intermittent energy processes ("batteries included"). So, for example, part of the resource intensity of solar PV accounts for the amount of storage that would be required to smooth its output.

So for energy processes we'd have a special property: `intermittent`. If the player researches new battery technology, all energy processes marked `intermittent` will have their resource intensity lowered by some amount.

Portable power, on the other hand, seems harder to figure out. A combination of batteries and electrification should make portable and non-portable power (which, I guess, is synonymous with electricity here) equivalent. So perhaps a widespread electrification project shifts demand for portable power to non-portable power. Or you could pursue hydrogen as an alternative to fossil fuels. [Update: the schema below might take care of this]

As for food/agriculture and plant calories vs meat calories; a special property `meat` might be enough. Some % of meat producing processes will make up the overall food mix; the player can ban those to stop meat production. *However* do we want to keep track of plant vs meat demand? I'm thinking of the case where a vegan mandate is introduced while there is still high demand for meat vs when there is reduced demand.

Could have a schema like:

```
trait Output {}

enum Calories {
    Plant,
    Meat
}
impl Output for Calories {}

// Also: does "Heat" need to be here? For industrial processes?
enum Energy {
    Portable, // or Fuel
    Immobile  // or Electricity
}

impl Output for Energy {}

struct Process<O: Output> {
    // ...
    output: O
    // ...
}
```

Then we have to track demand for each output type and variant.

Aside on the case of Projects: Projects also have resource requirements for their construction/implementation and active phases. This could also be represented as a sector:

```
Agriculture,
Materials,
Energy,
Other,
Projects
```

I'm not quite ready to start working on Projects yet but with this overlap I have to keep them in mind.

So this is the hierarchy that is starting to emerge:

- Sector: encompasses a set of roughly fungible outputs. Agriculture outputs calories in either plant or meta (almost totally fungible), energy outputs energy in fuel or electricity (fungible under certain conditions), materials outputs steel, concrete, plastics (maybe, haven't decided), also roughly fungible.
    - Sectors also manage a mix of production processes for its outputs
    - Sector outputs represent the different variants of its overall output, e.g. plant vs meat calories
    - When sectors update their process mixes, they respect the demand mix for the output variants, e.g. if people want 60% of their calories from plants and 40% from meat, then updating the mix will preserve that balance and only change process mixes within the same variant.
    - Demand is on a output variant basis

More on resource cells:

A problem I've run into is how to deduct resources from cells. If a sector uses 1000 of resource A in this production step, how do I distribute those uses across that sector's cells? Maybe we need to keep track of two things for each cell-resource: a yield rate and a stock size. The stock size is irrelevant for renewable resources. The stock size is a natural feature of a cell; we can use estimates from resource surveys (I hope) to fill that in. As for yields--that's dependent on a lot of things.

One possibility: a resource is not just developed or undeveloped; it has levels of development which correspond to yield rates (against the total stock there). When expanding resources, sectors can intensify the exploitation of its existing cell-resources for higher yields. I'm not sure yet how to capture the implications of that--should be higher energy usage, more pollution, etc for the processes that use those resources. But how exactly do we code that in? Update: now that I think about it, the additional impact should already be factored in. More of the resource means more of the output is created, and the impacts are calculated by how much of the output is created. The place where this would be a problem if additional resources are extracted but don't go into new output (e.g. each unit output requires 1 of resource A, increased exploitation yields an additional 0.5; so not enough to create a new output. Thus the impacts aren't added)

Part of what makes this complicated is that there are several resources per cell and we have to juggle which of a cell's resources are actually being used by a sector. It might be better to make it so that a cell can be exploited for only one of its resources at a time. E.g. a cell is developed for its coal resource and then its land, water, etc become unavailable.

Also need to distinguish between produced/refined resources (energy, food/feed, materials); the refined ones maybe need to be specified separately as resource requirements.

Two other resources that need to be distinguished are labor and land. Land doesn't fit the above schema of only one resource being exploited because everything requires land. So we need to figure out a better way of incorporating land use. Labor is also used everywhere. Here I think it should be treated more like the refined resources.

The challenge with the refined resources is how to allocate them across sectors? The planner handles allocation within sectors but, unlike the raw/natural resources, sectors don't lay claim to them in the same way.

Now I'm flip-flopping back to having the planner optimize over all sectors at once, rather than per-sector. That way we don't need to worry about how to allocate the refined resources; it happens automatically. And it simplifies some other aspects of resources in that we just dump resources into a general pool. I like all that--the main concern is an objective function that reasonably balances output across sectors. That might not even happen because each sector has relatively unique resource requirements. How this works out in practice may only be known after testing.

So notes for when I return:

- make it so only one of a cell's resources can be exploited at a time
    - this makes deductions easier
    - when a sector claims a cell it chooses what resource it will develop
- for expansion/contraction, we can do it more easily on a per-resource level.
    - group cells by the current resource that's being exploited from them
    - iterate over the surplus/deficit resources, scale up and down as needed
- land use?
- distinguish refined from raw resources
    - refined resources from the previous step are added onto raw resources for the current step as inputs
    - how are the refined resources distributed?
- return to the global resource pool

#### 9/24

I spent a long time trying to figure out how to optimize the production plan over a global resource pool.

As described above, the goal is to avoid the problem where e.g. 100% of output A is produced at the expense of any of output B. This does come up sometimes but I haven't yet tested with real data, so it's hard to say if this problem will occur in practice.

The problem formulation that would avoid this is:

Maximize `prod(p.amount_produced/p.amound_demanded for p in P)` where `P` are the active production processes, with the constraints that `p.amount_produced <= p.amount_demanded` (and `p.amount_produced >= 0`) and, for a given production process, `every(sum(p.resources_required[r] for p in P) < available_resources[r] for r in resources)`, that is, the total amount of resources used across all processes has to be within the available resources limits.

For example, if 600 plant calories were demanded and 400 tons of materials were demanded, say `p_p` and `p_m` are the amount of plant calories and tons of materials produced by the planner. We want to maximize `(p_p/600) * (p_m/400)`. If plant calories has the resource requirements of `[0.5, 0.2]` (assuming we only have two resource types A and B) and materials have the resource requirements of `[0.1, 0.3]`, then we also have the constraints `0.5*p_p + 0.1*p_m <= r_A` and `0.2*p_p + 0.3*p_m <= r_B` where `r_A, r_B` are the available amounts of resource A and B.

Overall there were three challenges in finding a package that could solve this:

1. Solve the problem as specified (i.e. nonlinear)
2. With the required constraints (total resource usage has to be less than the available amount of each resource)
3. Needs to compile to WASM

I tried:

- [`good_lp`](https://github.com/rust-or/good_lp/), which is the linear solver that I started with. It has a really good API, way better compared to most other optimization packages, and supports pure-Rust solvers (i.e. `minilp`). But unfortunately it doesn't solve quadratic problems.
- [`osqp.rs`](https://github.com/osqp/osqp.rs), which is a quadratic program solver. It relies on a C library, `OSQP`, and, based on these threads [1](https://github.com/rusqlite/rusqlite/issues/827), [2](https://github.com/rustwasm/team/issues/291), [3](https://www.reddit.com/r/rust/comments/i8snc5/compiling_rust_library_with_c_dependencies_to_wasm/), it sounds like compiling Rust with C dependencies to WASM is impossible right now. So basically any optimizer that is a wrapper around a C optimizer is a no-go.
- [`eigen-js`](https://github.com/BertrandBev/eigen-js), which is a WASM-compiled JS interface to Eigen, which itself uses OSQP. This would be pretty hacky--basically the Rust would use `wasm-bindgen` to call to the Javascript wrapper around OSQP. But I wasn't able to get it to work because I couldn't formulate my problem in a valid way for the solver (it takes the form `0.5 x' P x + q' x`, and `P + P' - diag(diag(P))` needs to have no negative eigenvalues, but `P` as an upper triangular matrix of all 1s, with the diagonal all 0s, doesn't satisfy this requirement).
- [`optimization-engine`](https://github.com/alphaville/optimization-engine), way more powerful--maybe too powerful--but was my last option. I can't figure out how to express all the constraints I need. In particular, the linear combination constraints over the variables.

It was a disappointingly fruitless effort and I am overwhelmed by how much I've forgotten/need to learn in the world of optimization. When I have more time I want to read up more on the topic and revisit this.

Again, I don't even know if the aforementioned trade-off problem will even come up in practice! So hopefully the linear formulation will work fine.

Something else this thought process brought up is that the optimization should happen over outputs and not processes. Right now if process A and B both output "plant calories", they are treated separately by allocating different shares of the total demand for "plant calories" according to their production mix shares. Say process A has 60% production share and process B has 40% production share. Here it's worth clarifying (for myself) that "production share" should really be read "production capacity share". We can think of this as process A has enough e.g. factories to, if there were enough resources, produce 60% of that sectors total output. But if process A is more resource intensive than process B and there is a shortage of resources, in practice process B might end up producing more than process A because it is a more efficient allocation of resources. The production capacity mix share is realized only when there is enough resources to saturate process A's production capacity (it only has enough factories to process so many resources); the remaining resources are then given to process B.

## 9/30

To discuss:

- Go over new architecture
    - Is this too complex? A lot of the complexity wrt to resources is hidden from the player.
    - The resource claiming can be made a lot simpler (and/or totally gotten rid of) if we just have global resource stocks for each resource. E.g. instead of keeping track of lithium deposits across the planet, we just have a total lithium deposit estimate and draw down from that. It would be a lot easier to get that data and simpler in the backend. But then we aren't able to do things like: this disaster struck this region, which is where a lot of lithium production happens.
    - My main concerns with the complexity:
        - That it will make events inscrutable: it might not be clear to the player why an event is occurring. That could be resolved by having clear dialogue from advisors explaining/speculating about the event's causes.
        - That it will make things very hard to debug:
            - There will already be a lot of tuning needed for the events (esp. their probability functions and effects)
            - I guess them being hard to debug is kind of the same as the previous issue (event inscrutability)--i.e. it's hard to debug because we also can't figure out why an event is happening.
        - That the emergent behavior is just incoherent/nonsensical.
    - The other option is to get rid of randomness for events. Instead events occur after some condition holds true for some amount of turns. Getting rid of randomness gets rid of one source of inscrutability for the player. Ideally use of randomness in games is limited to situations where it can be clearly communicated. If you do X, you have Y% chance of succeeding. We aren't going to show the player a menu of all possible events and their possibilities of occurring (it would be overwhelming), so we don't really have an opportunity to communicate the randomness.
            - The other place where randomness can work is where the player clearly has no control over it. It's where player agency and randomness are confused is where it's tricky.
        - The other benefit here is that randomness is so fickle (for example, it's possible for two rare events to occur in a row, when in practice we might not want that to happen) and people's perception of randomness is warped.
        - But there are still some cases where we want randomness, like with nuclear fusion or other moonshots that the player shouldn't rely on. It is fun to have some situations where those moonshots do work out. To support those situations we could have scenario-based randomness, where at the start of a run we randomly choose a scenario (e.g. the base scenario vs the rare scenario where fusion is discovered); then within that scenario there is no randomness. If you get the fusion scenario, fusion is always discovered after 80 years (or whatever).
        - Introducing timers and getting rid of randomness also lets us include "powerups"/abilities like better forecasting.
        - If we still want to maintain a small amount of randomness, we can specify turn countdown ranges and draw uniformly random from those ranges. E.g. this happens if X is true for 6-8 turns.
            - There may be some natural variation because we have a limit to how many events can happen per turn. The things that didn't trigger that month will trigger the following month. So maybe explicit ranges is unnecessary.
        - 0 turns means it happens immediately when that thing becomes true
        - There are still some events which feel like they should be random: heatwaves and other natural disasters, nuclear meltdowns...what to do about those?
- Go over new editor
    - Regions have to be formed from country groupings (rather than bioregions as proposed) because population data/projections are on a per-country basis. But...maybe we could do something like: bioregion X encompasses 40% of country Y, so we take 40% of country Y's population projections and add it into bioregion X. It's crude--we assume that the population is evenly distributed throughout a country which is basically never the case--but might be fine for our imprecise needs.
- Data requirements
    - Are these feasible? If not, can we fudge them somehow?
- Optimization planner
    - Better approach? Describe problem with the linear programming approach

More on randomness:

Another possible approach. Each event has one or more "condition sets" (a set of conditions joined by `AND`), each condition set maps to a "likelihood". These likelihoods are hard-coded probabilities--so we aren't dealing with individual probability functions. This makes it easier to assign probabilities to events without having to think in a lot of detail about how their probabilities are calculated:

```
Likelihood::Impossible     = 0
Likelihood::Improbable     = 0.00005
Likelihood::Rare           = 0.0005
Likelihood::Unlikely       = 0.005
Likelihood::Random         = 0.05
Likelihood::Likely         = 0.15
Likelihood::Guaranteed     = 1.0
```

See `studies/randomness.py` for the testing/tuning of these values.

As described above, condition sets must be true for 0 or more turns.

Condition sets are evaluated in order; the first one that's true is the likelihood that's returned.

Other event properties to tune randomness:

- Buffer: min turns that must pass before this can happen again
- Repeats: if it can only happen once

Now we need to specify how conditions work:

Two types of conditions:

- Comparison: compares a left and a right value, both numeric, using operators (`<, <=, ==, !=, >, >=`),
    - LocalVariable, GlobalVariable, Demand, Output, Resource, MixShare(Process)
- ProjectActive, ProjectStalled
- Flag: if some flag is set
- PersistFlag: if a persistent flag is set (persistent flags persist through game sessions)

# 10/16 Debugging the engine

The planner is failing to find solutions. It's really strange because it seems so inconsistent.

- If I remove the resource/feedstock constraints, it never fails
    - If I remove the just the feedstock constraints, it never fails
    - If I remove the just the resource constraints, it sometimes fails
        - This might just be because the feedstocks run out before the resources do
- If I set all the resource/feedstock constraints to `leq(0.)`, it always fails
    - So I thought it was because a resource/feedstock is gone
- If I set all the resource/feedstock constraints to `leq(1.)`, it sometimes fails
- If I set all the resource/feedstock constraints to `leq(1e20)`, it never fails

So it seems to be related to there not being enough of a resource or a feedstock. But in the `leq(1.)` case why does it work sometimes then? That should never be enough. And I would think that if there's not enough of a resource/feedstock the planner would just produce 0 of the associated output. Producing 0 output would satisfy the `leq(0.)` constraint.

It might have something to do with the processes/production orders. Aside from demand, that's the only thing changing between steps. With the constraints set to `leq(1.)` but `update_mix(...)` commented out, it never fails. But constraints with `leq(0.)` still always fails.

Similarly, if I set all order amounts to 0, then it never fails on the `leq(1.)` case AND the `leq(0.)` case.

Removing the `min(0.)` constraint for the amounts produced also doesn't fail, but then we get negative amounts produced.

If I set the minimum constraint for the amounts produced to `min(-1)` then it never fails. Really don't understand why, because the equation to optimize is basically maximize `av_0 + bv_1 + cv_2 ...`, and all the `v` variables must be at least 0, and these are subject to constraints of `av_0 <= feedstocks_0` (total amount must use less than or equal of the available feedstock). If all feedstocks are 0, then all `v` variables can just be 0 and satisfy the constraints.

In any case, `min(-1.)` appears to work and though negative amounts of outputs may be produced/resources consumed it looks negligible--it's not ideal but maybe I can revisit it if there's time.

# 10/16 Debugging starting data/parameters

The numbers for several things are way off. Electricity and fuel demand, the world's supply of coal gets used up within one year, land use requirements are way higher than they should be (requiring way more than total available habitable land), etc.

Electricity and fuel demand may be because electricity/fuel demand is double or triple counted: basically, there are three sources of demand in the model:

- Processes: for the purposes of the game (and thus land use, climate change, and biodiversity loss) these represent the most important industrial activities--agriculture (focused on food production) and energy production. We model these in terms of per-unit-output impacts (e.g. CO2, land, etc required per kWh).
- Industries: this represent consumption/production activity that isn't captured in detail as a process: so things like building energy and water usage, materials production (concrete, steel), etc.
- Per-capita demand: based on per-capita consumption figures across the world (categorized by a country's income level). In practice these values are (I believe) just that countries' aggregate consumption divided by its population. So for energy demands this should be redundant with energy demands yielded from the previous two sources. However, because processes and industries don't require food, this is where we get food demand.

Debugging ideas:

- Something else that might be wrong is time frames for demand. I think some of these were originally calculated for monthly, but now we're doing yearly, so those need to be adjusted.
- Debug food outputs (animal and plant calories).
    - Check aggregate land, water, and energy use, and compare against calibration values (e.g. agricultural land use for 2019).
    - Get gut-check values for global calorie consumptions to see if computed demand is within ballpark range
- Debug energy outputs (electricity and fuel). Check all values in the editor against the IEA reports
- For energy demands, ignore per-capita amounts; those really should be captured by just the industries and processes

To simplify things, and because I'm not sure the complexity is worth what we get out of it, I might remove the feedstocks system...it's just an additional thing to track/tune data for, and there's not a lot of time to do that. For most of these feedstocks (e.g. coal) I don't think supply will really be an issue until much later; and historically new reserves tend to be discovered/new processes developed that expand the lifetime of that feedstock (e.g. oil). It can't keep going on *forever* but maybe long enough that we don't need to worry about it.

10/17 Updates:

For energy consumption, the main driver of the errors were calorie production. There were two issues: one was that I was taking IEA energy data, which lumps agriculture into "other", and assuming that agriculture takes up enough of the "other" category that it was fine to assign the total consumption for the entire "other" category to just agriculture. I found more specific data that reduced energy usages for electricity by about 10x and for fuel by about 3x. That still wasn't enough though. Turns out I was dividing global agricultural energy usage by total global calories consumed for only a single day; multiplying that by 365 brought the numbers much closer into line. The energy consumption numbers overall (after removing the redundant energy per-capita demand):

```
  Agg. Demand:
    Fuel: 81448 TWh (Ref: 93333TWh)
    Electricity: 23142 TWh (Ref: 22778TWh)
    Animal Calories: 1433930 Tcals
    Plant Calories: 6819603 Tcals
```

It looks much closer, though fuel is off by a fair amount. That may be because I removed the EROI data; I'm guessing if I add that back in it will be closer.

Emissions are off:

- CO2 Emissions: 72.02 Gt (Ref: 36.45Gt)
- CH4 Emissions: 99.49 Mt (Ref: 570Mt)
- N2O Emissions: 0.01 Mt (Ref: 3.30Mt)
- CO2eq Emissions: 75.60 Gt

This might be because some of the process data I have is for CO2eq rather than separated out into CO2/CH4/N2O. Compared to the reference there is 35.57Gt extra CO2 emissions, which works out to 423.45Mt of CH4 and 119.36Mt of N2O. CH4 is off by about 470.51Mt and N2O by 3.29Mt.

However, 2019 CO2eq emissions was closer to 45.9Gt CO2eq, so the total is way off too.

Land use is also still off, and water demand is looking way too high as well:

```
  Energy Demand:
    Land: 1518057 km2
    Water: 17281 km3 (Ref: 4600km3)
  Calorie Demand:
    Land: 56068 km2
    Water: 13654 km3 (Ref: 4600km3)
```

For water requirements for calories, from <https://ourworldindata.org/grapher/water-requirement-per-kilocalorie>, the issue is probably because the water footprint there is a combination of three types:

- Blue water: "Volume of surface and groundwater consumed as a result of the production of a good or service. Consumption refers to the volume of freshwater used and then evaporated or incorporated into a product. It also includes water abstracted from surface or groundwater in a catchment and returned to another catchment or the sea. It is the amount of water abstracted from groundwater or surface water that does not return to the catchment from which it was withdrawn."
- Green water: "The precipitation on land that does not run off or recharge the groundwater but is stored in the soil or temporarily stays on top of the soil or vegetation. Eventually, this part of precipitation evaporates or transpires through plants. Green water can be made productive for crop growth (although not all green water can be taken up by crops, because there will always be evaporation from the soil and because not all periods of the year or areas are suitable for crop growth)."
- Grey Water: "The grey water footprint of a product is an indicator of freshwater pollution that can be associated with the production of a product over its full supply chain. It is defined as the volume of freshwater that is required to assimilate the load of pollutants based on natural background concentrations and existing ambient water quality standards. It is calculated as the volume of water that is required to dilute pollutants to such an extent that the quality of the water remains above agreed water quality standards."

More succinctly from the following water footprint in energy production paper:

The blue WF measures the consumptive use of surface and ground water; the green WF measures consumption of rain water (most relevant in agriculture and forestry); the grey WF is an indicator of water pollution.

Similarly, the source for water use in energy production, <https://waterfootprint.org/media/downloads/Mekonnen-et-al-2015_1.pdf>, uses blue and green water footprints (not enough data was available for grey water footprints).

The source for the 4,600km3 global water demand figure, <https://www.nature.com/articles/s41545-019-0039-9>, doesn't appear to be using the same concept (the "water footprint").

The original land problem (requiring way more than habitable land is available) was because I forgot to convert km2 to m2 for the available amount of land.

Land for calorie production should be much higher--<https://ourworldindata.org/land-use> has 51 million km2 in use by agriculture. Update: this ended up being another conversion problem; I had converted the land use by the wrong amount; fixing that ended up giving the following:

```
  Calorie Demand:
    Land: 56068096 km2 (Ref: 51000000km2)
    Water: 13654 km3 (Ref: 4600km3)
```

Which is way more reasonable!

For agricultural water use, Mekonnen, M. M., & Gerbens-Leenes, W. (2020). The water footprint of global food production. Water, 12(10), 2696. estimate the green+blue water footprint for agriculture to be 5938 to 8508 km3/year. For 2011, they have: 8362 km3/year (80% green,11% blue, and 9% grey). As I understand it the 4,600km3 figure is referring to the blue water footprint; if we assume that of our 13,645km3 figure, 11% of that is blue, then we get about 1,502km3. The number I calculated for non-modeled industries is 1428 km3.

But maybe it is more accurate to use these fuller water footprints anyways.

That leaves emissions as the last data to calibrate:

```
  Energy byproducts:
    CO2 Emissions: 79.04 Gt (Ref: 43.16Gt)
    CH4 Emissions: 99.45 Mt (Ref: 570Mt)
    N2O Emissions: 0.74 Mt (Ref: 9.99Mt)
  Calorie byproducts:
    CO2 Emissions: 0.00 Gt (Ref: 43.16Gt)
    CH4 Emissions: 0.04 Mt (Ref: 570Mt)
    N2O Emissions: 0.01 Mt (Ref: 9.99Mt)
  Industry byproducts:
    CO2 Emissions: 0.00 Gt (Ref: 43.16Gt)
    CH4 Emissions: 0.00 Mt (Ref: 570Mt)
    N2O Emissions: 0.00 Mt (Ref: 9.99Mt)
```

Energy emissions look way too high, calorie emissions way too low, industry probably too.

For calories, there was another conversion error (ugh) and after fixing that:

```
  Calorie byproducts:
    CO2 Emissions: 3.89 Gt
    CH4 Emissions: 37.80 Mt
    N2O Emissions: 6.68 Mt
```

Looks much more reasonable.

For energy emissions, the main culprit looks to be oil/petroleum emissions (as a fuel, not electricity generation):

```
  Petroleum byproducts:
    CO2 Emissions: 43.11 Gt
    CH4 Emissions: 26.56 Mt
    N2O Emissions: 0.00 Mt
```

This give some reference points for 2018: <https://ourworldindata.org/emissions-by-fuel>

- Gas: 7.49Gt
- Oil: 12.25Gt
- Coal: 14.62Gt
- Cement: 1.51Gt

I was using a value of 1005g CO2/kWh (based on a value in the book, which matches figures such as the one here: <https://www.eia.gov/tools/faqs/faq.php?id=74&t=11>, which is around 966gCO2/kWh. That latter figure is specifically for electricity generation; elsewhere I was seeing values closer to ~220gCO2/kWh of petroleum-derived fuels.

With that lower figure, I get:

```
  Petroleum byproducts:
    CO2 Emissions: 11.41 Gt
    CH4 Emissions: 26.56 Mt
    N2O Emissions: 0.00 Mt
```

which is closer to the reference values.

# 10/19 Firefox crashes

Firefox crashes after the game is running for a bit. This is the crash reason:

```
MozCrashReason: MOZ_RELEASE_ASSERT(ClientMatchPrincipalInfo(mClientInfo.PrincipalInfo(), aServiceWorker.PrincipalInfo()))
```

So it seems to be related to the service worker.

This thread might have some info: <https://bugzilla.mozilla.org/show_bug.cgi?id=1610772>
