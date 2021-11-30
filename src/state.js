import {reactive} from 'vue';

const init = {
  endYear: 0,
  gameState: null,
  phase: 'PLANNING',
  crisis: {
    points: 0,
    max: 100
  },

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

  sound: true,
}

const state = reactive(init);

export default state;
