const OUTPUT_UNITS = {
  fuel: 1e-9/1e3,            // per 1000 TWh
  electricity: 1e-9/1e3,     // per 1000 TWh
  plant_calories: 1e-9/2e4,  // per 20000 Tcals
  animal_calories: 1e-9/2e4, // per 20000 Tcals
};

function gtco2eq(byproducts, multiplier=1) {
  return Math.round(multiplier * (byproducts.co2 + byproducts.ch4 * 36 + byproducts.n2o * 298) * 1e-15); // Gt CO2eq;
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

export default {gtco2eq, output, outputs};
