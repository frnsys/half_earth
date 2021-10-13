const OUTPUTS = {
  'Fuel': 'kWh',
  'Electricity':	'kWh',
  'PlantCalories': 'kcal',
  'AnimalCalories': 'kcal',
  // 'Project': 'point',	// Used for projects/policies/research
  // 'Misc': 'point',	    // Catch-all for outputs not represented here, e.g. healthcare, transportation, etc
};

const RESOURCES = {
  'Land': 'm2',
  'Water': 'L',
  'Fuel': 'kWh',
  'Electricity': 'kWh',
}

const FEEDSTOCKS = {
  'Other': 'n/a',
  'Soil': 'fert',
  'Coal': 'g',
  'Oil': 'L',
  'NaturalGas': 'L',
  'Uranium': 'g',
  'Lithium': 'g',
};

const BYPRODUCTS = {
  'CO2': 'g',
  'CH4': 'g',
  'N2O': 'g',
  'Biodiversity': 'pressure',
}

const PROCESS_FEATURES = {
  'BuildsSoil': 'For agriculture; does the process improve soil health',
  'DegradesSoil': 'For agriculture; does the process harm soil health',
  'UsesPesticides': 'For agriculture; does the process use a significant amount of pesticides',
  'UsesSynFertilizer': 'For agriculture; does the process use a significant amount of synthetic fertilizer',
  'UsesLivestock': 'For agriculture; does the process use a significant amount of livestock',
  'IsIntermittent': 'For electricity sources; if the supply is intermittent',
  'IsNuclear': 'For electricity sources, if the supply is nuclear',
  'IsSolar': 'If the process depends on sunlight',
  'IsCCS': 'Whether this process produces CO2 that is then stored/transported/used',
}

const INCOME_LEVELS = [
  'Low',
  'Lower-Middle',
  'Upper-Middle',
  'High'
];

const COMPARATORS = ['<', '<=', '==', '!=', '>=', '>'];
const CONDITIONS = {
  LocalVariable: {
    compare: true,
    choices: [
      'Population', 'Health', 'Outlook',
      'Contentedness', 'Habitability']
  },
  WorldVariable: {
    compare: true,
    choices: [
      'Year', 'Population', 'Emissions',
      'Biodiversity', 'Temperature',
      'Precipitation', 'SeaLevelRise',
      'Outlook', 'Contentedness',
      'WaterStress',]
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
  Feedstock: {
    compare: true,
    choices: Object.keys(FEEDSTOCKS),
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
      'Outlook', 'BaseHabitability'],
    params: {
      'Change': Number
    }
  },

  WorldVariable: {
    choices: [
      'Population', 'Emissions', 'Health',
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

  Feedstock: {
    choices: Object.keys(FEEDSTOCKS),
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
  FEEDSTOCKS,
  PROCESS_FEATURES,
  EFFECTS,
  COMPARATORS,
  CONDITIONS,
  PROBABILITIES,
  INCOME_LEVELS
};
