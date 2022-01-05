const OUTPUTS = {
  'Fuel': 'kWh',
  'Electricity':	'kWh',
  'PlantCalories': 'kcal',
  'AnimalCalories': 'kcal',
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
  'Thorium': 'g',
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
  'UsesPesticides': 'For agriculture; does the process use a significant amount of pesticides',
  'UsesSynFertilizer': 'For agriculture; does the process use a significant amount of synthetic fertilizer',
  'UsesLivestock': 'For agriculture; does the process use a significant amount of livestock',
  'IsIntermittent': 'For electricity sources; if the supply is intermittent',
  'IsNuclear': 'For electricity sources, if the supply is nuclear',
  'IsSolar': 'If the process depends on sunlight',
  'IsCCS': 'Whether this process produces CO2 that is then stored/transported/used',
  'IsCombustion': 'If this process depends on combustion',
  'IsFossil': 'If this process uses fossil fuels',
}

const PROJECT_GROUPS = [
  'Other',
  'Space',
  'Nuclear',
  'Restoration',
  'Agriculture',
  'Diet',
  'Geoengineering',
  'Population',
  'Control',
  'Protection',
  'Electrification',
  'Behavior',
];

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
      'Population', 'Outlook',
      'Habitability']
  },
  WorldVariable: {
    compare: true,
    choices: [
      'Year', 'Population', 'Emissions',
      'ExtinctionRate', 'Temperature',
      'Precipitation',
      'SeaLevelRise', 'SeaLevelRiseRate',
      'Outlook',
      'WaterStress', 'PopulationGrowth']
  },
  PlayerVariable: {
    compare: true,
    choices: [
      'PoliticalCapital',
      'MalthusianPoints',
      'FALCPoints',
      'HESPoints',
    ],
  },
  Demand: {
    compare: true,
    choices: Object.keys(OUTPUTS),
  },
  OutputDemandGap: {
    compare: true,
    choices: Object.keys(OUTPUTS),
  },
  ResourcePressure: {
    compare: true,
    choices: Object.keys(RESOURCES),
  },
  FeedstockYears: {
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
  ProjectBuilding: {
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
  RunsPlayed: {
    compare: true,
  },
  RegionFlag: {
    flag: true
  },
  HasFlag: {
    flag: true,
  },
  HeavyProjects: {
    compare: true,
  },
  NPCRelationship: {
    choices: ['Neutral', 'Nemesis', 'Ally'],
    entity: 'NPC',
  }
}

const SUBPHASES = {
  World: ['Main', 'Start', 'End'],
  Planning: [
    'Start', 'End', 'Plan',
    'Regions', 'Coalition',
    'Dashboard', 'Research', 'Initiatives',
    'Policies', 'Processes', 'PlanChange'],
  Report: ['Start'],
  Break: ['Start'],
  End: ['Start'],
  Icon: [],
  Crisis: [],
}

const EFFECTS = {
  LocalVariable: {
    choices: ['Outlook', 'Habitability'],
    params: {
      'Change': Number
    }
  },

  WorldVariable: {
    choices: [
      'Emissions',
      'ExtinctionRate', 'Temperature',
      'Precipitation', 'Population',
      'SeaLevelRise', 'SeaLevelRiseRate',
      'Outlook', 'PopulationGrowth'],
    params: {
      'Change': Number
    }
  },

  PlayerVariable: {
    choices: [
      'PoliticalCapital',
      'ResearchPoints',
      'MalthusianPoints',
      'FALCPoints',
      'HESPoints',
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

  DemandAmount: {
    choices: Object.keys(OUTPUTS),
    params: {
      'Change': Number
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

  OutputForProcess: {
    entity: 'Process',
    params: {
      'PercentChange': Number
    }
  },

  CO2ForFeature: {
    choices: Object.keys(PROCESS_FEATURES),
    params: {
      'PercentChange': Number
    }
  },

  ProcessLimit: {
    entity: 'Process',
    params: {
      'Change': Number
    }
  },

  Resource: {
    choices: Object.keys(RESOURCES),
    params: {
      'Amount': Number
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
      'Delay (years)': Number,
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

  UnlocksNPC: {
    entity: 'NPC'
  },

  SetProjectStatus: {
    entity: 'Project',
    choices: ['Active', 'Stalled', 'Halted'],
    params: {
      'Duration': Number
    }
  },

  RegionLeave: {
  },

  Migration: {
  },

  GameOver: {
  },

  ProjectRequest: {
    choices:  ['Implement', 'Stop'],
    entity: 'Project',
    params: {
      'Bounty': Number
    }
  },
  ProcessRequest: {
    choices: ['Ban', 'Unban'],
    entity: 'Process',
    params: {
      'Bounty': Number
    }
  },

  AddFlag: {
    params: {
      'Flag': String,
      'Description': String,
    }
  },

  AddRegionFlag: {
    params: {
      'Flag': String
    }
  },

  AutoClick: {
    entity: 'IconEvent',
    params: {
      'Chance': Number,
    }
  },

  NPCRelationship: {
    entity: 'NPC',
    params: {
      'Change': Number,
    }
  },

  ModifyIndustryByproducts: {
    entity: 'Industry',
    choices:  Object.keys(BYPRODUCTS),
    params: {
      'Multiplier': Number
    }
  },
  ModifyIndustryResources: {
    entity: 'Industry',
    choices:  Object.keys(RESOURCES),
    params: {
      'Multiplier': Number
    }
  },
  ModifyIndustryResourcesAmount: {
    entity: 'Industry',
    choices:  Object.keys(RESOURCES),
    params: {
      'Amount': Number
    }
  },
  ModifyEventProbability: {
    entity: 'Event',
    params: {
      'Change': Number
    }
  },
  ModifyIndustryDemand: {
    entity: 'Industry',
    params: {
      'Change': Number
    }
  },

  // Scale by region based on region demand for the specified output
  DemandOutlookChange: {
    choices: Object.keys(OUTPUTS),
    params: {
      'Multiplier': Number
    }
  },

  // Scale by region based on region income level
  IncomeOutlookChange: {
    params: {
      'Multiplier': Number
    }
  },

  ProjectCostModifier: {
    entity: 'Project',
    params: {
      'Change': Number
    }
  },

  ProtectLand: {
    params: {
      'Percent': Number
    }
  }
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

const SPEAKERS = [
  '[GAME]',       // For scene description, etc
  'Gossy',
  'The Economist',
  'The Ecologist',
  'The Climatologist',
  'The Geoengineer',
  'The Farmer',
  'The Alien',
  'The Citizen',
  'The Agronomist',
  'The Doom Cultist',
  'The Meat Militia',
  'The Green Meanies',
  'The Wretched',
  'The Envoy',
  'The Spacer',
  'The Doctor',
  'The Engineer',
  'The Soldier',
  'The Authoritarian',
  'The Analyst',
];

const EVENT_TYPES = [
  'World',      // Occur in the world/event stream
  'Planning',   // Occur during planning sessions
  'Report',     // Occurs during the reports
  'Break',      // Occur between runs
  'End',        // Occur at the end of the game (victory)
  'Icon',       // Occur in the world/event stream, but only as icons
  'Crisis',     // Occurs when the crisis meter fills
];

const DYNAMIC_COST_FACTORS = Object.keys(OUTPUTS).concat(['Time', 'Income']);

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
  INCOME_LEVELS,
  SPEAKERS,
  EVENT_TYPES,
  SUBPHASES,
  DYNAMIC_COST_FACTORS,
  PROJECT_GROUPS
};
