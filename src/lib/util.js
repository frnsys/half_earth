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

function sign(v) {
  return `${v > 0 ? '+' : ''}${v}`;
}

function nearestMultiple(v, base) {
  return base * Math.round(v/base);
}

export {updateTransform, sign, nearestMultiple};
