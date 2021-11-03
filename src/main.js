import {sign} from './lib/util';
import {createApp} from 'vue';
import App from './components/App.vue';
import debug from './debug';

const app = createApp(App);
app.config.globalProperties['debug'] = debug;
app.config.globalProperties['sign'] = sign;
app.mount('#main');

window.onbeforeunload = () => {
  return 'Are you sure you want to quit?';
};
