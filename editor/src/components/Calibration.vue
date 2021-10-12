<template>
<div class="calibration">
  <div class="calibration-close" @click="close">X</div>
  <div class="calibration-body">
    <div class="calibration-multi">
      <LineChart title="Regional Populations (BAU)" :y="years" :xs="data.populations.regions"></LineChart>
      <LineChart title="Global Population (BAU)" :y="years" :xs="{Global: data.populations.global}"></LineChart>
    </div>
    <div class="calibration-multi">
      <LineChart v-if="data.demands" title="Calories (BAU; Tcals)"
        :y="years"
        :xs="{'Animal Calories': data.demands['AnimalCalories'], 'Plant Calories': data.demands['PlantCalories']}"></LineChart>
      <LineChart v-if="data.demands" title="Energy (BAU; TWh)"
        :y="years"
        :xs="{
          'Electricity': data.demands['Electricity'],
          'Reference (Electricity)': refLine('Electricity'),
          'Fuel': data.demands['Fuel'],
          'Reference (Fuel)': refLine('Fuel')}"></LineChart>
    </div>
    <div class="calibration-multi">
      <LineChart v-if="data.demands" title="Water (BAU)"
        :y="years"
        :xs="{'Water (km3)': data.demands['Water'], 'Reference': refLine('Water')}"></LineChart>
      <LineChart v-if="data.demands" title="Land (BAU)"
        :y="years"
        :xs="{
          'Land (km2)': data.demands['Land'],
          'Reference': refLine('Land')}"></LineChart>
    </div>
    <div class="calibration-multi">
      <LineChart v-if="data.byproducts" title="CO2 (BAU)"
        :y="years"
        :xs="{
          'CO2 (Gt)': data.byproducts['CO2'],
          'Reference (2016)': refLine('CO2')}"></LineChart>
      <LineChart v-if="data.byproducts" title="CH4 (BAU)"
        :y="years"
        :xs="{
          'CH4': data.byproducts['CH4'],
          'Reference (CH4)': refLine('CH4')}"></LineChart>
      <LineChart v-if="data.byproducts" title="N2O (BAU)"
        :y="years"
        :xs="{
          'N2O': data.byproducts['N2O'],
          'Reference (N2O, 2014)': refLine('N2O')}"></LineChart>
    </div>
    <div class="calibration-multi">
      <LineChart v-if="data.feedstocks" title="Oil (BAU)"
        :y="years"
        :xs="{
          'Oil (m3)': data.feedstocks['Oil'],
          'Reference (2016)': refLine('Oil')}"></LineChart>
      <LineChart v-if="data.feedstocks" title="Coal (BAU)"
        :y="years"
        :xs="{
          'Coal (Gt)': data.feedstocks['Coal'],
          'Reference (2016)': refLine('Coal')}"></LineChart>
      <LineChart v-if="data.feedstocks" title="Natural Gas (BAU)"
        :y="years"
        :xs="{
          'Natural Gas (m3)': data.feedstocks['NaturalGas'],
          'Reference (2017)': refLine('Natural Gas')}"></LineChart>
    </div>
    <div class="calibration-multi">
      <LineChart v-if="data.feedstocks" title="Uranium (BAU)"
        :y="years"
        :xs="{
          'Uranium (t)': data.feedstocks['Uranium'],
          'Reference': refLine('Uranium')}"></LineChart>
      <LineChart v-if="data.feedstocks" title="Lithium (BAU)"
        :y="years"
        :xs="{'Lithium (t)': data.feedstocks['Lithium']}"></LineChart>
    </div>
  </div>
</div>
</template>

<script>
import state from '../state';
import consts from '../consts';
import LineChart from './LineChart.vue';

const n = 2100-2020;
const years = [...Array(n).keys()].map((i) => 2020+i);

const referenceValues = {
  'Oil': 5.635, // 2016 consumption, in km3 (35,442,913,090 barrels)
  'Water': 4600, // km3, https://www.nature.com/articles/s41545-019-0039-9
  'Coal': 8.5, // 2016 consumption, in Gt
  'Natural Gas': 3746, // 2017 consumption, in km3
  'Uranium': 62800, // t, https://en.wikipedia.org/wiki/Peak_uranium
  'CO2': 36.45, // 2019, Gt, https://ourworldindata.org/grapher/annual-co-emissions-by-region
  'CH4': 570, // Mt, https://www.iea.org/reports/methane-tracker-2020
  'N2O': 3.3, // 2014, Mt, https://agupubs.onlinelibrary.wiley.com/doi/abs/10.1029/2020GB006698
  'Land': 104000, // km2, total habitable land area
  'Electricity': 27000, // TWh, https://www.iea.org/data-and-statistics/charts/electricity-generation-by-fuel-and-scenario-2018-2040
  'Fuel': 156.75 // TWh, https://www.eia.gov/todayinenergy/detail.php?id=46596
};

