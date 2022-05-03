import format from './format';
import state from '/src/state';
import display from './display';
import effects from './effects';
import intensity from './intensity';
import {activeEffects} from './project';
import consts from '/src/consts.json';
import EVENTS from '/assets/content/events.json';

const VARS = ['land', 'water', 'energy', 'emissions', 'biodiversity', 'contentedness'];
const DEMAND_VARS = ['electricity', 'fuel', 'plant_calories', 'animal_calories'];

// https://stackoverflow.com/a/50636286/1097920
function partition(array, filter) {
  let pass = [], fail = [];
  array.forEach((e, idx, arr) => (filter(e, idx, arr) ? pass : fail).push(e));
  return [pass, fail];
}

function effectsFactor(k, effs) {
  if (k == 'emissions') {
    return effs
      .filter((eff) => eff.subtype == 'Emissions')
      .reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'water') {
    let val = effs
      .filter((eff) => eff.type == 'Resource' && eff.subtype == 'Water')
      .reduce((acc, eff) => acc + eff.param, 0);
    return format.sign(format.output(val, k));
  } else if (k == 'land') {
    return effs
      .filter((eff) => eff.type == 'ProtectLand')
      .reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'energy') {
      return effs
      .filter((eff) => {
        return eff.type == 'DemandAmount' && (eff.subtype == 'Electricity' || eff.subtype == 'Fuel')
      }).reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'fuel') {
      return effs
      .filter((eff) => {
        return eff.type == 'DemandAmount' && eff.subtype == 'Fuel';
      }).reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'electricity') {
      return effs
      .filter((eff) => {
        return eff.type == 'DemandAmount' && eff.subtype == 'Electricity';
      }).reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'plant_calories') {
      return effs
      .filter((eff) => {
        return eff.type == 'DemandAmount' && eff.subtype == 'PlantCalories';
      }).reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'animal_calories') {
      return effs
      .filter((eff) => {
        return eff.type == 'DemandAmount' && eff.subtype == 'AnimalCalories';
      }).reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'contentedness') {
    return effs.reduce((acc, eff) => {
      let amount = 0;
      if (eff.subtype == 'Outlook') {
        amount = eff.param;
      } else if (eff.type == 'IncomeOutlookChange') {
        amount = effects.incomeOutlookChange(state.gameState.world, eff.param);
        amount = Math.round(amount);
      } else if (eff.type == 'DemandOutlookChange') {
        let k = display.enumKey(eff.subtype);
        amount = effects.demandOutlookChange(state.gameState.world, k, eff.param);
        amount = Math.round(amount);
      }
      return acc + amount
    }, 0);
  } else if (k == 'biodiversity') {
    return effs
      .filter((eff) => eff.subtype == 'ExtinctionRate')
      .reduce((acc, eff) => acc + eff.param, 0);
  }
}

function projectFactors(k) {
  return state.gameState.projects.filter((p) => {
    return p.status == 'Active' || p.status == 'Finished';
  }).map((p) => {
    let effs = activeEffects(p);
    let amount = effectsFactor(k, effs);
    let displayAmount;
    if (k == 'energy' || DEMAND_VARS.includes(k)) {
      let demand;
      if (k == 'energy') {
        demand = state.gameState.output_demand.electricity + state.gameState.output_demand.fuel;
      } else {
        demand = state.gameState.output_demand[k];
      }
      displayAmount = format.percent(amount/demand, true);
    } else {
      displayAmount = amount;
    }
    return {
      name: p.name,
      type: 'Project',
      amount,
      displayAmount,
    };
  }).filter((p) => p.amount !== 0);
}

function eventFactors(k) {
  return state.events.map(([eventId, _regionId, _refId]) => {
    let event = EVENTS[eventId];
    return {
      name: event.name,
      type: 'Event',
      amount: effectsFactor(k, event.effects)
    };
  }).filter((p) => p.amount !== 0);
}

function regionalFactors(k) {
  return state.gameState.world.regions.map((region) => {
    let intensity = region.income_level + 1;
    return {
      name: region.name,
      type: 'Region',
      intensity,
      amount: format.output(region.demand[k], k),
      displayAmount: format.percent(region.demand[k]/state.gameState.output_demand[k], true),
    };
  }).filter((p) => p.amount !== 0);
}

function productionFactors(k) {
  let contributors = state.gameState.processes.map((p, i) => {
    return {
      demand: state.gameState.produced_by_process[i],
      ...p
    };
  }).concat(state.gameState.industries);

  return contributors.map((p) => {
    let base = 0;
    if (k == 'land' || k == 'water') {
      base = p.resources[k];
    } else if (k == 'energy') {
      base = (p.resources['electricity'] + p.resources['fuel']);
    } else if (k == 'emissions') {
      base = format.co2eq(p.byproducts);
    } else if (k == 'biodiversity') {
      base = p.extinction_rate;
    } else if (k == 'electricity' || k == 'fuel') {
      base = p.resources[k];
    }

    let type =
      (p.output == 'Electricity' || p.output == 'Fuel')
      ? 'energy' : 'calories';

    let total = base * p.demand * (state.gameState.output_demand_modifier[k] || 1);
    let inten = intensity.intensity(base, k, type);

    let out = p.output ? display.enumKey(p.output) : null;
    let displayAmount = total;
    if (k == 'energy') {
      let energyDemand = state.gameState.output_demand.electricity + state.gameState.output_demand.fuel;
      displayAmount = format.percent(total/energyDemand, true);
    } else if (k == 'electricity' || k == 'fuel') {
      let demand = state.gameState.output_demand[k];
      displayAmount = format.percent(total/demand, true);
    } else {
      displayAmount = format.formatResource[k](total);
    }

    return {
      name: p.name,
      produced: p.demand,
      output: out,
      intensity: inten,
      amount: total,
      displayAmount: displayAmount,
      displayProduced: out != null ? format.percent(p.demand/state.gameState.output_demand[out], true) : null,
    }
  }).filter((p) => p.amount !== 0 && (p.output != null || (p.output == null && p.amount !== 0)));
}

