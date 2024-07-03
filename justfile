# Run the development server
run:
    cd hes-game && trunk serve

# Run the application
run-app:
    cd hes-game && cargo tauri dev

# Run tests
test:
    cd hes-engine && cargo test
