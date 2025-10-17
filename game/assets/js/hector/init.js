import wasmHector from './hector.js';
import emissions from './emissions.js';

function setValue(h, section, variable, value) {
  if (Array.isArray(value)) {
    if (value.length == 2 && typeof(value[1]) == 'string') {
      h._set_double_unit(section, variable, value[0], value[1]);
    } else {
      value.forEach((v) => {
        if (v.length == 3) {
          h._set_timed_double_unit(section, variable, v[0], v[1], v[2]);
        } else {
          h._set_timed_double(section, variable, v[0], v[1]);
        }
      });
    }
  } else {
    switch (typeof(value)) {
      case 'string':
        h._set_string(section, variable, value);
        break;
      case 'number':
        h._set_double(section, variable, value);
        break;
      case 'boolean':
        h._set_double(section, variable, value ? 1.0 : 0.0);
        break;
      default:
        console.error(`Unrecognized config value type: ${value}`);
        break;
    }
  }
}

function set_emissions(h, scenario, wasm) {
  const startYear = scenario['startYear'];
  const columns = Object.keys(scenario['data']);
  Object.keys(emissions).forEach((section) => {
    emissions[section].forEach((source) => {
      if (columns.includes(source)) {
        let yearsVec = new wasm.VectorSizeT();
        Object.keys(scenario['data'][source]).map((v) => yearsVec.push_back(startYear + parseInt(v)));
        let valuesVec = new wasm.VectorDouble();
        Object.values(scenario['data'][source]).forEach((v) => valuesVec.push_back(v));
        h._set_timed_array(section, source, yearsVec, valuesVec);
      }
    });
  });
}

async function initHector(config, outputs) {
  return new wasmHector().then((wasm) => {
    const hector = new wasm.Hector();

    // Set config
    Object.keys(config).forEach((section) => {
      Object.keys(config[section]).forEach((variable) => {
        let val = config[section][variable];
        setValue(hector, section, variable, val);
      });
    });

    // Create observers for output variables
    // The more there are, the slower it runs!
    Object.keys(outputs).forEach((k) => {
        hector.add_observable(
          outputs[k]['component'],
          outputs[k]['variable'],
          outputs[k]['needs_date'] || false,
          false
        )
    });

    let prepared = false;
    return {
      run: (endDate, emissionsScenario) => {
        set_emissions(hector, emissionsScenario, wasm);
        if (!prepared) {
          hector.prepareToRun();
          prepared = true;
        }
        try {
          hector.run(endDate);

          // Collect observables
          let results = {}
          Object.keys(outputs).forEach((k) => {
            results[k] = hector.get_observable(
                outputs[k]['component'], outputs[k]['variable'], false
            );

            // Convert to JS array
            let arr = [];
            for (let i=0; i<results[k].size(); i++) {
              arr.push(results[k].get(i));
            }
            results[k] = arr;
          });

          return results;

        } catch (exception) {
          if (typeof(exception) === 'number') {
            console.error(wasm.getExceptionMessage(exception));
          } else {
            console.error(exception);
          }
        }

        // Reset to the end date so we
        // can pick up from here
        hector.reset(endDate);
      },
      reset: (resetDate) => {
        hector.reset(resetDate);
      },
      shutdown: () => {
        hector.shutdown();

        // Need to destroy to avoid memory leaks
        hector.delete();
      }
    };
  });
}

export default initHector;
