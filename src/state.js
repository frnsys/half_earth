import {reactive} from 'vue';
import consts from './consts';
import events from './content/events';
import projects from './content/projects';

function randChoice(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}

function randEnum(en) {
  return randChoice(Object.values(en));
}

const init = {
  phase: 'GLOBE',
  time: {
    start: 2025,
    end: 2100
  },
  player: {
    year: 2025,
    projects: [],
    political_capital: 10,
    resources: {
      energy: {
        value: 10,
        baseChange: 2
      },
      labor: {
        value: 10,
        baseChange: 1
      }
      // TODO
    }
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

  projects, events
}

export default reactive(init);
