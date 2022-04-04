<template>
<div class="effects">
  <div v-for="{tip, text} in renders" class="effect" v-tip="tip ? tip : 'missing tip'">
    <div class="effect--text" v-html="text" />
  </div>
</div>
</template>

<script>
import icons from 'components/icons';
import {slugify} from 'lib/util';
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import format from '/src/display/format';
import factors from '/src/display/factors';
import effects from '/src/display/effects';
import display from '/src/display/display';
import {activeEffects} from '/src/display/project';
import FLAGS from '/assets/content/flags.json';
import EVENTS from '/assets/content/events.json';
import ICONEVENTS from '/assets/content/icon_events.json';

const FLAG_TIPS = {
  'Electrified': (demand) => {
    let changedDemand = parseInt((demand.fuel * 0.8).toFixed(0));
    return {
      icon: 'electricity',
      text: `Fuel demand will change from <img src="${icons.fuel}">${demand.fuel} to <img src="${icons.fuel}">${demand.fuel - changedDemand} and electricity demand will change from <img src="${icons.electricity}">${demand.electricity} to <img src="${icons.electricity}">${demand.electricity + changedDemand}.`
    };
  },
  'Vegan': (demand) => {
    let changedDemand = parseInt((demand.animal_calories * 0.9).toFixed(0));
    return {
      icon: 'plant_calories',
      text: `Animal calorie demand will change from <img src="${icons.animal_calories}">${demand.animal_calories} to <img src="${icons.animal_calories}">${demand.animal_calories - changedDemand} and plant calorie demand will change from <img src="${icons.plant_calories}">${demand.plant_calories} to <img src="${icons.plant_calories}">${demand.plant_calories + changedDemand}.`
    }
  },
  'Vegetarian': (demand) => {
    let changedDemand = parseInt((demand.animal_calories * 0.75).toFixed(0));
    return {
      icon: 'plant_calories',
      text: `Animal calorie demand will change from <img src="${icons.animal_calories}">${demand.animal_calories} to <img src="${icons.animal_calories}">${demand.animal_calories - changedDemand} and plant calorie demand will change from <img src="${icons.plant_calories}">${demand.plant_calories} to <img src="${icons.plant_calories}">${demand.plant_calories + changedDemand}.`
    }
  },
  'ClosedBorders': (demand) => {
    return {
      icon: 'closed_borders',
      text: 'Migrations will have less of an impact when they occur. But there might be other consequences.'
    }
  },
  'HyperResearch': (demand) => {
    return {
      icon: 'research',
      text: `Research points are 1<img src="${icons.political_capital}"> cheaper.`
    }
  },
  'StopDevelopment': (demand) => {
    return {
      icon: 'ban',
      text: 'Stops regional development throughout the world.'
    }
  },
  'FastDevelopment': (demand) => {
    return {
      icon: 'development',
      text: 'Accelerates regional development throughout the world.'
    }
  },
  'Degrowth': (demand) => {
    return {
      icon: 'degrowth',
      text: 'Contract the economies of the wealthiest regions.'
    }
  },
  'DeepSeaMining': (demand) => {
    return {
      icon: 'ocean',
      text: 'Prevents or stops metal shortages.'
    }
  },
  'ParliamentSuspended': (demand) => {
    return {
      icon: 'The Authoritarian',
      text: 'A parliamentary majority is no longer required for any project.',
    }
  },
  'MoreLabor': (demand) => {
    return {
      icon: 'labor',
      text: 'Research and infrastructure take 10% less time to complete.',
    }
  },
  'MoreLeisure': (demand) => {
    return {
      icon: 'labor',
      text: 'Research and infrastructure take 10% more time to complete.',
    }
  },
  'MoreAutomation': (demand) => {
    return {
      icon: 'labor',
      text: 'Research and infrastructure take 10% less time to complete.',
    }
  },
  'EcosystemModeling': (demand) => {
    return {
      icon: 'birb',
      text: 'Restoration projects take 10% less time to complete.',
    }
  },
  'LaborResistance': (demand) => {
    return {
      icon: 'labor',
      text: 'Research and infrastructure take 5% more time to complete.',
    }
  },
  'LaborSabotage': (demand) => {
    return {
      icon: 'labor',
      text: 'Research and infrastructure take 5% more time to complete.',
    }
  },
};

