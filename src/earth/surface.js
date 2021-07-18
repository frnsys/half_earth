import util from './util';
import {EarthSurface} from "half-earth-engine";
import { memory } from "half-earth-engine/half_earth_engine_bg.wasm";

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
  }

  updateTexture() {
    this._surface.update_surface();

    // Update the shared array buffer
    this.pixels.set(this._pixels);
  }
}

export default Surface;
