import * as THREE from 'three';
import Scene from '../3d/scene';
import HexSphere from './hexsphere';
import loadLabelsTexture from './labels';
import vertexShader from './shaders/globe/vertex.glsl';
import fragmentShader from './shaders/globe/fragment.glsl';

const texLoader = new THREE.TextureLoader();

class Globe {
  constructor(el) {
    this.scene = new Scene({});
    el.appendChild(this.scene.renderer.domElement);

    loadLabelsTexture('./assets/grid_landuse.png').then((labels) => {
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
            value: labels.generateTexture(labels.colors)
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
    });
  }

  render() {
    // stats.begin();
    this.scene.render();
    // stats.end();
    requestAnimationFrame(this.render.bind(this));
  }
}

export default Globe;
