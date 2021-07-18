import Scene from './3d/scene';
import Stats from 'stats.js';
import App from './components/App.vue'
// import * as Vue from 'vue';

import Globe from './earth/globe';
const globe = new Globe(document.getElementById('main'));
globe.render();
globe.init();

// import hexgrid from './hexgrid/main';
// hexgrid(scene);

// var stats = new Stats();
// stats.showPanel(0); // fps
// document.body.appendChild(stats.dom);

// const scene = new Scene({});
// const main = document.getElementById('main');
// main.appendChild(scene.renderer.domElement);

// function render() {
//   stats.begin();
//   scene.render();
// 	stats.end();
//   requestAnimationFrame(render);
// }
// render();

// const app = Vue.createApp(App);
// app.mount('#main');
