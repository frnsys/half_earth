import * as THREE from 'three';
import Hexasphere from 'hexasphere.js';

const tooltip = document.createElement('div');
tooltip.id = 'tooltip';
document.body.appendChild(tooltip);
tooltip.innerText = 'hello world\ntesting';
tooltip.style.padding = '0.25em 0.5em';
tooltip.style.background = '#fff';
tooltip.style.borderRadius = '0.5em';
tooltip.style.textAlign = 'center';
tooltip.style.position = 'fixed';
tooltip.style.display = 'none';

const texLoader = new THREE.TextureLoader();
const hexMaterial = new THREE.MeshBasicMaterial({color: 0xeeeeee, transparent: true});
const hexMaterialFocus = new THREE.MeshBasicMaterial({color: 0xff0000, transparent: true});
hexMaterial.opacity = 0.1;
hexMaterialFocus.opacity = 0.1;
const raycaster = new THREE.Raycaster();

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
  constructor(scene, radius, subdivisions, tileWidth) {

    this.selectables = [];
    this.scene = scene;
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
      scene.add(tile.mesh);

      let bnd = tile.boundary;
      let normal = calculateSurfaceNormal(bnd[1], bnd[2], bnd[3]);
      tile.normal = new THREE.Vector3(normal.x, normal.y, normal.z);

      let center = tile.centerPoint;
      tile.centerPointVec = new THREE.Vector3(center.x, center.y, center.z);
    });

    // Interaction
    this.mouse = new THREE.Vector2();
    scene.renderer.domElement.addEventListener('mousedown', this.onMouseDown.bind(this), false);
    scene.renderer.domElement.addEventListener('touchstart', this.onTouchStart.bind(this), false);
    scene.renderer.domElement.addEventListener('mousemove', this.onMouseMove.bind(this), false);
    this.scene.controls.onUpdate.push(() => {
      tooltip.style.display = 'none';
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
    this.selectables.push(sprite);
  }

  setMouse(ev) {
    // adjust browser mouse position for three.js scene
    this.mouse.x = ( ( ev.clientX - this.scene.renderer.domElement.offsetLeft ) / this.scene.renderer.domElement.clientWidth ) * 2 - 1;
    this.mouse.y = - ( ( ev.clientY - this.scene.renderer.domElement.offsetTop ) / this.scene.renderer.domElement.clientHeight ) * 2 + 1;
  }

  onMouseDown(ev) {
    ev.preventDefault();
    this.setMouse(ev);
    raycaster.setFromCamera(this.mouse, this.scene.camera);

    let intersects = raycaster.intersectObjects(this.selectables.filter(s => s.visible));
    if (intersects.length > 0) {
      // Rotate orbital controls camera to center on this point
      const mesh = intersects[0].object;
      const pos = mesh.position;
      const targetSpherical = new THREE.Spherical();
      targetSpherical.setFromCartesianCoords(pos.x, pos.y, pos.z);

      const orbit = this.scene.controls;
      orbit.sphericalDelta.phi = targetSpherical.phi - orbit.spherical.phi;
      orbit.sphericalDelta.theta = targetSpherical.theta - orbit.spherical.theta;
      orbit.update();
      intersects[0].object.parent.material = hexMaterialFocus;

      // TODO this gets the correct position but is hacky
      // WE can just flag an active tooltip and update its position
      // to the parent mesh on render
      setTimeout(() => {
        let screenPos = new THREE.Vector3();
        screenPos = screenPos.setFromMatrixPosition(mesh.matrixWorld);
        screenPos.project(this.scene.camera);

        let width = this.scene.renderer.domElement.clientWidth;
        let height = this.scene.renderer.domElement.clientHeight;
        let widthHalf = width / 2;
        let heightHalf = height / 2;

        screenPos.x = (screenPos.x * widthHalf) + widthHalf;
        screenPos.y = - (screenPos.y * heightHalf) + heightHalf;
        screenPos.z = 0;
        tooltip.style.display = 'block';
        let box = tooltip.getBoundingClientRect();
        tooltip.style.top = `${screenPos.y - box.height}px`;
        tooltip.style.left = `${screenPos.x - box.width/2}px`;
      }, 100);
    }
  }

  onTouchStart(ev) {
    ev.preventDefault();
    ev.clientX = ev.touches[0].clientX;
    ev.clientY = ev.touches[0].clientY;
    this.onMouseDown(ev);
  }

  onMouseMove(ev) {
    ev.preventDefault();
    this.setMouse(ev);
    raycaster.setFromCamera(this.mouse, this.scene.camera);

    let intersects = raycaster.intersectObjects(this.selectables);
    if (intersects.length > 0) {
      // let mesh = intersects[0].object,
      //     pos = intersects[0].point,
      //     obj = mesh.obj;
    }
  }
}

export default HexSphere;
