import {reactive} from 'vue';

const init = {
  gameState: null,
  phase: 'PLANNING',
  points: {
    Research: {
      total: 10,
      available: 10,
    },
    Initiative: {
      total: 10,
      available: 10,
    },
    Policy: {
      total: 10,
      available: 10,
    },
  },

  plan: {
    targets: {
      biodiversity: {
        value: 5,
        wager: 0,
        valence: 1,
      },
      carbon: {
        value: 5,
        wager: 0,
        valence: -1
      }
    },
  },
  world: {
    biodiversity: {
      value: 5,
      baseChange: -1,
      history: [],
      preindustrial: 10,
      labels: {
        max: 'Eden Earth',
        min: 'Dead Planet'
      }
    },
    carbon: {
      value: 5,
      baseChange: 1,
      history: [],
      preindustrial: 0,
      labels: {
        max: 'Stability',
        min: 'Hot House'
      }
    },
    contentedness: {
      value: 5,
      baseChange: 0,
      history: []
    },
    temperature: {
      value: 1,
      baseChange: 0.025,
      history: [],
      preindustrial: 0
    },
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
