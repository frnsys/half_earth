import path from 'path';
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

import * as Sentry from "@sentry/vue";
import { Wasm as WasmIntegration } from "@sentry/wasm";
import {sessionId, startSession} from '/src/log';

// Older browers don't support replaceAll,
// kind of hacky sticking it here but...
/**
 * String.prototype.replaceAll() polyfill
 * https://gomakethings.com/how-to-replace-a-section-of-a-string-with-another-one-with-vanilla-js/
 * @author Chris Ferdinandi
 * @license MIT
 */
if (!String.prototype.replaceAll) {
	String.prototype.replaceAll = function(str, newStr) {
		// If a regex pattern
		if (Object.prototype.toString.call(str).toLowerCase() === '[object regexp]') {
			return this.replace(str, newStr);
		}

		// If a string
		return this.replace(new RegExp(str, 'g'), newStr);
	};
}

loadLanguage(() => {
  const app = createApp(App);

  if (process.env.NODE_ENV === 'production') {
    Sentry.init({
      app,
      release: VERSION,
      dsn: "https://9c8cd525d7c64214836351b406f6e860@o545203.ingest.sentry.io/6087646",
      integrations: [new WasmIntegration()],
      beforeSend: function(event, hint) {
        // console.error(hint.originalException || hint.syntheticException);
        let stacktrace = event.exception && event.exception.values[0].stacktrace;
        if (stacktrace && stacktrace.frames) {
          stacktrace.frames.forEach(function(frame) {
            const filename = path.basename(frame.filename);
            frame.filename = `app:///dist/${filename}`
          });
        }
        return event;
      }
    });
    Sentry.setContext("session", {
      id: sessionId,
      version: VERSION,
    });
  }

  document.addEventListener('keydown', (ev) => {
    if (ev.shiftKey && ev.key == 'I') {
      alert(`This is your game session id:\n${sessionId}\nPlease include this if you are submitting a bug report.`);
    }
  });

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
