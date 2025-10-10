# Generate translation files.
translate:
    cargo run --bin i18n

# Generate pre-computed earth surface textures.
surfaces:
    cargo run --bin surface --release

# Generate sharing images.
sharing:
    cargo run --bin sharing --release
