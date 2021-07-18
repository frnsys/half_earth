import UPNG from 'upng-js';

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
      size: {width: png.width, height: png.height}
    };
  });
}

export default {loadPNG};
