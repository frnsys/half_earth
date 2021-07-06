import Scene from './3d/scene';
import globe from './globe/main';
import makeCards from './cards/main';
import hexgrid from './hexgrid/main';
import Stats from 'stats.js';
import * as Vue from 'vue';
import App from './components/App.vue'

// var stats = new Stats();
// stats.showPanel(0); // fps
// document.body.appendChild(stats.dom);

// const scene = new Scene({});
// const main = document.getElementById('main');
// main.appendChild(scene.renderer.domElement);

// // hexgrid(scene);
// globe(scene);

// function render() {
//   stats.begin();
//   scene.render();
// 	stats.end();
//   requestAnimationFrame(render);
// }
// render();

// makeCards();

const app = Vue.createApp(App);
app.mount('#main');
