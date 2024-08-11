Build app versions of the game and editor, using Electron.

Electron is basically just being used as a browser to serve the otherwise static game/editor.

As such we just include both together (with separate `package.json` files). `yarn` unfortunately doesn't support pointing to a specific `package.json` file so when building e.g. the game you should copy `package.game.json` to `package.json`.

Similarly we have separate icons (in `build/`) for the game and the editor; the appropriate one should be copied to `build/icon.png` before building.

The static site is expected at `site/`, so first you'd run e.g. for the game:

```
trunk build --release --config hes-game/Trunk.toml --dist build/site
```

With the `package.json` and site assets in place you can then run:

```
yarn build
```

You can refer to the Github workflow for an example.
