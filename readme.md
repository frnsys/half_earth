# Half-Earth Socialism

The project consists of two artifacts:

- The _Half-Earth Socialism_ game (includes web browser and standalone app versions).
- The _Half-Earth Socialism_ game editor (available only as a standalone app).
  - Note: The only thing preventing the editor from having a web browser version is that it needs system file access.

## Overview

- `hes-engine` defines the model that drives the game.
- `hes-game` is the game itself, which is mostly a visual layer over the engine.
- `hes-editor` is an editor to change parts of the game/engine, e.g. what projects are available, their parameters, etc, and is used to build custom `.world` files that can be loaded into new games.

We also include [a version](https://github.com/frnsys/hector-wasm) of the [Hector simple climate model](https://jgcri.github.io/hector/) that we've adapted to run in the browser. The process for building Hector for WASM is complicated so pre-compiled versions are included here.


## Setup

```bash
git submodule init
git submodule update
```

## Dependencies

```bash
# Tauri is to wrap the web stack as an application.
cargo install tauri-cli

# Leptos is the main framework used for both the game and the editor.
cargo install trunk
cargo install cargo-leptos

# WASM target is required for frontend code.
rustup target add wasm32-unknown-unknown
```

## Development

There is some functionality which is better kept in JS rather than ported to Rust/WASM. In particular, the rendering of the globe (which depends on three.js), handling of audio, and interfacing with the Hector WASM module. We'd use our [Rust adapter for Hector](https://github.com/frnsys/hector-rs) directly but Rust/WASM doesn't work with C++ FFI, so we have to stick with using JS as a bridge.

The globe and Hector modules need to re-built if any of their files are edited. They can be rebuilt by doing:

1. `cd hes-game/public/js`
2. If you haven't already, run `just setup`.
3. Then run `just build`.


## Running

You can use [`just`](https://github.com/casey/just) to run most tasks:

```
Available recipes:
    run-web     # Run development web game (browser)
    run-app     # Run development app game (tauri)
    test-engine # Run engine tests
    build-apps  # Build the app release versions (tauri)
                # Note: this is just native, for cross-platform see below.
    build-web   # Build the web release version (browser)
```

## Building & Deploying

- Cross-platform builds are handled by Github Actions (using a `workflow_dispatch, i.e. manual trigger).
- The web version of the game is built using `just build-web` and then managed as a `systemd` unit:

```ini
# /etc/systemd/system/apps.half.earth.service

# Place the build artifacts (`hes-game`, `site/`) at
# `/srv/projects/half-earth/game`.

[Unit]
Description=half-earth socialism game
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
Restart=always
WorkingDirectory=/srv/projects/half-earth/game
ExecStart=/srv/projects/half-earth/game/hes-game
Environment=LEPTOS_SITE_ADDR=0.0.0.0:8888
Environment=LEPTOS_SITE_ROOT=./site

[Install]
WantedBy=multi-user.target
```

---

# Releases

You can play it at:
* [half.earth](https://play.half.earth/) (web)
* [Steam](https://store.steampowered.com/app/2071530/HalfEarth_Socialism/) (Windows/macOS/Linux) (Account needed)
  - _Note_: The build process for the Steam version is more complicated so it's less likely to be up-to-date.
* [Itch.io](https://frnsys.itch.io/half-earth-socialism) (Windows/macOS/Linux)
