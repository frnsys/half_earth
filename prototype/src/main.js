import {createApp} from 'vue';
import consts from './consts'
import App from './components/App.vue'

const app = createApp(App);

Object.keys(consts).forEach((k) => {
  app.config.globalProperties[k] = consts[k];
});

app.mount('#main');