export LEPTOS_WASM_OPT_VERSION := "version_118"

# Run the development web game.
run-web:
    trunk serve --config hes-game/Trunk.toml

# Run the development app game.
run-app:
    cd hes-game && cargo tauri dev

# Run the development editor.
run-editor:
    cd hes-editor && cargo tauri dev

# Run the engine tests.
test:
    cargo test --all-features

# Build the game and editor apps for release.
build-apps:
    cd hes-game && cargo tauri build --releaese
    cd hes-editor && cargo tauri build --release

# Build the game web version.
build-web:
    rm -rf build/web && mkdir -p build/web
    trunk build --release --config hes-game/Trunk.toml --dist build/web

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
