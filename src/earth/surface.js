import png from './png';
import {EarthSurface} from 'half-earth-engine';
import { memory } from 'half-earth-engine/half_earth_engine_bg.wasm';
import initHector from 'hector-wasm';
import defaultEmissions from '../../assets/hector/rcp26.default_emissions.json';

// A grayscale image where each value
// indicates the label of that pixel
const biomeLabelsSrc = '/assets/surface/landuse.png';

// A grayscale image that maps
// temp (x-axis) and precip (y-axis)
// to a biome label.
const biomeLookupSrc = '/assets/surface/biomes.png';

// Base emissions scenario for Hector
const baseEmissionsScenario = '/assets/hector/rcp26.to_2050.json';

// Earth surface scaling
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
  constructor(startYear) {
    this.startYear = startYear;

    fetch('/assets/hector/config.json')
      .then((resp) => resp.json())
      .then((config) => {
        this.config = config;
      });


  }

  async init() {
    let loadLabels = Promise.all([
      png.load(biomeLabelsSrc),
      png.load(biomeLookupSrc)
    ]).then(([labels, lookup]) => {
      this._surface = EarthSurface.new(
        labels.data, labels.size.width, labels.size.height, scale,
        lookup.data);
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

      // Actually we aren't using SharedArrayBuffer because support
      // is still kind of uneven. Perhaps by the game's release we can switch over.
      // But for now just use a regular ArrayBuffer and send it via the RPC
      // this.pixelsBuf = new SharedArrayBuffer(memory.buffer.byteLength);
      this.pixelsBuf = new ArrayBuffer(memory.buffer.byteLength);
      this.pixels = new Uint8Array(this.pixelsBuf);
      this.pixels.set(this._pixels);
    });

    // Load the base emissions scenario history,
    // for computing temperature changes
    let loadScenario = fetch(baseEmissionsScenario)
      .then((resp) => resp.json()).then((baseScenario) => {
        this.emissions = {
          startYear: baseScenario['startYear'],
          data: {}
        };

        // Only get the base scenario data up to the game starting year
        let baseYears = this.startYear - baseScenario['startYear'];
        Object.keys(baseScenario['data']).forEach((k) => {
          this.emissions['data'][k] = baseScenario['data'][k].slice(0, baseYears);
        });
      });

    return Promise.all([loadLabels, loadScenario]);
  }

  /*
   * Adds a year of emissions data.
   * `emissions` should have keys and values for each required
   * emissions type. Refer to `/assets/hector/rcp.to_2050.json`
   * for the required keys.
   */
  addEmissions(emissions) {
    Object.keys(defaultEmissions).forEach((k) => {
      let val = emissions[k] !== undefined ? emissions[k] : defaultEmissions[k];
      this.emissions['data'][k].push(val);
    });
  }

  updateTemperature() {
    let ready = this._hector ?
      Promise.resolve(this._hector) :
      initHector(this.config, hectorOutputVars).then((hector) => {
        this._hector = hector;
      });

    return ready.then(() => {
      // Calculate new avg global temp
      // Only compute up to the current year,
      // so the last returned tgav is the current tgav
      let endDate = this.emissions.startYear + this.emissions.data['ffi_emissions'].length;
      let results = this._hector.run(endDate, this.emissions);
      let avgGlobalTemps = results['temperature.Tgav'];
      let avgGlobalTemp = avgGlobalTemps[avgGlobalTemps.length - 1];
      return avgGlobalTemp;
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
  updateBiomes(avgGlobalTemp) {
    // Calculate biome changes
    this._surface.update_biomes(avgGlobalTemp);

    // Update the pixel buffer.
    this.updateTexture();
  }

  updateTexture() {
    this._surface.update_surface();

    // Update the (shared) array buffer
    // so the main thread has it
    this.pixels.set(this._pixels);
  }
}

export default Surface;
