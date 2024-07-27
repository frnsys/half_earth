import * as esbuild from 'esbuild'
import { glsl } from "esbuild-plugin-glsl";

await esbuild.build({
  entryPoints: ['globe.js'],
  bundle: true,
  platform: "browser",
  format: "esm",
  outfile: 'globe.pkg.js',
	plugins: [glsl({
		minify: true
	})]
})
