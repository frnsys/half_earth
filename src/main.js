import {createApp} from 'vue';
import {sign} from '/src/lib/util';
import consts from '/src/consts';
// import display from 'lib/display';
import icons from 'components/icons';
import tip from 'components/tip/directive';
import App from 'components/App.vue';
import debug from './debug';

import * as Sentry from "@sentry/browser";
import { Integrations } from "@sentry/tracing";
import {sessionId, startSession} from '/src/log';

if (process.env.NODE_ENV === 'production') {
  Sentry.init({
    dsn: "https://9c8cd525d7c64214836351b406f6e860@o545203.ingest.sentry.io/6087646",
    integrations: [new Integrations.BrowserTracing()],

    // Set tracesSampleRate to 1.0 to capture 100%
    // of transactions for performance monitoring.
    // We recommend adjusting this value in production
    tracesSampleRate: 1.0,
  });
  Sentry.setContext("session", {
    id: sessionId,
    version: VERSION,
  });
}

const app = createApp(App);
app.directive('tip', tip);
app.config.globalProperties['consts'] = consts;
app.config.globalProperties['icons'] = icons;
// app.config.globalProperties['display'] = display;
app.config.globalProperties['debug'] = debug;
app.config.globalProperties['sign'] = sign;
app.mount('#main');
startSession();

// window.onbeforeunload = () => {
//   return 'Are you sure you want to quit?';
// };