function defaultObj(obj, defaultFn) {
  return Object.keys(obj).reduce((acc, key) => {
    acc[key] = defaultFn();
    return acc;
  }, {});
}

function scale(vals, scale) {
  return vals.map((v) => v * scale);
}

function add(valsA, valsB) {
  return valsA.map((a, i) => a + valsB[i]);
}

function fnFromCoefs(coefs) {
  return (x) => {
    return coefs[0] + x*coefs[1] + (x**2)*coefs[2] + (x**3)*coefs[3];
  };
}

function parseFloatList(vl) {
  return vl.split('\n').map((v) => parseFloat(v));
}

function itemsOfType(type) {
  return Object.values(state.items)
    .filter((i) => i._type == type && !i.deleted);
}

function getIncomeConsts() {
  const incomeVars = {};
  const consts = itemsOfType('Const');
  consts.forEach((c) => {
    if (c.name.includes('income')) {
      let [incomeLevel, name] = c.name.split('__');
      let incomeName;
      switch (incomeLevel) {
        case 'low_income':
          incomeName = 'Low';
          break;
        case 'lower_middle_income':
          incomeName = 'Lower-Middle';
          break;
        case 'upper_middle_income':
          incomeName = 'Upper-Middle';
          break;
        case 'high_income':
          incomeName = 'High';
          break;
      }
      if (!(name in incomeVars)) incomeVars[name] = {};
      if (name == 'pop_change_coefs') {
        incomeVars[name][incomeName] = fnFromCoefs(parseFloatList(c.value));
      } else {
        incomeVars[name][incomeName] = parseFloat(c.value);
      }
    }
  });
  return incomeVars;
}

