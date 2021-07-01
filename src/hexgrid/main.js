import Grid from './grid';
import * as THREE from 'three';

export default (scene) => {
  const cellSize = 10;
  const nCols = 10;
  const nRows = 10;
  const grid = new Grid(nCols, nRows, cellSize);
  scene.add(grid.group);

  for (let col=0; col<nCols; col++) {
    for (let row=0; row<nRows; row++) {
      let cell = grid.setCellAt(col, row, 0xff0000);

      // Outline cells
      const edges = new THREE.EdgesGeometry( cell.geometry );
      const line = new THREE.LineSegments( edges, new THREE.LineBasicMaterial( { color: 0xffffff } ) );
      line.position.x = cell.mesh.position.x;
      line.position.y = cell.mesh.position.y;
      scene.add(line);
    }
  }
}
