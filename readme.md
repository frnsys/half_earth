# Half-Earth SOcialism

## Dependencies

```bash
cargo install tauri-cli
cargo install trunk
```

## Running

You can use [`just`](https://github.com/casey/just) to run most tasks:

```
Available recipes:
    run     # Run the development server
    run-app # Run the application
    test    # Run tests
```

---

This is the repository for the game Half-Earth Socialism.

You can play it at:
* [half.earth](https://play.half.earth/) (web)
* [Steam](https://store.steampowered.com/app/2071530/HalfEarth_Socialism/) (Windows/macOS/Linux) (Account needed)
  - _Note_: The build process for the Steam version is more complicated so it's less likely to be up-to-date.
* [Itch.io](https://frnsys.itch.io/half-earth-socialism) (Windows/macOS/Linux)

## Notes

- Heavy-lifting code implemented in Rust (in `engine/`), provided to the frontend via WebAssembly (wasm), which is managed via a web worker.
- The [Hector Simple Climate Model](https://github.com/JGCRI/hector) is integrated via WebAssembly, managed by the same web worker managing the Rust components
- Main libraries used for the UI are three.js and Vue.
- Some of the assets are generated programmatically; look in `assets/src/` for the scripts.
