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

  points: {
    research: 10,
    initiative: 10,
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
