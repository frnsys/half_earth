<template>
<div class="effects">
  <div v-for="{tip, text} in renders" class="effect" v-tip="tip ? tip : 'missing tip'">
    <div class="effect--text" v-html="text" />
  </div>
</div>
</template>

<script>
import t from '/src/i18n';
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
import * as Sentry from "@sentry/vue";

const FLAG_TIPS = {
  'Electrified': (demand) => {
    let changedDemand = parseInt((demand.fuel * 0.8).toFixed(0));
    return {
      icon: 'electricity',
      text: t('Fuel demand will change from <img src="{iconFuel}">{prevDemandFuel} to <img src="{iconFuel}">{nextDemandFuel} and electricity demand will change from <img src="{iconElec}">{prevDemandElec} to <img src="{iconElec}">{nextDemandElec}.', {
        iconFuel: icons.fuel,
        iconElec: icons.electricity,
        prevDemandFuel: demand.fuel,
        nextDemandFuel: demand.fuel - changedDemand,
        prevDemandElec: demand.electricity,
        nextDemandElec: demand.electricity + changedDemand,
      })
    };
  },
  'Vegan': (demand) => {
    let changedDemand = parseInt((demand.animal_calories * 0.9).toFixed(0));
    return {
      icon: 'plant_calories',
      text: t(`Animal calorie demand will change from <img src="{iconACals}">{prevDemandACals} to <img src="{iconACals}">{nextDemandACals} and plant calorie demand will change from <img src="{iconPCals}">{prevDemandPCals} to <img src="{iconPCals}">{nextDemandPCals}.`, {
        iconACals: icons.animal_calories,
        iconPCals: icons.plant_calories,
        prevDemandACals: demand.animal_calories,
        nextDemandACals: demand.animal_calories - changedDemand,
        prevDemandPCals: demand.pnimal_calories,
        nextDemandPCals: demand.animal_calories + changedDemand,
      })
    }
  },
  'Vegetarian': (demand) => {
    let changedDemand = parseInt((demand.animal_calories * 0.75).toFixed(0));
    return {
      icon: 'plant_calories',
      text: t(`Animal calorie demand will change from <img src="{iconACals}">{prevDemandACals} to <img src="{iconACals}">{nextDemandACals} and plant calorie demand will change from <img src="{iconPCals}">{prevDemandPCals} to <img src="{iconPCals}">{nextDemandPCals}.`, {
        iconACals: icons.animal_calories,
        iconPCals: icons.plant_calories,
        prevDemandACals: demand.animal_calories,
        nextDemandACals: demand.animal_calories - changedDemand,
        prevDemandPCals: demand.pnimal_calories,
        nextDemandPCals: demand.animal_calories + changedDemand,
      })
    }
  },
  'ClosedBorders': (demand) => {
    return {
      icon: 'closed_borders',
      text: t('Migrations will have less of an impact when they occur. But there might be other consequences.')
    }
  },
  'HyperResearch': (demand) => {
    return {
      icon: 'research',
      text: t(`Research points are 1<img src="{iconPC}"> cheaper.`, {iconPC: icons.political_capital})
    }
  },
  'StopDevelopment': (demand) => {
    return {
      icon: 'ban',
      text: t('Stops regional development throughout the world.')
    }
  },
  'FastDevelopment': (demand) => {
    return {
      icon: 'development',
      text: t('Accelerates regional development throughout the world.')
    }
  },
  'Degrowth': (demand) => {
    return {
      icon: 'degrowth',
      text: t('Contract the economies of the wealthiest regions.')
    }
  },
  'DeepSeaMining': (demand) => {
    return {
      icon: 'ocean',
      text: t('Prevents or stops metal shortages.')
    }
  },
  'ParliamentSuspended': (demand) => {
    return {
      icon: 'The Authoritarian',
      text: t('A parliamentary majority is no longer required for any project.'),
    }
  },
  'MoreLabor': (demand) => {
    return {
      icon: 'labor',
      text: t('Research and infrastructure take 10% less time to complete.'),
    }
  },
  'MoreLeisure': (demand) => {
    return {
      icon: 'labor',
      text: t('Research and infrastructure take 10% more time to complete.'),
    }
  },
  'MoreAutomation': (demand) => {
    return {
      icon: 'labor',
      text: t('Research and infrastructure take 10% less time to complete.'),
    }
  },
  'EcosystemModeling': (demand) => {
    return {
      icon: 'birb',
      text: t('Restoration projects take 10% less time to complete.'),
    }
  },
  'LaborResistance': (demand) => {
    return {
      icon: 'labor',
      text: t('Research and infrastructure take 5% more time to complete.'),
    }
  },
  'LaborSabotage': (demand) => {
    return {
      icon: 'labor',
      text: t('Research and infrastructure take 5% more time to complete.'),
    }
  },
};

