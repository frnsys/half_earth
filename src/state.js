import {reactive} from 'vue';

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
    endYear: 0,
    gameState: null,
    phase: 'PLANNING',

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
  };

  let settings = loadSettings();
  state.help = settings.help || {};
  state.hideHelp = settings.hideHelp !== undefined ? settings.hideHelp : false;
  state.sound = settings.sound !== undefined ? settings.sound : false;
  return state;
}

const init = initState();
const state = reactive(init);

export {initState, saveSettings};
export default state;
