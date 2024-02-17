const defaultLanguage = 'en';
const availableLanguages = ['en', 'pt', 'pt-br', 'pt-pt', 'es', 'de-de', 'jp', 'fr-fr'];

function getPreferredLanguages() {
  if (navigator.languages && navigator.languages.length) {
    return navigator.languages;
  } else {
    let nav = navigator;
    let lang = nav.userLanguage || nav.language || nav.browserLanguage || defaultLanguage;
    return [lang, defaultLanguage];
  }
}

// Just default to en unless another language is explicitly chosen.
let lang = localStorage.getItem('lang') || defaultLanguage;
if (!availableLanguages.includes(lang)) {
  lang = defaultLanguage;
}

// Load phrases for language
let phrases = {};
function loadLanguage(cb) {
  if (lang == 'en') {
    cb();
  } else {
    fetch(`/assets/lang/${lang}.json`)
      .then(response => response.json())
      .then(json => {
        phrases = json;
        setupFormatter(lang);
        cb();
      });
  }
}

function t(key, data) {
  data = data || {};
  let tmpl = phrases[key] || key;
  return Object.keys(data).reduce((acc, k) => {
    return acc.replaceAll(`{${k}}`, data[k]);
  }, tmpl);
}

const formatter = {};
function setupFormatter(lang) {
  formatter.number = new Intl.NumberFormat(lang);
  formatter.percent = new Intl.NumberFormat(lang, {
    style: 'percent',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  })
  formatter.list = new Intl.ListFormat(lang);
}
setupFormatter('en-US');

export { loadLanguage, formatter, lang };
export default t;
