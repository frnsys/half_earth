const GAME_SEED = Math.random();

function clone(obj) {
  return JSON.parse(JSON.stringify(obj));
}

function randChoice(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}

function slugify(text) {
  return text.toLowerCase().replaceAll(' ', '_');
}

// Update a CSS transform rule while
// preserving existing values
// https://stackoverflow.com/a/60813685
function updateTransform(el, updates) {
  let transforms = Array.from(el.style.transform.matchAll(/(\w+)\((.+?)\)/gm))
    .reduce((agg, [, key, val]) => {
      agg[key] = val;
      return agg;
    }, {});
  transforms = Object.assign(transforms, updates)
  el.style.transform = Object.keys(transforms).reduce((acc, k) => acc += `${k}(${transforms[k]}) `, '');
}

function detectCenterElement(parent, elements) {
  let rect = parent.getBoundingClientRect();
  let targetX = rect.x + parent.clientWidth/2;

  let minDist = null;
  let closest = -1;

  elements.forEach((el, idx) => {
    let rect = el.getBoundingClientRect();
    let pos = rect.x + rect.width/2;
    let dist = Math.abs(targetX - pos);
    if (minDist === null || dist < minDist) {
      minDist = dist;
      closest = idx;
    }
  });
  return closest;
}

// https://stackoverflow.com/a/38336308/1097920
function sumDigits(n) {
  let sum = 0;
  while (n) {
      sum += n % 10;
      n = Math.floor(n / 10);
  }
  return sum;
}

// Seedable RNG
// https://stackoverflow.com/a/47593316/1097920
function mulberry32(a) {
  return function() {
    var t = a += 0x6D2B79F5;
    t = Math.imul(t ^ t >>> 15, t | 1);
    t ^= t + Math.imul(t ^ t >>> 7, t | 61);
    return ((t ^ t >>> 14) >>> 0) / 4294967296;
  }
}

function rngForYear(year) {
  let seed = sumDigits(year) * GAME_SEED;
  return mulberry32(seed);
}

// https://stackoverflow.com/a/4819886/1097920
function _isTouchDevice() {
  return (('ontouchstart' in window) ||
     (navigator.maxTouchPoints > 0) ||
     (navigator.msMaxTouchPoints > 0));
}

function scaleText(el, minSize) {
  if (el.scrollHeight > el.clientHeight) {
    let intv = setInterval(() => {
      let fontSize = parseFloat(getComputedStyle(el).fontSize);
      fontSize = Math.round(fontSize);
      while (el.scrollHeight > el.clientHeight && fontSize > minSize) {
        fontSize--;
        el.style.fontSize = `${fontSize}px`;
      }
      clearInterval(intv);
    }, 1);
  }
}

// Cache this value
const isTouchDevice = _isTouchDevice();

export {clone, randChoice, rngForYear, scaleText,
  slugify, updateTransform, detectCenterElement, isTouchDevice};
