import * as esbuild from 'esbuild'
import { glsl } from "esbuild-plugin-glsl";

await esbuild.build({
  entryPoints: {
    tgav: 'hector/tgav.js',
    globe: 'earth/globe.js',
  },
  outdir: 'dist',
  bundle: true,
  minify: true,
  platform: "browser",
  format: "esm",
	plugins: [glsl({
		minify: true
	})]
})
