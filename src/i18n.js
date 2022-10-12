const defaultLanguage = 'en';
const availableLanguages = ['en', 'pt', 'pt-br'];

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

export { loadLanguage, formatter, lang };
export default t;