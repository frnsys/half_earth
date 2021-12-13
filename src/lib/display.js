import state from '/src/state';
import icons from 'components/icons';
import consts from '/src/consts.json';
import EVENTS from '/assets/content/events.json';
import PROJECTS from '/assets/content/projects.json';

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
    'Land': 'land',
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

function activeEffects(project) {
  let details = PROJECTS[project.id];
  let activeOutcomeEffects = project.active_outcome == null ? [] : details.outcomes[project.active_outcome].effects;
  if (project.status == 'Inactive') {
    return details.effects.concat(outcomeEffects(details));
  } else if (project.level === 0) {
    return details.effects.concat(activeOutcomeEffects);
  } else {
    return details.upgrades[project.level - 1].effects.concat(activeOutcomeEffects);
  }
}

function outcomeEffects(projectDetails) {
  let allEffects = {};
  projectDetails.outcomes.forEach(({effects}) => {
    for (const effect of effects) {
      let key = `${effect.type}${effect.subtype ? effect.subtype : ''}`;
      let hash = JSON.stringify(effect);
      if (!(key in allEffects)) {
        allEffects[key] = {
          effect,
          count: 1,
          hashes: new Set([hash]),
        };
      } else {
        allEffects[key].count += 1;
        allEffects[key].hashes.add(hash);
      }
    }
  });

  return Object.values(allEffects).map(({effect, count, hashes}) => {
    effect.random = count !== projectDetails.outcomes.length || hashes.size > 1;
    if (hashes.size > 1) effect.param = '?';
    return effect;
  });
}

function projectRankings(k) {
  let active = state.gameState.projects.filter((p) => {
    return p.status == 'Active' || p.status == 'Finished';
  }).map((p) => {
    let effects = activeEffects(p);
    let relevantEffect = 0;
    if (k == 'emissions') {
      relevantEffect = effects.reduce((acc, eff) => {
        return acc + (eff.subtype == 'Emissions' ? eff.param : 0);
      }, 0);
    } else if (k == 'water') {
      // TODO no effects that influence this directly
    } else if (k == 'land') {
      relevantEffect = effects.reduce((acc, eff) => {
        return acc + (eff.type == 'ProtectLand' ? eff.param : 0);
      }, 0);
    } else if (k == 'energy') {
      // TODO no effects that influence this directly
    } else if (k == 'contentendness') {
      relevantEffect = effects.reduce((acc, eff) => {
        return acc + (eff.subtype == 'Outlook' ? eff.param : 0);
      }, 0);
    } else if (k == 'biodiversity') {
      relevantEffect = effects.reduce((acc, eff) => {
        return acc + (eff.subtype == 'ExtinctionRate' ? eff.param : 0);
      }, 0);
    }
    return {name: p.name, type: 'Project', amount: relevantEffect};
  }).filter((p) => p.amount !== 0);
  return active;
}

function eventRankings(k) {
  let active = state.events.map(([eventId, regionId]) => {
    let event = EVENTS[eventId];
    let effects = event.effects;
    let relevantEffect = 0;
    if (k == 'emissions') {
      relevantEffect = effects.reduce((acc, eff) => {
        return acc + (eff.subtype == 'Emissions' ? eff.param : 0);
      }, 0);
    } else if (k == 'water') {
      // TODO no effects that influence this directly
    } else if (k == 'land') {
      relevantEffect = effects.reduce((acc, eff) => {
        return acc + (eff.type == 'ProtectLand' ? eff.param : 0);
      }, 0);
    } else if (k == 'energy') {
      // TODO no effects that influence this directly
    } else if (k == 'contentendness') {
      relevantEffect = effects.reduce((acc, eff) => {
        return acc + (eff.subtype == 'Outlook' ? eff.param : 0);
      }, 0);
    } else if (k == 'biodiversity') {
      relevantEffect = effects.reduce((acc, eff) => {
        return acc + (eff.subtype == 'ExtinctionRate' ? eff.param : 0);
      }, 0);
    }
    return {name: event.name, type: 'Event', amount: relevantEffect};
  }).filter((p) => p.amount !== 0);
  return active;
}

function resourceRankings() {
  // TODO is this the best place for this?
  let resourceRankings = {};
  let contributors = state.gameState.processes.map((p, i) => {
    return {
      demand: state.gameState.produced_by_process[i],
      ...p
    };
  }).concat(state.gameState.industries);
  ['land', 'water', 'energy', 'emissions', 'biodiversity', 'contentedness'].forEach((k) => {
    let rankings = [];
    if (k !== 'contentedness') {
      rankings = contributors.map((p) => {
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

        let total = base * p.demand;
        let inten = intensity(base, k, type);

        let out = p.output ? enumKey(p.output) : null;
        return {
          name: out == null ? `${p.name}*` : p.name,
          produced: p.demand,
          output: out,
          intensity: inten,
          amount: total,
          displayAmount: resourceFormats[k](total),
          displayProduced: out != null ? output(p.demand, out) : null,
        }
      });
      rankings = rankings.filter((p) => p.output != null || p.output == null && p.amount !== 0);
    }

    rankings = rankings.concat(projectRankings(k));
    rankings = rankings.concat(eventRankings(k));

    if (k == 'contentedness') {
      if (state.gameState.world.temp_outlook !== 0) {
        rankings.push({
          type: 'Event',
          name: 'Temperature Change',
          amount: state.gameState.world.temp_outlook
        });
      }
    } else if (k == 'biodiversity') {
        rankings.push({
          type: 'Event',
          name: 'Sea Level Rise',
          amount: Math.round(state.gameState.world.sea_level_rise**2)
        });
        rankings.push({
          type: 'Event',
          name: 'Temperature Change',
          amount: Math.round(state.gameState.world.temperature**2)
        });
    }

    rankings.sort((a, b) => Math.abs(a.amount) > Math.abs(b.amount) ? -1 : 1)
    resourceRankings[k] = rankings;
  });

  return resourceRankings;
}

const rankingTips = {
  emissions: (text, current) => {
    return {
      text,
      icon: 'emissions',
      card: {
        type: 'Resource',
        data: {
          icon: 'emissions',
          type: 'emissions',
          current,
        }
      }
    }
  },
  biodiversity: (text, current) => {
    return {
      text,
      icon: 'extinction_rate',
      card: {
        type: 'Resource',
        data: {
          icon: 'extinction_rate',
          type: 'biodiversity',
          current,
        }
      }
    }
  },
  land: (text, current) => {
    return {
      text,
      icon: 'land',
      card: {
        type: 'Resource',
        data: {
          icon: 'land',
          type: 'land',
          current,
        }
      }
    }
  },
  contentedness: (text, current) => {
    return {
      text,
      icon: 'contentedness',
      card: {
        type: 'Resource',
        data: {
          icon: 'contentedness',
          type: 'contentedness',
          current,
        }
      }
    }
  }

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
  resourceRankings, resourceFormats,
  projectRankings, eventRankings,
  rankingTips, activeEffects};
