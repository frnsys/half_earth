import {createApp} from 'vue';
import App from './components/App.vue';
import debug from './debug';

const app = createApp(App);
app.config.globalProperties['debug'] = debug;
app.mount('#main');
