import {reactive} from 'vue';

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
    project: null,
    props: {
      fertility: 3,
      biodiversity: 4,
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
    name: 'A Global Policy',
    unlocked: true,
    popularity: -1,
    global: true,
    construction: {
      years: 2,
      resources: {
        energy: 7
      }
    },
    operation: {
      resources: {
        energy: -1
      }
    },
    destruction: {
      years: 1,
      resources: {
        energy: 2
      }
    }
  }, {
    name: 'A Spatial Project',
    unlocked: true,
    popularity: 1,
    global: false,
    construction: {
      years: 2,
      resources: {
        energy: 7
      }
    },
    operation: {
      resources: {
        energy: -1
      }
    },
    destruction: {
      years: 1,
      resources: {
        energy: 2
      }
    }
  }],

  research: [{
    name: 'Nuclear Fusion',
    desc: 'TK',
  }]
}

export default reactive(init);
