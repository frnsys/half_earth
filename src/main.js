import * as Vue from 'vue';
import Stats from 'stats.js';
import Scene from './3d/scene';
import App from './components/App.vue'

import loadHector from 'hector-wasm';
import scenario from 'hector-wasm/scenarios/rcp45';
import defaultConfig from 'hector-wasm/config/defaultConfig';

let outputVars = {
  "temperature.Tgav": {
    "component": "temperature",
    "description": "global atmospheric temperature anomaly",
    "unit": "degC",
    "variable": "Tgav"
  }
};
loadHector().then(({Hector, run}) => {
  console.log(`Hector version ${Hector.version()}`);

  console.log('Running...');
  var t0 = performance.now()
  let results = run(defaultConfig, scenario, outputVars);
  var t1 = performance.now()
  console.log(`Done running in ${t1 - t0}ms`);
});

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
