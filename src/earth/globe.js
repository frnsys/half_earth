import RPC from './rpc';
import util from './util';
import HexSphere from './hex';
import Scene from '../3d/scene';
import vertexShader from './shaders/globe/vertex.glsl';
import fragmentShader from './shaders/globe/fragment.glsl';
import * as THREE from 'three';

// A grayscale image where each value
// indicates the label of that pixel
const biomeLabelsSrc = '/assets/grid_landuse.png';

const texLoader = new THREE.TextureLoader();

const Surface = RPC.initialize(
  new Worker(new URL('./worker.js', import.meta.url))
);

const startYear = 2020;

class Globe {
  constructor(el) {
    this.scene = new Scene({});
    el.appendChild(this.scene.renderer.domElement);
  }

  async init() {
    let {labels, size} = await util.loadPNG(biomeLabelsSrc);
    this.surface = await new Surface(startYear, labels, size);

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
        biomesTexture: {
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

    // TODO add test icons to sphere
    [52, 128, 191].forEach((idx) => hexsphere.showIcon('alert', idx));
    [238, 351].forEach((idx) => hexsphere.showIcon('advisor', idx));

    const canvas = this.scene.renderer.domElement;
    material.uniforms.screenRes.value.set(canvas.width, canvas.height, 1);

    await this.surface.updateTexture();
    surfaceTexture.needsUpdate = true;
  }

  // Calculate world update.
  // See comments for Surface.addEmissions
  // for what `emissions` should look like.
  async addEmissionsThenUpdate(emissions) {
    await this.surface.addEmissions(emissions);
    await this.surface.updateBiomes();
    surfaceTexture.needsUpdate = true;
  }

  render() {
    this.scene.render();
    requestAnimationFrame(this.render.bind(this));
  }
}

export default Globe;
