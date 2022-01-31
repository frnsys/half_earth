export default {
  parliamentSeats: [2,3,4,5,5,5,4,3,2],
  maxRelationship: 6,
  processPointsPerCycle: 3,

  // How much PC is earned when completing a project
  pcPerCompletedProject: 5,

  // PC earned per intensity level of contentedness and extinction
  contentednessPc: [0, 0, 5, 10, 20],
  extinctionPc: [20, 10, 0, -5, -5, -10],

  // PC change per -0.1C temp change
  temperaturePc: 10,
  // PC change per -0.5Gt emissions change
  emissionsPc: 5,

  // PC cost for research/infrastructure points
  pointCost: 3,

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
    Diet: {
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
    }
  }
};

