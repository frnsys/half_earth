<template>
  <div v-for="{tip, text} in renders" class="effect" v-tip="tip ? tip : 'missing tip'">
    <div class="effect--text" v-html="text" />
  </div>
</template>

<script>
import icons from 'components/icons';
import {slugify} from 'lib/util';
import game from '/src/game';
import state from '/src/state';
import format from '/src/display/format';
import factors from '/src/display/factors';
import effects from '/src/display/effects';
import display from '/src/display/display';
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
  'EVs': (demand) => {
    // TODO
    return {
      icon: 'car',
      text: 'TODO flag is not implemented in the engine yet'
    }
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
      text: 'TODO'
    }
  },
  'StopDevelopment': (demand) => {
    return {
      icon: 'ban',
      text: 'Stops regional development throughout the world.'
    }
  },
};

function changeDir(change, random) {
  if (change == '?') {
    return 'Changes';
  } else if (random) {
    return `${change < 0 ? 'Could reduce' : 'Could increase'}`
  } else {
    return `${change < 0 ? 'Reduces' : 'Increases'}`
  }
}

function formatParam(param) {
  if (param == '?') {
    return '<span class="unknown-param">?</span>';
  } else {
    return Math.abs(param);
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
              text: 'TODO'
            },
            text: `[contentedness] ${changeDir(e.param, e.random)} world contentedness by ${formatParam(e.param)}.`,
          }
        }
        case 'Emissions': {
          return {
            tip: {
              icon: 'emissions',
              text: `This will directly change annual emissions by ${e.param == '?' ? 'an unknown amount' : format.sign(e.param)}.${e.param !== '?' ? ` That's a ${(e.param/state.gameState.emissions * 100).toFixed(1)}% change.` : ''}`,
            },
            text: `[emissions] ${changeDir(e.param, e.random)} emissions by ${formatParam(e.param)}.`
          }
        }
        case 'ExtinctionRate': {
          return {
            tip: {
              icon: 'extinction_rate',
              text: `Current biodiversity pressure is ${state.gameState.world.extinction_rate.toFixed(0)}.`,
            },
            text: `[extinction_rate] ${changeDir(e.param, e.random)} biodiversity pressure by ${formatParam(e.param)}.`,
          }
        }
        case 'Temperature': {
          return {
            tip: {
              icon: 'warming',
              text: `This will directly change the global temperature anomaly by ${format.sign(e.param)}C.`,
            },
            text: `[warming] ${changeDir(e.param, e.random)} the global temperature by ${formatParam(e.param)}C.`
          };
        }
        case 'PopulationGrowth': {
          return {
            tip: {
              icon: 'population',
              text: 'The number of people on the planet.',
            },
            text: `[population] ${changeDir(e.param, e.random)} global population growth by ${formatParam(e.param)}%.`,
          };
        }
        case 'Population': {
          return {
            tip: {
              icon: 'population',
              text: 'The number of people on the planet.',
            },
            text: `[population] ${changeDir(e.param, e.random)} global population by ${formatParam(e.param)}.`,
          };
        }
        case 'SeaLevelRiseRate': {
          return {
            tip: {
              icon: 'sea_level_rise',
              text: `The amount of sea level rise is currently ${state.gameState.world.sea_level_rise.toFixed(2)}m.`,
            },
            text: `[sea_level_rise] ${changeDir(e.param, e.random)} the rate of sea level rise by ${formatParam(e.param * 1000)}mm/year.`,
          };
        }
        default: {
          console.log(`Unhandled WorldVariable effect type: ${e.subtype}`);
          console.log(e);
        }
      }
      return;
    }
    case 'LocalVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return {
            tip: {
              icon: 'contentedness',
              text: 'TODO',
            },
            text: `[contentedness] ${changeDir(e.param, e.random)} contentedness in TODO by ${e.param}.`,
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
    case 'Output': {
      let k = display.enumKey(e.subtype);
      let base = format.output(state.gameState.produced[k], k);
      let changed = (base * (1+e.param)).toFixed(0);
      return {
        tip: {
          icon: k,
          text: `Global ${display.displayName(e.subtype)} output will change from <img src="${icons[k]}">${base} to <img src="${icons[k]}">${changed} with no change in impacts.`
        },
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} all ${display.displayName(e.subtype)} production by ${e.param*100}%.`,
      }
    }
    case 'OutputForProcess': {
      let process = state.gameState.processes[e.entity];
      let tip = {
        icon: display.enumKey(process.output),
        text: `Changes the output for this process by ${e.param*100}% with no change in impacts.`,
        card: {
          type: 'Process',
          data: process,
        }
      };
      let tag = display.cardTag(process.name, process.output.toLowerCase());
      return {
        tip: tip,
        text: `[${process.output.toLowerCase()}] ${changeDir(e.param, e.random)} ${tag} output by ${e.param*100}%.`
      }
    }
    case 'OutputForFeature': {
      let tip = {
        icon: display.enumKey(e.subtype),
        text: `Changes the output for these processes by ${e.param*100}% without changing their impacts.`,
        card: {
          type: 'Processes',
          data: state.gameState.processes.filter((p) => p.features.includes(e.subtype))
        }
      };
      return {
        tip,
        text: `${changeDir(e.param, e.random)} output for ${display.describeFeature(e.subtype)} by ${e.param*100}%.`
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
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} demand for ${display.displayName(e.subtype)} by ${e.param*100}%.`,
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
      return {
        tip: {
          icon: k,
          text: `This changes ${name} demand from <img src="${icons[k]}">${currentDemand} to <img src="${icons[k]}">${afterDemand}.`
        },
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} demand for ${display.displayName(e.subtype)} by <img src="${icons[k]}">${Math.abs(val)}.`,
      }
    }
    case 'UnlocksProject': {
      let project = state.gameState.projects[e.entity];
      let tip = {
        icon: 'unlocks',
        text: `This new project ${e.random ? 'might' : 'will'} be unlocked:`,
        card: {
          type: 'Project',
          data: project
        }
      };
      let tag = display.cardTag(project.name, project.kind.toLowerCase());
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] Could unlock the ${tag} project.` : `Unlocks the ${tag} project.`}`,
      }
    }
    case 'UnlocksProcess': {
      let process = state.gameState.processes[e.entity];
      let tip = {
        icon: 'unlocks',
        text: `This new process ${e.random ? 'might' : 'will'} be unlocked:`,
        card: {
          type: 'Process',
          data: process
        }
      };
      let tag = display.cardTag(process.name, display.enumKey(process.output));
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] Could unlock the ${tag} process.` : `Unlocks the ${tag} process.`}`,
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
        text: `[unlocks] ${e.random ? `[chance] Could unlock ${npc.name}.` : `Unlocks ${npc.name}.`}`,
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
      let tip = {
        icon: 'cost',
        text: `This effect reduces the ${kind} of this project from ${tipIcon}${project.cost}${unit} to ${tipIcon}${project.cost + amount}${unit}.`,
        card: {
          type: 'Project',
          data: project,
        }
      };
      return {
        tip: tip,
        text: `[cost] ${e.random ? '[chance]' : ''}${changeDir(e.param, e.random)} ${kind} of ${tag} by ${icon}${formatParam(amountName)}${unit}.`,
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
        text: FLAGS[flag],
      }
    }
    case 'ModifyIndustryDemand': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let tip = {
        icon: 'demand',
        text: e.param == '?' ?
          `Changes demand for ${industry.name} by an unknown amount.`
          : `Changes demand for ${industry.name} by ${p.toFixed(0)}%.`,
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: `${changeDir(e.param, e.random)} demand for ${tag} by ${e.param == '?' ? formatParam(e.param) : `${p.toFixed(0)}%`}.`,
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
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} ${resource.toLowerCase()} demand for ${tag} by ${e.param == '?' ? formatParam(e.param) : `${p.toFixed(0)}%`}.`,
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
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} ${resource.toLowerCase()} demand for ${tag} by ${e.param == '?' ? formatParam(e.param) : `${Math.abs(demandAfter - demandBefore)}`}.`,
      }
    }
    case 'ModifyIndustryByproducts': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let emissionsBefore = format.co2eq(industry.byproducts) * industry.demand * 1e-15;
      let emissionsAfter = emissionsBefore * (1 + e.param);
      let emissionsChange = (emissionsAfter - emissionsBefore)/state.gameState.emissions * 100;
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
        text: `[emissions] ${changeDir(e.param, e.random)} ${e.subtype} emissions for ${tag} by ${e.param == '?' ? formatParam(e.param) : `${p.toFixed(0)}%`}.`,
      }
    }
    case 'DemandOutlookChange': {
      let k = display.enumKey(e.subtype);
      let change = effects.demandOutlookChange(state.gameState.world, e.param);
      change = Math.round(change);
      return {
        tip: {
          icon: 'contentedness',
          subicon: k,
          text: `Regional contentedness changes based on demand for ${display.displayName(e.subtype)}.  Global contentedness will change by ${change}.`
        },
        text: `[contentedness] [${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} contentedness by ${Math.abs(change)}.`
      }
    }
    case 'IncomeOutlookChange': {
      let change = effects.incomeOutlookChange(state.gameState.world, e.param);
      change = Math.round(change);
      return {
        tip: {
          icon: 'contentedness',
          subicon: 'wealth',
          text: `Regional contentedness changes by ${e.param} per income level (wealthier regions will feel it more). Global contentedness will change by ${change}.`
        },
        text: `[contentedness] ${changeDir(e.param, e.random)} contentedness by ${Math.abs(change)}.`
      }
    }
    case 'ModifyEventProbability': {
      let event = EVENTS[e.entity].name;
      let p = e.param * 100;
      return {
        tip: {
          icon: 'chance',
          text: 'TODO'
        },
        text: `${changeDir(p, e.random)} the chance of ${event} by ${Math.abs(p)}%`,
      }
    }
    case 'ProtectLand': {
      return {
        tip: {
          icon: 'land',
          text: 'This will limit the amount of land that processes can use.'
        },
        text: `[land] Place ${e.param}% of land under protection.`,
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
        text = 'TODO';
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
        text: `[${k}] ${changeDir(e.param, e.random)} ${name} supply by ${e.param*100}%.`,
      }
    }

    default:
      console.log(`Unhandled effect type: ${e.type}`);
      console.log(e);
      return null;
  }
}

export default {
  props: ['effects'],
  computed: {
    renders() {
      let descs = this.effects
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

.card-tag {
  border-radius: 0.2em;
  display: inline-block;
  font-size: 0.9em;
  background: #475664;
  padding: 0.05em 0.2em;
}
.card-tag img {
  height: 13px;
  margin-right: 3px;
}

.unknown-param {
  color: #9dbbd8;
}
</style>
