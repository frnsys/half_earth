import {decode, toRGBA8} from 'upng-js';

// Load a grayscale PNG
function loadPNG(url) {
  return fetch(url, {
    headers: {
      'Response-Type': 'arraybuffer',
    },
    method: 'GET'
  }).then((res) => {
    return res.arrayBuffer();
  }).then((buf) => {
    let png = decode(buf);

    // There are a variety of PNG color types;
    // this converts them into a consistent RGBA type
    var rgba = toRGBA8(png);

    // Data is flat RGBA values
    // since the images we're working with is grayscale,
    // we only need the first of every four values
    let data = [];
    let vals = new Uint8Array(rgba[0]);
    for (let i=0; i<vals.length; i=i+4) {
      data.push(vals[i]);
    }
    return {
      data,
      size: {width: png.width, height: png.height}
    };
  });
}

export default {loadPNG};
