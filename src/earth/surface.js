import util from './util';
import {EarthSurface} from 'half-earth-engine';
import { memory } from 'half-earth-engine/half_earth_engine_bg.wasm';
import loadHector from 'hector-wasm';

const scale = 4;
const hectorOutputVars = {
  'temperature.Tgav': {
    'component': 'temperature',
    'description': 'global atmospheric temperature anomaly',
    'unit': 'degC',
    'variable': 'Tgav'
  }
};


class Surface {
  constructor(startYear, labels, size) {
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

    // Load the base emissions scenario history,
    // for computing temperature changes
    fetch('/assets/hector/rcp45.to_2050.json')
      .then((resp) => resp.json()).then((baseScenario) => {
        this.emissions = {
          startYear: baseScenario['startYear'],
          data: {}
        };

        // Only get the base scenario data up to the game starting year
        let baseYears = startYear - baseScenario['startYear'];
        Object.keys(baseScenario['data']).forEach((k) => {
          this.emissions['data'][k] = baseScenario['data'][k].slice(0, baseYears);
        });
      });
  }

  /*
   * Adds a year of emissions data.
   * `emissions` should have keys and values for each required
   * emissions type. Refer to `/assets/hector/rcp.to_2050.json`
   * for the required keys.
   */
  addEmissions(emissions) {
    Object.keys(emissions).forEach((k) => {
      this.emissions['data'][k].push(emissions[k]);
    });
  }

  /*
   * Calculates average global temperature based on
   * current emissions history (`this.emissions`)
   * and then updates biomes and surface pixels accordingly.
   *
   * The texture that shares the pixel buffer needs to be marked for update,
   * e.g. `surfaceTexture.needsUpdate = true;`, for the new data to
   * actually show up.
   */
  updateBiomes() {
    let ready = this._hectorRun ? Promise.resolve(this._hectorRun) : Promise.all([
      loadHector(),
      fetch('/assets/hector/config.json')
        .then((resp) => resp.json()),
    ]).then(([{Hector, run}, config]) => {
      this._hectorRun = () => {
        return run(config, this.emissions, outputVars);
      };
      this._hectorRun;
    });

    ready.then((run) => {
      // Calculate new avg global temp
      let results = this._hectorRun(emissions);
      let avgGlobalTemp = results['temperature.Tgav'];

      // Calculate biome changes
      this._surface.update_biomes(avgGlobalTemp);

      // Update the pixel buffer.
      this.updateTexture();
    });
  }

  updateTexture() {
    this._surface.update_surface();

    // Update the shared array buffer
    this.pixels.set(this._pixels);
  }
}

export default Surface;
