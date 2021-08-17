import RPC from './rpc';
import HexSphere from './hex';
import Scene from '../3d/scene';
import globeVert from './shaders/globe/vertex.glsl';
import globeFrag from './shaders/globe/fragment.glsl';
import cloudsVert from './shaders/clouds/vertex.glsl';
import cloudsFrag from './shaders/clouds/fragment.glsl';
import * as THREE from 'three';

import debug from '../debug';
import Stats from 'stats.js';

if (process.env.NODE_ENV === 'development') {
  console.log('this only shows up in dev builds');
}

let stats;
if (debug.fps) {
  stats = new Stats();
  stats.showPanel(0);
  document.body.appendChild(stats.dom);
}

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
    this.surfaceTexture = new THREE.DataTexture(pixels, width, height, THREE.RGBFormat);
    this.surfaceTexture.flipY = true;

    this.material = new THREE.ShaderMaterial({
      uniforms: {
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
          value: this.surfaceTexture
        },
        screenRes: {
          value: new THREE.Vector3()
        }
      },
      vertexShader: globeVert,
      fragmentShader: globeFrag
    });

    const sphere = new THREE.Mesh(
      new THREE.SphereGeometry(5, 32, 32),
      this.material
    );
    this.scene.add(sphere);

    this.hexsphere = new HexSphere(this.scene, 5.2, 12, 0.98);

    // TODO add test icons to sphere
    [52, 32, 191].forEach((idx) => this.hexsphere.showIcon('alert', idx));
    [238, 351].forEach((idx) => this.hexsphere.showIcon('advisor', idx));

    this.cloudsMaterial = new THREE.ShaderMaterial({
      uniforms: {
        time: {
          value: 0.0
        }
      },
      vertexShader: cloudsVert,
      fragmentShader: cloudsFrag
    });
    this.cloudsMaterial.transparent = true;
    const clouds = new THREE.Mesh(
      new THREE.SphereGeometry(5.3, 32, 32),
      this.cloudsMaterial
    );
    this.scene.add(clouds);

    const canvas = this.scene.renderer.domElement;
    this.material.uniforms.screenRes.value.set(canvas.width, canvas.height, 1);

    this._onReady.forEach((fn) => fn(this));

    await this.updateSurface();
  }

  async updateSurface() {
    // Since SharedArrayBuffer support is lacking
    // in some mobile browsers, do this instead.
    await this.surface.updateTexture();
    let newPixelsBuf = await this.surface.pixelsBuf;
    let newPixels = new Uint8Array(newPixelsBuf);
    this.surfaceTexture.image.data.set(newPixels);
    this.surfaceTexture.needsUpdate = true;

    // With SharedArrayBuffer we'd only need to do:
    // await this.surface.updateTexture();
    // this.surfaceTexture.needsUpdate = true;
  }

  // Calculate world update.
  // See comments for Surface.addEmissions
  // for what `emissions` should look like.
  async addEmissionsThenUpdate(emissions) {
    await this.surface.addEmissions(emissions);
    await this.surface.updateBiomes();
    await this.updateSurface();
  }

  render(timestamp) {
    if (debug.fps) stats.begin();
    this.scene.render();
    if (this.cloudsMaterial) {
      this.cloudsMaterial.uniforms.time.value = timestamp;
    }
    if (debug.fps) stats.end();
    requestAnimationFrame(this.render.bind(this));
  }
}

export default Globe;
