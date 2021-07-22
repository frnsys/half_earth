// Tree-shaking doesn't really work with three.js,
// so the workaround is to manually export only what we need.
// The process here is:
//  1. Load the page. See if any errors resulting from missing three.js imports
//  2. Refer to <https://github.com/mrdoob/three.js/blob/2510609955cd3710b267c8aba69ce1ef6ba97ed6/src/Three.js> and add in the missing export.

export * from 'three/src/constants.js';
export * from 'three/src/core/BufferAttribute.js';
export { Raycaster } from 'three/src/core/Raycaster.js';
export { TextureLoader } from 'three/src/loaders/TextureLoader.js';
export { EventDispatcher } from 'three/src/core/EventDispatcher.js';
export { Scene } from 'three/src/scenes/Scene.js';
export { WebGLRenderer } from 'three/src/renderers/WebGLRenderer.js';
export { HemisphereLight } from 'three/src/lights/HemisphereLight.js';
export { OrthographicCamera } from 'three/src/cameras/OrthographicCamera.js';
export { Vector2 } from 'three/src/math/Vector2.js';
export { Vector3 } from 'three/src/math/Vector3.js';
export { Quaternion } from 'three/src/math/Quaternion.js';
export { Spherical } from 'three/src/math/Spherical.js';
export { DataTexture } from 'three/src/textures/DataTexture.js';
export { Mesh } from 'three/src/objects/Mesh.js';
export { MeshBasicMaterial, SpriteMaterial, ShaderMaterial } from 'three/src/materials/Materials.js';
export { SphereGeometry } from 'three/src/geometries/Geometries.js';
export { BufferGeometry } from 'three/src/core/BufferGeometry.js';
export { Sprite } from 'three/src/objects/Sprite.js';
