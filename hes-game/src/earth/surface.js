import png from './png';
import {EarthSurface} from 'half-earth-engine';
import { memory } from 'half-earth-engine/half_earth_engine_bg.wasm';

// A grayscale image where each value
// indicates the label of that pixel
const biomeLabelsSrc = '/assets/surface/landuse.png';

// A grayscale image that maps
// temp (x-axis) and precip (y-axis)
// to a biome label.
const biomeLookupSrc = '/assets/surface/biomes.png';

// Earth surface scaling
const scale = 4;

class Surface {
  async init() {
    return Promise.all([
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
      let length = this.width * this.height * 3;
      this._pixels = new Uint8Array(memory.buffer, pixelsPtr, length);

      // Actually we aren't using SharedArrayBuffer because support
      // is still kind of uneven. Perhaps by the game's release we can switch over.
      // But for now just use a regular ArrayBuffer and send it via the RPC
      // this.pixelsBuf = new SharedArrayBuffer(memory.buffer.byteLength);
      // this.pixelsBuf = new SharedArrayBuffer(length);
      this.pixelsBuf = new ArrayBuffer(length);
      this.pixels = new Uint8Array(this.pixelsBuf);
      this.pixels.set(this._pixels);
    });
  }

  /*
   * Update biomes and surface pixels accordingly to avg temp.
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
