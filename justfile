# Run the development server
run:
    cd hes-game && cargo leptos watch

# Run the application
run-app:
    cd hes-game && cargo tauri dev

# Run tests
test:
    cd hes-engine && cargo test
