const params = new URLSearchParams(window.location.search);

const defaultLanguage = 'en';
const availableLanguages = ['en', 'es'];

function getPreferredLanguages() {
  if (navigator.languages && navigator.languages.length) {
    return navigator.languages;
  } else {
    let nav = navigator;
    let lang = nav.userLanguage || nav.language || nav.browserLanguage || defaultLanguage;
    return [lang, defaultLanguage];
  }
}

// Get specified language, if one is
// Get most preferred language that is supported
// Fallback to 'en' if lang is undefined
let lang = params.get('lang') || getPreferredLanguages().filter(l => availableLanguages.includes(l))[0] || defaultLanguage;
if (!availableLanguages.includes(lang)) {
  lang = defaultLanguage;
}

// Load phrases for language
let phrases = {};
function loadLanguage(cb) {
  // TODO
  // fetch(`/static/lang/${lang}.json`)
  //   .then(response => response.json())
  //   .then(json => {
  //     phrases = json;
  //     cb();
  //   });
  cb();
}

function t(key, data) {
  data = data || {};
  let tmpl = phrases[key] || key;
  return Object.keys(data).reduce((acc, k) => {
    return acc.replace(`{${k}}`, data[k]);
  }, tmpl);
}

const formatter = {
  number: new Intl.NumberFormat('en-US'),
  percent: new Intl.NumberFormat('en-US', {
    style: 'percent',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  })
};
if (lang == 'es') {
  formatter.number = new Intl.NumberFormat('es')
  formatter.percent = new Intl.NumberFormat('es', {
    style: 'percent',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  });
}

export { loadLanguage, formatter };
export default t;