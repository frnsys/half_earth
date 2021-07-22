import util from './util';
import {EarthSurface} from "half-earth-engine";
import { memory } from "half-earth-engine/half_earth_engine_bg.wasm";
import loadHector from 'hector-wasm';

const scale = 4;

class Surface {
  constructor(labels, size) {
    this._surface = EarthSurface.new(labels, size.width, size.height, scale);
    this.width = this._surface.width();
    this.height = this._surface.height();

    // Support for shared wasm memory across workers is in relatively
    // early stages and complicated.
    // See <https://github.com/richwandell/rust_wasm_webcam/blob/8a86f5fe2c1d8bcba34f747f529f4c14826a1d9c/README.md>
    // For compatibility reasons and to avoid a lot of extra configuration,
    // just copy the pixel data from wasm memory
    // info a SharedArrayBuffer that the main thread has access to,
    // and re-copy whenever it updates.
    let pixelsPtr = this._surface.surface();
    this._pixels = new Uint8Array(memory.buffer, pixelsPtr, this.width * this.height * 3);
    this.pixelsBuf = new SharedArrayBuffer(memory.buffer.byteLength);
    this.pixels = new Uint8Array(this.pixelsBuf);
    this.pixels.set(this._pixels);

    const outputVars = {
      "temperature.Tgav": {
        "component": "temperature",
        "description": "global atmospheric temperature anomaly",
        "unit": "degC",
        "variable": "Tgav"
      }
    };
    Promise.all([
      loadHector(),
      fetch('/assets/hector/config.json')
        .then((resp) => resp.json()),
      fetch('/assets/hector/rcp45.json')
        .then((resp) => resp.json())
    ]).then(([{Hector, run}, config, scenario]) => {
      console.log(`Hector version ${Hector.version()}`);
      console.log('Running...');
      var t0 = performance.now()
      let results = run(config, scenario, outputVars);
      var t1 = performance.now()
      console.log(`Done running in ${t1 - t0}ms`);
    });
  }

  updateTexture() {
    this._surface.update_surface();

    // Update the shared array buffer
    this.pixels.set(this._pixels);
  }
}

export default Surface;
