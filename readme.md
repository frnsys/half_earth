## Notes

- Heavy-lifting code implemented in Rust (in `engine/`), provided to the frontend via WebAssembly (wasm), which is managed via a web worker.
- The [Hector Simple Climate Model](https://github.com/JGCRI/hector) is integrated via WebAssembly, managed by the same web worker managing the Rust components
- Main libraries used for the UI are three.js and Vue.
- Some of the assets are generated programmatically; look in `assets/src/` for the scripts.

## Setup

```
# Install JS dependencies
npm install -d

# Compile Rust code
npm run build-wasm
```

## Usage

Run the development server with:

```
npm start
```

If you modify the Rust code, run `npm run build-wasm`

## Tests

Run Cargo and WASM tests with:

```
npm run test
```

## Benchmarking

For Rust code benchmarking, run:

```
cargo bench
```

See [the `criterion` docs](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html) for more info.

In particular:

```
# Save a new baseline
cargo bench -- --save-baseline <name>

# Compare against a baseline
cargo bench -- --baseline <name>
```