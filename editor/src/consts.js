const OUTPUTS = {
  'Fuel': 'barrels?',
  'Electricity':	'TWh',
  'PlantCalories': 'kcals',
  'AnimalCalories': 'kcals',
  'Concrete': 'tons',
  'Steel': 'tons',
  'Minerals': 'tons',
  // 'Project': 'points',	// Used for projects/policies/research
  // 'Misc': 'points',	    // Catch-all for outputs not represented here, e.g. healthcare, transportation, etc
};

const RESOURCES = {
  'Sun': '?units',
  'Wind': '?units',
  'Soil': '?units',
  'Water': 'm3',
  'Biomass?': '?units',
  'Lumber?': '?units',
  'Coal': 'tons',
  'Oil': '?units',
  'Uranium': '?units',
  'Lithium': '?units',
  'Labor': 'hours',
  'Fuel': 'barrels?',
  'Electricity': 'TWh',
  'PlantCalories': 'kcals',
  'Minerals': 'tons',
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
  'IsIntermittent': 'For electricity sources; if the supply is intermittent',
  'IsNuclear': 'For electricity sources, if the supply is nuclear',
  'IsSolar': 'For electricity sources, if the supply is solar',
  'IsCCS': 'Whether this process produces CO2 that is then stored/transported/used',
}

const COMPARATORS = ['<', '<=', '==', '!=', '>=', '>'];
const CONDITIONS = {
  LocalVariable: {
    compare: true,
    choices: [
      'Population', 'Health',
      'Safety', 'Outlook',
      'Contentedness', 'Habitability']
  },
  WorldVariable: {
    compare: true,
    choices: [
      'Year', 'Population', 'Emissions',
      'Biodiversity', 'Temperature',
      'Precipitation', 'SeaLevelRise',
      'Outlook', 'Contentedness',
      'ResourceIntensity']
  },
  Demand: {
    compare: true,
    choices: Object.keys(OUTPUTS),
  },
  Output: {
    compare: true,
    choices: Object.keys(OUTPUTS),
  },
  OutputDemandGap: {
    compare: true,
    choices: Object.keys(OUTPUTS),
  },
  Resource: {
    compare: true,
    choices: Object.keys(RESOURCES),
  },
  ResourceDemandGap: {
    compare: true,
    choices: Object.keys(RESOURCES),
  },
  ProcessMixShare: {
    compare: true,
    entity: 'Process',
  },
  ProcessMixShareFeature: {
    compare: true,
    choices: Object.keys(PROCESS_FEATURES),
  },
  ProjectActive: {
    entity: 'Project',
  },
  ProjectInactive: {
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
      'Safety', 'Outlook',
      'BaseHabitability'],
    params: {
      'Change': Number
    }
  },

  WorldVariable: {
    choices: [
      'Population', 'Emissions', 'Health',
      'Safety',
      'Biodiversity', 'Temperature',
      'Precipitation', 'SeaLevelRise',
      'Outlook'],
    params: {
      'Change': Number
    }
  },

  PlayerVariable: {
    choices: [
      'PoliticalCapital',
    ],
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

  OutputForFeature: {
    choices: Object.keys(PROCESS_FEATURES),
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

  UnlocksProcess: {
    entity: 'Process'
  },

  SetFlag: {
    entity: 'Flag'
  },

  RegionLeave: {
  },

  Migration: {
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
