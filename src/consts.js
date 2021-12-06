import {Priority} from 'half-earth-engine';

export default {
  maxRelationship: 6,

  Priority: Priority,
  priorities: {
    [Priority.Scarcity]: {
      icon: 'output',
      name: 'Efficiency',
    },
    [Priority.Land]: {
      icon: 'land',
      name: 'Land Use',
    },
    [Priority.Emissions]: {
      icon: 'emissions',
      name: 'Emissions',
    },
    [Priority.Energy]: {
      icon: 'energy',
      name: 'Energy Use',
    },
    [Priority.Labor]: {
      icon: 'labor',
      name: 'Labor',
    },
    [Priority.Water]: {
      icon: 'water',
      name: 'Water Use',
    },
  }
};

