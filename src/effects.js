import {sign} from 'lib/util';
import state from '/src/state';
import iconEvents from '/assets/content/icon_events.json';


const outputNames = {
  'Fuel': 'fuel',
  'Electricity': 'electricity',
  'PlantCalories': 'plant-based food',
  'AnimalCalories': 'animal-based food',
};

function describeEffect(e) {
  switch (e.type) {
    case 'WorldVariable': {
      switch (e.subtype) {
        case 'Outlook': {
          return `${sign(e.param)} <img src="/assets/icons/contentedness.png"> globally`;
        }
        case 'Emissions': {
          return `${sign(e.param)} <img src="/assets/icons/emissions.png">`;
        }
        case 'ExtinctionRate': {
          return `${sign(e.param)} <img src="/assets/icons/extinction.png">`;
        }
      }
    }
    case 'Output': {
      return `${sign(e.param*100)}% ${outputNames[e.subtype]} production`;
    }
    case 'Demand': {
      return `${sign(e.param*100)}% ${outputNames[e.subtype]} demand`;
    }
    case 'AddEvent': {
      let id = e.entity.toString()
      if (id in iconEvents) {
        let event = iconEvents[id];
        let desc = `${event.name} will appear as <img src="/assets/icons/pips/${event.icon}.png"> on the globe.`;
        if (!state.clickExplained) {
          state.clickExplained = true;
          return `${desc}<br />Tap on these to send aid and gain <img src="/assets/icons/pips/political_capital.png">political capital.`;
        } else {
          return desc;
        }
      } else {
        // Other event triggers are hidden...surprises!
        return
      }
    }
    case 'UnlocksProject': {
      let project = state.gameState.projects[e.entity];
      return `${project.name} is now available in Gosplant.`;
    }
    case 'UnlocksProcess': {
      let process = state.gameState.processes[e.entity];
      return `${process.name} is now avaliable in Gosplant.`;
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
    default:
      console.log(`Unhandled event type: ${e.type}`);
      console.log(e);
      break;
  }
}

export {describeEffect};
