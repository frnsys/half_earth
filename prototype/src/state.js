import {reactive} from 'vue';
import consts from './consts';

function randChoice(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}

function randEnum(en) {
  return randChoice(Object.values(en));
}

// TODO initial game state
const init = {
  consts: {
    MAX_HAND_SIZE: 8
  },
  phase: 'PLANNING',
  player: {
    year: 2025,
    hand: [],
    research: [],
    projects: [],
    political_capital: 10,
    resources: {
      energy: {
        value: 10,
        change: 0,
        baseChange: 2
      }
      // TODO
    }
  },
  region: [...Array(16).keys()].map(() => ({
    type: randEnum(consts.PLOT_TYPE),
    connected: true,
    project: null,
    props: {
      fertility: 3,
      biodiversity: 4,
      temperature: 25,
      population: 10,
    },
  })),
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
    contentedness: 5
  },
  world: {
    biodiversity: {
      value: 5,
      change: 0,
      baseChange: -1
    },
    emissions: {
      value: 5,
      change: 0,
      baseChange: 1
    },
    contentedness: {
      value: 5,
      change: 0,
      baseChange: 0
    }
  },

  projects: [{
    name: 'Mandated Veganism',
    unlocked: true,
    popularity: -1,
    global: true,
    construction: {
      years: 1,
      resources: {},
      impacts: {
        contentedness: -3
      }
    },
    operation: {
      resources: {},
      impacts: {
        emissions: -2
      }
    },
    destruction: {
      years: 1,
      resources: {
        contentedness: 2,
        emissions: 3
      },
      impacts: {}
    }
  }, {
    name: 'A Spatial Project',
    unlocked: true,
    popularity: 1,
    global: false,
    toxic: true,
    construction: {
      years: 2,
      resources: {
        energy: 7
      },
      impacts: {}
    },
    operation: {
      resources: {
        energy: -1
      },
      impacts: {}
    },
    destruction: {
      years: 1,
      resources: {
        energy: 2
      },
      impacts: {}
    }
  }],

  research: [{
    name: 'Nuclear Fusion',
    estimate: null
  }, {
    name: 'New SRM Method',
    estimate: 5,
  }, {
    name: 'New Battery Technology',
    estimate: 5,
  }, {
    name: 'Low-Carbon Concrete Production',
    estimate: 5
  }, {
    name: 'Perennial Cereals',
    estimate: 10
  }]
}

export default reactive(init);
