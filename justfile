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
test-engine:
    cd hes-engine && cargo test

# Build the game and editor apps for release.
build-apps:
    cd hes-game && cargo tauri build
    cd hes-editor && cargo tauri build

# Build the game web version.
build-web:
    cd hes-game && cargo leptos build --release
    rm -rf build/web && mkdir -p build/web
    cp hes-game/target/release/hes-game build/web/hes-game
    cp -r hes-game/target/release/site build/web/site
    echo "To run: `LEPTOS_SITE_ROOT="site" ./hes-game`"
