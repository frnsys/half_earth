A simple example for `use_websocket`.

If you don't have it installed already, install [Trunk](https://trunkrs.dev/) and [Tailwind](https://tailwindcss.com/docs/installation)
as well as the nightly toolchain for Rust and the wasm32-unknown-unknown target:

```bash
cargo install trunk
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

Then, open two terminals. In the first one, run:

```
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

In the second one, run:

```bash
trunk serve --open
```
