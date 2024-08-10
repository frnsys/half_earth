export LEPTOS_WASM_OPT_VERSION := "version_118"

# Run the development game (browser).
game:
    trunk serve --config hes-game/Trunk.toml

# Run the development game (tauri).
game-app:
    cd hes-game && cargo tauri dev

# Run the development editor (browser).
editor:
    trunk serve --config hes-editor/Trunk.toml

# Run the development editor (tauri).
game-editor:
    cd hes-editor && cargo tauri dev

# Run tests.
test:
    cargo test --all-features

# Build the game and editor web versions.
build:
    rm -rf build/game && mkdir -p build/game
    rm -rf build/editor && mkdir -p build/editor
    trunk build --release --config hes-game/Trunk.toml --dist build/game
    trunk build --release --config hes-editor/Trunk.toml --dist build/editor

# Build the game and editor apps for release.
build-apps:
    cd hes-game && cargo tauri build --release
    cd hes-editor && cargo tauri build --release

# Extract strings for translation.
translate:
    cd hes-game && cargo expand --lib --ugly --color never | tr -d '\n' > /tmp/expanded
    cargo run --bin i18n

# Generate pre-computed earth surface textures.
surfaces:
    cargo run --bin surface --release

# Generate sharing images.
sharing:
    cargo run --bin sharing --release
