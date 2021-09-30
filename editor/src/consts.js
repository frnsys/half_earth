const OUTPUTS = {
  'Fuel': 'barrels?',
  'Electricity':	'kWh',
  'PlantCalories': 'kcals',
  'MeatCalories': 'kcals',
  'Concrete': 'tons',
  'Steel': 'tons',
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
  'Feed': 'units',
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
  'Biodiversity': 'units?',
}

const PROCESS_FEATURES = {
  'BuildsSoil': 'For agriculture; does the process improve soil health',
  'DegradesSoil': 'For agriculture; does the process harm soil health',
  'UsesPesticides': 'For agriculture; does the process use a significant amount of pesticides',
  'UsesSynFertilizer': 'For agriculture; does the process use a significant amount of synthetic fertilizer',
  'UsesLivestock': 'For agriculture; does the process use a significant amount of livestock',
  'Intermittent': 'For electricity sources; if the supply is intermittent',
}

const EFFECTS = {
  LocalVariable: {
    choices: [
      'Population', 'Health',
      'Safety', 'Outlook', 'Satiety'],
    params: {
      'Change': Number
    }
  },

  WorldVariable: {
    choices: [
      'Population', 'Emissions',
      'Biodiversity', 'Temperature',
      'Precipitation', 'SeaLevelRise',
      'OzoneDamage'],
    params: {
      'Change': Number
    }
  },

  Demand: {
    choices: Object.keys(OUTPUTS),
    params: {
      'Change': Number
    }
  },

  Output: {
    choices: Object.keys(OUTPUTS),
    params: {
      'Change': Number
    }
  },

  TriggerEvent: {
    entity: 'Event',
    params: {
      'Delay (months)': Number
    }
  },

  AddEvent: {
    entity: 'Event'
  }
};


export default {
  OUTPUTS,
  RESOURCES,
  BYPRODUCTS,
  PROCESS_FEATURES,
  EFFECTS,
};
