import {reactive} from 'vue';

const init = {
  gameState: null,
  phase: 'PLANNING',
  planChanges: [],

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
}

const state = reactive(init);

export default state;
