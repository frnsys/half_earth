## Notes

- Heavy-lifting code implemented in Rust (in `engine/`), provided to the frontend via WebAssembly (wasm), which is managed via a web worker.
- Main libraries used for the UI are three.js and Vue.
- Some of the assets are generated programmatically; look in `assets/src/` for the scripts.

## Setup

```
# Install JS dependencies
npm install -d

# Compile Rust code
cd engine; bash build.sh
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