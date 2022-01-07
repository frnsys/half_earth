export default {
  parliamentSeats: [2,3,4,5,5,5,4,3,2],
  maxRelationship: 6,
  processPointsPerCycle: 5,

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
};

