## Notes

- Heavy-lifting code implemented in Rust (in `engine/`), provided to the frontend via WebAssembly (wasm), which is managed via a web worker.
- The [Hector Simple Climate Model](https://github.com/JGCRI/hector) is integrated via WebAssembly, managed by the same web worker managing the Rust components
- Main libraries used for the UI are three.js and Vue.
- Some of the assets are generated programmatically; look in `assets/src/` for the scripts.

1. Use the editor in `editor/` to update game content, then run `parse_content.py` to parse the editor data into Rust code and JSON assets. See `update_content.sh`

## Setup

```
# Install JS dependencies
npm install -d

# Install hector-rs
git clone https://github.com/frnsys/hector-rs.git hector/hector-rs
cd hector/hector-rs
bash setup.sh
cd ../../

# Fetch and parse latest content
# Requires `pip3 install pillow`
bash update_content.sh
```

## Usage

Run the development server with:

```
npm start
```

If you modify the Rust code, run `npm run build-wasm`.

If changes are made in the editor, run `bash update_content.sh`.

## Tests

Run Cargo and WASM tests with:

```
npm run test
```

## Calibration

Go into the `engine/` folder, then run `./calibrate.sh build` to compile the calibration code, and then `./calibrate.sh` to generate the calibration data and show plots.

You can specify different scenarios to include (see `engine/examples/simulate.rs`), e.g. `./calibrate.sh BanFossilFuels,Electrification,Nuclear` to apply those scenarios.

Note that whenever you update content (e.g. with `update_content.sh`) you need to re-run `./calibrate.sh build` to compile in the updated content.

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

## Debugging

Using Sentry for monitoring. If something breaks, check the error there. A more detailed traceback of Rust errors will be in the "Breadcrumbs" section as JS console output. Also check the `version` session variable, which is the git commit hash of the version that was running.

## Deployment

On older versions of `nginx` you may need to manually add the `wasm` MIME type:

```
vi /etc/nginx/mime.types

# Add the line:
application/wasm    wasm

systemctl restart nginx
```

You also to configure the server to add the proper cross-origin isolation headers so that `SharedArrayBuffer` can be used. In `nginx` you add the following to your `server` block:

```
server {
    // ...

    add_header Cross-Origin-Opener-Policy 'same-origin';
    add_header Cross-Origin-Embedder-Policy 'require-corp';

    // ...
}