<template>
  <div class="effects">
    <div v-for="{tip, icon, subicon, supicon, text} in renders" class="effect" v-tip="tip ? tip : 'missing tip'">
      <div class="effect--icon">
        <img :src="assets.icons[icon]" />
        <img :src="assets.icons[subicon]" v-if="subicon" class="effect--subicon" />
        <img :src="assets.icons[supicon]" v-if="subicon" class="effect--supicon" />
      </div>
      <div class="effect--text">{{text}}</div>
    </div>
  </div>
</template>

<script>
import {sign, slugify} from 'lib/util';
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
            icon: 'contentedness',
            text: sign(e.param * state.gameState.world.regions.length),
          }
        }
        case 'Emissions': {
          return {
            tip: {
              icon: 'emissions',
              text: `Changes emissions by ${sign(e.param)}GtCO2eq.`
            },
            icon: 'emissions',
            text: sign(e.param),
          }
        }
        case 'ExtinctionRate': {
          return {
            tip: {
              icon: 'extinction',
              text: `Changes the extinction rate by ${sign(e.param)}.`,
            },
            icon: 'extinction',
            text: sign(e.param),
          }
        }
        case 'Temperature': {
          return {
            tip: {
              icon: 'warming',
              text: `${sign(e.param)}C to the global temperature.`
            },
            icon: 'warming',
            text: `${sign(e.param)}C`,
          };
        }
        case 'PopulationGrowth': {
          return {
            tip: {
              icon: 'population',
              text: `Changes global population growth by ${sign(e.param)}%.`,
            },
            icon: 'population',
            text: `${sign(e.param)}%`,
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
            icon: 'contentedness',
            text: sign(e.param)
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
        icon: e.subtype.toLowerCase(),
        text: `${sign(e.param*100)}%`,
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
        icon: 'TODO PROCESS ICON',
        subicon: 'TODO PROCESS ICON',
        text: `${sign(e.param*100)}%`
      }
    }
    case 'OutputForFeature': {
      // TODO process feature icons
      return {
        tip: {
          icon: 'TODO PROCESS ICON',
          text: 'TODO PROCESS ICON',
        },
        icon: 'TODO PROCESS ICON',
        subicon: 'TODO PROCESS ICON',
        text: `${sign(e.param*100)}% to ${e.subtype} output.`
      }
    }
    case 'Demand': {
      return {
        tip: {
          icon: e.subtype.toLowerCase(),
          text: `Changes demand for ${consts.outputs.names[e.subtype]} by ${sign(e.param*100)}%. Current demand is TODO`,
        },
        icon: 'demand',
        subicon: e.subtype.toLowerCase(),
        text: `${sign(e.param*100)}%`,
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
        icon: 'demand',
        subicon: e.subtype.toLowerCase(),
        text: sign(val)
      }
    }
    case 'UnlocksProject': {
      let project = state.gameState.projects[e.entity];
      return {
        tip: {
          icon: 'unlocks',
          subicon: project.kind.toLowerCase(),
          text: e.random ? `Might unlock ${project.name}.` : `Unlocks ${project.name}.`,
          card: {
            type: 'Project',
            data: project,
          }
        },
        icon: 'unlocks',
        subicon: project.kind.toLowerCase(),
        text: project.name
      };
    }
    case 'UnlocksProcess': {
      let process = state.gameState.processes[e.entity];
      return {
        tip: {
          icon: 'unlocks',
          text: e.random ? `Might unlock the ${process.name} process.` : `Unlocks the ${process.name} process.`,
          card: {
            type: 'Process',
            data: process,
          }
        },
        icon: 'unlocks',
        text: process.name
      }
    }
    case 'ProjectCostModifier': {
      let project = state.gameState.projects[e.entity];
      let p = e.param * 100;
      return {
        tip: {
          icon: 'cost',
          text: `${e.param < 0 ? 'Reduces' : 'Increases'} cost of ${project.name} by ${Math.abs(p)}%.`,
          card: {
            type: 'Project',
            data: project,
          }
        },
        icon: 'cost',
        text: `${sign(p)}% ${project.name}`,
      }
    }
    case 'ProjectRequest': {
      // TODO display requester
      let project = state.gameState.projects[e.entity];
      if (e.subtype == 'Ban') {
        return {
          tip: {
            icon: 'ban',
            text: `I request that you stop ${project.name}. (+${e.param}PC)`,
            card: {
              type: 'Project',
              data: project,
            }
          },
          icon: 'request',
          subicon: 'ban',
          text: project.name
        }
      } else {
        return {
          tip: {
            icon: 'implement',
            text: `I request that you implement ${project.name}. (+${e.param}PC)`,
            card: {
              type: 'Project',
              data: project,
            }
          },
          icon: 'request',
          subicon: 'implement',
          text: project.name
        }
      }
    }
    case 'ProcessRequest': {
      // TODO replace with process icon
      let process = state.gameState.processes[e.entity];
      if (e.subtype == 'Ban') {
        return {
          tip: {
            icon: 'ban',
            text: `I request that you stop ${process.name}. (+${e.param}PC)`,
            card: {
              type: 'Process',
              data: process,
            }
          },
          icon: 'request',
          subicon: 'ban',
          text: process.name
        }
      } else {
        return {
          tip: {
            icon: 'implement',
            text: `I request that you implement ${process.name}. (+${e.param}PC)`,
            card: {
              type: 'Process',
              data: process,
            }
          },
          icon: 'request',
          subicon: 'implement',
          text: process.name
        }
      }
    }
    case 'AddFlag': {
      let flag = e.param.split('::')[1];
      return {
        tip: {
          icon: 'warming', // TODO TEMP
          text: FLAGS[flag],
        },
        icon: 'warming', // TODO TEMP
        text: FLAGS[flag],
      }
    }
    case 'ModifyIndustryDemand': {
      let industry = state.gameState.industries[e.entity].name;
      let p = e.param * 100;
      // TODO attach industry card
      return {
        tip: {
          icon: 'demand',
          subicon: slugify(industry),
          text: `${e.param < 0 ? 'Reduces' : 'Increases'} demand for ${industry} by ${Math.abs(p).toFixed(0)}%.`,
        },
        icon: 'demand',
        subicon: slugify(industry),
        text: `${sign(p.toFixed(0))}%`,
      }
    }
    case 'ModifyIndustryResources': {
      let industry = state.gameState.industries[e.entity].name;
      let p = Math.abs(1 - e.param) * 100;
      // TODO attach industry card
      return {
        tip: {
          icon: slugify(industry),
          subicon: e.subtype.toLowerCase(),
          text: `${e.param < 1 ? 'Reduces' : 'Increases'} ${e.subtype.toLowerCase()} demand for ${industry} by ${p.toFixed(0)}%.`,
        },
        icon: slugify(industry),
        subicon: e.subtype.toLowerCase(),
        text: `${sign(p.toFixed(0))}%`,
      }
    }
    case 'ModifyIndustryByproducts': {
      let industry = state.gameState.industries[e.entity].name;
      let p = (1 - e.param) * 100;
      // TODO attach industry card
      return {
        tip: {
          subicon: 'emissions',
          icon: slugify(industry),
          text: `${e.param < 1 ? 'Reduces' : 'Increases'} ${e.subtype} emissions for ${industry} by ${Math.abs(p).toFixed(0)}%.`,
        },
        icon: 'demand',
        subicon: 'emissions',
        icon: slugify(industry),
        text: `${sign(p.toFixed(0))}%`,
      }
    }
    case 'DemandOutlookChange': {
      let k = consts.outputs.keys[e.subtype];
      console
      let outlookChange = Math.floor(state.gameState.output_demand[k] * e.param);
      return {
        tip: {
          icon: 'contentedness',
          subicon: e.subtype.toLowerCase(),
          text: `Changes contentedness based on demand for ${consts.outputs.names[e.subtype]}.`,
        },
        icon: 'contentedness',
        subicon: e.subtype.toLowerCase(),
        text: sign(outlookChange)
      }
    }
    case 'IncomeOutlookChange': {
      // TODO
      /* let outlookChange = Math.floor(game.total_income_level() * e.param); */
      let outlookChange = 0;
      return {
        tip: {
          icon: 'contentedness',
          subicon: 'wealth',
          text: `Changes contentedness based on region income levels.`,
        },
        icon: 'contentedness',
        subicon: 'wealth',
        text: sign(outlookChange),
      }
    }
    case 'ModifyEventProbability': {
      let event = EVENTS[e.entity].name;
      let p = e.param * 100;
      return {
        tip: {
          icon: 'chance',
          text: `${e.param < 1 ? 'Reduces' : 'Increases'} chance of ${event} by ${Math.abs(p).toFixed(0)}%.`,
        },
        icon: 'chance',
        text: `${sign(p)}% ${event}`, // TODO
      }
    }
    case 'ProtectLand': {
      return {
        tip: {
          icon: 'land',
          subicon: 'protect',
          text: `Place ${e.param}% of land under protection.`,
        },
        icon: 'land',
        subicon: 'protect',
        text: `${e.param}%`,
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
              desc.tip.subicon = 'chance';
              desc.subicon = 'chance';
            }
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
.effects {
  text-align: center;
}
.effect {
  display: inline-block;
  margin: 0 0.1em;
}
.effect--icon {
  width: 32px;
  position: relative;
}
.effect--subicon {
  position: absolute;
  width: 16px;
  right: -4px;
  bottom: -4px;
}
.effect--supicon {
  position: absolute;
  width: 16px;
  right: -4px;
  top: -4px;
}
</style>
