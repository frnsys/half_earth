import * as THREE from 'three';
import Hexasphere from 'hexasphere.js';
import iconNames from '/assets/content/icons.json';
import debug from '../debug';

const raycaster = new THREE.Raycaster();
const vertAxis = new THREE.Vector3(0,1,0);

// For showing tile indices
const hexMaterial = new THREE.MeshBasicMaterial({color: 0xeeeeee, transparent: true, opacity: 0.5});

// For displaying text
const loader = new THREE.FontLoader();
let threeFont;
loader.load('/assets/fonts/helvetiker_bold.typeface.json', (font) => {
  threeFont = font;
});
const textMaterial = new THREE.MeshBasicMaterial({color: 0xEA060A, transparent: true});

// Load icons
const texLoader = new THREE.TextureLoader();
const icons = iconNames.concat(['political_capital', 'discontent', 'content']).reduce((acc, name) => {
  const map = texLoader.load(`./assets/icons/pips/${name}.png`);
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

function generateTileMesh(tile) {
  let geometry = new THREE.BufferGeometry();
  let vertices = new Float32Array(tile.boundary.map(
    (bp) => [bp.x, bp.y, bp.z]).flat());
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
  return new THREE.Mesh(geometry, hexMaterial);
}

class HexSphere {
  constructor(scene, parent, radius, subdivisions, tileWidth) {
    this.selectables = [];
    this.scene = scene;
    this.parent = parent;
    this.hexasphere = new Hexasphere(radius, subdivisions, tileWidth);
    this.hexasphere.tiles.forEach((tile, idx) => {
      // We don't really need to render the hexes,
      // and not rendering them saves many FPS
      if (debug.showTiles) {
        tile.mesh = generateTileMesh(tile);
        parent.add(tile.mesh);
      }

      let bnd = tile.boundary;
      let normal = calculateSurfaceNormal(bnd[1], bnd[2], bnd[3]);
      tile.normal = new THREE.Vector3(normal.x, normal.y, normal.z);

      let center = tile.centerPoint;
      tile.centerPointVec = new THREE.Vector3(center.x, center.y, center.z);

      if (debug.showTiles) {
        this.showText(`${idx}`, idx);
        this.selectables.push(tile.mesh);
      }
    });

    // Interaction
    this.mouse = new THREE.Vector2();
    scene.renderer.domElement.addEventListener(
      'mousedown', this.onMouseDown.bind(this), false);
    scene.renderer.domElement.addEventListener(
      'touchstart', this.onTouchStart.bind(this), false);
    this._onClick = [];
  }

  get tiles() {
    return this.hexasphere.tiles;
  }

  onClick(fn) {
    this._onClick.push(fn);
  }

  showIcon(iconName, hexIdx, {size, selectable}) {
    size = size || 0.75;
    let tile = this.hexasphere.tiles[hexIdx];
    let iconMat = icons[iconName];
    const sprite = new THREE.Sprite(iconMat.clone());
    sprite.scale.set(size, size, size);
    sprite.position.copy(
      tile.centerPointVec.add(tile.normal));

    this.parent.add(sprite);

    if (selectable) {
      sprite.userData.hexIdx = hexIdx;
      this.selectables.push(sprite);
    }
    return sprite;
  }

  showText(text, hexIdx, {size}) {
    size = size || 0.2;

    let tile = this.hexasphere.tiles[hexIdx];

    const textGeom = new THREE.TextGeometry(text, {
      size,
      font: threeFont,
      height: 0.05,
      curveSegments: 2,
      bevelEnabled: false,
    });

    // Center pivot
    textGeom.center();

    let label = new THREE.Mesh(textGeom, textMaterial.clone());

    label.position.copy(
      tile.centerPointVec.add(tile.normal.multiplyScalar(1.1)));
    label.lookAt(tile.normal);
    label.rotateOnAxis(vertAxis, Math.PI);
    this.parent.add(label);
    return label;
  }

  setMouse(ev) {
    // adjust browser mouse position for three.js scene
    this.mouse.x = (ev.offsetX / this.scene.renderer.domElement.clientWidth) * 2 - 1;
    this.mouse.y = -(ev.offsetY / this.scene.renderer.domElement.clientHeight) * 2 + 1;
  }

  onMouseDown(ev) {
    ev.preventDefault();
    this.setMouse(ev);
    raycaster.setFromCamera(this.mouse, this.scene.camera);

    let intersects = raycaster.intersectObjects(this.selectables.filter(s => s.visible));
    // if (intersects.length > 0) {
      // Rotate orbital controls camera to center on this point
      // const pos = mesh.position;
      // this.centerOnPosition(pos);
    // }
    this._onClick.forEach((fn) => fn(intersects));
  }

  centerOnIndex(idx) {
    let tile = this.hexasphere.tiles[idx];
    this.centerOnPosition(tile.centerPointVec);
  }

  centerOnPosition(pos) {
    const targetSpherical = new THREE.Spherical();
    targetSpherical.setFromCartesianCoords(pos.x, pos.y, pos.z);

    const orbit = this.scene.controls;
    orbit.sphericalDelta.phi = targetSpherical.phi - orbit.spherical.phi;
    orbit.sphericalDelta.theta = targetSpherical.theta - orbit.spherical.theta;
    orbit.update();
  }

  onTouchStart(ev) {
    ev.preventDefault();
    ev.clientX = ev.touches[0].clientX;
    ev.clientY = ev.touches[0].clientY;
    this.onMouseDown(ev);
  }
}

export default HexSphere;
