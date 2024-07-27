# Half-Earth Socialism

The project consists of two artifacts:

- The _Half-Earth Socialism_ game (includes web browser and standalone app versions).
- The _Half-Earth Socialism_ game editor (available only as a standalone app).
  - Note: The only thing preventing the editor from having a web browser version is that it needs system file access.

## Overview

- `hes-engine` defines the model that drives the game.
- `hes-game` is the game itself, which is mostly a visual layer over the engine.
- `hes-editor` is an editor to change parts of the game/engine, e.g. what projects are available, their parameters, etc, and is used to build custom `.world` files that can be loaded into new games.
- `hector-rs` is our adapted version of the [Hector simple climate model](https://jgcri.github.io/hector/).

The original version of the game ran purely in the browser, except for some auxiliary components (share image generation, logging, etc). This newer version relaxes that requirement so that more is handled on the server (e.g. running Hector) but it's bundled together more seamlessly.


## Setup

```bash
git submodule init
git submodule update
```

## Dependencies

```bash
# C++ and Boost required to compile Hector.
apt install gcc g++ libboost-all-dev

# Tauri is to wrap the web stack as an application.
cargo install tauri-cli

# Leptos is the main framework used for both the game and the editor.
cargo install trunk
cargo install cargo-leptos

# WASM target is required for frontend code.
rustup target add wasm32-unknown-unknown
```

## Running

You can use [`just`](https://github.com/casey/just) to run most tasks:

```
Available recipes:
    run-web     # Run development web game
    run-app     # Run development app game
    test-engine # Run engine tests
    build-apps  # Build the app release versions
    build-web   # Build the web release version
```

## Building & Deploying

- Cross-platform builds are handled by Github Actions.
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
