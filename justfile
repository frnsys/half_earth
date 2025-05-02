export LEPTOS_WASM_OPT_VERSION := "version_123"

# Run the development game (browser).
game:
    trunk serve --locked --config hes-game/Trunk.toml

# Run the development editor (browser).
editor:
    trunk serve --locked --config hes-editor/Trunk.toml

# Run tests.
test:
    cargo test --all-features

# Build the game and editor web versions.
build:
    rm -rf /tmp/hes/game && mkdir -p /tmp/hes/game
    rm -rf /tmp/hes/editor && mkdir -p /tmp/hes/editor
    trunk build --locked --release --config hes-game/Trunk.toml --dist /tmp/hes/game
    trunk build --locked --release --config hes-editor/Trunk.toml --dist /tmp/hes/editor

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