function rank() {
  let factors = {};
  VARS.concat(DEMAND_VARS).forEach((k) => {
    let rankings = [];

    if (k !== 'contentedness' && k !== 'animal_calories' && k !== 'plant_calories') {
      rankings = rankings.concat(productionFactors(k));
    }

    if (DEMAND_VARS.includes(k)) {
      rankings = rankings.concat(regionalFactors(k));
    } else {
      rankings = rankings.concat(eventFactors(k));
    }
    rankings = rankings.concat(projectFactors(k));

    if (k == 'contentedness') {
      if (state.gameState.world.temp_outlook !== 0) {
        rankings.push({
          type: 'Event',
          name: 'Temperature Change',
          amount: Math.round(state.gameState.world.temp_outlook)
        });
      }
      if (state.gameState.world.shortages_outlook !== 0) {
        rankings.push({
          type: 'Event',
          name: 'Production Shortages',
          amount: Math.round(-state.gameState.world.shortages_outlook)
        });
      }
      rankings.push({
        type: 'Event',
        name: 'Post-Revolution Optimism',
        amount: 30
      });
    } else if (k == 'land') {
      rankings.push({
        type: 'Event',
        name: 'Nature Preserves',
        displayAmount: '10%',
        amount: 0.1 * consts.starting_resources.land,
      });
    } else if (k == 'biodiversity') {
        rankings.push({
          type: 'Event',
          name: 'Sea Level Rise',
          amount: Math.round(state.gameState.world.slr_extinction_rate)
        });
        rankings.push({
          type: 'Event',
          name: 'Temperature Change',
          amount: Math.round(state.gameState.world.tgav_extinction_rate)
        });
    }

    // Split into modifiers (which don't make up demand percentages)
    // and contributors (who do make up demand percentages);
    let [modifiers, contribs] = partition(rankings, (r) => {
      let amount = r.displayAmount || r.amount;
      return typeof amount === 'string' && (amount.startsWith('+') || amount.startsWith('-'));
    });

    modifiers.sort();
    contribs.sort((a, b) => Math.abs(a.amount) > Math.abs(b.amount) ? -1 : 1)
    factors[k] = modifiers.concat(contribs);
  });

  return factors;
}

const tips = {
  emissions: (text, current) => {
    return {
      text,
      icon: 'emissions',
      card: {
        type: 'Factors',
        data: {
          icon: 'emissions',
          type: 'emissions',
          total: `${state.gameState.world.emissions.toFixed(1)}Gt`,
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
        type: 'Factors',
        data: {
          icon: 'extinction_rate',
          type: 'biodiversity',
          total: Math.max(0, Math.round(state.gameState.world.extinction_rate)),
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
        type: 'Factors',
        data: {
          icon: 'land',
          type: 'land',
          total: `${Math.round(format.landUsePercent(state.gameState.resources_demand.land))}%`,
          current,
        }
      }
    }
  },
  energy: (text, current) => {
    let demand = state.gameState.output_demand;
    return {
      text,
      icon: 'energy',
      card: {
        type: 'Factors',
        data: {
          icon: 'energy',
          type: 'energy',
          total: `${format.twh(demand.electricity + demand.fuel)}TWh`,
          current,
        }
      }
    }
  },
  water: (text, current) => {
    return {
      text,
      icon: 'water',
      card: {
        type: 'Factors',
        data: {
          icon: 'water',
          type: 'water',
          total: `${format.output(state.gameState.resources_demand.water, 'water')}/${format.output(state.gameState.resources.water, 'water')}`,
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
        type: 'Factors',
        data: {
          icon: 'contentedness',
          type: 'contentedness',
          total: Math.round(state.gameState.world.contentedness),
          current,
        }
      }
    }
  },
  electricity: (text, current) => {
    return {
      text,
      icon: 'electricity',
      card: {
        type: 'Factors',
        data: {
          icon: 'electricity',
          type: 'electricity',
          total: format.output(state.gameState.output_demand.electricity, 'electricity'),
          current,
        }
      }
    }
  },
  fuel: (text, current) => {
    return {
      text,
      icon: 'fuel',
      card: {
        type: 'Factors',
        data: {
          icon: 'fuel',
          type: 'fuel',
          total: format.output(state.gameState.output_demand.fuel, 'fuel'),
          current,
        }
      }
    }
  },
  plant_calories: (text, current) => {
    return {
      text,
      icon: 'plant_calories',
      card: {
        type: 'Factors',
        data: {
          icon: 'plant_calories',
          type: 'plant_calories',
          total: format.output(state.gameState.output_demand.plant_calories, 'plant_calories'),
          current,
        }
      }
    }
  },
  animal_calories: (text, current) => {
    return {
      text,
      icon: 'animal_calories',
      card: {
        type: 'Factors',
        data: {
          icon: 'animal_calories',
          type: 'animal_calories',
          total: format.output(state.gameState.output_demand.animal_calories, 'animal_calories'),
          current,
        }
      }
    }
  },
}


export default {rank, tips};
