import {createApp} from 'vue';
import {sign} from '/src/lib/util';
import consts from '/src/consts';
// import display from 'lib/display';
import icons from 'components/icons';
import tip from 'components/tip/directive';
import App from 'components/App.vue';
import debug from './debug';

const app = createApp(App);
app.directive('tip', tip);
app.config.globalProperties['consts'] = consts;
app.config.globalProperties['icons'] = icons;
// app.config.globalProperties['display'] = display;
app.config.globalProperties['debug'] = debug;
app.config.globalProperties['sign'] = sign;
app.mount('#main');

// window.onbeforeunload = () => {
//   return 'Are you sure you want to quit?';
// };
