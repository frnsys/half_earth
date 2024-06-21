import state from '/src/state';
import consts from '/src/consts.json';

const OUTPUT_UNITS = {
  fuel: 1e-9/1e3,            // per 1000 TWh
  electricity: 1e-9/1e3,     // per 1000 TWh
  plant_calories: 1e-9/2e4,  // per 20000 Tcals
  animal_calories: 1e-9/2e4, // per 20000 Tcals
  water: 1e-12/50,           // per 50 km3
  land: 100/consts.starting_resources.land  // percent of habitable land
};

function formatNumber(val) {
  if (val >= 1e9) {
    return `${(val/1e9).toFixed(1)}b`;
  } else if (val >= 1e6) {
    return `${(val/1e6).toFixed(1)}m`;
  } else if (val >= 1e3) {
    return `${(val/1e3).toFixed(1)}k`;
  } else {
    return val;
  }
}

function co2eq(byproducts) {
  return byproducts.co2 + byproducts.ch4 * 36 + byproducts.n2o * 298;
}

function gtco2eq(byproducts, multiplier=1) {
  return Math.round(multiplier * co2eq(byproducts) * 1e-15); // Gt CO2eq;
}

function twh(amount) {
  return Math.round(amount * 1e-9);
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
  return m2/consts.starting_resources.land * 100;
}

function waterUsePercent(l) {
  // return l/consts.starting_resources.water * 100;
  return l/state.gameState.resources.water * 100;
}

function demandPercent(demand, totalDemand, k, round) {
  return percent(demand[k]/(totalDemand[k] || 1), round);
}

function percent(p, round) {
  let percent = p * 100;
  if (percent < 1 && percent > 0) {
    return '<1%';
  } else {
    if (round) {
      return `${Math.round(percent)}%`;
    } else {
      return `${percent.toFixed(1)}%`;
    }
  }
}

function sign(v) {
  return `${v > 0 ? '+' : ''}${v}`;
}

const formatResource = {
  'land': (v) => percent(landUsePercent(v)/100, true),
  'emissions': (v) => `${(v * 1e-15).toFixed(1)}Gt`,
  'water': (v) => percent(waterUsePercent(v)/100, true),
  'energy': (v) => `${(v*1e-9).toFixed(1)}TWh`,
  'biodiversity': (v) => `${v.toFixed(0)}`,
};

export default {
  sign,
  co2eq, gtco2eq, twh,
  output, outputs,
  landUsePercent, waterUsePercent,
  demandPercent, percent,
  formatNumber, formatResource};