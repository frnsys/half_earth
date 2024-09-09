import HexSphere from './hex';
import Scene from './3d/scene';
import globeVert from './shaders/globe/vertex.glsl';
import globeFrag from './shaders/globe/fragment.glsl';
import cloudsVert from './shaders/clouds/vertex.glsl';
import cloudsFrag from './shaders/clouds/fragment.glsl';
import * as THREE from 'three';

const isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
const isSafari = navigator.userAgent.indexOf('Safari') > -1 && navigator.userAgent.indexOf('Chrome') <= -1;
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

  start() {
    this.active = true;
    this.render();
  }

  stop() {
    this.active = false;
    this.clear();
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

  setRotation(rotate) {
    this.rotate = rotate;
  }

  setClouds(visible) {
    this.clouds.visible = visible;
  }

  setZoom(zoom) {
    this.scene.camera.zoom = zoom;
    this.scene.camera.updateProjectionMatrix();
  }

  highlightRegion(regionName) {
    this.hexsphere.highlightRegion(regionName);
  }

  init(texPath) {
    this.surfaceTexture = texLoader.load(texPath);
    this.surfaceTexture.flipY = true;

    this.material = new THREE.ShaderMaterial({
      uniforms: {
        heightmap: {
          value: texLoader.load('/assets/surface/heightmap.png')
        },
        shadows: {
          value: texLoader.load('/assets/surface/shadows.png')
        },
        satTexture: {
          value: texLoader.load('/assets/surface/satellite.bw.jpg')
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
  }

  updateSurface(texPath) {
    this.surfaceTexture = texLoader.load(texPath);
    this.surfaceTexture.needsUpdate = true;
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

  showIconEvent(regionName, includeCoasts, icon, intensity) {
    let hexIdx = this.hexsphere.randomTileForRegion(regionName, includeCoasts);
    this.show({
      icon,
      hexIdx
    });

    // Also show discontent icon.
    let args = {
      icon: 'discontent',
      hexIdx,
      ping: true,
      iconSize: 0.35
    };
    this.show(args);

    if (intensity > 1) {
      let outlookInterval = setInterval(() => {
        if (intensity <= 0) {
          clearInterval(outlookInterval);
        } else {
          intensity--;
          this.show(args);
        }
      }, 250);
    }
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
