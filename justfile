# Run the game
run:
    cargo run

# Generate pre-computed earth surface textures.
surfaces:
    cargo run --bin surface --release

# Generate sharing images.
sharing:
    cargo run --bin sharing --release
