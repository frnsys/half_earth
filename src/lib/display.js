import state from '/src/state';
import icons from 'components/icons';

const baseOutlook = 50;
const baseHabitability = 100;
const totalLand = 104e12;
const totalWater = 4.55e16;

const OUTPUT_UNITS = {
  fuel: 1e-9/1e3,            // per 1000 TWh
  electricity: 1e-9/1e3,     // per 1000 TWh
  plant_calories: 1e-9/2e4,  // per 20000 Tcals
  animal_calories: 1e-9/2e4, // per 20000 Tcals
};

const DISPLAY_NAMES = {
    'Fuel': 'fuel',
    'Electricity': 'electricity',
    'PlantCalories': 'plant-based food',
    'AnimalCalories': 'animal-based food',
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

function waterUsePercent(l) {
  return l/totalWater * 100;
}

function fillIcons(text) {
  let matches = [...text.matchAll(/\[([a-z_]+)\]/g)];
  for (const match of matches) {
    text = text.replaceAll(match[0], `<img src="${icons[match[1]]}">`);
  }
  return text;
}

function enumKey(v) {
  return v.split(/(?=[A-Z])/).join('_').toLowerCase();
}

function enumDisplay(v) {
  return enumKey(v).replace('_', ' ');
}

function relationshipName(relationship) {
  if (relationship >= 5) {
    return 'Ally';
  } else if (relationship <= 1) {
    return 'Nemesis';
  } else {
    return 'Neutral';
  }
}

const intensities = {
  'land': {
    'energy': [0, 0.001, 0.01, 0.1],
    'calories': [0, 0.001, 0.002, 0.01],
  },
  'labor': {
    'energy': [0, 0.001, 0.01, 0.1], // TODO labor
    'calories': [0, 0.001, 0.002, 0.01], // TODO labor
  },
  'energy': {
    'energy': [0, 0.001, 0.01, 0.1], // TODO EROI
    'calories': [0, 0.00015, 0.0005, 0.001],
  },
  'water': {
    'energy': [0, 1, 2, 5],
    'calories': [0, 1, 2, 3],
  },
  'emissions': {
    'energy': [-2000, 0, 200, 800],
    'calories': [-1, 0, 0.5, 1],
  },
  'biodiversity': {
    'energy': [0, 1, 2, 3],
    'calories': [0, 1, 2, 3],
  }
};

function intensity(val, key, type) {
  let stops = intensities[key][type];
  for (let i = 0; i < stops.length - 1; i++) {
    if (val >= stops[i] && val < stops[i+1]) {
      return i+1;
    }
  }
  return stops.length;
}

function scaleIntensity(val, key) {
  switch (key) {
    case 'outlook': return Math.round(val/baseOutlook * 4);
    case 'habitability': return Math.round(val/baseHabitability * 4);
  }
}

function displayName(key) {
  return DISPLAY_NAMES[key];
}

const resourceFormats = {
  'land': (v) => `${landUsePercent(v).toFixed(1)}%`,
  'emissions': (v) => `${(v * 1e-15).toFixed(1)}Gt`,
  'water': (v) => `${(waterUsePercent(v)).toFixed(1)}%`,
  'energy': (v) => `${(v*1e-9).toFixed(1)}TWh`,
  'biodiversity': (v) => `${v.toFixed(0)}`,
};

function resourceRankings() {
  // TODO is this the best place for this?
  // TODO add in industries as well
  let resourceRankings = {};
  ['land', 'water', 'energy', 'emissions', 'biodiversity'].forEach((k) => {
    let rankings = state.gameState.processes.map((p, i) => {
      let produced = state.gameState.produced_by_process[i];
      let base = 0;
      if (k == 'land' || k == 'water') {
        base = p.resources[k];
      } else if (k == 'energy') {
        base = (p.resources['electricity'] + p.resources['fuel']);
      } else if (k == 'emissions') {
        base = co2eq(p.byproducts);
      } else if (k == 'biodiversity') {
        // TODO tweak this
        base = p.byproducts[k] + (p.resources['land'] * 10);
      }

      let type =
        (p.output == 'Electricity' || p.output == 'Fuel')
        ? 'energy' : 'calories';

      let total = base * produced;
      let inten = intensity(base, k, type);

      let out = enumKey(p.output);
      return {
        name: p.name,
        produced,
        output: out,
        intensity: inten,
        amount: total,
        displayAmount: resourceFormats[k](total),
        displayProduced: output(produced, out),
      }
    });
    rankings.sort((a, b) => a.amount > b.amount ? -1 : 1)
    resourceRankings[k] = rankings;
  });

  return resourceRankings;
}

export default {co2eq, gtco2eq, output, outputs,
  landUsePercent, waterUsePercent,
  fillIcons,
  displayName, enumKey, enumDisplay,
  relationshipName,
  intensity, scaleIntensity,
  resourceRankings, resourceFormats};
