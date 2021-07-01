import * as THREE from 'three';
import HexSphere from './hexsphere';
import setupPaletteGUI from './design';
import loadLabelsTexture from './labels';
import vertexShader from './shaders/globe/vertex.glsl';
import fragmentShader from './shaders/globe/fragment.glsl';

const texLoader = new THREE.TextureLoader();

export default (scene) => {
  loadLabelsTexture('./assets/grid_landuse.png').then((labels) => {
    const material = new THREE.ShaderMaterial({
      uniforms: {
        heightmap: {
          value: texLoader.load('./assets/heightmap.png')
        },
        satTexture: {
          value: texLoader.load('./assets/satellite.bw.png')
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

    setupPaletteGUI(labels.colors, (colors) => {
      material.uniforms.labelsTexture.value = labels.generateTexture(colors);
    });

    const sphere = new THREE.Mesh(
      new THREE.SphereGeometry(5, 128, 128),
      material
    );
    scene.add(sphere);

    const hexsphere = new HexSphere(5.2, 12, 0.98);
    hexsphere.tiles.forEach((tile) => scene.add(tile.mesh));
    [52, 128, 191].forEach((idx) => hexsphere.showIcon('alert', idx));
    [238, 351].forEach((idx) => hexsphere.showIcon('advisor', idx));

    const canvas = scene.renderer.domElement;
    material.uniforms.screenRes.value.set(canvas.width, canvas.height, 1);
  });
};
