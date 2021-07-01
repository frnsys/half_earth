import * as THREE from 'three';
import Hexasphere from 'hexasphere.js';

const texLoader = new THREE.TextureLoader();
const hexMaterial = new THREE.MeshBasicMaterial({color: 0xeeeeee, transparent: true});
hexMaterial.opacity = 0.1;

const iconNames = ['alert', 'advisor'];
const icons = iconNames.reduce((acc, name) => {
  const map = texLoader.load(`./assets/icons/${name}.png`);
  const iconMat = new THREE.SpriteMaterial({map});
  acc[name] = iconMat;
  return acc;
}, {});

function vector(p1, p2) {
  return {
    x: p2.x - p1.x,
    y: p2.y - p1.y,
    z: p2.z - p1.z
  }
}

function calculateSurfaceNormal(p1, p2, p3) {
  U = vector(p1, p2)
  V = vector(p1, p3)
  N = {
    x: U.y * V.z - U.z * V.y,
    y: U.z * V.x - U.x * V.z,
    z: U.x * V.y - U.y * V.x
  };
  return N;
}

class HexSphere {
  constructor(radius, subdivisions, tileWidth) {
    this.hexasphere = new Hexasphere(radius, subdivisions, tileWidth);
    this.hexasphere.tiles.forEach((tile) => {
      let geometry = new THREE.BufferGeometry();
      let vertices = new Float32Array(tile.boundary.map((bp) => [bp.x, bp.y, bp.z]).flat());
      geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));

      // Create faces
      if (tile.boundary.length > 5) {
        // Hexagon
        geometry.setIndex([
          0, 1, 2,
          0, 2, 3,
          3, 5, 0,
          3, 4, 5,
        ]);
      } else {
        // Pentagon
        geometry.setIndex([
          0, 1, 2,
          0, 2, 3,
          0, 3, 4,
        ]);
      }

      let mesh = new THREE.Mesh(geometry, hexMaterial);
      tile.mesh = mesh;

      let bnd = tile.boundary;
      let normal = calculateSurfaceNormal(bnd[1], bnd[2], bnd[3]);
      tile.normal = new THREE.Vector3(normal.x, normal.y, normal.z);

      let center = tile.centerPoint;
      tile.centerPointVec = new THREE.Vector3(center.x, center.y, center.z);
    });
  }

  get tiles() {
    return this.hexasphere.tiles;
  }

  // TODO col/row addressing
  showIcon(iconName, hexIdx) {
    let tile = this.hexasphere.tiles[hexIdx];
    let iconMat = icons[iconName];
    const sprite = new THREE.Sprite(iconMat);
    sprite.scale.set(0.5, 0.5, 0.5);
    sprite.position.copy(
      tile.centerPointVec.add(tile.normal.multiplyScalar(2.)));
    tile.mesh.add(sprite);
  }
}

export default HexSphere;