function changeDir(change, ev) {
  if (change == '?') {
    return 'Changes';
  } else if (ev.random) {
    return t(`${change < 0 ? `${ev.probability} reduce` : `${ev.probability} increase`}`);
  } else {
    return `${change < 0 ? `<strong>${t('Reduces')}</strong>` : `<strong>${t('Increases')}</strong>`}`;
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
  Sentry.setContext("effect", {
    effect: e
  });

  let demand = format.outputs(state.gameState.output_demand);
  switch (e.type) {
    case 'WorldVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return {
            tip: {
              icon: 'contentedness',
              text: t(`Current world contentedness is {contentedness}<span class="type-total">/{maxContentedness}</span>.`, {
                contentedness: Math.round(state.gameState.world.contentedness),
                maxContentedness: consts.maxValues['contentedness']
              }),
            },
            text: `[contentedness] ${t('{changeDir} world contentedness by {amount}.', {
              changeDir: changeDir(e.param, e),
              amount: formatParam(e.param),
            })}`,
          }
        }
        case 'Emissions': {
          return {
            tip: {
              icon: 'emissions',
              text: t(`This will directly change annual emissions by {amount}.{percent}`, {
                amount: e.param == '?' ? t('an unknown amount') : format.sign(e.param),
                percent: e.param !== '?' ? ` ${t("That's a {percent}% change.", {percent: (e.param/state.gameState.world.emissions * 100).toFixed(1)})}` : ''
              })
            },
            text: `[emissions] ${t('{changeDir} emissions by {amount}.', {
              changeDir: changeDir(e.param, e),
              amount: formatParam(e.param)
            })}`
          }
        }
        case 'ExtinctionRate': {
          return {
            tip: {
              icon: 'extinction_rate',
              text: t(`Current biodiversity pressure is {amount}<span class="type-total">/{maxAmount}</span>.`, {
                amount: state.gameState.world.extinction_rate.toFixed(0),
                maxAmount: consts.maxValues['biodiversity']
              }),
            },
            text: `[extinction_rate] ${t('{changeDir} biodiversity pressure by {amount}.', {
              changeDir: changeDir(e.param, e),
              amount: formatParam(e.param)
            })}`
          }
        }
        case 'Temperature': {
          return {
            tip: {
              icon: 'warming',
              text: t(`This will directly change the global temperature anomaly by {amount}<strong>°c</strong>.`, {
                amount: format.sign(e.param)
              }),
            },
            text: `[warming] ${t('{changeDir} the global temperature by {amount}<strong>°c</strong>.', {
              changeDir: changeDir(e.param, e),
              amount: formatParam(e.param)
            })}`
          };
        }
        case 'Precipitation': {
          return {
            tip: {
              icon: 'water',
              text: t(`This will directly change global precipitation by {amount}<strong>cm/yr</strong>.`, {
                amount: format.sign(e.param)
              }),
            },
            text: `[water] ${t('{changeDir} global precipitation by {amount}<strong>cm/yr</strong>.', {
              changeDir: changeDir(e.param, e),
              amount: formatParam(e.param)
            })}`
          };
        }
        case 'PopulationGrowth': {
          return {
            tip: {
              icon: 'population',
              text: t('The number of people on the planet.'),
            },
            text: `[population] ${t('{changeDir} global population growth by {amount}<strong>%.</strong>', {
              changeDir: changeDir(e.param, e),
              amount: formatParam(e.param)
            })}`,
          };
        }
        case 'Population': {
          return {
            tip: {
              icon: 'population',
              text: t('The number of people on the planet.'),
            },
            text: `[population] ${t('{changeDir} global population by {amount}.', {
              changeDir: changeDir(e.param, e),
              amount: formatParam(e.param)
            })}`,
          };
        }
        case 'SeaLevelRiseRate': {
          return {
            tip: {
              icon: 'sea_level_rise',
              text: t(`The amount of sea level rise is currently {amount}m.`, {
                amount: state.gameState.world.sea_level_rise.toFixed(2)
              }),
            },
            text: `[sea_level_rise] ${t('{changeDir} the rate of sea level rise by {amount}mm/year.', {
              changeDir: changeDir(e.param, e),
              amount: formatParam(e.param * 1000)
            })}`,
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
              text: t('Research points: Allocate them to research projects!')
            },
            text: `[research] ${t('{random}+{amount} research points.', {
              random: e.random ? 'Possible ' : '',
              amount: formatParam(e.param)
            })}`,
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
      let text = t(`{changeDir} maximum output for {process} by <strong>{amount}</strong>`, {
        amount: p,
        process: process.name,
        changeDir: changeDir(e.param, e),
      });
      return {
        tip: {
          icon: 'alert',
          text: text,
        },
        text: text,
      }
    }
    case 'RegionHabitability': {
      return {
        tip: {
          icon: 'habitability',
          text: t(`Lower habitability means unhappier people who may need to migrate to more hospitable locales.`)
        },
        text: `[habitability] ${t('{changeDir} habitability in {type} regions by {amount}.', {
          amount: formatParam(e.param),
          type: t(e.subtype.toLowerCase()),
          changeDir: changeDir(e.param, e),
        })}`,
      }
    }
    case 'Resource': {
      let k = display.enumKey(e.subtype);
      let amount = format.output(e.param, k);
      let percent = (e.param/state.gameState.resources[k] * 100).toFixed(1);
      let text = t(`{changeDir} {name} supply by <img src="{icon}">{amount} ({percent}% of current supply).`, {
          percent: format.sign(percent),
          amount: Math.abs(amount),
          icon: icons[k],
          name: display.enumDisplay(k),
          changeDir: changeDir(e.param, e),
      });
      return {
        tip: factors.tips[k](text),
        text: t(`[{icon}] {changeDir} {name} supply by [{icon}]{amount}.`, {
          amount: Math.abs(amount),
          name: display.enumDisplay(k),
          changeDir: changeDir(e.param, e),
          icon: k,
        }),
      }
    }
    case 'Output': {
      let k = display.enumKey(e.subtype);
      let base = format.output(state.gameState.produced[k], k);
      let changed = (base * (1+e.param)).toFixed(0);
      return {
        tip: {
          icon: k,
          text: t(`Global {name} output will change from <img src="{icon}">{base} to <img src="{icon}">{changed} with no change in impacts.`, {
            changed: changed,
            base: base,
            icon: icons[k],
            name: display.displayName(e.subtype),
          })
        },
        text: t(`[{icon}] {changeDir} all {name} production by <strong>{percent}%.</strong>`, {
          percent: (Math.abs(e.param)*100).toFixed(0),
          name: display.displayName(e.subtype),
          changeDir: changeDir(e.param, e),
          icon: e.subtype.toLowerCase(),
        }),
      }
    }
    case 'OutputForProcess': {
      let process = state.gameState.processes[e.entity];
      let tip = {
        icon: display.enumKey(process.output),
        text: t(`Changes the output for this process by {percent}% with no change in impacts.`, {
          percent: (e.param*100).toFixed(0),
        }),
        card: {
          type: 'Process',
          data: process,
        }
      };
      let tag = display.cardTag(process.name, process.output.toLowerCase());
      return {
        tip: tip,
        text: t(`[{icon}] {changeDir} {tag} output by <strong>{percent}%.</strong>`, {
          percent: (Math.abs(e.param)*100).toFixed(0),
          tag: tag,
          changeDir: changeDir(e.param, e),
          icon: process.output.toLowerCase(),
        })
      }
    }
    case 'OutputForFeature': {
      let tip = {
        icon: e.subtype,
        text: t(`Changes the output for these processes by {percent}% without changing their impacts.`, {
          percent: (e.param*100).toFixed(0),
        }),
        card: {
          type: 'Processes',
          data: state.gameState.processes.filter((p) => p.features.includes(e.subtype))
        }
      };
      return {
        tip,
        text: t(`{changeDir} output for <span><img class="effect-feature" src="{icon}" /><strong>{feature}</strong></span> by <strong>{percent}%.</strong>`, {
          percent: (e.param*100).toFixed(0),
          feature: display.describeFeature(e.subtype),
          icon: icons[e.subtype],
          changeDir: changeDir(e.param, e),
        })
      }
    }
    case 'CO2ForFeature': {
      let amount = e.param * 100;
      let label = Math.abs(amount) >= 1 ? amount.toFixed(0) : '<1';
      let tip = {
        icon: e.subtype,
        text: t(`{changeDir} the CO2 emissions for these processes by <strong>{percent}%.</strong>`, {
          percent: label,
          changeDir: changeDir(e.param, e),
        }),
        card: {
          type: 'Processes',
          data: state.gameState.processes.filter((p) => p.features.includes(e.subtype))
        }
      };
      return {
        tip,
        text: t(`{changeDir} CO2 emissions for <span><img class="effect-feature" src="{icon}" />{feature}</span> by <strong>{percent}%.</strong>`, {
          percent: label,
          feature: display.describeFeature(e.subtype),
          icon: icons[e.subtype],
          changeDir: changeDir(e.param, e),
        })
      }
    }
    case 'BiodiversityPressureForFeature': {
      let tip = {
        icon: e.subtype,
        text: t(`Changes the biodiversity pressure for these processes by <strong>{amount}.</strong>`, {
          amount: e.param,
        }),
        card: {
          type: 'Processes',
          data: state.gameState.processes.filter((p) => p.features.includes(e.subtype))
        }
      };
      return {
        tip,
        text: t(`{changeDir} biodiversity pressure for <span><img class="effect-feature" src="{icon}" />{feature}</span> by <strong>{amount}.</strong>`, {
          amount: e.param,
          feature: display.describeFeature(e.subtype),
          icon: icons[e.subtype],
          changeDir: changeDir(e.param, e),
        })
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
          text: t(`This changes {name} demand from <img src="{icon}">{currentDemand} to <img src="{icon}">{afterDemand}.`, {
            afterDemand: Math.round(afterDemand),
            currentDemand: currentDemand,
            icon: icons[k],
            name: name,
          })
        },
        text: t(`[{icon}] {changeDir} demand for {name} by <strong>{percent}%</strong>.`, {
          percent: (Math.abs(e.param)*100).toFixed(0),
          name: display.displayName(e.subtype),
          changeDir: changeDir(e.param, e),
          icon: e.subtype.toLowerCase(),
        }),
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
          text: t(`This changes {name} demand from <img src="{icon}">{currentDemand} to <img src="{icon}">{afterDemand}. This is a {percent}% change of all {name} demand.`, {
            percent: demandChange.toFixed(0),
            afterDemand: afterDemand,
            currentDemand: currentDemand,
            icon: icons[k],
            name: name,
          })
        },
        text: t(`[{type}] {changeDir} demand for {name} by <img src="{icon}">{amount}.`, {
          amount: Math.abs(val),
          icon: icons[k],
          name: display.displayName(e.subtype),
          changeDir: changeDir(e.param, e),
          type: e.subtype.toLowerCase(),
        }),
      }
    }
    case 'UnlocksProject': {
      let project = state.gameState.projects[e.entity];
      let tip = {
        icon: 'unlocks',
        text: t(`${e.random ? e.probability : 'Will'} unlock this project:`),
        card: {
          type: 'Project',
          data: project
        }
      };
      let tag = display.cardTag(project.name, project.kind.toLowerCase());
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] ${t(`${e.probability} unlock the {tag} project.`, {tag})}` : t('<strong>Unlocks</strong> the {tag} project.', {tag})}`,
      }
    }
    case 'UnlocksProcess': {
      let process = state.gameState.processes[e.entity];
      let tip = {
        icon: 'unlocks',
        text: t(`${e.random ? e.probability : 'Will'} unlock this process:`),
        card: {
          type: 'Process',
          data: process
        }
      };
      let tag = display.cardTag(process.name, display.enumKey(process.output));
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] ${t(`${e.probability} unlock the {tag} process.`, {tag})}` : t('<strong>Unlocks</strong> the {tag} process.', {tag})}`,
      }
    }
    case 'UnlocksNPC': {
      let npc = state.gameState.npcs[e.entity];
      let tip = {
        icon: 'unlocks',
        text: t('This new character will be unlocked:'),
        card: {
          type: 'NPC',
          data: npc
        }
      };
      return {
        tip: tip,
        text: `[unlocks] ${e.random ? `[chance] ${t(`${e.probability} unlock {name}.`, {name: npc.name})}` : t('<strong>Unlocks</strong> {name}.', {name: npc.name})}`,
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
      let kind = t('cost');
      if (project.kind == 'Research') {
        kind = t('research time');
      } else if (project.kind == 'Initiative') {
        kind = t('development time');
      }
      let amountName = e.param == '?' ? e.param : Math.ceil(Math.abs(amount));
      let tipAmount = e.param == '?' ? t('by an unknown amount') : t(`from {tipIcon}{cost}{unit} to {tipIcon}{newCost}{unit}.`, {
        newCost: project.cost + amount,
        unit: unit,
        cost: project.cost,
        tipIcon: tipIcon,
      });
      let tip = {
        icon: 'cost',
        text: t(`This effect {changeDir} the {kind} of this project {tipAmount}.`, {
          tipAmount: tipAmount,
          kind: kind,
          changeDir: changeDir(e.param, e).toLowerCase(),
        }),
        card: {
          type: 'Project',
          data: project,
        }
      };
      return {
        tip: tip,
        text: `[cost] ${e.random ? '[chance]' : ''}${t('{changeDir} {kind} of {tag} by {icon}{amount}{unit}.', {
          changeDir: changeDir(e.param, e),
          kind: kind,
          tag: tag,
          icon: icon,
          unit: unit,
          amount: formatParam(amountName)
        })}`,
      }
    }
    case 'ProjectRequest': {
      let project = state.gameState.projects[e.entity];
      if (e.subtype == 'Ban') {
        let tip = {
          icon: 'request',
          text: t(`You received a request to stop this project:`),
          card: {
            type: 'Project',
            data: project
          }
        };
        return {
          tip: tip,
          text: `[ban] ${t('I request that you stop {name}.', {name: project.name})} (+${e.param}PC)`,
        }
      } else {
        let tip = {
          icon: 'request',
          text: t(`You received a request to implement this project:`),
          card: {
            type: 'Project',
            data: project
          }
        };
        return {
          tip: tip,
          text: `[implement] ${t('I request that you implement {name}.', {name: project.name})} (+${e.param}PC)`,
        }
      }
    }
    case 'ProcessRequest': {
      let process = state.gameState.processes[e.entity];
      if (e.subtype == 'Ban') {
        let tip = {
          icon: 'request',
          text: t(`You received a request to ban this process:`),
          card: {
            type: 'Process',
            data: process
          }
        };
        return {
          tip: tip,
          text: `[ban] ${t('I request that you stop {name}.', {name: process.name})} (+${e.param}PC)`,
        }
      } else {
        let tip = {
          icon: 'request',
          text: t(`You received a request to promote this process:`),
          card: {
            type: 'Process',
            data: process
          }
        };
        return {
          tip: tip,
          text: `[implement] ${t('I request that you implement {name}.', {name: process.name})} (+${e.param}PC)`,
        }
      }
    }
    case 'AddFlag': {
      let flag = e.param.split('::')[1];
      return {
        tip: FLAG_TIPS[flag](demand),
        text: '<strong>' + t(FLAGS[flag]) + '</strong>',
      }
    }
    case 'ModifyIndustryDemand': {
      let industry = state.gameState.industries[e.entity];
      let p = Math.abs(e.param * 100);
      let tip = {
        icon: 'demand',
        text: e.param == '?' ?
          t(`Changes demand for {name} by an unknown amount.`, {name: industry.name})
          : t(`Changes demand for {name} by <strong>{percent}%.</strong>`, {
            percent: p.toFixed(0),
            name: industry.name,
          }),
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: t(`{changeDir} demand for {tag} by {amount}.`, {
          amount: e.param == '?' ? formatParam(e.param) : `<strong>${p.toFixed(0)}%</strong>`,
          tag: tag,
          changeDir: changeDir(e.param, e),
        }),
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
          t(`This will change {resource} demand for {name} by some unknown amount.`, {
            name: industry.name,
            resource: resource,
          })
          : t(`This will change {resource} demand for {name} from <img src="{icon}">{demandBefore} to <img src="{icon}">{demandAfter}. This is a {percent}% change of all {resource} demand.`, {
              percent: demandChange.toFixed(0),
              demandAfter: demandAfter < 1 ? '<1' : demandAfter.toFixed(0),
              demandBefore: demandBefore,
              icon: icons[k],
              name: industry.name,
              resource: resource,
          }),
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: t(`[{icon}] {changeDir} {resource} demand for {tag} by {amount}.`, {
          amount: e.param == '?' ? formatParam(e.param) : `<strong>${p.toFixed(0)}%</strong>`,
          tag: tag,
          resource: resource.toLowerCase(),
          changeDir: changeDir(e.param, e),
          icon: e.subtype.toLowerCase(),
        }),
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
          t(`This will change {resource} demand for {name} by some unknown amount.`, {
            name: industry.name,
            resource: resource,
          })
          : t(`This will change {resource} demand for {name} from <img src="{icon}">{demandBefore} to <img src="{icon}">{demandAfter}. This is a {percent}% change of all {resource} demand.`, {
            percent: demandChange.toFixed(0),
            demandAfter: demandAfter < 1 ? '<1' : demandAfter.toFixed(0),
            demandBefore: demandBefore,
            icon: icons[k],
            name: industry.name,
            resource: resource,
          }),
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: t(`[{icon}] {changeDir} {resource} demand for {tag} by {amount}.`, {
          amount: e.param == '?' ? formatParam(e.param) : `${Math.abs(demandAfter - demandBefore)}`,
          tag: tag,
          resource: resource.toLowerCase(),
          changeDir: changeDir(e.param, e),
          icon: e.subtype.toLowerCase(),
        }),
      }
    }

    case 'ModifyProcessByproducts': {
      let process = state.gameState.processes[e.entity];
      let p = Math.abs(e.param * 100);
      let emissionsBefore = format.co2eq(process.byproducts) * state.gameState.produced_by_process[e.entity] * 1e-15;
      let emissionsAfter = emissionsBefore * (1 + e.param);
      let emissionsChange = (emissionsAfter - emissionsBefore)/state.gameState.world.emissions * 100;
      let label = e.subtype == 'Biodiversity' ? t('biodiversity pressure') : t(`{type} emissions`, {type: e.subtype});
      let short = e.subtype == 'Biodiversity' ? t('biodiversity pressure') : t('emissions');
      let icon = e.subtype == 'Biodiversity' ? 'biodiversity' : 'emissions';
      let change = e.subtype == 'Biodiversity' ?
        t(`{fromAmount} to {toAmount}<img src="{icon}">.`, {
          icon: icons[icon],
          toAmount: e.param,
          fromAmount: process.byproducts.biodiversity,
        })
        : t(`{emissionsBefore} to <img src="{icon}">{emissionsAfter}. This is a {emissionsChange}% change of all emissions.`, {
          icon: icons.emissions,
          emissionsChange: emissionsChange > 0 && emissionsChange < 1 ? '<1' : emissionsChange.toFixed(1),
          emissionsAfter: emissionsAfter > 0 && emissionsAfter < 1 ? '<1' : emissionsAfter.toFixed(1),
          emissionsBefore: emissionsBefore > 0 && emissionsBefore < 1 ? '<1' : emissionsBefore.toFixed(1),
        })
      let tip = {
        icon: icon,
        text: e.param == '?' ?
          t(`Changes {label} for {name} by an unknown amount.`, {
            name: process.name,
            label: label,
          })
          : t(`This will change {short} for {name} from <img src="{icon}">{change}`, {
            change: change,
            icon: icons[icon],
            name: process.name,
            short: short,
          }),
        card: {
          type: 'Process',
          data: process,
        }
      };
      let tag = display.cardTag(process.name);
      return {
        tip: tip,
        text: t(`[{icon}] {changeDir} {label} for {tag} by <strong>{percent}</strong>.`, {
          percent: e.param == '?' ? formatParam(e.param) : `${p.toFixed(0)}%`,
          tag: tag,
          label: label,
          changeDir: changeDir(e.param, e),
          icon: icon,
        }),
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
          t(`Changes emissions for {name} by an unknown amount.`, {
            name: industry.name,
          })
          : t(`This will change emissions for {name} from <img src="{icon}">{emissionsBefore} to <img src="{icon}">{emissionsAfter}. This is a {emissionsChange}% change of all emissions.`, {
            name: industry.name,
            icon: icons.emissions,
            emissionsChange: emissionsChange > 0 && emissionsChange < 1 ? '<1' : emissionsChange.toFixed(1),
            emissionsAfter: emissionsAfter > 0 && emissionsAfter < 1 ? '<1' : emissionsAfter.toFixed(1),
            emissionsBefore: emissionsBefore > 0 && emissionsBefore < 1 ? '<1' : emissionsBefore.toFixed(1),
          }),
        card: {
          type: 'Industry',
          data: industry,
        }
      };
      let tag = display.cardTag(industry.name);
      return {
        tip: tip,
        text: t(`[emissions] {changeDir} {type} emissions for {tag} by <strong>{percent}</strong>.`, {
          percent: e.param == '?' ? formatParam(e.param) : `${p.toFixed(0)}%`,
          tag: tag,
          type: e.subtype,
          changeDir: changeDir(e.param, e),
        }),
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
          text: t(`This changes regional contentedness based on demand for {name}. Current world contentedeness is {amount}<span class="type-total">/{maxAmount}</span>.`, {
            name: display.displayName(e.subtype),
            maxAmount: consts.maxValues['contentedness'],
            amount: Math.round(state.gameState.world.contentedness),
          }),
        },
        text: t(`[contentedness] [{icon}] {changeDir} world contentedness by <strong>{amount}</strong>.`, {
          amount: Math.abs(change),
          changeDir: changeDir(e.param, e),
          icon: e.subtype.toLowerCase(),
        })
      }
    }
    case 'IncomeOutlookChange': {
      let change = effects.incomeOutlookChange(state.gameState.world, e.param);
      change = Math.round(change);
      return {
        tip: {
          icon: 'contentedness',
          subicon: 'wealth',
          text: t(`This changes regional contentedness by {amount} per income level (wealthier regions will feel it more). Current world contentedeness is {contentedness}<span class="type-total">/{maxContentedness}</span>.`, {
            maxContentedness: consts.maxValues['contentedness'],
            contentedness: Math.round(state.gameState.world.contentedness),
            amount: e.param,
          }),
        },
        text: t(`[contentedness] {changeDir} contentedness by <strong>{amount}</strong>.`, {
          amount: Math.abs(change),
          changeDir: changeDir(e.param, e),
        })
      }
    }
    case 'ModifyEventProbability': {
      let event = EVENTS[e.entity].name;
      let p = e.param == '?' ? '?' : e.param * 100;
      let text = t(`{changeDir} the chance of "{event}" by {percent}%`, {
        event: t(event),
        percent: formatParam(p),
        changeDir: changeDir(p, e),
      });
      return {
        tip: {
          icon: 'chance',
          text: text,
        },
        text: text
      }
    }
    case 'ProtectLand': {
      return {
        tip: {
          icon: 'land',
          text: t('This will limit the amount of land that processes can use.')
        },
        text: t(`[land] Place <strong>{percent}%</strong> of land under protection.`, {
          percent: e.param,
        }),
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
        text = t("We aren't tracking this feedstock.");
      } else if (estimate == 0) {
        text = t('This feedstock has been depleted.');
      } else if (isFinite(estimate)) {
        text = t(`At current usage rates the estimated supply is expected to last {estimate} years.`, {
          estimate: estimate,
        });
      } else {
        text = t(`At current usage rates the estimated supply is expected to last indefinitely.`);
      }
      return {
        tip: {
          icon: k,
          text,
        },
        text: t(`[{icon}] {changeDir} {name} supply by <strong>{percent}%.</strong>`, {
          icon: k,
          name: name,
          percent: e.param*100,
          changeDir: changeDir(e.param, e),
        }),
      }
    }
    case 'LocksProject': {
      let project = state.gameState.projects[e.entity];
      let tag = display.cardTag(project.name, project.kind.toLowerCase());
      return {
        tip: {
          icon: 'alert',
          text: t(`{name} will be unavailable while this project is active.`, {
            name: project.name,
          }),
          card: {
            type: 'Project',
            data: project
          }
        },
        text: t(`[locks] <strong>Locks</strong> {tag}`, {
          tag: tag,
        }),
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
          text: t(`This will directly change the global temperature anomaly by {amount}<strong>°c</strong>.`, {
            amount: format.sign(temp_change),
          }),
        },
        text: t(`[warming] {changeDir} the global temperature by {amount}<strong>°c</strong>.`, {
          amount: formatParam(temp_change),
          changeDir: changeDir(temp_change, e),
        })
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
