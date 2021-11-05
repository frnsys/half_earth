<template>
  <div class="effects">
    <div v-for="{tip, text} in renders" class="effect" v-tip="tip ? tip : 'missing tip'" v-html="text" />
  </div>
</template>

<script>
import {sign} from 'lib/util';
import game from '/src/game';
import state from '/src/state';
import consts from '/src/consts';
import display from 'lib/display';
import assets from 'components/assets';
import FLAGS from '/assets/content/flags.json';
import EVENTS from '/assets/content/events.json';
import ICONEVENTS from '/assets/content/icon_events.json';

function render(e) {
  let demand = display.outputs(state.gameState.output_demand);
  switch (e.type) {
    case 'WorldVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return {
            tip: {
              icon: 'contentedness',
              text: `Changes contentedness by ${sign(e.param)} in every region.`,
            },
            text: `${sign(e.param * state.gameState.world.regions.length)} <img src="/assets/icons/contentedness.png">`,
          }
        }
        case 'Emissions': {
          return {
            tip: {
              icon: 'emissions',
              text: `Changes emissions by ${sign(e.param)}GtCO2eq.`
            },
            text: `${sign(e.param)} <img src="/assets/icons/emissions.png">`,
          }
        }
        case 'ExtinctionRate': {
          return {
            tip: {
              icon: 'extinction',
              text: `Changes the extinction rate by ${sign(e.param)}.`,
            },
            text: `${sign(e.param)} <img src="/assets/icons/extinction.png">`,
          }
        }
        case 'Temperature': {
          return {
            tip: {
              icon: 'warming',
              text: `${sign(e.param)}C to the global temperature.`
            },
            text: `${sign(e.param)}C <img src="/assets/icons/warming.png">`,
          };
        }
        case 'PopulationGrowth': {
          return {
            tip: {
              icon: 'population',
              text: `Changes global population growth by ${sign(e.param)}%.`,
            },
            text: `${sign(e.param)}% <img src="/assets/icons/labor.png">`,
          };
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
              text: `Changes contentedness in TODO by ${sign(e.param)}.`,
            },
            text: `${sign(e.param)} <img src="/assets/icons/contentedness.png">`
          }
        }
      }
      return;
    }
    case 'Output': {
      return {
        tip: {
          icon: e.subtype.toLowerCase(),
          text: `Changes ${consts.outputs.names[e.subtype]} production by ${sign(e.param*100)}%.`,
        },
        text: `${sign(e.param*100)}% <img src="${assets.icons[e.subtype.toLowerCase()]}">`,
      }
    }
    case 'OutputForProcess': {
      let process = state.gameState.processes[e.entity];
      // TODO process icons
      return {
        tip: {
          icon: 'TODO PROCESS ICON',
          text: 'TODO PROCESS ICON',
        },
        text: `${sign(e.param*100)}% to ${process.name} output.`
      }
    }
    case 'OutputForFeature': {
      // TODO process feature icons
      return {
        tip: {
          icon: 'TODO PROCESS ICON',
          text: 'TODO PROCESS ICON',
        },
        text: `${sign(e.param*100)}% to ${e.subtype} output.`
      }
    }
    case 'Demand': {
      return {
        tip: {
          icon: e.subtype.toLowerCase(),
          text: `Changes demand for ${consts.outputs.names[e.subtype]} by ${sign(e.param*100)}%. Current demand is TODO`,
        },
        text: `<img src="${assets.icons['population']}"> ${sign(e.param*100)}%<img src="${assets.icons[e.subtype.toLowerCase()]}">`,
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
        tip: {
          icon: e.subtype.toLowerCase(),
          text: `Changes demand for ${consts.outputs.names[e.subtype]} by ${sign(val)}<img src="/assets/icons/electricity.png">. Current demand is ${demand[consts.outputs.keys[e.subtype]]}<img src="/assets/icons/electricity.png">.`
        },
        text: `<img src="${assets.icons['population']}"> ${sign(val)}<img src="${assets.icons[e.subtype.toLowerCase()]}">`
      }
    }
    case 'UnlocksProject': {
      let project = state.gameState.projects[e.entity];
      return {
        tip: {
          icon: project.kind.toLowerCase(),
          text: `Unlocks ${project.name}.`,
          card: {
            type: 'Project',
            data: project,
          }
        },
        text: `<img src="${assets.icons.unlocks}"> ${project.name}`
      };
    }
    case 'UnlocksProcess': {
      let process = state.gameState.processes[e.entity];
      return {
        tip: {
          icon: process.output.toLowerCase(),
          text: `Unlocks the ${process.name} process.`,
          card: {
            type: 'Process',
            data: process,
          }
        },
        text: `<img src="${assets.icons.unlocks}"> ${process.name}`
      }
    }
    case 'ProjectCostModifier': {
      let project = state.gameState.projects[e.entity];
      let p = Math.abs(e.param) * 100;
      return `${e.param < 0 ? 'Reduces' : 'Increases'} cost of ${project.name} by ${p}%.`;
    }
    case 'ProjectRequest': {
      let project = state.gameState.projects[e.entity];
      if (e.subtype == 'Ban') {
        return `Request: Implement ${project.name} (+${e.param}PC)`
      } else {
        return `Request: Stop ${project.name} (+${e.param}PC)`
      }
    }
    case 'ProcessRequest': {
      let process = state.gameState.processes[e.entity];
      if (e.subtype == 'Ban') {
        return `Request: Ban ${process.name} (+${e.param}PC)`
      } else {
        return `Request: Unban ${process.name} (+${e.param}PC)`
      }
    }
    case 'AddFlag': {
      let flag = e.param.split('::')[1];
      return FLAGS[flag];
    }
    case 'ModifyIndustryDemand': {
      let industry = state.gameState.industries[e.entity].name;
      let p = Math.abs(e.param) * 100;
      return `${e.param < 0 ? 'Reduces' : 'Increases'} demand for ${industry} by ${p.toFixed(0)}%.`
    }
    case 'ModifyIndustryResources': {
      let industry = state.gameState.industries[e.entity].name;
      let p = Math.abs(1 - e.param) * 100;
      return `${e.param < 1 ? 'Reduces' : 'Increases'} ${e.subtype.toLowerCase()} demand for ${industry} by ${p.toFixed(0)}%.`
    }
    case 'ModifyIndustryByproducts': {
      let industry = state.gameState.industries[e.entity].name;
      let p = Math.abs(1 - e.param) * 100;
      return `${e.param < 1 ? 'Reduces' : 'Increases'} ${e.subtype} emissions for ${industry} by ${p.toFixed(0)}%.`
    }
    case 'DemandOutlookChange': {
      let k = consts.outputs.keys[e.subtype];
      let outlookChange = Math.floor(state.gameState.output_demand[k] * e.param);
      return `${sign(outlookChange)} <img src="/assets/icons/contentedness.png"> globally`;
    }
    case 'IncomeOutlookChange': {
      // TODO
      /* let outlookChange = Math.floor(game.total_income_level() * e.param); */
      let outlookChange = 0;
      return `${sign(outlookChange)} <img src="/assets/icons/contentedness.png"> globally`;
    }
    case 'ModifyEventProbability': {
      let event = EVENTS[e.entity].name;
      let p = Math.abs(e.param) * 100;
      return `${e.param < 1 ? 'Reduces' : 'Increases'} chance of ${event} by ${p.toFixed(0)}%.`
    }
    case 'ProtectLand': {
      return `Place ${e.param}% of land under protection.`;
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
            return {
              tip: desc.tip,
              text: `${ev.random ? '🎲 ' : ''}${desc.text}`,
            }
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
.effects {
  text-align: center;
}
.effect {
  border: 1px solid;
  display: inline-block;
  padding: 0.1em 0.25em;
  border-radius: 0.2em;
  margin: 0 0.1em;
}
.effect img {
  width: 19px;
  vertical-align: middle;
}
</style>
