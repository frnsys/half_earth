- `cd hes-game`
  - `npm run dev` - browser version
  - `npm run tauri dev` - desktop version
- `cd hes-editor`
  - `trunk serve` - browser version
  - `cargo tauri dev` - desktop version

# Dependencies

```bash
cargo install tauri-cli
cargo install trunk
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

Most tasks are handled via `just`:

```
Available recipes:
    run          # Run the development server
    test         # Run tests
    setup        # Setup dev environment
    update       # Update editor data
    update_local # Update using local editor data
    build_rust   # Build the Rust code
```

If you modify the Rust code, run `just build_rust`.

If changes are made in the editor, run `just update`. Note that for this to work you need a `.env` file with:

```
EDITOR_USER=username
EDITOR_PASS=pass
EDITOR_URL=https://my.editor.url
```

The editor for the live game is private so you will need to setup your own, or you can directly edit `editor/data.json`, or you can run a local copy of the editor (see the `editor` folder). If you do the latter two you should instead just run `just update_local` to use your local editor data.

---

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
