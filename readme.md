# Half-Earth Socialism

The project consists of two artifacts:

- The [_Half-Earth Socialism_ game](https://play.half.earth) (both web browser and standalone app versions).
- The [_Half-Earth Socialism_ editor](https://editor.half.earth) (both web browser and standalone app).

[This video walks through an example of how to use the editor.](https://youtu.be/U8rmVcehZlg)

## Overview

- `hes-engine` defines the model that drives the game.
- `hes-game` is the game itself, which is mostly a visual layer over the engine.
- `hes-editor` is an editor to change parts of the game/engine, e.g. what projects are available, their parameters, etc, and is used to build custom `.world` files that can be loaded into new games.

We also include [a version](https://github.com/frnsys/hector-wasm) of the [Hector simple climate model](https://jgcri.github.io/hector/) that we've adapted to run in the browser. The process of building Hector for WASM is complicated so pre-compiled versions are included here.


## Setup

```bash
git submodule init
git submodule update
```

## Dependencies

This requires Rust Nightly to compile.

```bash
# Leptos is the main framework used for both the game and the editor.
cargo install trunk
cargo install cargo-leptos

# WASM target is required for frontend code.
rustup target add wasm32-unknown-unknown
```

If you're using Nix, you can run `nix-shell` instead of doing the above.

## Development

There is some functionality which is better kept in JS rather than ported to Rust/WASM. In particular, the rendering of the globe (which depends on three.js), handling of audio, and interfacing with the Hector WASM module. We'd use our [Rust adapter for Hector](https://github.com/frnsys/hector-rs) directly but Rust/WASM doesn't work with C++ FFI, so we have to stick with using JS as a bridge.

The globe and Hector JS modules need to re-built if any of their files are edited. They can be rebuilt by doing:

1. `cd hes-game/public/js`
2. If you haven't already, run `just setup`.
3. Then run `just build`.

If debugging the game there are a few options you can pass as URL parameters which can help. These are used a comma-separated following `debug=`, e.g. `http://localhost:3000/?debug=all-projects,all-processes`.

- `check-events`: Show a list of all events and updates and click on them to trigger them.
- `all-projects`: All projects are unlocked at the start.
- `all-processes`: All processes are unlocked at the start.
- `skip-to-planning`: Skip the intro and just go to the planning phase.
- `skip-tutorial`: Skip the tutorial.
- `skip-events`: Skip all events.
- `fast-years`: Speed up years in the world events phase.
- `always-skip-world`: Skip the world events phase.
- `i-am-the-state`: Start with 1000 political capital.


## Running

You can use [`just`](https://github.com/casey/just) to run most tasks:

```
Available recipes:
    game        # Run development game (browser)
    editor      # Run development editor (browser)
    build       # Build the web release versions (browser)
    test        # Run tests
    translate   # Extract translation strings and update the translation mappings from the source CSVs.
    surfaces    # Generate biome surface textures and regional climates.
    sharing     # Generate sharing images.
```

## Building & Deploying

- Cross-platform builds are handled by Github Actions (using a `workflow_dispatch`, i.e. manual trigger).
- The web versions of the game and editor are built using `just build` and are hosted as static sites.

---

# Releases

You can play it at:
* [half.earth](https://play.half.earth/) (web; and [the editor](https://editor.half.earth/))
* [Steam](https://store.steampowered.com/app/2071530/HalfEarth_Socialism/) (Windows/macOS/Linux) (Account needed)
  - _Note_: The build process for the Steam version is more complicated so it's less likely to be up-to-date.
* [Itch.io](https://frnsys.itch.io/half-earth-socialism) (Windows/macOS/Linux)

# Community

We have [a Discord for the game and book here](https://discord.gg/cUBtbDfzn5).
