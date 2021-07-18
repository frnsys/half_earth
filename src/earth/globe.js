import util from './util';
import * as THREE from 'three';
import Scene from '../3d/scene';
import Surface from './surface';
import HexSphere from './hexsphere';
import vertexShader from './shaders/globe/vertex.glsl';
import fragmentShader from './shaders/globe/fragment.glsl';
import RPC from './rpc';

const texLoader = new THREE.TextureLoader();

// A grayscale image where each value
// indicates the label of that pixel
const biomeLabelsSrc = '/assets/grid_landuse.png';

// const worker = new Worker(new URL('./worker.js', import.meta.url));

// let tex;
// worker.addEventListener('message', ({data}) => {
//   console.log(`worker sent: ${data}`);
//   console.log(data);
//   if (data.uuid) {
//     tex = data;
//     console.log(tex.flipY);
//   }
// });
// setTimeout(() => {
//   console.log('SENDING MESSAGE');
//   worker.postMessage('hello there');
// }, 5000);
// setTimeout(() => {
//   console.log(tex.flipY);
// }, 10000);

// <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy>
// function setup(worker) {
//   let id = 0;
//   const proxy = {};
//   worker.postMessage(['init']);
//   return new Promise((resolve, reject) => {
//     worker.addEventListener('message', (msg) => {
//       const [key, data] = msg.data;
//       if (key == 'setup') {
//         console.log(data);
//         data.forEach((method) => {
//           proxy[method] = function(id) {
//             postMessage([method, [...arguments]]);
//           }
//         });
//         resolve(proxy);
//       } else if (key == 'log') {
//         console.log(`worker: ${data}`);
//       }
//     });
//   });
//   // }).then((proxy) => {
//   //   return () => {
//   //     return {
//   //       id: id++
//   //     }
//   //   }
//   // });
// }

// setup(worker).then((proxy) => {;
//   console.log(proxy);
//   proxy.test(4, 7);
// });

// const Surface = Comlink.wrap(
//   new Worker(new URL('./worker.js', import.meta.url))
// );

// const Surface = Comlink.wrap(worker);

const TestSurface = RPC.initialize(
  new Worker(new URL('./worker.js', import.meta.url))
);

class Globe {
  constructor(el) {
    this.scene = new Scene({});
    el.appendChild(this.scene.renderer.domElement);
  }

  async init() {
    let {labels, size} = await util.loadPNG(biomeLabelsSrc);
    this.surface = await TestSurface.construct(labels, size);

    let pixelsBuf = await this.surface.pixelsBuf;
    let width = await this.surface.width;
    let height = await this.surface.height;
    let pixels = new Uint8Array(pixelsBuf);
    let surfaceTexture = new THREE.DataTexture(pixels, width, height, THREE.RGBFormat);
    surfaceTexture.flipY = true;

    const material = new THREE.ShaderMaterial({
      uniforms: {
        heightmap: {
          value: texLoader.load('./assets/heightmap.png')
        },
        shadows: {
          value: texLoader.load('./assets/shadows.processed.png')
        },
        satTexture: {
          value: texLoader.load('./assets/satellite.bw.jpg')
        },
        labelsTexture: {
          value: surfaceTexture
        },
        screenRes: {
          value: new THREE.Vector3()
        }
      },
      vertexShader: vertexShader,
      fragmentShader: fragmentShader
    });

    const sphere = new THREE.Mesh(
      new THREE.SphereGeometry(5, 256, 256),
      material
    );
    this.scene.add(sphere);

    const hexsphere = new HexSphere(this.scene, 5.2, 12, 0.98);
    [52, 128, 191].forEach((idx) => hexsphere.showIcon('alert', idx));
    [238, 351].forEach((idx) => hexsphere.showIcon('advisor', idx));

    const canvas = this.scene.renderer.domElement;
    material.uniforms.screenRes.value.set(canvas.width, canvas.height, 1);

    await this.surface.updateTexture();
    surfaceTexture.needsUpdate = true;
    console.log(pixels);

    // util.loadPNG(biomeLabelsSrc).then(({labels, size}) => {
    //   this.surface = new Surface(labels, size);


    //   const material = new THREE.ShaderMaterial({
    //     uniforms: {
    //       heightmap: {
    //         value: texLoader.load('./assets/heightmap.png')
    //       },
    //       shadows: {
    //         value: texLoader.load('./assets/shadows.processed.png')
    //       },
    //       satTexture: {
    //         value: texLoader.load('./assets/satellite.bw.jpg')
    //       },
    //       labelsTexture: {
    //         value: this.surface.texture
    //       },
    //       screenRes: {
    //         value: new THREE.Vector3()
    //       }
    //     },
    //     vertexShader: vertexShader,
    //     fragmentShader: fragmentShader
    //   });

    //   const sphere = new THREE.Mesh(
    //     new THREE.SphereGeometry(5, 256, 256),
    //     material
    //   );
    //   this.scene.add(sphere);

    //   const hexsphere = new HexSphere(this.scene, 5.2, 12, 0.98);
    //   [52, 128, 191].forEach((idx) => hexsphere.showIcon('alert', idx));
    //   [238, 351].forEach((idx) => hexsphere.showIcon('advisor', idx));

    //   const canvas = this.scene.renderer.domElement;
    //   material.uniforms.screenRes.value.set(canvas.width, canvas.height, 1);

    //   this.surface.updateTexture();
    // });
  }

  render() {
    // stats.begin();
    this.scene.render();
    // stats.end();
    requestAnimationFrame(this.render.bind(this));
  }
}

export default Globe;
