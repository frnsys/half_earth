# Half-Earth Socialism

This repo includes:

- `hes-engine` defines the model that drives the game.
- `hes-game` is the game itself, which is mostly a visual layer over the engine.
- `hes-editor` is an editor to change parts of the game/engine, e.g. what projects are available, their parameters, etc, and is used to build custom `.world` files that can be loaded into new games.

Note that there are effectively two versions of the game: web and native. We don't include the editor for the web version but that should be the only major difference between the world. Worlds created in the editor can be used in the web version.

We also include [a version](https://github.com/frnsys/hector-wasm) of the [Hector simple climate model](https://jgcri.github.io/hector/) that we've adapted to run in the browser. The process of building Hector for WASM is complicated so pre-compiled versions are included here.

For native (i.e. non-web) builds we instead use [a Rust wrapper for Hector](https://github.com/frnsys/hector-rs).

## Editor

[This video walks through an example of how to use the editor.](https://youtu.be/U8rmVcehZlg)

## Setup & Dependencies

```bash
git submodule init
git submodule update
```

Building was last tested with Rust stable `1.89.0 (29483883e 2025-08-04)`.


## Debugging

Debug options can be specified through the env vars `DEBUG` and `DEBUG_VIEW`, e.g. with:

```bash
DEBUG=SKIP_TUTORIAL,ALL_PROJECTS,SKIP_EVENTS DEBUG_VIEW=Plan cargo run
```

See `game/src/debug.rs`.

## Running

You can use [`just`](https://github.com/casey/just) to run most tasks:

```
Available recipes:
    run         # Run the game.
    surfaces    # Generate biome surface textures and regional climates.
    sharing     # Generate sharing images.
```

# Releases

You can play it at:
* [half.earth](https://play.half.earth/) (web; and [the editor](https://editor.half.earth/))
* [Steam](https://store.steampowered.com/app/2071530/HalfEarth_Socialism/) (Windows/macOS/Linux) (Account needed)
  - _Note_: The build process for the Steam version is more complicated so it's less likely to be up-to-date.
* [Itch.io](https://frnsys.itch.io/half-earth-socialism) (Windows/macOS/Linux)

# Community

We have [a Discord for the game and book here](https://discord.gg/cUBtbDfzn5).
