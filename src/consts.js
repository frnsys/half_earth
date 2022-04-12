import debug from './debug';

export default {
  cardScale: window.innerHeight < 600 ? 0.9 : 1.0,

  // Have to all be below these values to win
  winState: {
    emissions: 0,
    extinction: 10,
    temperature: 1,
  },

  msPerYear: debug.fastYears ? 2000 : 4000,

// Set an upper cap to the amount of emissions we pass to hector,
// because very large numbers end up breaking it.
  maxEmissions: 200, // GtCO2eq

  parliamentSeats: [2,3,4,5,5,5,4,3,2],
  maxRelationship: 6,
  processPointsPerCycle: 3,

  // How much PC is earned when completing a project
  pcPerCompletedProject: 5,

  // PC earned per intensity level of contentedness and extinction
  contentednessPc: [0, 0, 5, 10, 20, 30],
  extinctionPc: [20, 10, 0, -5, -5, -10],

  // Bonus PC for the first n years
  honeymoonPc: 15,
  honeymoonYears: 5,

  // PC change per -0.1C temp change
  temperaturePc: 2,
  // PC change per -0.5Gt emissions change
  emissionsPc: 5,

  // PC cost for research/infrastructure points
  pointCost: 3,

  // Max points for a project
  maxPoints: 12,

  // In seconds
  projectCardScanTime: 0.7,
  projectCardWithdrawTime: 1.4,
  processCardScanTime: 0.5,
  processCardWithdrawTime: 0.4,

  maxValues: {
    biodiversity: 80,
    contentedness: 40,
  },

  // Project group colors
  groupStyle: {
    Restoration: {
      background: '#247f24',
    },
    Protection: {
      background: '#53a553'
    },
    Nuclear: {
      background: 'orange'
    },
    Agriculture: {
      background: 'wheat'
    },
    Control: {
      background: '#d83535',
    },
    Population: {
      background: '#6b6bec',
    },
    Food: {
      background: '#f3ff56',
    },
    Space: {
      background: '#250441',
      color: '#d0c0e4',
    },
    Geoengineering: {
      background: '#61688b',
    },
    Electrification: {
      background: '#fcba03',
    },
    Behavior: {
      background: '#b8ad91',
    },
    Limits: {
      background: '#4B5A85',
      color: '#ffffff',
    },
    Energy: {
      background: '#fee94a',
    },
    Materials: {
      background: '#5f2929',
      color: '#ffffff',
    },
    Buildings: {
      background: '#8f7ea9'
    },
    Cities: {
      background: '#566b6a',
      color: '#ffffff',
    },
  }
};

