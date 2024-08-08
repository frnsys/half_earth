export LEPTOS_WASM_OPT_VERSION := "version_118"

# Run the development web game.
run-web:
    cd hes-game && cargo leptos watch

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
    cargo leptos build --release
    rm -rf build/web && mkdir -p build/web
    cp target/release/hes-game build/web/hes-game
    cp -r target/hes-game build/web/site
    echo "To run: LEPTOS_SITE_ROOT="site" ./hes-game"

# Extract strings for translation.
strings:
    cd hes-game && cargo expand --lib --ugly --color never | tr -d '\n' > /tmp/expanded
    cargo run --bin hes-game-i18n
