<template>
  <div v-for="{tip, text} in renders" class="effect" v-tip="tip ? tip : 'missing tip'">
    <div class="effect--text" v-html="text" />
  </div>
</template>

<script>
import {sign, slugify} from 'lib/util';
import game from '/src/game';
import state from '/src/state';
import display from 'lib/display';
import FLAGS from '/assets/content/flags.json';
import EVENTS from '/assets/content/events.json';
import ICONEVENTS from '/assets/content/icon_events.json';

const tips = {
  contentedness: {
    icon: 'contentedness',
    text: `Contentedness is X`,
  },
  emissions: {
    icon: 'emissions',
    text: `Emissions is X`,
  },
  extinction: {
    icon: 'extinction_rate',
    text: `Extinction rate is X`,
  },
  temperature: {
    icon: 'warming',
    text: `Temperature is X`,
  },
  population: {
    icon: 'population',
    text: `Population is X`,
  },
  fuel: {
    icon: 'fuel',
    text: 'Fuel: portable energy...'
  },
  electricity: {
    icon: 'electricity',
    text: 'Electricity: ...'
  },
  plantcalories: {
    icon: 'plantcalories',
    text: 'Plant calories: ...'
  },
  animalcalories: {
    icon: 'animalcalories',
    text: 'Animal calories: ...'
  },
  land: {
    icon: 'land',
    text: 'Land: ...'
  },
  events: {
    icon: 'events',
    text: 'Events: ...'
  },
  demand: {
    icon: 'demand',
    text: 'Demand: ...'
  },
  request_ban: {
    icon: 'ban',
    text: 'Ban Request: ...'
  },
  request_implement: {
    icon: 'implement',
    text: 'Implement Request: ...'
  },
  cost: {
    icon: 'cost',
    text: 'Cost: ...'
  },
  unlocks_npc: {
    icon: 'unlocks',
    text: 'Unlocks NPC: ...'
  },
  unlocks_process: {
    icon: 'unlocks',
    text: 'Unlocks Process: ...'
  },
  unlocks_project: {
    icon: 'unlocks',
    text: 'Unlocks Project: ...'
  },
  research_points: {
    icon: 'research',
    text: 'Research Points: ...'
  }
};

function changeDir(change, random) {
  if (change == '?') {
    return 'Changes';
  } else if (random) {
    return `${change < 0 ? 'Might reduce' : 'Might increase'}`
  } else {
    return `${change < 0 ? 'Reduces' : 'Increases'}`
  }
}

function formatParam(param, sign) {
  if (param == '?') {
    return '<span class="unknown-param">?</span>';
  } else if (sign) {
    return sign(param);
  } else {
    return Math.abs(param);
  }
}

