import RPC from './rpc';
import HexSphere from './hex';
import Scene from '../3d/scene';
import globeVert from './shaders/globe/vertex.glsl';
import globeFrag from './shaders/globe/fragment.glsl';
import cloudsVert from './shaders/clouds/vertex.glsl';
import cloudsFrag from './shaders/clouds/fragment.glsl';
import * as THREE from 'three';
import state from '/src/state';

const Surface = RPC.initialize(
  new Worker(new URL('./worker.js', import.meta.url))
);
const texLoader = new THREE.TextureLoader();

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

    this.rotationPaused = false;
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
        this.pauseRotation(2000);
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

  // Show/ping an icon and/or text
  // at the specified hex
  show({icon, text, hexIdx, ping, iconSize}) {
    let textMesh = text ? this.hexsphere.showText(text, hexIdx, {
      size: 0.5
    }) : null;
    let iconMesh = icon ? this.hexsphere.showIcon(icon, hexIdx, {
      size: iconSize || 0.6,
      selectable: true
    }) : null;
    if (ping) {
      if (textMesh) this.pings.push(textMesh);
      if (iconMesh) this.pings.push(iconMesh);
    }
    return {textMesh, iconMesh};
  }

  tickPings() {
    // Update pings
    this.pings = this.pings.filter((mesh) => {
      // Keep text facing the camera
      mesh.lookAt(this.scene.camera.position);

      // Move text pings up and fade out
      mesh.position.y += 0.02;
      mesh.material.opacity -= 0.001;

      let done = mesh.material.opacity <= 0;
      if (done) {
        mesh.geometry.dispose();
        mesh.material.dispose();
        mesh.parent.remove(mesh);
      }
      return !done;
    });
  }

  pauseRotation(countdown) {
    if (this.pauseTimeout) clearTimeout(this.pauseTimeout);
    this.rotationPaused = true;
    if (countdown) {
      this.pauseTimeout = setTimeout(() => {
        this.rotationPaused = false;
      }, countdown);
    }
  }

  resumeRotation() {
    this.rotationPaused = false;
  }

  render(timestamp) {
    this.scene.render();

    // Animate clouds
    if (this.cloudsMaterial) {
      this.cloudsMaterial.uniforms.time.value = timestamp;
    }

    // Rotate world
    if (this.sphere && !this.rotationPaused) {
      this.sphere.rotation.y += 0.003;
    }
    this.tickPings();

    requestAnimationFrame(this.render.bind(this));
  }
}

export default Globe;
