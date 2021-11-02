import {sign} from 'lib/util';
import game from '/src/game';
import state from '/src/state';
import FLAGS from '/assets/content/flags.json';
import iconEvents from '/assets/content/icon_events.json';
import events from '/assets/content/events.json';


const outputNames = {
  'Fuel': 'fuel',
  'Electricity': 'electricity',
  'PlantCalories': 'plant-based food',
  'AnimalCalories': 'animal-based food',
};
const outputKeys = {
  'Fuel': 'fuel',
  'Electricity': 'electricity',
  'PlantCalories': 'plant_calories',
  'AnimalCalories': 'animal_calories',
};

function describeEffect(e) {
  switch (e.type) {
    case 'WorldVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return `${sign(e.param)} <img src="/assets/icons/contentedness.png"> globally`;
        }
        case 'Emissions': {
          return `${sign(e.param)} <img src="/assets/icons/emissions.png"> emissions`;
        }
        case 'ExtinctionRate': {
          return `${sign(e.param)} <img src="/assets/icons/extinction.png"> extinction rate`;
        }
        case 'Temperature': {
          return `${sign(e.param)}C temperature`;
        }
        case 'PopulationGrowth': {
          return `${sign(e.param)}% population growth`;
        }
      }
      return;
    }
    case 'LocalVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return `${sign(e.param)} <img src="/assets/icons/contentedness.png"> locally`;
        }
      }
      return;
    }
    case 'Output': {
      return `${sign(e.param*100)}% ${outputNames[e.subtype]} production`;
    }
    case 'OutputForProcess': {
      let process = state.gameState.processes[e.entity];
      return `${sign(e.param*100)}% to ${process.name} output.`;
    }
    case 'OutputForFeature': {
      return `${sign(e.param*100)}% to ${e.subtype} output.`;
    }
    case 'Demand': {
      return `${sign(e.param*100)}% ${outputNames[e.subtype]} demand`;
    }
    case 'DemandAmount': {
      let val = e.param;
      let unit = '';
      if (e.subtype == 'Electricity' || e.subtype == 'Fuel') {
        val *= 1e-9;
        unit = 'TWh';
      } else {
        val *= 1e-9;
        unit = 'Tcals';
      }
      return `${sign(val)} ${unit} to ${outputNames[e.subtype]} demand`;
    }
    case 'AddEvent': {
      let id = e.entity.toString()
      if (id in iconEvents) {
        let event = iconEvents[id];
        let desc = `${event.name} will appear as <img src="/assets/icons/pips/${event.icon}.png"> on the globe.`;
        return `${desc}<br />Tap on these to send aid and gain <img src="/assets/icons/pips/political_capital.png"> political capital.`;
      } else {
        // Other event triggers are hidden...surprises!
        return
      }
    }
    case 'UnlocksProject': {
      let project = state.gameState.projects[e.entity];
      return `Unlocks ${project.name}.`;
    }
    case 'UnlocksProcess': {
      let process = state.gameState.processes[e.entity];
      return `Unlocks the ${process.name} process.`;
    }
    case 'ProjectCostModifier': {
      let project = state.gameState.projects[e.entity];
      let p = Math.abs(e.param) * 100;
      return `${e.param < 0 ? 'Reduces' : 'Icnreases'} cost of ${project.name} by ${p}%.`;
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
    case 'AutoClick': {
      let id = e.entity.toString()
      let event = iconEvents[id];
      return `${e.param}% chance to auto-respond to <img src="/assets/icons/pips/${event.icon}.png">${event.name}.`
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
      let k = outputKeys[e.subtype];
      let outlookChange = Math.round(state.gameState.output_demand[k] * e.param);
      return `${sign(outlookChange)} <img src="/assets/icons/contentedness.png"> globally`;
    }
    case 'IncomeOutlookChange': {
      let outlookChange = Math.round(game.totalIncomeLevel() * e.param);
      return `${sign(outlookChange)} <img src="/assets/icons/contentedness.png"> globally`;
    }
    case 'ModifyEventProbability': {
      let event = events[e.entity].name;
      let p = Math.abs(e.param) * 100;
      return `${e.param < 1 ? 'Reduces' : 'Increases'} chance of ${event} by ${p.toFixed(0)}%.`
    }
    default:
      console.log(`Unhandled effect type: ${e.type}`);
      console.log(e);
      break;
  }
}

export {describeEffect};

