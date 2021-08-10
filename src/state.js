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
  phase: 'PLANNING',
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
      emissions: {
        value: 5,
        wager: 0,
        valence: -1
      }
    },
  },
  world: {
    biodiversity: {
      value: 5,
      baseChange: -1
    },
    emissions: {
      value: 5,
      baseChange: 1
    },
    contentedness: {
      value: 5,
      baseChange: 0
    },
    temperature: {
      value: 5,
      baseChange: 1
    },
  },

  projects, events
}

export default reactive(init);
