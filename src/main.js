import {createApp} from 'vue';
import App from './components/App.vue';
import consts from './consts';

const app = createApp(App);
Object.keys(consts).forEach((k) => {
  app.config.globalProperties[k] = consts[k];
});
app.mount('#main');
