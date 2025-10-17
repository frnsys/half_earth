import * as esbuild from 'esbuild'

await esbuild.build({
  entryPoints: {
    tgav: 'hector/tgav.js',
  },
  outdir: 'dist',
  bundle: true,
  minify: true,
  platform: "browser",
  format: "esm",
})
