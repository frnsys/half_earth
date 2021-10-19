import RPC from './rpc';
import HexSphere from './hex';
import Scene from '../3d/scene';
import globeVert from './shaders/globe/vertex.glsl';
import globeFrag from './shaders/globe/fragment.glsl';
import cloudsVert from './shaders/clouds/vertex.glsl';
import cloudsFrag from './shaders/clouds/fragment.glsl';
import * as THREE from 'three';
import state from '../state';

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

class Globe {
  constructor(el) {
    let width = el.clientWidth;
    let height = el.clientHeight;
    this.scene = new Scene({
      width, height
    });
    el.appendChild(this.scene.renderer.domElement);
    this._onReady = [];
    this.pings = [];
  }

  onReady(fn) {
    this._onReady.push(fn);
  }

  async init() {
    let startYear = state.gameState.world.year;
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

    this.hexsphere = new HexSphere(this.scene, 5.2, 8, 0.98);

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
    let tgav = await this.surface.updateTemperature();
    await this.surface.updateBiomes(tgav);
    await this.updateSurface();
    return tgav;
  }

  showIconText(icon, text, hexIdx) {
    let iconMesh = this.hexsphere.showIcon(icon, hexIdx);
    let textMesh = this.hexsphere.showTextAt(text, hexIdx, 0.5);
    this.pings.push({text: textMesh, icon: iconMesh});
  }

  render(timestamp) {
    if (debug.fps) stats.begin();
    this.scene.render();
    if (this.cloudsMaterial) {
      this.cloudsMaterial.uniforms.time.value = timestamp;
    }
    if (debug.fps) stats.end();
    this.pings = this.pings.filter(({text, icon}) => {
      text.lookAt(this.scene.camera.position);
      text.position.y += 0.02;
      text.material.opacity -= 0.005;
      let done = text.material.opacity <= 0;
      if (done) {
        text.geometry.dispose();
        text.material.dispose();
        icon.geometry.dispose();
        icon.material.dispose();
        this.scene.remove(text);
        this.scene.remove(icon);
      }
      return !done;
    });
    requestAnimationFrame(this.render.bind(this));
  }
}

export default Globe;
