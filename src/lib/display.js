import assets from 'components/assets';

const totalLand = 104e12;

const OUTPUT_UNITS = {
  fuel: 1e-9/1e3,            // per 1000 TWh
  electricity: 1e-9/1e3,     // per 1000 TWh
  plant_calories: 1e-9/2e4,  // per 20000 Tcals
  animal_calories: 1e-9/2e4, // per 20000 Tcals
};

function co2eq(byproducts) {
  return byproducts.co2 + byproducts.ch4 * 36 + byproducts.n2o * 298;
}

function gtco2eq(byproducts, multiplier=1) {
  return Math.round(multiplier * co2eq(byproducts) * 1e-15); // Gt CO2eq;
}

function output(amount, output) {
  return Math.round(amount * OUTPUT_UNITS[output]);
}

function outputs(outputs) {
  return Object.keys(outputs).reduce((acc, k) => {
    acc[k] = output(outputs[k], k);
    return acc;
  }, {});
}

function landUsePercent(m2) {
  return m2/totalLand * 100;
}

function fillIcons(text) {
  let icons = [...text.matchAll(/\[([a-z_]+)\]/g)];
  for (const match of icons) {
    text = text.replaceAll(match[0], `<img src="${assets.icons[match[1]]}">`);
  }
  return text;
}

function enumToSlug(v) {
  return v.split(/(?=[A-Z])/).join('_').toLowerCase();
}

function enumToDisplay(v) {
  return enumToSlug(v).replace('_', ' ');
}

export default {co2eq, gtco2eq, output, outputs, landUsePercent, fillIcons, enumToSlug, enumToDisplay};
