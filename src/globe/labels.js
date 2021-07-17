import UPNG from 'upng-js';
import * as THREE from 'three';
import * as wasm from "half-earth-engine";
import { memory } from "half-earth-engine/half_earth_engine_bg.wasm";

function loadPNG(url) {
  return fetch(url, {
    headers: {
      'Response-Type': 'arraybuffer',
    },
    method: 'GET'
  }).then((res) => {
    return res.arrayBuffer();
  }).then((buf) => {
    let png = UPNG.decode(buf);

    // There are a variety of PNG color types;
    // this converts them into a consistent RGBA type
    var rgba = UPNG.toRGBA8(png);

    // Data is flat RGBA values
    // since the images we're working with is grayscale,
    // we only need the first of every four values
    let labels = [];
    let vals = new Uint8Array(rgba[0]);
    for (let i=0; i<vals.length; i=i+4) {
      labels.push(vals[i]);
    }
    return {
      labels,
      dims: [png.width, png.height]
    };
  });
}

function generateLabelsTexture(dims, labels) {
  let [n_cols, n_rows] = dims;

  var t0 = performance.now()
  const earthSurface = wasm.EarthSurface.new(labels, n_cols, n_rows);
  const pixelsPtr = earthSurface.surface();
  const effectTextureArr = new Uint8Array(memory.buffer, pixelsPtr, earthSurface.width() * earthSurface.height() * 3);
  // let effectTextureArr = wasm.oil_paint_effect(scaledTextureArr, n_cols, n_rows);
  var t1 = performance.now()
  console.log("wasm: " + (t1 - t0) + "milliseconds.")

  let tex = new THREE.DataTexture(effectTextureArr, earthSurface.width(), earthSurface.height(), THREE.RGBFormat);
  tex.flipY = true;
  return tex;
}


// src is a path to a
// grayscale image where each value
// indicates the label of that pixel
function loadLabelsTexture(src) {
  return loadPNG(src).then(({labels, dims}) => {
    return {
      labels,
      generateTexture: () => {
        return generateLabelsTexture(dims, labels);
      }
    }
  });
}

export default loadLabelsTexture;
