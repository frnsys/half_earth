import * as THREE from 'three';
import OrbitControls from './orbit';

const NEAR = 0;
const FAR = 10000;
const D = 1;

class Scene {
  constructor(opts) {
    let width = opts.width || window.innerWidth;
    let height = opts.height || window.innerHeight;
    opts.brightness = opts.brightness || 0.5;
    this.opts = opts;

    this.scene = new THREE.Scene();
    this.renderer = new THREE.WebGLRenderer({
      antialias: false,
      alpha: true,
      preserveDrawingBuffer: false
    });
    this.renderer.setPixelRatio(window.devicePixelRatio);
    this.renderer.setSize(width, height);
    // this.renderer.setClearColor(0xeeeeee, 1);
    // this.renderer.setClearColor(0xffffe8, 1);

    let hemiLight = new THREE.HemisphereLight(0xeeeeee, 0x000000, opts.brightness);
    this.scene.add(hemiLight);
    this.hemiLight = hemiLight;
    hemiLight.baseIntensity = hemiLight.intensity;

    // soft white light, to fill shadows
    // let ambiLight = new THREE.AmbientLight( 0x999999, 1 );
    // this.scene.add(ambiLight);
    // this.ambiLight = ambiLight;
    // ambiLight.baseIntensity = ambiLight.intensity;

    // Prefer to not use this for performance
    // let light = new THREE.DirectionalLight( 0xffffff, 0.1 );
    // light.position.y = 200;
    // light.baseIntensity = light.intensity;
    // this.scene.add(light);
    // this.sun = light;

    let aspect = width/height;
    this.camera = new THREE.OrthographicCamera(-D*aspect, D*aspect, D, -D, NEAR, FAR);
    this.resetCamera();

    window.addEventListener('resize', () => {
      this.resize();
    }, false);

    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableRotate = true;
    this.controls.enablePan = false;
    this.controls.maxZoom = 0.6;
    this.controls.minZoom = 0.001;
    // this.controls.maxPolarAngle = Math.PI/2;
  }

  add(mesh) {
    this.scene.add(mesh);
  }

  remove(mesh) {
    this.scene.remove(mesh);
  }

  render() {
    this.renderer.render(this.scene, this.camera);
  }

  resetCamera() {
    this.camera.zoom = 0.08;
    this.camera.position.z = 200;
    this.camera.position.y = 0;
    this.camera.position.x = 0;
    this.camera.lookAt(this.scene.position);
    this.camera.updateProjectionMatrix();
  }

  resize() {
    let parent = this.renderer.domElement.parentElement;
    let width = parent.clientWidth;
    let height = parent.clientHeight;
    let aspect = width/height;
    this.camera.left = -D * aspect;
    this.camera.right = D * aspect;
    this.camera.top = D;
    this.camera.bottom = -D;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height);
  }
}


export default Scene;
