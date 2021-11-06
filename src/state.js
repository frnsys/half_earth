import {reactive} from 'vue';

const init = {
  gameState: null,
  phase: 'PLANNING',

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

  projects: [],
  processes: [],
}

const state = reactive(init);
fetch('/assets/content/projects.json')
  .then((resp) => resp.json())
  .then((json) => state.projects = json);
fetch('/assets/content/processes.json')
  .then((resp) => resp.json())
  .then((json) => state.processes = json);

export default state;
