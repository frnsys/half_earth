import {createApp} from 'vue';
import App from './components/App.vue';
import consts from './consts';
import debug from './debug';

const app = createApp(App);
Object.keys(consts).forEach((k) => {
  app.config.globalProperties[k] = consts[k];
});
app.config.globalProperties['debug'] = debug;
app.mount('#main');
