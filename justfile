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

# Build the game and editor for release.
build-app:
    cd hes-game && cargo tauri build
    cd hes-editor && cargo tauri build