export default {
  components: {
    LineChart
  },
  data() {
    return {
      years,
      state
    }
  },
  mounted() {
    document.body.style.overflow = 'hidden';
  },
  methods: {
    close() {
      document.body.style.overflow = 'hidden';
      this.$emit('close');
    },
    refLine(k) {
      return years.map(() => referenceValues[k]);
    }
  },
  computed: {
    data() {
      const industries = itemsOfType('Industry');
      const regions = itemsOfType('Region');
      const incomeConsts = getIncomeConsts();

      if (Object.keys(incomeConsts).length == 0) {
        return {
          populations: {
            global: 0,
            regions: 0
          },
        }
      };

      // Population for each income level for each year
      const incomePops = consts.INCOME_LEVELS.reduce((acc, lvl) => {
        acc[lvl] = years.map((_) => 0);
        return acc;
      }, {});

      // Population for each region for each year
      const regionPops = regions.reduce((acc, r) => {
        let pops = [r.population];
        incomePops[r.income_level][0] += r.population;
        years.forEach((year, i) => {
          const changeFn = incomeConsts['pop_change_coefs'][r.income_level];
          const change = (1+changeFn(i));
          const newPop = pops[i] * change;
          pops.push(newPop);
          incomePops[r.income_level][i+1] += newPop;
        });
        acc[r.name] = pops;
        return acc;
      }, {});

      // Population for the world for each year
      const globalPops = years.map((year, i) => Object.values(regionPops).reduce((acc, vs) => acc + vs[i], 0));

      // Group processes by output type
      const processes = itemsOfType('Process');
      const processesByOutput = defaultObj(consts.OUTPUTS, () => []);
      processes.forEach((p) => {
        processesByOutput[p.output].push(p);
      });

      // Calculate global population in LIC equivalents for a given resource
      const LICPopForResource = (i, resource) => {
        const totalLICPop = incomePops['Low'][i] +
          (incomePops['Lower-Middle'][i] * incomeConsts[resource]['Lower-Middle']) +
          (incomePops['Upper-Middle'][i] * incomeConsts[resource]['Upper-Middle']) +
          (incomePops['High'][i] * incomeConsts[resource]['High'])
        return totalLICPop;
      };

      // Calculate industry + base per capita consumption demand for a resource
      const baseDemandForResource = (resource, incomeVarKey) => {
        return years.map((year, i) => {
          let perLICDemand = industries.reduce((acc, ind) => {
            return acc + (ind.resources[resource] || 0);
          }, 0) + incomeConsts[`${incomeVarKey}_per_capita`]['Low'];
          let pop = LICPopForResource(i, incomeVarKey);
          let totalDemand = pop * perLICDemand;
          return totalDemand;
        });
      };
      // Calculate demand for resources for a given output
      // and the total demand for that output
      const resourceDemandsForOutputs = (outputDemands) => {
        const resourceDemand = defaultObj(consts.RESOURCES, () => years.map(() => 0));
        Object.keys(outputDemands).forEach((output) => {
          let outputDemandByYear = outputDemands[output];
          outputDemandByYear.forEach((outputDemand, i) => {
            processesByOutput[output].forEach((p) => {
              if (p.mix_share > 0) {
                let demandShare = (p.mix_share/100) * outputDemand;
                Object.keys(p.resources).forEach((r) => {
                  resourceDemand[r][i] += p.resources[r] * demandShare;
                });
              }
            });
          });
        });
        return resourceDemand;
      };

      const byproductsForOutputs = (outputDemands) => {
        const byproducts = defaultObj(consts.BYPRODUCTS, () => years.map(() => 0));
        Object.keys(outputDemands).forEach((output) => {
          let outputDemandByYear = outputDemands[output];
          outputDemandByYear.forEach((outputDemand, i) => {
            processesByOutput[output].forEach((p) => {
              if (p.mix_share > 0) {
                let demandShare = (p.mix_share/100) * outputDemand;
                Object.keys(p.byproducts).forEach((b) => {
                  byproducts[b][i] += p.byproducts[b] * demandShare;
                });
              }
            });
          });
        });
        return byproducts;
      };

      const feedstocksForOutputs = (outputDemands) => {
        const feedstockDemand = defaultObj(consts.FEEDSTOCKS, () => years.map(() => 0));
        Object.keys(outputDemands).forEach((output) => {
          let outputDemandByYear = outputDemands[output];
          outputDemandByYear.forEach((outputDemand, i) => {
            processesByOutput[output].forEach((p) => {
              if (p.mix_share > 0) {
                let demandShare = (p.mix_share/100) * outputDemand;
                feedstockDemand[p.feedstock][i] += p.feedstock_amount * demandShare;
              }
            });
          });
        });
        return feedstockDemand;
      };

      // Electricity demand
      // Convert from kWh to TWh
      let outputDemands = {
        'Electricity': baseDemandForResource('Electricity', 'electricity'),
        'Fuel': baseDemandForResource('Fuel', 'fuel'),
        'AnimalCalories': baseDemandForResource('AnimalCalories', 'animal_calories'),
        'PlantCalories': baseDemandForResource('PlantCalories', 'plant_calories'),
      };
      let processResourceDemands = resourceDemandsForOutputs(outputDemands);

      outputDemands['Electricity'] = add(outputDemands['Electricity'], processResourceDemands['Electricity']);
      outputDemands['Fuel'] = add(outputDemands['Fuel'], processResourceDemands['Fuel']);
      let byproducts = byproductsForOutputs(outputDemands);
      let feedstocks = feedstocksForOutputs(outputDemands);
      console.log(feedstocks);

      let demands = {
        'Electricity': scale(outputDemands['Electricity'], 1e-9), // kWh to TWh
        'Fuel': scale(outputDemands['Fuel'], 1e-9), // kWh to TWh
        'AnimalCalories': scale(outputDemands['AnimalCalories'], 1e-9), // kcals to Tcals
        'PlantCalories': scale(outputDemands['PlantCalories'], 1e-9), // kcals to Tcals
        'Water': scale(add(baseDemandForResource('Water', 'water'), processResourceDemands['Water']), 1e-12), // L to km3
        'Land': scale(processResourceDemands['Land'], 1e-6), // m2 to km2; no base demand for land because it's negligible
      };

      byproducts['CO2'] = scale(byproducts['CO2'], 1e-15); // g to Gt
      byproducts['CH4'] = scale(byproducts['CH4'], 1e-12); // g to Mt
      byproducts['N2O'] = scale(byproducts['N2O'], 1e-12); // g to Mt

      feedstocks['Uranium'] = scale(feedstocks['Uranium'], 1e-6); // g to t
      feedstocks['Lithium'] = scale(feedstocks['Lithium'], 1e-6); // g to t
      feedstocks['Oil'] = scale(feedstocks['Oil'], 1e-12); // L to m3
      feedstocks['Coal'] = scale(feedstocks['Coal'], 1e-15); // g to t
      feedstocks['NaturalGas'] = scale(feedstocks['NaturalGas'], 1e-12); // L to m3
      return {
        populations: {
          global: globalPops,
          regions: regionPops
        },
        demands,
        byproducts,
        feedstocks,
      };
    }
  },
}
</script>

<style>
.calibration {
  width: 100%;
  height: 100vh;
  left: 0;
  top: 0;
  position: fixed;
  background: #222;
  z-index: 100;
  overflow-y: scroll;
}
.calibration-body {
}
.calibration-multi {
  display: flex;
}
.calibration-multi > .chart {
  flex: 1;
}
.calibration-close {
	cursor: pointer;
	opacity: 0.7;
  user-select: none;
	color: #aaa;
	font-size: 2em;
  position: absolute;
  right: 0.5em;
  top: 0.5em;
}
.calibration-close:hover {
  opacity: 1;
}

</style>
