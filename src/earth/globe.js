import RPC from './rpc';
import HexSphere from './hex';
import Scene from '../3d/scene';
import vertexShader from './shaders/globe/vertex.glsl';
import fragmentShader from './shaders/globe/fragment.glsl';
import * as THREE from 'three';

import Stats from 'stats.js';

let stats = new Stats();
stats.showPanel(0);
document.body.appendChild(stats.dom);

const texLoader = new THREE.TextureLoader();

const Surface = RPC.initialize(
  new Worker(new URL('./worker.js', import.meta.url))
);

const startYear = 2020;

class Globe {
  constructor(el) {
    let width = el.clientWidth;
    let height = el.clientHeight;
    this.scene = new Scene({
      width, height
    });
    el.appendChild(this.scene.renderer.domElement);
    this._onReady = [];
  }

  onReady(fn) {
    this._onReady.push(fn);
  }

  async init() {
    this.surface = await new Surface(startYear);
    await this.surface.init();

    let pixelsBuf = await this.surface.pixelsBuf;
    let width = await this.surface.width;
    let height = await this.surface.height;
    let pixels = new Uint8Array(pixelsBuf);
    let surfaceTexture = new THREE.DataTexture(pixels, width, height, THREE.RGBFormat);
    surfaceTexture.flipY = true;

    this.material = new THREE.ShaderMaterial({
      uniforms: {
        time: {
          value: 0.0
        },
        heightmap: {
          value: texLoader.load('./assets/surface/heightmap.png')
        },
        shadows: {
          value: texLoader.load('./assets/surface/shadows.png')
        },
        satTexture: {
          value: texLoader.load('./assets/surface/satellite.bw.jpg')
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
      this.material
    );
    this.scene.add(sphere);

    this.hexsphere = new HexSphere(this.scene, 5.2, 12, 0.98);

    // TODO add test icons to sphere
    [52, 128, 191].forEach((idx) => this.hexsphere.showIcon('alert', idx));
    [238, 351].forEach((idx) => this.hexsphere.showIcon('advisor', idx));

    const canvas = this.scene.renderer.domElement;
    this.material.uniforms.screenRes.value.set(canvas.width, canvas.height, 1);

    this._onReady.forEach((fn) => fn(this));

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

  render(timestamp) {
    stats.begin();
    this.scene.render();
    if (this.material) {
      this.material.uniforms.time.value = timestamp;
    }
    stats.end();
    requestAnimationFrame(this.render.bind(this));
  }
}

export default Globe;
