import * as THREE from 'three';
import {shadeColor} from './color';

const geoCache = {};
const colorCache = {};
const matConf = { vertexColors: THREE.VertexColors };
// const matConf = { color: 0x0000ff };
const material = new THREE.MeshBasicMaterial(matConf);


class Cell {
  constructor(x, y, size, color, data) {
    this.x = x;
    this.y = y;
    this.data = data || {};
    this.color = color;

    let key = size.toString();
    if (!(key in geoCache)) {
      geoCache[key] = makeHexagon(size);
    }

    this.geometry = geoCache[key];
    this.mesh = new THREE.Mesh(this.geometry, material);
    this.mesh.position.x = x;
    this.mesh.position.y = y;
    this.setColor(color);

    // to recover this object from raycasting intersection
    this.mesh.obj = this;
  }

  // color order:
  // top right, top center, top left, bottom left, bottom center, bottom right
  setColor(color) {
    let colors = [
      color,
      color,
      shadeColor(color, 0.3),
      color,
      color,
      shadeColor(color, -0.2),
    ];

    colors = colors.map((c) => {
      if (!(c in colorCache)) {
        colorCache[c] = new THREE.Color(c);
      }
      return colorCache[c];
    });

    const count = this.geometry.attributes.position.count;
    this.geometry.setAttribute( 'color', new THREE.BufferAttribute( new Float32Array( count * 3 ), 3 ) );
    for (let i=0; i<count; i++) {
      this.geometry.attributes.color.setXYZ(i, colors[i].r, colors[i].g, colors[i].b);
    }
    this.geometry.elementsNeedUpdate = true;
  }

  focus() {
    this.setColor(0xf99090);
  }

  unfocus() {
    this.setColor(this.color);
  }
}


function makeHexagon(size) {
  let vertices = [];
  for (let i=0; i<6; i++) {
    let angle_deg = 60 * i + 30;
    let angle_rad = Math.PI / 180 * angle_deg;
    let vx = size * Math.cos(angle_rad);
    let vy = size * Math.sin(angle_rad);
    vertices.push(vx, vy, 0);
  }
  let geometry = new THREE.BufferGeometry();
  geometry.setAttribute(
    'position', new THREE.BufferAttribute(new Float32Array(vertices), 3));

  // Connect vertices to form triangles
  geometry.setIndex([
    0, 1, 2,
    0, 2, 3,
    3, 5, 0,
    3, 4, 5,
  ]);
  return geometry;
}


export default Cell;