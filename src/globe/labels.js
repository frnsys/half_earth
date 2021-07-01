import UPNG from 'upng-js';
import labelColors from './colors';
import * as THREE from 'three';


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

// Process colors into uint8 RGB values
function processColors(colors) {
  return colors.map((c) => {
    let color = new THREE.Color(c);
    let r = Math.floor(color.r * 255);
    let g = Math.floor(color.g * 255);
    let b = Math.floor(color.b * 255);
    return {r, g, b};
  });
}

function generateLabelsTexture(dims, labels, colors) {
  const [n_cols, n_rows] = dims;
  const size = n_cols * n_rows;
  const textureMatrix = new Uint8Array(3*size);
  for (let i=0; i<size; i++) {
    // const label = labels[i];
    // let {r, g, b} = label == 255 ? colors[0] : colors[label+1];
    const label = labels[i];
    let {r, g, b} = label == 255 ? colors[0] : colors[label];
    const stride = i * 3;
    textureMatrix[stride] = r;
    textureMatrix[stride+1] = g;
    textureMatrix[stride+2] = b;
  }
  let tex = new THREE.DataTexture(textureMatrix, n_cols, n_rows, THREE.RGBFormat);
  tex.flipY = true;
  return tex;
}


// src is a path to a
// grayscale image where each value
// indicates the label of that pixel
function loadLabelsTexture(src) {
  return loadPNG(src).then(({labels, dims}) => {
    let colors = processColors(labelColors)
    return {
      labels,
      colors,
      generateTexture: (colors) => {
        return generateLabelsTexture(dims, labels, colors);
      }
    }
  });
}

export default loadLabelsTexture;
