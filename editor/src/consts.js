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
  'MakesNuclearWaste': 'For electricity sources, if the supply produces nuclear waste',
  'CanMeltdown': 'For electricity sources, if the supply has a meltdown risk',
  'IsSolar': 'If the process depends on sunlight',
  'IsCCS': 'Whether this process produces CO2 that is then stored/transported/used',
  'IsCombustion': 'If this process depends on combustion',
  'IsFossil': 'If this process uses fossil fuels',
  'UsesOil': 'If this process uses oil',
  'IsLaborIntensive': 'If this process is especially labor intensive',
}

const PROJECT_GROUPS = [
  'Other',
  'Space',
  'Nuclear',
  'Restoration',
  'Agriculture',
  'Food',
  'Geoengineering',
  'Population',
  'Control',
  'Protection',
  'Electrification',
  'Behavior',
  'Limits',
  "Energy",
  "Materials",
  "Buildings",
  "Cities",
];

const INCOME_LEVELS = [
  'Low',
  'Lower-Middle',
  'Upper-Middle',
  'High'
];

const LATITUDES = [
  'Tropic',
  'Subtropic',
  'Temperate',
  'Frigid'
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
      'YearsToDeath',
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
  ProcessOutput: {
    compare: true,
    entity: 'Process',
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
  ActiveProjectUpgrades: {
    entity: 'Project',
    compare: true,
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
  WithoutFlag: {
    flag: true,
  },
  HeavyProjects: {
    compare: true,
  },
  NPCRelationship: {
    choices: ['Neutral', 'Nemesis', 'Ally'],
    entity: 'NPC',
  },
  ProtectLand: {
    compare: true
  }
}

const SUBPHASES = {
  World: ['Main', 'Start', 'End'],
  Planning: [
    'Start', 'End', 'Plan',
    'Regions', 'Parliament',
    'Dashboard', 'Research', 'Initiatives',
    'Policies', 'Processes', 'PlanChange'],
  Report: ['Start'],
  Break: ['Start'],
  End: ['Start'],
  Interstitial: ['Start', 'Win'],
  Icon: [],
  Cutscene: ['Intro'],
}

const EFFECTS = {
  WorldVariable: {
    choices: [
      'Emissions',
      'ExtinctionRate', 'Temperature',
      'Precipitation', 'Population',
      'SeaLevelRise', 'SeaLevelRiseRate',
      'Outlook', 'PopulationGrowth'],
    params: {
      'Change': Number
    },
    desc: {
      'Emissions': 'Change annual emissions by a fixed amount, in gigatonnes',
      'ExtinctionRate': 'Change extinction rate by a fixed amount. 60 is the worst extinction rate; 0 is baseline extinction rate',
      'Temperature': 'Change temperature by a fixed amount, in C',
      'Precipitation': 'Change average precipitation by a fixed amount, cm/yr',
      'Population': 'Change global population by a fixed amount',
      'SeaLevelRise': 'Change sea level rise by a fixed amount, in meters',
      'SeaLevelRiseRate': 'Change sea level rise rate by a fixed amount, in m/yr',
      'Outlook': 'Change global outlook; global outlook starts at 20; 0 is the worst',
      'PopulationGrowth': 'Increase/decrease pop growth by this amount, e.g. 20 is 20% faster growth'
    }
  },

  RegionHabitability: {
    choices: LATITUDES,
    params: {
      'Change': Number
    },
    desc: 'Change habitability of a region: habitability starts at 10 (best); 0 is uninhabitable'
  },

  PlayerVariable: {
    choices: [
      'PoliticalCapital',
      'ResearchPoints',
    ],
    params: {
      'Change': Number
    },
    desc: 'Change the selected player variable by a fixed amount'
  },

  Demand: {
    choices: Object.keys(OUTPUTS),
    params: {
      'PercentChange': Number
    },
    desc: 'Increase/decrease demand for selected output, e.g. 20 is a 20% increase in demand.'
  },

  DemandAmount: {
    choices: Object.keys(OUTPUTS),
    params: {
      'Change': Number
    },
    desc: 'Increase/decrease demand for selected output by a fixed amount.'
  },

  Output: {
    choices: Object.keys(OUTPUTS),
    params: {
      'PercentChange': Number
    },
    desc: 'Increase/decrease production for selected output (without changing impacts), e.g. 20 is a 20% increase in output.'
  },

  OutputForFeature: {
    choices: Object.keys(PROCESS_FEATURES),
    params: {
      'PercentChange': Number
    },
    desc: 'Increase/decrease production for processes with the selected feature (without changing impacts), e.g. 20 is a 20% increase in output.'
  },

  OutputForProcess: {
    entity: 'Process',
    params: {
      'PercentChange': Number
    },
    desc: 'Increase/decrease production for the selected process (without changing impacts), e.g. 20 is a 20% increase in output.'
  },

  CO2ForFeature: {
    choices: Object.keys(PROCESS_FEATURES),
    params: {
      'PercentChange': Number
    },
    desc: 'Increase/decrease emissions for the selected process (without changing its output), e.g. 20 is a 20% increase in emissions.'
  },
  BiodiversityPressureForFeature: {
    choices: Object.keys(PROCESS_FEATURES),
    params: {
      'Change': Number
    },
    desc: 'Increase/decrease biodiversity pressure for the selected process (without changing its output), e.g. 5 increase biodiversity pressure by 5.'
  },
  ProcessLimit: {
    entity: 'Process',
    params: {
      'Change': Number
    },
    desc: 'Change the maximum output the selected process can produce by a fixed amount.'
  },

  Resource: {
    choices: Object.keys(RESOURCES),
    params: {
      'Amount': Number
    },
    desc: 'Change the available amount of a resource by a fixed amount.'
  },

  Feedstock: {
    choices: Object.keys(FEEDSTOCKS),
    params: {
      'PercentChange': Number
    },
    desc: 'Increase/decrease the available amount of a feedstock, e.g. 20 is a 20% increase in that feedstock.'
  },

  TriggerEvent: {
    entity: 'Event',
    params: {
      'Delay (years)': Number,
    },
    desc: 'Trigger an event in the specified number of years.'
  },

  AddEvent: {
    entity: 'Event',
    desc: 'Add an event to the event pool. It may or may not trigger, depending on if its conditions are satisfied.'
  },

  UnlocksProject: {
    entity: 'Project',
    desc: 'Unlocks a project.'
  },

  LocksProject: {
    entity: 'Project',
    desc: 'Locks a project.'
  },

  UnlocksProcess: {
    entity: 'Process',
    desc: 'Unlocks a process.'
  },

  UnlocksNPC: {
    entity: 'NPC',
    desc: 'Unlocks an NPC.'
  },

  TerminationShock: {
    desc: 'Special effect for termination shock'
  },

  RegionLeave: {
    desc: 'A region leaves Gosplant'
  },

  Migration: {
    desc: 'Trigger a migration event (the game engine will figure out where)'
  },

  GameOver: {
    desc: 'Trigger a game over'
  },

  ProjectRequest: {
    choices:  ['Implement', 'Stop'],
    entity: 'Project',
    params: {
      'Bounty': Number
    },
    desc: 'A request for the player to implement or stop a project, rewarding them with the bounty in political capital'
  },
  ProcessRequest: {
    choices: ['Ban', 'Unban'],
    entity: 'Process',
    params: {
      'Bounty': Number
    },
    desc: 'A request for the player to ban or unban a process, rewarding them with the bounty in political capital'
  },

  AddFlag: {
    params: {
      'Flag': String,
      'Description': String,
    },
    desc: 'Set the specified flag on the player. These have to be manually implemented in the game engine.'
  },

  AddRegionFlag: {
    params: {
      'Flag': String
    },
    desc: 'Set the specified flag for a region. These have to be manually implemented in the game engine.'
  },

  AutoClick: {
    entity: 'IconEvent',
    params: {
      'Chance': Number,
    },
    desc: '[Not in use]'
  },

  NPCRelationship: {
    entity: 'NPC',
    params: {
      'Change': Number,
    },
    desc: 'Changes the player\'s relationship with this NPC by a fixed amount. Relationships are from 0-6, <=1 is a nemesis and >=5 is an ally/in the coalition.'
  },

  ModifyProcessByproducts: {
    entity: 'Process',
    choices:  Object.keys(BYPRODUCTS),
    params: {
      'Multiplier': Number
    },
    desc: 'Changes byproducts for an process, e.g. -0.2 is 20% fewer byproducts.'
  },
  ModifyIndustryByproducts: {
    entity: 'Industry',
    choices:  Object.keys(BYPRODUCTS),
    params: {
      'Multiplier': Number
    },
    desc: 'Changes byproducts for an industry, e.g. -0.2 is 20% fewer byproducts.'
  },
  ModifyIndustryResources: {
    entity: 'Industry',
    choices:  Object.keys(RESOURCES),
    params: {
      'Multiplier': Number
    },
    desc: 'Changes the resource required by a industry, e.g. -0.2 is 20% less of that resource required.'
  },
  ModifyIndustryResourcesAmount: {
    entity: 'Industry',
    choices:  Object.keys(RESOURCES),
    params: {
      'Amount': Number
    },
    desc: 'Changes the resource required by an industry by a fixed amount (in units for that resource, i.e. energy=KWh; food=calories).'
  },
  ModifyEventProbability: {
    entity: 'Event',
    params: {
      'Change': Number
    },
    desc: 'Changes probability for the specified event, e.g. -0.2 is 20% less likely.'
  },
  ModifyIndustryDemand: {
    entity: 'Industry',
    params: {
      'Change': Number
    },
    desc: 'Changes demand for the specified industry, e.g. -0.2 is 20% less demand.'
  },

  // Scale by region based on region demand for the specified output
  DemandOutlookChange: {
    choices: Object.keys(OUTPUTS),
    params: {
      'Multiplier': Number
    },
    desc: 'Changes regional outlooks based on their demand for this output; i.e. demand level * this value; demand level correlates with income level'
  },

  // Scale by region based on region income level
  IncomeOutlookChange: {
    params: {
      'Multiplier': Number
    },
    desc: 'Changes regional outlooks based on their income level; i.e. income level * this value.'
  },

  ProjectCostModifier: {
    entity: 'Project',
    params: {
      'Change': Number
    },
    desc: 'Change the cost of the specified project, e.g. -0.5 is 50% cheaper.'
  },

  ProtectLand: {
    params: {
      'Percent': Number
    },
    desc: 'Place the specified amount of land under protection, e.g. 20 means 20% of land under protection.'
  },

  BailOut: {
    params: {
      'Amount': Number
    },
    desc: 'Reset the player\'s political capital to this amount.'
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
  'The Doom Cultist',
  'The Leather Underground',
  'The Earth Liberation Front',
  'The Wretched',
  'The Spacer',
  'The Doctor',
  'The Engineer',
  'The Soldier',
  'The Authoritarian',
  'The Analyst',
  'The Animal Liberationist',
  'The Posadist',
  'The Utopian',
  'The Accelerationist',
  'The Malthusian',
  'The Ecofeminist',
  'The Consumerist',
  'The Fanonist',
  'The Environmentalist',
  'The Hero',
];

const EVENT_TYPES = [
  'World',      // Occur in the world/event stream
  'Planning',   // Occur during planning sessions
  'Report',     // Occurs during the reports
  'Break',      // Occur between runs
  'End',        // Occur at the end of the game (victory)
  'Interstitial', // Occurs before planning sessions
  'Icon',       // Occur in the world/event stream, but only as icons
  'Cutscene',    // Cutscene dialogue
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
  PROJECT_GROUPS,
  LATITUDES,
};
