import path from 'path';
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import vitePluginRequire from "vite-plugin-require";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),

    // Let's us use `require(...)`
		vitePluginRequire.default(),

    wasm(),

    // > You also need the vite-plugin-top-level-await plugin
    // > unless you target very modern browsers only
    // > (i.e. set build.target to esnext).
    topLevelAwait()
  ],

  define: {
    VERSION: JSON.stringify(process.env.version),
    TIMESTAMP: JSON.stringify(process.env.timestamp),
    PLATFORM: JSON.stringify(process.env.platform),
  },

  resolve: {
    alias: {
      'lib': path.resolve('./src/lib'),
      'components': path.resolve('./src/components'),
      'content': path.resolve('./src/assets/content'),
      'assets': path.resolve('./src/assets'),

      // Proxy three.js exports to reduce bundle size
      'three$': path.resolve('./src/3d/three.js'),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
