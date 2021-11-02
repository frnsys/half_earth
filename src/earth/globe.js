import RPC from './rpc';
import HexSphere from './hex';
import Scene from '../3d/scene';
import globeVert from './shaders/globe/vertex.glsl';
import globeFrag from './shaders/globe/fragment.glsl';
import cloudsVert from './shaders/clouds/vertex.glsl';
import cloudsFrag from './shaders/clouds/fragment.glsl';
import * as THREE from 'three';
import state from '/src/state';
import game from '/src/game';

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

    this.pauseRotation = false;
    this.pauseTimeout = null;
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

    // Create the earth
    this.sphere = new THREE.Mesh(
      new THREE.SphereGeometry(5, 32, 32),
      this.material
    );
    this.scene.add(this.sphere);

    // Set up hexsphere for locating icons
    this.hexsphere = new HexSphere(this.scene, this.sphere, 5.2, 8, 0.98);
    this.hexsphere.onClick((intersects) => {
      // Pause rotation on click
      if (intersects.length === 0) {
        if (this.pauseTimeout) clearTimeout(this.pauseTimeout);
        this.pauseRotation = true;
        this.pauseTimeout = setTimeout(() => {
          this.pauseRotation = false;
        }, 2000);
      } else {
        intersects.forEach((intersect) => {
          let mesh = intersect.object;
          let hexIdx = mesh.userData.hexIdx;
          this.respondToEvent(mesh, hexIdx, mesh.userData);
        });
      }
    });

    // Create the clouds layer
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
    this.sphere.add(clouds);

    const canvas = this.scene.renderer.domElement;
    this.material.uniforms.screenRes.value.set(canvas.width, canvas.height, 1);

    this._onReady.forEach((fn) => fn(this));

    await this.updateSurface();
  }

  respondToEvent(mesh, hexIdx, userData) {
    let pc = userData.event.intensity * 2;
    let outlook = userData.event.intensity;
    game.changePoliticalCapital(pc);
    game.changeLocalOutlook(outlook, userData.region.id);

    this.pingIcon('content', hexIdx);
    let outlookInterval = setInterval(() => {
      if (outlook <= 0) {
        clearInterval(outlookInterval);
      } else {
        outlook--;
        this.pingIcon('content', hexIdx);
      }
    }, 250);

    setTimeout(() => {
      this.pingIcon('political_capital', hexIdx);
      let pcInterval = setInterval(() => {
        if (pc <= 0) {
          clearInterval(pcInterval);
        } else {
          pc--;
          this.pingIcon('political_capital', hexIdx);
        }
      }, 250);
    }, 500);

    mesh.visible = false;
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

  // Calculate new temperature anomaly
  // and update surface biomes/coloring accordingly.
  // See comments for Surface.addEmissions
  // for what `emissions` should look like.
  async addEmissionsThenUpdate(emissions) {
    await this.surface.addEmissions(emissions);
    let tgav = await this.surface.updateTemperature();
    await this.surface.updateBiomes(tgav);
    await this.updateSurface();
    return tgav;
  }

  // Show an icon and ping text
  // at the specified hex
  showIconText(iconName, text, hexIdx) {
    let iconMesh = this.hexsphere.showIcon(iconName, hexIdx, 0.75, true);
    let textMesh = this.hexsphere.showTextAt(text, hexIdx, 0.5);
    this.pings.push({mesh: textMesh, icon: iconMesh});
    return iconMesh;
  }

  showIcon(iconName, hexIdx, data) {
    let iconMesh = this.hexsphere.showIcon(iconName, hexIdx, 0.75, true);
    iconMesh.userData = {...data, ...iconMesh.userData};
    this.pings.push({mesh: null, icon: iconMesh});
    return iconMesh;
  }

  pingIcon(iconName, hexIdx) {
    let iconMesh = this.hexsphere.showIcon(iconName, hexIdx, 0.5);
    this.pings.push({mesh: iconMesh, icon: null});
    return iconMesh;
  }

  tickPings() {
    // Update pings
    this.pings = this.pings.filter(({mesh, icon}) => {
      // Keep text facing the camera
      if (mesh) {
        mesh.lookAt(this.scene.camera.position);

        // Move text pings up and fade out
        mesh.position.y += 0.02;
        mesh.material.opacity -= 0.0005;
      }

      if (icon) {
        icon.material.opacity -= 0.0005;
      }

      let done = icon ? icon.material.opacity <= 0 : mesh.material.opacity <= 0;
      if (done) {
        if (mesh) {
          mesh.geometry.dispose();
          mesh.material.dispose();
          mesh.parent.remove(mesh);
        }
        if (icon) {
          icon.geometry.dispose();
          icon.material.dispose();
          icon.parent.remove(icon);
        }
      }
      return !done;
    });
  }

  render(timestamp) {
    if (debug.fps) stats.begin();

    this.scene.render();

    // Animate clouds
    if (this.cloudsMaterial) {
      this.cloudsMaterial.uniforms.time.value = timestamp;
    }

    // Rotate world
    if (this.sphere && !this.pauseRotation) {
      this.sphere.rotation.y += 0.003;
    }
    this.tickPings();

    if (debug.fps) stats.end();

    requestAnimationFrame(this.render.bind(this));
  }
}

export default Globe;
