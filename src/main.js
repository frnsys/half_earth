import {createApp} from 'vue';
import {sign} from '/src/lib/util';
import consts from '/src/consts';
import assets from 'components/assets';
import tip from 'components/tip/directive';
import App from 'components/App.vue';
import debug from './debug';

const app = createApp(App);
app.directive('tip', tip);
app.config.globalProperties['consts'] = consts;
app.config.globalProperties['assets'] = assets;
app.config.globalProperties['debug'] = debug;
app.config.globalProperties['sign'] = sign;
app.mount('#main');

window.onbeforeunload = () => {
  return 'Are you sure you want to quit?';
};
