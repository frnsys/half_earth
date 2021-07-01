import * as dat from 'dat.gui';

// For palette design
function setupPaletteGUI(colors, cb) {
  const palette = {};
  const gui = new dat.GUI();
  colors.forEach(({r, g, b}, i) => {
    palette[i] = [r, g, b];
    const ctrl = gui.addColor(palette, i);
    ctrl.onChange((rgb) => {
      colors[i] = {
        r: rgb[0],
        g: rgb[1],
        b: rgb[2]
      };
      cb(colors);
    });
  });

  gui.add({
    save: () => {
      let hexColors = [];
      colors.forEach((_, i) => {
        let hex = new THREE.Color(...palette[i].map((v) => v/255)).getHexString();
        hexColors.push(`0x${hex}`);
      });
      console.log(hexColors.join('\n'));
    }
  },'save');
}

export default setupPaletteGUI;