function changeDir(change, ev) {
  if (change == '?') {
    return 'Changes';
  } else if (ev.random) {
    return `${change < 0 ? `${ev.probability} reduce` : `${ev.probability} increase`}`
  } else {
    return `${change < 0 ? '<strong>Reduces</strong>' : '<strong>Increases</strong>'}`
  }
}

function formatParam(param) {
  if (param == '?') {
    return '<span class="unknown-param">?</span>';
  } else {
    return '<strong>'+Math.abs(param)+'</strong>';
  }
}

function render(e) {
  let demand = format.outputs(state.gameState.output_demand);
  switch (e.type) {
    case 'WorldVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return {
            tip: {
              icon: 'contentedness',
              text: `Current world contentedeness is ${Math.round(state.gameState.world.contentedness)}<span class="type-total">/${consts.maxValues['contentedness']}</span>.`,
            },
            text: `[contentedness] ${changeDir(e.param, e)} world contentedness by ${formatParam(e.param)}.`,
          }
        }
        case 'Emissions': {
          return {
            tip: {
              icon: 'emissions',
              text: `This will directly change annual emissions by ${e.param == '?' ? 'an unknown amount' : format.sign(e.param)}.${e.param !== '?' ? ` That's a ${(e.param/state.gameState.world.emissions * 100).toFixed(1)}% change.` : ''}`,
            },
            text: `[emissions] ${changeDir(e.param, e)} emissions by ${formatParam(e.param)}.`
          }
        }
        case 'ExtinctionRate': {
          return {
            tip: {
              icon: 'extinction_rate',
              text: `Current biodiversity pressure is ${state.gameState.world.extinction_rate.toFixed(0)}<span class="type-total">/${consts.maxValues['biodiversity']}</span>.`,
            },
            text: `[extinction_rate] ${changeDir(e.param, e)} biodiversity pressure by ${formatParam(e.param)}.`,
          }
        }
        case 'Temperature': {
          return {
            tip: {
              icon: 'warming',
              text: `This will directly change the global temperature anomaly by ${format.sign(e.param)}<strong>°c</strong>.`,
            },
            text: `[warming] ${changeDir(e.param, e)} the global temperature by ${formatParam(e.param)}<strong>°c</strong>.`
          };
        }
        case 'Precipitation': {
          return {
            tip: {
              icon: 'water',
              text: `This will directly change global precipitation by ${format.sign(e.param)}<strong>cm/yr</strong>.`,
            },
            text: `[water] ${changeDir(e.param, e)} global precipitation by ${formatParam(e.param)}<strong>cm/yr</strong>.`
          };
        }
        case 'PopulationGrowth': {
          return {
            tip: {
              icon: 'population',
              text: 'The number of people on the planet.',
            },
            text: `[population] ${changeDir(e.param, e)} global population growth by ${formatParam(e.param)}<strong>%.</strong>`,
          };
        }
        case 'Population': {
          return {
            tip: {
              icon: 'population',
              text: 'The number of people on the planet.',
            },
            text: `[population] ${changeDir(e.param, e)} global population by ${formatParam(e.param)}.`,
          };
        }
        case 'SeaLevelRiseRate': {
          return {
            tip: {
              icon: 'sea_level_rise',
              text: `The amount of sea level rise is currently ${state.gameState.world.sea_level_rise.toFixed(2)}m.`,
            },
            text: `[sea_level_rise] ${changeDir(e.param, e)} the rate of sea level rise by ${formatParam(e.param * 1000)}mm/year.`,
          };
        }
        default: {
          if (VERSION === 'dev') {
            console.log(`Unhandled WorldVariable effect type: ${e.subtype}`);
            console.log(e);
          }
        }
      }
      return;
    }
    case 'PlayerVariable': {
      switch (e.subtype) {
        case 'ResearchPoints': {
          return {
            tip: {
              icon: 'research',
              text: 'Research points: Allocate them to research projects!'
            },
            text: `[research] ${e.random ? 'Possible ' : ''}+${formatParam(e.param)} research points.`,
          }
        }
      }
      return;
    }
    case 'ProcessLimit': {
      let process = state.gameState.processes[e.entity];
      let p = Math.abs(e.param/process.limit * 100);
      if (p < 1) {
        p = '<1%';
      } else {
        p = `${Math.round(p)}%`;
      }
      return {
        tip: {
          icon: 'alert',
          text: `.`
        },
        text: `${changeDir(e.param, e)} maximum output for ${process.name} by <strong>${p}</strong>`,
      }
    }
    case 'RegionHabitability': {
      return {
        tip: {
          icon: 'habitability',
          text: `Lower habitability means unhappier people who may need to migrate to more hospitable locales.`
        },
        text: `[habitability] ${changeDir(e.param, e)} habitability in ${e.subtype.toLowerCase()} regions by ${formatParam(e.param)}.`,
      }
    }
    case 'Resource': {
      let k = display.enumKey(e.subtype);
      let amount = format.output(e.param, k);
      let percent = (e.param/state.gameState.resources[k] * 100).toFixed(1);
      let text = `${changeDir(e.param, e)} ${display.enumDisplay(k)} supply by <img src="${icons[k]}">${Math.abs(amount)} (${format.sign(percent)}% of current supply).`;
      return {
        tip: factors.tips[k](text),
        text: `[${k}] ${changeDir(e.param, e)} ${display.enumDisplay(k)} supply by [${k}]${Math.abs(amount)}.`,
      }
    }
    case 'Output': {
      let k = display.enumKey(e.subtype);
      let base = format.output(state.gameState.produced[k], k);
      let changed = (base * (1+e.param)).toFixed(0);
      return {
        tip: {
          icon: k,
          text: `Global ${display.displayName(e.subtype)} output will change from <img src="${icons[k]}">${base} to <img src="${icons[k]}">${changed} with no change in impacts.`
        },
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e)} all ${display.displayName(e.subtype)} production by <strong>${(Math.abs(e.param)*100).toFixed(0)}%.</strong>`,
      }
    }
    case 'OutputForProcess': {
      let process = state.gameState.processes[e.entity];
      let tip = {
        icon: display.enumKey(process.output),
        text: `Changes the output for this process by ${(e.param*100).toFixed(0)}% with no change in impacts.`,
        card: {
          type: 'Process',
          data: process,
        }
      };
      let tag = display.cardTag(process.name, process.output.toLowerCase());
      return {
        tip: tip,
        text: `[${process.output.toLowerCase()}] ${changeDir(e.param, e)} ${tag} output by <strong>${(Math.abs(e.param)*100).toFixed(0)}%.</strong>`
      }
    }
    case 'OutputForFeature': {
      let tip = {
        icon: e.subtype,
        text: `Changes the output for these processes by ${(e.param*100).toFixed(0)}% without changing their impacts.`,
        card: {
          type: 'Processes',
          data: state.gameState.processes.filter((p) => p.features.includes(e.subtype))
        }
      };
      return {
        tip,
        text: `${changeDir(e.param, e)} output for<span><img class="effect-feature" src="${icons[e.subtype]}" /><strong>${display.describeFeature(e.subtype)}</strong></span> by <strong>${(e.param*100).toFixed(0)}%.</strong>`
      }
    }
    case 'CO2ForFeature': {
      let amount = e.param * 100;
      let label = Math.abs(amount) >= 1 ? amount.toFixed(0) : '<1';
      let tip = {
        icon: e.subtype,
        text: `${changeDir(e.param, e)} the CO2 emissions for these processes by <strong>${label}%.</strong>`,
        card: {
          type: 'Processes',
          data: state.gameState.processes.filter((p) => p.features.includes(e.subtype))
        }
      };
      return {
        tip,
        text: `${changeDir(e.param, e)} CO2 emissions for <span><img class="effect-feature" src="${icons[e.subtype]}" />${display.describeFeature(e.subtype)}</span> by <strong>${label}%.</strong>`
      }
    }
    case 'BiodiversityPressureForFeature': {
      let tip = {
        icon: e.subtype,
        text: `Changes the biodiversity pressure for these processes by <strong>${e.param}.</strong>`,
        card: {
          type: 'Processes',
          data: state.gameState.processes.filter((p) => p.features.includes(e.subtype))
        }
      };
      return {
        tip,
        text: `${changeDir(e.param, e)} biodiversity for <span><img class="effect-feature" src="${icons[e.subtype]}" />${display.describeFeature(e.subtype)}</span> by <strong>${e.param}.</strong>`
      }
    }
    case 'Demand': {
      let k = display.enumKey(e.subtype);
      let name = display.displayName(e.subtype);
      let currentDemand = demand[k];
      let afterDemand = demand[k] * (1+e.param);
      return {
        tip: {
          icon: k,
          text: `This changes ${name} demand from <img src="${icons[k]}">${currentDemand} to <img src="${icons[k]}">${Math.round(afterDemand)}.`
        },
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e)} demand for ${display.displayName(e.subtype)} by <strong>${(Math.abs(e.param)*100).toFixed(0)}%</strong>.`,
      }
    }
    case 'DemandAmount': {
      let k = display.enumKey(e.subtype);
      let name = display.displayName(e.subtype);
      let val = e.param;
      if (e.subtype == 'Electricity' || e.subtype == 'Fuel') {
        val = format.output(val, 'electricity'); // same as fuel
      } else {
        val = format.output(val, 'plant_calories'); // same as animal cals
      }
      let currentDemand = demand[k];
      let afterDemand = demand[k] + val;
      let demandChange = (afterDemand - currentDemand)/demand[k] * 100;
      return {
        tip: {
          icon: k,
          text: `This changes ${name} demand from <img src="${icons[k]}">${currentDemand} to <img src="${icons[k]}">${afterDemand}. This is a ${demandChange.toFixed(0)}% change of all ${name} demand.`
        },
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e)} demand for ${display.displayName(e.subtype)} by <img src="${icons[k]}">${Math.abs(val)}.`,
      }
    }
    case 'UnlocksProject': {
      let project = state.gameState.projects[e.entity];
      let tip = {
        icon: 'unlocks',
        text: `${e.random ? e.probability : 'Will'} unlock this project:`,
        card: {
          type: 'Project',
          data: project
        }
      };
      let tag = display.cardTag(project.name, project.kind.toLowerCase());
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] ${e.probability} unlock the ${tag} project.` : `<strong>Unlocks</strong> the ${tag} project.`}`,
      }
    }
    case 'UnlocksProcess': {
      let process = state.gameState.processes[e.entity];
      let tip = {
        icon: 'unlocks',
        text: `${e.random ? e.probability : 'Will'} unlock this process:`,
        card: {
          type: 'Process',
          data: process
        }
      };
      let tag = display.cardTag(process.name, display.enumKey(process.output));
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] ${e.probability} unlock the ${tag} process.` : `<strong>Unlocks</strong> the ${tag} process.`}`,
      }
    }
    case 'UnlocksNPC': {
      let npc = state.gameState.npcs[e.entity];
      let tip = {
        icon: 'unlocks',
        text: 'This new character will be unlocked:',
        card: {
          type: 'NPC',
          data: npc
        }
      };
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] ${e.probability} unlock ${npc.name}.` : `Unlocks ${npc.name}.`}`,
      }
    }
    case 'ProjectCostModifier': {
      let project = state.gameState.projects[e.entity];
      let p = e.param * 100;
      let amount = e.param * project.cost;
      let icon = project.kind == 'Policy' ? '[political_capital]' : '';
      let tipIcon = project.kind == 'Policy' ? `<img src="${icons.political_capital}">` : '';
      let unit = project.kind == 'Policy' ? '' : ' years';
      let tag = display.cardTag(project.name, project.kind.toLowerCase());
      let kind = 'cost';
      if (project.kind == 'Research') {
        kind = 'research time';
      } else if (project.kind == 'Initiative') {
        kind = 'development time';
      }
      let amountName = e.param == '?' ? e.param : Math.ceil(Math.abs(amount));
      let tipAmount = e.param == '?' ? 'by an unknown amount' : `from ${tipIcon}${project.cost}${unit} to ${tipIcon}${project.cost + amount}${unit}.`;
      let tip = {
        icon: 'cost',
        text: `This effect ${changeDir(e.param, e).toLowerCase()} the ${kind} of this project ${tipAmount}.`,
        card: {
          type: 'Project',
          data: project,
        }
      };
      return {
        tip: tip,
        text: `[cost] ${e.random ? '[chance]' : ''}${changeDir(e.param, e)} ${kind} of ${tag} by ${icon}${formatParam(amountName)}${unit}.`,
      }
    }
    case 'ProjectRequest': {
      let project = state.gameState.projects[e.entity];
      if (e.subtype == 'Ban') {
        let tip = {
          icon: 'request',
          text: `You received a request to stop this project:`,
          card: {
            type: 'Project',
            data: project
          }
        };
        return {
          tip: tip,
          text: `[ban] I request that you stop ${project.name}. (+${e.param}PC)`,
        }
      } else {
        let tip = {
          icon: 'request',
          text: `You received a request to implement this project:`,
          card: {
            type: 'Project',
            data: project
          }
        };
        return {
          tip: tip,
          text: `[implement] I request that you implement ${project.name}. (+${e.param}PC)`,
        }
      }
    }
    case 'ProcessRequest': {
      let process = state.gameState.processes[e.entity];
      if (e.subtype == 'Ban') {
        let tip = {
          icon: 'request',
          text: `You received a request to ban this process:`,
          card: {
            type: 'Process',
            data: process
          }
        };
        return {
          tip: tip,
          text: `[ban] I request that you stop ${process.name}. (+${e.param}PC)`,
        }
      } else {
        let tip = {
          icon: 'request',
          text: `You received a request to promote this process:`,
          card: {
            type: 'Process',
            data: process
          }
        };
        return {
          tip: tip,
          text: `[implement] I request that you implement ${process.name}. (+${e.param}PC)`,
        }
      }
    }
    case 'AddFlag': {
      let flag = e.param.split('::')[1];
      return {
        tip: FLAG_TIPS[flag](demand),
        text: '<strong>' + FLAGS[flag] + '</strong>',
      }
    }
    case 'ModifyIndustryDemand': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let tip = {
        icon: 'demand',
        text: e.param == '?' ?
          `Changes demand for ${industry.name} by an unknown amount.`
          : `Changes demand for ${industry.name} by <strong>${p.toFixed(0)}%.</strong>`,
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: `${changeDir(e.param, e)} demand for ${tag} by ${e.param == '?' ? formatParam(e.param) : `<strong>${p.toFixed(0)}%</strong>`}.`,
      }
    }
    case 'ModifyIndustryResources': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let k = display.enumKey(e.subtype);
      let resource = display.displayName(e.subtype);
      let demandBefore = format.output(industry.resources[k] * industry.demand, k);
      let demandAfter = demandBefore * (1 + e.param);
      let demandChange = (demandAfter - demandBefore)/demand[k] * 100;
      let tip = {
        icon: k,
        text: e.param == '?' ?
          `This will change ${resource} demand for ${industry.name} by some unknown amount.`
          : `This will change ${resource} demand for ${industry.name} from <img src="${icons[k]}">${demandBefore} to <img src="${icons[k]}">${demandAfter < 1 ? '<1' : demandAfter.toFixed(0)}. This is a ${demandChange.toFixed(0)}% change of all ${resource} demand.`,
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e)} ${resource.toLowerCase()} demand for ${tag} by ${e.param == '?' ? formatParam(e.param) : `<strong>${p.toFixed(0)}%</strong>`}.`,
      }
    }
    case 'ModifyIndustryResourcesAmount': {
      let industry = state.gameState.industries[e.entity];
      let k = display.enumKey(e.subtype);
      let resource = display.displayName(e.subtype);
      let demandBefore = format.output(industry.resources[k] * industry.demand, k);
      let demandAfter = demandBefore + format.output((industry.resources[k] + e.param) * industry.demand, k);
      let demandChange = (demandAfter - demandBefore)/demand[k] * 100;
      let tip = {
        icon: k,
        text: e.param == '?' ?
          `This will change ${resource} demand for ${industry.name} by some unknown amount.`
          : `This will change ${resource} demand for ${industry.name} from <img src="${icons[k]}">${demandBefore} to <img src="${icons[k]}">${demandAfter < 1 ? '<1' : demandAfter.toFixed(0)}. This is a ${demandChange.toFixed(0)}% change of all ${resource} demand.`,
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e)} ${resource.toLowerCase()} demand for ${tag} by ${e.param == '?' ? formatParam(e.param) : `${Math.abs(demandAfter - demandBefore)}`}.`,
      }
    }
    case 'ModifyProcessByproducts': {
      let process = state.gameState.processes[e.entity];
      let p = Math.abs(e.param * 100);
      let emissionsBefore = format.co2eq(process.byproducts) * state.gameState.produced_by_process[e.entity] * 1e-15;
      let emissionsAfter = emissionsBefore * (1 + e.param);
      let emissionsChange = (emissionsAfter - emissionsBefore)/state.gameState.world.emissions * 100;
      let label = e.subtype == 'Biodiversity' ? 'biodiversity pressure' : `${e.subtype} emissions`;
      let short = e.subtype == 'Biodiversity' ? 'biodiversity pressure' : 'emissions';
      let icon = e.subtype == 'Biodiversity' ? 'biodiversity' : 'emissions';
      let change = e.subtype == 'Biodiversity' ? `${process.byproducts.biodiversity} to ${e.param}<img src="${icons[icon]}."` : `${emissionsBefore > 0 && emissionsBefore < 1 ? '<1' : emissionsBefore.toFixed(1)} to <img src="${icons.emissions}">${emissionsAfter > 0 && emissionsAfter < 1 ? '<1' : emissionsAfter.toFixed(1)}. This is a ${emissionsChange > 0 && emissionsChange < 1 ? '<1' : emissionsChange.toFixed(1)}% change of all emissions.`
      let tip = {
        icon: icon,
        text: e.param == '?' ?
          `Changes ${label} for ${process.name} by an unknown amount.`
          : `This will change ${short} for ${process.name} from <img src="${icons[icon]}">${change}`,
        card: {
          type: 'Process',
          data: process,
        }
      };
      let tag = display.cardTag(process.name);
      return {
        tip: tip,
        text: `[${icon}] ${changeDir(e.param, e)} ${label} for ${tag} by <strong>${e.param == '?' ? formatParam(e.param) : `${p.toFixed(0)}%`}</strong>.`,
      }
    }

    case 'ModifyIndustryByproducts': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let emissionsBefore = format.co2eq(industry.byproducts) * industry.demand * 1e-15;
      let emissionsAfter = emissionsBefore * (1 + e.param);
      let emissionsChange = (emissionsAfter - emissionsBefore)/state.gameState.world.emissions * 100;
      let tip = {
        icon: 'emissions',
        text: e.param == '?' ?
          `Changes emissions for ${industry.name} by an unknown amount.`
          : `This will change emissions for ${industry.name} from <img src="${icons.emissions}">${emissionsBefore > 0 && emissionsBefore < 1 ? '<1' : emissionsBefore.toFixed(1)} to <img src="${icons.emissions}">${emissionsAfter > 0 && emissionsAfter < 1 ? '<1' : emissionsAfter.toFixed(1)}. This is a ${emissionsChange > 0 && emissionsChange < 1 ? '<1' : emissionsChange.toFixed(1)}% change of all emissions.`,
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: `[emissions] ${changeDir(e.param, e)} ${e.subtype} emissions for ${tag} by <strong>${e.param == '?' ? formatParam(e.param) : `${p.toFixed(0)}%`}</strong>.`,
      }
    }
    case 'DemandOutlookChange': {
      let k = display.enumKey(e.subtype);
      let change = effects.demandOutlookChange(state.gameState.world, k, e.param);
      change = Math.round(change);
      return {
        tip: {
          icon: 'contentedness',
          subicon: k,
          text: `This changes regional contentedness based on demand for ${display.displayName(e.subtype)}. Current world contentedeness is ${Math.round(state.gameState.world.contentedness)}<span class="type-total">/${consts.maxValues['contentedness']}</span>.`,
        },
        text: `[contentedness] [${e.subtype.toLowerCase()}] ${changeDir(e.param, e)} world contentedness by <strong>${Math.abs(change)}</strong>.`
      }
    }
    case 'IncomeOutlookChange': {
      let change = effects.incomeOutlookChange(state.gameState.world, e.param);
      change = Math.round(change);
      return {
        tip: {
          icon: 'contentedness',
          subicon: 'wealth',
          text: `This changes regional contentedness by ${e.param} per income level (wealthier regions will feel it more). Current world contentedeness is ${Math.round(state.gameState.world.contentedness)}<span class="type-total">/${consts.maxValues['contentedness']}</span>.`,
        },
        text: `[contentedness] ${changeDir(e.param, e)} contentedness by <strong>${Math.abs(change)}</strong>.`
      }
    }
    case 'ModifyEventProbability': {
      let event = EVENTS[e.entity].name;
      let p = e.param == '?' ? '?' : e.param * 100;
      return {
        tip: {
          icon: 'chance',
          text: `${changeDir(p, e)} the chance of "${event}" by ${formatParam(p)}%`,
        },
        text: `${changeDir(p, e)} the chance of "${event}" by ${formatParam(p)}%`,
      }
    }
    case 'ProtectLand': {
      return {
        tip: {
          icon: 'land',
          text: 'This will limit the amount of land that processes can use.'
        },
        text: `[land] Place <strong>${e.param}%</strong> of land under protection.`,
      }
    }
    case 'Feedstock': {
      let k = display.enumKey(e.subtype);
      let name = display.enumDisplay(e.subtype);

      let estimate;
      if (k == 'other' || k == 'soil') {
        estimate = null;
      } else {
        estimate = state.gameState.feedstocks[k]/state.gameState.consumed_feedstocks[k];
        estimate = Math.round(estimate);
      }

      let text;
      if (estimate == null) {
        text = 'We aren\'t tracking this feedstock.';
      } else if (estimate == 0) {
        text = 'This feedstock has been depleted.';
      } else if (isFinite(estimate)) {
        text = `At current usage rates the estimated supply is expected to last ${estimate} years.`;
      } else {
        text = `At current usage rates the estimated supply is expected to last indefinitely.`;
      }
      return {
        tip: {
          icon: k,
          text,
        },
        text: `[${k}] ${changeDir(e.param, e)} ${name} supply by <strong>${e.param*100}%.</strong>`,
      }
    }
    case 'LocksProject': {
      let project = state.gameState.projects[e.entity];
      let tag = display.cardTag(project.name, project.kind.toLowerCase());
      return {
        tip: {
          icon: 'alert',
          text: `${project.name} will be unavailable while this project is active.`,
          card: {
            type: 'Project',
            data: project
          }
        },
        text: `[locks] <strong>Locks</strong> ${tag}`,
      }
    }
    case 'TerminationShock': {
      let proj = state.gameState.projects.find((p) => p.name == 'Solar Radiation Management');
      let effects = activeEffects(proj);
      let temp_change = 0;
      let temp_effect = effects.find((eff) => eff.subtype == 'Temperature');
      if (temp_effect) {
        temp_change = -temp_effect.param;
      }
      return {
        tip: {
          icon: 'warming',
          text: `This will directly change the global temperature anomaly by ${format.sign(temp_change)}<strong>°c</strong>.`,
        },
        text: `[warming] ${changeDir(temp_change, e)} the global temperature by ${formatParam(temp_change)}<strong>°c</strong>.`
      };
    }
    default: {
      if (VERSION === 'dev') {
        console.log(`Unhandled effect type: ${e.type}`);
        console.log(e);
      }
      return null;
    }
  }
}

export default {
  props: ['effects'],
  computed: {
    renders() {
      let descs = this.effects
        .filter((eff) => !eff.hidden)
        .map((ev) => {
          let desc = render(ev);
          if (desc) {
            if (ev.random) {
              desc.tip.supicon = 'chance';
            }
            desc.text = display.fillIcons(desc.text);
            return desc;
          }
        })
        .filter((desc) => desc !== undefined);

      // Remove duplicates
      return descs.filter((item, i) => {
        return descs.indexOf(item) == i;
      });

    }
  },
}
</script>

<style>
.effect--text {
  font-size: 0.9em;
  text-align: center;
}
.effect--text img {
  height: 16px;
  vertical-align: middle;
  margin-top: -2px;
}
.effect--text span {
  white-space: nowrap;
}
.effect--text .effect-feature {
  background: #222;
  border-radius: 1.2em;
  padding: 0.2em;
  height: 20px;
  margin: 0 2px;
  border: 1px solid #888;
}

.card-tag {
  border-radius: 0.2em;
  display: inline-block;
  font-size: 0.9em;
  background-color: rgba(0,0,0,0.3);
  padding: 0.05em 0.2em;
  color: rgba(255,255,255,0.9);
  font-weight: 600;
  letter-spacing: 0.01em;
}
.card-tag:hover{
  background-color: rgba(0,0,0,0.5);
}

.card-tag img {
  height: 13px;
  margin-right: 3px;
}

.unknown-param {
  color: #fff;
  background: rgba(0,0,0,0.7);
  border-radius: 2em;
  font-weight: bold;
  padding: 0.25em 0.6em;
  font-size: 0.9em;
}

</style>
