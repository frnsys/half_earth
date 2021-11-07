import {Priority} from 'half-earth-engine';

export default {
  outputs: {
    names: {
      'Fuel': 'fuel',
      'Electricity': 'electricity',
      'PlantCalories': 'plant-based food',
      'AnimalCalories': 'animal-based food',
    },
    keys: {
      'Fuel': 'fuel',
      'Electricity': 'electricity',
      'PlantCalories': 'plant_calories',
      'AnimalCalories': 'animal_calories',
    },
  },
  icons: {
    'fuel': '⛽',
    'electricity': '⚡',
    'plant_calories': '🌾',
    'animal_calories': '🥩',
    'emissions': '☁️',
  },
  maxRelationship: 6,

  Priority: Priority,
  priorities: {
    [Priority.Scarcity]: {
      icon: 'output',
      name: 'Scarcity',
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

