import {reactive} from 'vue';

const init = {
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

  sound: false,
  tips: true,
}

const state = reactive(init);

export default state;
