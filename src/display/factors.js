import format from './format';
import state from '/src/state';
import display from './display';
import effects from './effects';
import intensity from './intensity';
import {activeEffects} from './project';
import EVENTS from '/assets/content/events.json';
import {process_extinction_rate,
  slr_extinction_rate, tgav_extinction_rate} from 'half-earth-engine';

const VARS = ['land', 'water', 'energy', 'emissions', 'biodiversity', 'contentedness'];
const DEMAND_VARS = ['electricity', 'fuel', 'plant_calories', 'animal_calories'];

function effectsFactor(k, effs) {
  if (k == 'emissions') {
    return effs
      .filter((eff) => eff.subtype == 'Emissions')
      .reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'water') {
    // TODO no effects that influence this directly
    // TODO update for desalination etc, resource effect
    return 0;
  } else if (k == 'land') {
    return effs
      .filter((eff) => eff.type == 'ProtectLand')
      .reduce((acc, eff) => acc + eff.param, 0);
  } else if (k == 'energy') {
    // TODO no effects that influence this directly
    return 0;
  } else if (k == 'contentedness') {
    return effs.reduce((acc, eff) => {
      let amount = 0;
      if (eff.subtype == 'Outlook') {
        amount = eff.param;
      } else if (eff.type == 'IncomeOutlookChange') {
        amount = effects.incomeOutlookChange(state.gameState.world, eff.param);
        amount = Math.round(amount);
      } else if (eff.type == 'DemandOutlookChange') {
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
    return {
      name: p.name,
      type: 'Project',
      amount: effectsFactor(k, effs)
    };
  }).filter((p) => p.amount !== 0);
}

function eventFactors(k) {
  return state.events.map(([eventId, _regionId]) => {
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
    let intensity = 1;
    switch (region.income) {
      case 'Low': intensity = 1;
      case 'LowerMiddle': intensity = 2;
      case 'UpperMiddle': intensity = 3;
      case 'High': intensity = 4;
    };
    return {
      name: region.name,
      type: 'Region',
      intensity,
      amount: format.output(region.demand[k], k)
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
      base = process_extinction_rate(p.byproducts[k], p.resources['land'], 1);
    } else if (k == 'electricity' || k == 'fuel') {
      base = p.resources[k];
    }

    let type =
      (p.output == 'Electricity' || p.output == 'Fuel')
      ? 'energy' : 'calories';

    let total = base * p.demand;
    let inten = intensity.intensity(base, k, type);

    let out = p.output ? display.enumKey(p.output) : null;
    let displayAmount = total;
    if (k == 'electricity' || k == 'fuel') {
      displayAmount = format.output(total, k);
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
      displayProduced: out != null ? format.output(p.demand, out) : null,
    }
  }).filter((p) => p.output != null || (p.output == null && p.amount !== 0));
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
      rankings = rankings.concat(projectFactors(k));
      rankings = rankings.concat(eventFactors(k));
    }

    if (k == 'contentedness') {
      if (state.gameState.world.temp_outlook !== 0) {
        rankings.push({
          type: 'Event',
          name: 'Temperature Change',
          amount: Math.round(state.gameState.world.temp_outlook)
        });
      }
    } else if (k == 'biodiversity') {
        rankings.push({
          type: 'Event',
          name: 'Sea Level Rise',
          amount: Math.round(slr_extinction_rate(state.gameState.world.sea_level_rise))
        });
        rankings.push({
          type: 'Event',
          name: 'Temperature Change',
          amount: Math.round(tgav_extinction_rate(state.gameState.world.temperature))
        });
    }

    rankings.sort((a, b) => Math.abs(a.amount) > Math.abs(b.amount) ? -1 : 1)
    factors[k] = rankings;
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
          total: `${state.gameState.emissions.toFixed(1)}Gt`,
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
          total: Math.round(state.gameState.world.extinction_rate),
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
          total: `${Math.round(format.waterUsePercent(state.gameState.resources_demand.water))}%`,
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
          total: Math.round(state.gameState.contentedness),
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
