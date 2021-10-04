const OUTPUTS = {
  'Fuel': 'barrels?',
  'Electricity':	'kWh',
  'PlantCalories': 'kcal',
  'MeatCalories': 'kcal',
  'Concrete': 'ton',
  'Steel': 'ton',
  // 'Project': 'points',	// Used for projects/policies/research
  // 'Misc': 'points',	    // Catch-all for outputs not represented here, e.g. healthcare, transportation, etc
};

const RESOURCES = {
  'Sun': '?units',
  'Wind': '?units',
  'Soil': '?units',
  'Water': '?units',
  'Biomass?': '?units',
  'Lumber?': '?units',
  'Coal': 'tons',
  'Oil': '?units',
  'Uranium': '?units',
  'Lithium': '?units',
  'Labor': 'hours',
  'Fuel': 'barrels?',
  'Electricity': 'kWh',
  'PlantCalories': 'kcals',
  'Material': 'tons',
  'CO2': 'tons',
  'Concrete': 'tons',
  'Steel': 'tons',
  'Land': 'ha',
};

const BYPRODUCTS = {
  'CO2': 'tons',
  'Methane': 'tons',
  'Pollution': 'ppm?',
  'Biodiversity': 'e/msy',
}

const PROCESS_FEATURES = {
  'BuildsSoil': 'For agriculture; does the process improve soil health',
  'DegradesSoil': 'For agriculture; does the process harm soil health',
  'UsesPesticides': 'For agriculture; does the process use a significant amount of pesticides',
  'UsesSynFertilizer': 'For agriculture; does the process use a significant amount of synthetic fertilizer',
  'UsesLivestock': 'For agriculture; does the process use a significant amount of livestock',
  'Intermittent': 'For electricity sources; if the supply is intermittent',
}

const COMPARATORS = ['<', '<=', '==', '!=', '>=', '>'];
const CONDITIONS = {
  LocalVariable: {
    compare: true,
    choices: [
      'Population', 'Health',
      'Safety', 'Outlook', 'Satiety',
      'Contentedness', 'Habitability']
  },
  WorldVariable: {
    compare: true,
    choices: [
      'Year', 'Population', 'Emissions',
      'Biodiversity', 'Temperature',
      'tgav', 'Precipitation', 'SeaLevelRise',
      'OzoneDamage', 'Outlook',
      'LandUsePercent']
  },
  Demand: {
    compare: true,
    choices: Object.keys(OUTPUTS),
  },
  Output: {
    compare: true,
    choices: Object.keys(OUTPUTS),
  },
  ProcessMixShare: {
    compare: true,
    entity: 'Process',
  },
  ProcessMixShareFeature: {
    compare: true,
    choices: PROCESS_FEATURES,
  },
  ProjectActive: {
    entity: 'Project',
  },
  ProjectFinished: {
    entity: 'Project',
  },
  ProjectStalled: {
    entity: 'Project',
  },
  ProjectHalted: {
    entity: 'Project',
  },
  Flag: {
    entity: 'Flag',
  },
  RunsPlayed: {
    compare: true,
  },
}

const EFFECTS = {
  LocalVariable: {
    choices: [
      'Population', 'Health',
      'Safety', 'Outlook', 'Satiety',
      'BaseHabitability'],
    params: {
      'Change': Number
    }
  },

  WorldVariable: {
    choices: [
      'Population', 'Emissions',
      'Biodiversity', 'Temperature',
      'Precipitation', 'SeaLevelRise',
      'OzoneDamage', 'Outlook'],
    params: {
      'Change': Number
    }
  },

  Demand: {
    choices: Object.keys(OUTPUTS),
    params: {
      'PercentChange': Number
    }
  },

  Output: {
    choices: Object.keys(OUTPUTS),
    params: {
      'PercentChange': Number
    }
  },

  Resource: {
    choices: Object.keys(RESOURCES),
    params: {
      'PercentChange': Number
    }
  },

  TriggerEvent: {
    entity: 'Event',
    params: {
      'Delay (months)': Number,
    }
  },

  AddEvent: {
    entity: 'Event',
  },

  UnlocksProject: {
    entity: 'Project'
  },

  SetFlag: {
    entity: 'Flag'
  },
};

const PROBABILITIES = [
  'Impossible',
  'Improbable',
  'Rare',
  'Unlikely',
  'Random',
  'Likely',
  'Guaranteed'
];

export default {
  OUTPUTS,
  RESOURCES,
  BYPRODUCTS,
  PROCESS_FEATURES,
  EFFECTS,
  COMPARATORS,
  CONDITIONS,
  PROBABILITIES,
};
