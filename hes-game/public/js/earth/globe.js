import RPC from './rpc';
import HexSphere from './hex';
import Scene from './3d/scene';
import globeVert from './shaders/globe/vertex.glsl';
import globeFrag from './shaders/globe/fragment.glsl';
import cloudsVert from './shaders/clouds/vertex.glsl';
import cloudsFrag from './shaders/clouds/fragment.glsl';
import * as THREE from 'three';

const isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
const isSafari = navigator.userAgent.indexOf('Safari') > -1 && navigator.userAgent.indexOf('Chrome') <= -1;
const Surface = RPC.initialize(
  new Worker(new URL('./surface.worker.js', import.meta.url))
);
const Temperature = RPC.initialize(
  new Worker(new URL('./temp.worker.js', import.meta.url))
);
const texLoader = new THREE.TextureLoader();
const objLoader = new THREE.ObjectLoader();


class Globe {
  constructor(el) {
    let width = el.clientWidth;
    let height = el.clientHeight;
    this.scene = new Scene({
      width, height
    });
    el.appendChild(this.scene.renderer.domElement);
    this._onReady = [];
    this._onClick = [];
    this.pings = [];
    this.icons = [];

    this.active = true;
    this.rotate = true;
    this.rotationPaused = false;
    this.pauseTimeout = null;
  }

  resetCamera() {
    this.scene.resetCamera();

    // Zoom out a bit more on
    // these devices to save resources
    if (isMobile || isSafari) {
      this.scene.camera.zoom = 0.06;
      this.scene.camera.updateProjectionMatrix();
    }
  }

  setEl(el) {
    el.appendChild(this.scene.renderer.domElement);
  }

  clear() {
    this.pings.forEach((mesh) => {
      mesh.geometry.dispose();
      mesh.material.dispose();
      mesh.parent.remove(mesh);
    });
    this.pings = [];

    this.icons.forEach((mesh) => {
      mesh.geometry.dispose();
      mesh.material.dispose();
      mesh.parent.remove(mesh);
    });
    this.icons = [];
  }

  onReady(fn) {
    this._onReady.push(fn);
  }

  onClick(fn) {
    this._onClick.push(fn);
  }

  async init(startYear) {
    this.temperature = await new Temperature(startYear);
    await this.temperature.init();

    if (!isMobile && !isSafari) {
      this.surface = await new Surface();
      await this.surface.init();

      let pixelsBuf = await this.surface.pixelsBuf;
      let width = await this.surface.width;
      let height = await this.surface.height;
      let pixels = new Uint8Array(pixelsBuf);
      this.surfaceTexture = new THREE.DataTexture(pixels, width, height, THREE.RGBFormat);
    } else {
      this.surfaceTexture = texLoader.load('./assets/surface/static_surface.png');
    }
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
    this.clouds = new THREE.Mesh(
      new THREE.SphereGeometry(5.3, 32, 32),
      this.cloudsMaterial
    );
    this.sphere.add(this.clouds);

    // Set up hexsphere for locating icons
    this.hexsphere = new HexSphere(this.scene, this.sphere, 5.4, 8, 0.98);
    this.hexsphere.onClick((intersects) => {
      // Pause rotation on click
      if (this.rotate) this.pauseRotation(2000);
      this._onClick.forEach((fn) => fn(intersects));
    });

    const canvas = this.scene.renderer.domElement;
    this.material.uniforms.screenRes.value.set(canvas.width, canvas.height, 1);

    this._onReady.forEach((fn) => fn(this));

    await this.updateSurface();
  }

  async updateSurface() {
    if (this.surface) {
      await this.surface.updateTexture();

      // Since SharedArrayBuffer support is lacking
      // in some mobile browsers, do this instead.
      // await this.surface.updateTexture();
      let newPixelsBuf = await this.surface.pixelsBuf;
      let newPixels = new Uint8Array(newPixelsBuf);

      this.surfaceTexture.image.data.set(newPixels);
      this.surfaceTexture.needsUpdate = true;

      // With SharedArrayBuffer we'd only need to do:
      // await this.surface.updateTexture();
      // this.surfaceTexture.needsUpdate = true;
    }
  }

  // Calculate new temperature anomaly
  // and update surface biomes/coloring accordingly.
  // See comments for Surface.addEmissions
  // for what `emissions` should look like.
  async addEmissionsThenUpdate(emissions) {
    await this.temperature.addEmissions(emissions);
    let tgav = await this.temperature.updateTemperature();
    if (this.surface) {
      await this.surface.updateBiomes(tgav);
      await this.updateSurface();
    }
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
    } else {
      if (textMesh) this.icons.push(textMesh);
      if (iconMesh) this.icons.push(iconMesh);
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
    if (this.sphere && this.rotate && !this.rotationPaused) {
      this.sphere.rotation.y += 0.003;
    }

    // Rotate orbital
    if (this.sphere && this.rotate && this.orbital) {
      this.orbital.rotation.z -= 0.01;
    }

    this.tickPings();

    if (this.active) {
      requestAnimationFrame(this.render.bind(this));
    }
  }
}

export { Globe };