function render(e) {
  let demand = display.outputs(state.gameState.output_demand);
  switch (e.type) {
    case 'WorldVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return {
            tip: tips['contentedness'],
            text: `[contentedness] ${changeDir(e.param, e.random)} contentedness by ${formatParam(e.param)} in every region.`,
          }
        }
        case 'Emissions': {
          return {
            tip: tips['emissions'],
            text: `[emissions] ${changeDir(e.param, e.random)} emissions by ${formatParam(e.param)}GtCO2eq.`
          }
        }
        case 'ExtinctionRate': {
          return {
            tip: tips['extinction'],
            text: `[extinction_rate] ${changeDir(e.param, e.random)} the extinction rate by ${formatParam(e.param)}.`,
          }
        }
        case 'Temperature': {
          return {
            tip: tips['temperature'],
            text: `[warming] ${changeDir(e.param, e.random)} the global temperature by ${formatParam(e.param)}C.`
          };
        }
        case 'PopulationGrowth': {
          return {
            tip: tips['population'],
            text: `[population] ${changeDir(e.param, e.random)} global population growth by ${formatParam(e.param)}%.`,
          };
        }
      }
      return;
    }
    case 'LocalVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return {
            tip: tips['contentedness'],
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
            tip: tips['research_points'],
            text: `[research] ${e.random ? 'Possible ' : ''}${formatParam(e.param, true)} research points.`,
          }
        }
      }
      return;
    }
    case 'Output': {
      return {
        tip: tips[e.subtype.toLowerCase()],
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} all ${display.displayName(e.subtype)} production by ${e.param*100}%.`,
      }
    }
    case 'OutputForProcess': {
      let process = state.gameState.processes[e.entity];
      let tip = {...tips[process.output.toLowerCase()]};
      tip.card = {
        type: 'Process',
        data: process,
      };
      let tag = display.cardTag(process.name, process.output.toLowerCase());
      return {
        tip: tip,
        text: `[${process.output.toLowerCase()}] ${changeDir(e.param, e.random)} ${tag} output by ${e.param*100}%.`
      }
    }
    case 'OutputForFeature': {
      let tip = {...tips[e.subtype]};
      tip.card = {
        type: 'Processes',
        data: state.gameState.processes.filter((p) => p.features.includes(e.subtype))
      };
      return {
        tip: tip,
        text: `${changeDir(e.param, e.random)} output for ${display.describeFeature(e.subtype)} by ${e.param*100}%`
      }
    }
    case 'Demand': {
      // TODO maybe these should all be expressed as per-capita?
      // TODO show current demand?
      return {
        tip: tips[e.subtype.toLowerCase()],
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} demand for ${display.displayName(e.subtype)} by ${e.param*100}%`,
      }
    }
    case 'DemandAmount': {
      let val = e.param;
      if (e.subtype == 'Electricity' || e.subtype == 'Fuel') {
        val = display.output(val, 'electricity'); // same as fuel
      } else {
        val = display.output(val, 'plant_calories'); // same as animal cals
      }
      return {
        tip: tips[e.subtype.toLowerCase()],
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} demand for ${display.displayName(e.subtype)} by ${Math.abs(val)}%`,
      }
    }
    case 'UnlocksProject': {
      let project = state.gameState.projects[e.entity];
      let tip = {...tips['unlocks_project']};
      tip.card = {
        type: 'Project',
        data: project
      };
      let tag = display.cardTag(project.name, project.kind.toLowerCase());
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] Might unlock the ${tag} project.` : `Unlocks the ${tag} project.`}`,
      }
    }
    case 'UnlocksProcess': {
      let process = state.gameState.processes[e.entity];
      let tip = {...tips['unlocks_process']};
      tip.card = {
        type: 'Process',
        data: process
      };
      let tag = display.cardTag(process.name, display.enumKey(process.output));
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] Might unlock the ${tag} process.` : `Unlocks the ${tag} process.`}`,
      }
    }
    case 'UnlocksNPC': {
      let npc = state.gameState.npcs[e.entity];
      let tip = {...tips['unlocks_npc']};
      tip.card = {
        type: 'NPC',
        data: npc
      };
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] Might unlock ${npc.name}.` : `Unlocks ${npc.name}.`}`,
      }
    }
    case 'ProjectCostModifier': {
      let project = state.gameState.projects[e.entity];
      let p = e.param * 100;
      let amount = e.param * project.cost;
      let tip = {...tips['cost']};
      tip.card = {
        type: 'Project',
        data: project,
      };
      let tag = display.cardTag(project.name, project.kind.toLowerCase());
      let kind = 'cost';
      if (project.kind == 'Research') {
        kind = 'research time';
      } else if (project.kind == 'Initiative') {
        kind = 'development time';
      }
      return {
        tip: tip,
        text: `[cost] ${e.random ? '[chance]' : ''}${changeDir(e.param, e.random)} ${kind} of ${tag} by ${project.kind == 'Policy' ? '[political_capital]' : ''}${Math.abs(amount)}${project.kind == 'Policy' ? '' : ' years'}.`,
      }
    }
    case 'ProjectRequest': {
      // TODO display requester
      let project = state.gameState.projects[e.entity];
      if (e.subtype == 'Ban') {
        let tip = {...tips['request_ban']};
        tip.card = {
          type: 'Project',
          data: project
        };
        return {
          tip: tip,
          text: `[ban] I request that you stop ${project.name}. (+${e.param}PC)`,
        }
      } else {
        let tip = {...tips['request_implement']};
        tip.card = {
          type: 'Project',
          data: project
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
        let tip = {...tips['request_ban']};
        tip.card = {
          type: 'Process',
          data: process
        };
        return {
          tip: tip,
          text: `[ban] I request that you stop ${process.name}. (+${e.param}PC)`,
        }
      } else {
        let tip = {...tips['request_implement']};
        tip.card = {
          type: 'Process',
          data: process
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
        tip: tips['TODO'],
        text: FLAGS[flag],
      }
    }
    case 'ModifyIndustryDemand': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let tip = {...tips['demand']};
      tip.card = {
        type: 'Industry',
        data: industry,
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: `${changeDir(e.param, e.random)} demand for ${tag} by ${p.toFixed(0)}%.`,
      }
    }
    case 'ModifyIndustryResources': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let tip = {...tips[e.subtype.toLowerCase()]};
      tip.card = {
        type: 'Industry',
        data: industry,
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: `[${e.subtype.toLowerCase()}] ${changeDir(e.param, e.random)} ${e.subtype.toLowerCase()} demand for ${tag} by ${p.toFixed(0)}%.`,
      }
    }
    case 'ModifyIndustryByproducts': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let tip = {...tips['emissions']};
      tip.card = {
        type: 'Industry',
        data: industry,
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: `[emissions] ${changeDir(e.param, e.random)} ${e.subtype} emissions for ${tag} by ${p.toFixed(0)}%.`, // TODO should show the amount. e.g. this is X emissions/X% of total emissions
      }
    }
    case 'DemandOutlookChange': {
      let k = display.displayName(e.subtype);
      let outlookChange = Math.floor(state.gameState.output_demand[k] * e.param);
      return {
        tip: tips['contentedness'],
        text: `[contentedness] [${e.subtype.toLowerCase()}] ${changeDir(outlookChange, e.random)} contentedness by ${outlookChange}`
      }
    }
    case 'IncomeOutlookChange': {
      // TODO
      /* let outlookChange = Math.floor(game.total_income_level() * e.param); */
      let outlookChange = 0;
      return {
        tip: tips['contentedness'],
        text: sign(outlookChange),
      }
    }
    case 'ModifyEventProbability': {
      let event = EVENTS[e.entity].name;
      let p = e.param * 100;
      return {
        tip: tips['events'],
        text: `${changeDir(p, e.random)} the chance of ${event} by ${Math.abs(p)}%`,
      }
    }
    case 'ProtectLand': {
      return {
        tip: tips['land'],
        text: `[land] Place ${e.param}% of land under protection.`,
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
              desc.supicon = 'chance';
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
