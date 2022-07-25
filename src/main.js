import {createApp} from 'vue';
import consts from '/src/consts';
import icons from 'components/icons';
import format from '/src/display/format';
import factors from '/src/display/factors';
import display from '/src/display/display';
import tip from 'components/tip/directive';
import App from 'components/App.vue';
import debug from './debug';
import t, {loadLanguage} from '/src/i18n';

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

loadLanguage(() => {
  const app = createApp(App);
  app.directive('tip', tip);
  app.config.globalProperties['t'] = t;
  app.config.globalProperties['icons'] = icons;
  app.config.globalProperties['consts'] = consts;
  app.config.globalProperties['format'] = format;
  app.config.globalProperties['factors'] = factors;
  app.config.globalProperties['display'] = display;
  app.config.globalProperties['debug'] = debug;
  app.config.globalProperties['PLATFORM'] = PLATFORM;
  app.mount('#main');
  // startSession();
});

// window.onbeforeunload = () => {
//   return 'Are you sure you want to quit?';
// };
