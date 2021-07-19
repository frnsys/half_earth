import * as Vue from 'vue';
import Stats from 'stats.js';
import Scene from './3d/scene';
import App from './components/App.vue'

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

const app = Vue.createApp(App);
app.mount('#main');
