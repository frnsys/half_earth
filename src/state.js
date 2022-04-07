import {reactive} from 'vue';
import debug from './debug';
import tutorial from '/src/tutorial';

function saveSettings() {
  let data = {
    help: state.help,
    hideHelp: state.hideHelp,
    sound: state.sound,
  };
  localStorage.setItem('gameSettings', JSON.stringify(data));
}

function loadSettings() {
  let data = localStorage.getItem('gameSettings');
  if (data !== null) {
    return JSON.parse(data);
  } else {
    return {};
  }
}

function initState() {
  let state = {
    gameState: null,
    phase: 'INTRO',
    newRunCount: 0,
    tutorial: debug.hideIntro ? tutorial.READY + 1 : 0,

    // Track which events have occurred
    events: [],

    annualRegionEvents: {},

    // Track planned process mix changes
    processMixChanges: {
      Electricity: {},
      Fuel: {},
      PlantCalories: {},
      AnimalCalories: {},
    },

    // Track changes made to the plan
    // in a given session, so they can
    // be reversed/refunded
    planChanges: {},
    queuedUpgrades: {},

    // Compare beginning and end
    cycleStartState: {
      emissions: 0,
      extinctionRate: 0,
      contentedness: 0
    },
    history: {
      emissions: [],
      land_use: [],
    },

    points: {
      research: 0,
      initiative: 0,
    },
    refundableResearchPoints: 0,

    // Kind of hacky
    extraSeats: {}
  };

  let settings = loadSettings();
  state.help = settings.help || {};
  state.hideHelp = settings.hideHelp !== undefined ? settings.hideHelp : false;
  state.sound = settings.sound !== undefined ? settings.sound : false;
  return state;
}

const init = initState();
const state = reactive(init);

export {
  initState, saveSettings,
}
export default state;
