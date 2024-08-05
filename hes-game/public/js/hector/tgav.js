import initHector from './init';

// Base emissions scenario for Hector
const baseEmissionsScenario = '/assets/hector/rcp26.to_2050.json';
const defaultEmissionsScenario = '/assets/hector/rcp26.default_emissions.json';

const hectorOutputVars = {
  'temperature.Tgav': {
    'component': 'temperature',
    'description': 'global atmospheric temperature anomaly',
    'unit': 'degC',
    'variable': 'Tgav'
  }
};

class Temperature {
  constructor(startYear) {
    console.log("Constructing TEMPERATURE");
    this.startYear = startYear;

    fetch('/assets/hector/config.json')
      .then((resp) => resp.json())
      .then((config) => {
        this.config = config;
      });

    fetch(defaultEmissionsScenario)
      .then((resp) => resp.json())
      .then((defaultEmissions) => {
        this.defaultEmissions = defaultEmissions;
      });

    // Load the base emissions scenario history,
    // for computing temperature changes
    fetch(baseEmissionsScenario)
      .then((resp) => resp.json()).then((baseScenario) => {
        this.emissions = {
          startYear: baseScenario['startYear'],
          data: {}
        };

        // Only get the base scenario data up to the game starting year
        let baseYears = this.startYear - baseScenario['startYear'];
        Object.keys(baseScenario['data']).forEach((k) => {
          this.emissions['data'][k] = baseScenario['data'][k].slice(0, baseYears);
        });
      });
  }

  /*
   * Adds a year of emissions data.
   * `emissions` should have keys and values for each required
   * emissions type. Refer to `/assets/hector/rcp.to_2050.json`
   * for the required keys.
   */
  addEmissions(emissions) {
    Object.keys(this.defaultEmissions).forEach((k) => {
      let val = emissions[k] !== undefined ? emissions[k] : this.defaultEmissions[k];
      this.emissions['data'][k].push(val);
    });
  }

  updateTemperature() {
    let ready = this._hector ?
      Promise.resolve(this._hector) :
      initHector(this.config, hectorOutputVars).then((hector) => {
        this._hector = hector;
      });

    return ready.then(() => {
      // Calculate new avg global temp
      // Only compute up to the current year,
      // so the last returned tgav is the current tgav
      let endDate = this.emissions.startYear + this.emissions.data['ffi_emissions'].length;
      let results = this._hector.run(endDate, this.emissions);
      let avgGlobalTemps = results['temperature.Tgav'];
      let avgGlobalTemp = avgGlobalTemps[avgGlobalTemps.length - 1];
      return avgGlobalTemp;
    });
  }
}

export { Temperature };
