set dotenv-load

# Run the game
run:
    cargo run

# Generate pre-computed earth surface textures.
surfaces:
    cargo run --bin surface --release

# Generate sharing images.
sharing:
    cargo run --bin sharing --release

# Build for web
build-web:
    cd game && trunk build --release

# Build and deploy for web
deploy-web: build-web
    rsync -ravu --progress --delete game/dist/ $SERVER
