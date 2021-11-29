import state from '/src/state';
import icons from 'components/icons';
import consts from '/src/consts.json';

const OUTPUT_UNITS = {
  fuel: 1e-9/1e3,            // per 1000 TWh
  electricity: 1e-9/1e3,     // per 1000 TWh
  plant_calories: 1e-9/2e4,  // per 20000 Tcals
  animal_calories: 1e-9/2e4, // per 20000 Tcals
  water: 1e-12/50,           // per 50 km3
};

const DISPLAY_NAMES = {
    'Fuel': 'fuel',
    'Electricity': 'electricity',
    'PlantCalories': 'plant-based food',
    'AnimalCalories': 'animal-based food',
};

const FEATURE_DESCS = {
  'IsSolar': 'solar processes',
  'IsIntermittent': 'intermittent processes',
  'IsNuclear': 'nuclear processes',
  'IsCombustion': 'combustion processes',
  'IsFossil': 'fossil fuel processes',
  'IsCCS': 'carbon capture processes',
  'UsesLivestock': 'processes that use livestock',
  'UsesPesticides': 'processes that use pesticides',
  'UsesSynFertilizer': 'processes that use synthetic fertilizers',
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
  return m2/consts.starting_resources.land * 100;
}

function waterUsePercent(l) {
  return l/consts.starting_resources.water * 100;
}

function fillIcons(text) {
  let matches = [...text.matchAll(/\[([a-z_]+)\]/g)];
  for (const match of matches) {
    text = text.replaceAll(match[0], `<img src="${icons[match[1]]}">`);
  }
  return text;
}

function fillVars(text, context) {
  let vars = [...text.matchAll('{([a-z_]+)}')];
  for (const match of vars) {
    text = text.replaceAll(match[0], context[match[1]]);
  }
  return text;
}

function cardTag(name, icon) {
  if (icon) {
    return `<div class="card-tag"><img src="${icons[icon]}">${name}</div>`
  } else {
    return `<div class="card-tag">${name}</div>`
  }
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
  },
};

function intensity(val, key, type) {
  let stops;
  if (key in intensities) {
    stops = intensities[key][type];
  } else {
    stops = consts.demand_levels.map((o) => Math.floor(o[key]));
  }
  for (let i = 0; i < stops.length - 1; i++) {
    if (val >= stops[i] && val < stops[i+1]) {
      return i+1;
    }
  }
  return stops.length;
}

function scaleIntensity(val, key) {
  switch (key) {
    case 'outlook': return Math.round(val/consts.base_outlook * 4);
    case 'extinction': return Math.round(val/60 * 4);
    case 'habitability': return Math.round(val/consts.base_habitability * 4);
    case 'world_outlook': return Math.round(val/(consts.base_outlook+consts.base_world_outlook) * 4);
  }
}

function intensityColor(intensity, invert) {
  intensity = invert ? 5 - intensity : intensity;
  if (intensity === 1) {
    return '#43CC70';
  } else if (intensity === 2) {
    return '#FBC011';
  } else if (intensity === 3) {
    return '#f28435';
  } else {
    return '#EF3838';
  }
}

function displayName(key) {
  return DISPLAY_NAMES[key];
}

function describeFeature(feature) {
  return FEATURE_DESCS[feature];
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
        base = (p.byproducts[k]/1e4 + p.resources['land']/consts.starting_resources.land) * 100;
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

export default {co2eq, gtco2eq, output, outputs,
  formatNumber, intensityColor,
  cardTag, describeFeature,
  landUsePercent, waterUsePercent,
  fillIcons, fillVars,
  displayName, enumKey, enumDisplay,
  relationshipName,
  intensity, scaleIntensity,
  resourceRankings, resourceFormats};
