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

export {clone, randChoice, slugify,
  updateTransform, detectCenterElement};
