import {Priority} from 'half-earth-engine';

export default {
  maxRelationship: 6,

  Priority: Priority,
  priorities: {
    [Priority.Scarcity]: {
      icon: 'output',
      name: 'Efficiency',
      desc: 'Prioritize efficiency of all resources, based on their scarcity.'
    },
    [Priority.Land]: {
      icon: 'land',
      name: 'Land Use',
      desc: 'Prioritize minimizing land use above all else.'
    },
    [Priority.Emissions]: {
      icon: 'emissions',
      name: 'Emissions',
      desc: 'Prioritize minimizing emissions above all else.'
    },
    [Priority.Energy]: {
      icon: 'energy',
      name: 'Energy Use',
      desc: 'Prioritize minimizing energy use above all else.'
    },
    [Priority.Labor]: {
      icon: 'labor',
      name: 'Labor',
      desc: 'Prioritize minimizing labor above all else.'
    },
    [Priority.Water]: {
      icon: 'water',
      name: 'Water Use',
      desc: 'Prioritize minimizing water use above all else.'
    },
  }
};

