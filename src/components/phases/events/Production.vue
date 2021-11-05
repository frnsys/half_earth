<template>
<div class="production">
  <div class="production--demand">
    <div v-for="v, k in produced">
      <template v-if="demand[k] !== undefined">
        {{v}}/{{demand[k]}}{{consts.icons[k]}}
      </template>
      <template v-else>
        {{v}}{{consts.icons[k]}}
      </template>
    </div>
  </div>
  <div class="production--processes">
    <div class="production--process" v-for="process in processes">
      <div class="production--process-name">{{process.name}}</div>
      <div class="production--process-amounts">
        <div>{{process.amount}} {{consts.icons[process.output]}}</div>
        <div>{{process.emissions}} {{consts.icons['emissions']}}</div>
      </div>
    </div>
  </div>
  <button @click="$emit('done')">Ok</button>
</div>
</template>

<script>
import state from '/src/state';

// TODO iterate through industries and their emissions
const outputDemandUnits = {
  fuel: 1e-9/1e3,            // per 1000 TWh
  electricity: 1e-9/1e3,     // per 1000 TWh
  plant_calories: 1e-9/2e4,  // per 20000 Tcals
  animal_calories: 1e-9/2e4, // per 20000 Tcals
};

const convertOutput = {
  'Fuel': 'fuel',
  'Electricity': 'electricity',
  'PlantCalories': 'plant_calories',
  'AnimalCalories': 'animal_calories',
}

export default {
  data() {
    let processes = state.gameState.processes.map((p, i) => {
      let baseAmount = state.gameState.produced_by_process[i];
      let amount = baseAmount * outputDemandUnits[convertOutput[p.output]];
      amount = amount > 0 ? Math.max(Math.round(amount), 1) : Math.round(amount);

      let emissions = baseAmount * (p.byproducts.co2 + p.byproducts.ch4 * 36 + p.byproducts.n2o * 298);
      emissions *= 1e-15; // Gt CO2eq
      emissions = emissions > 0 ? Math.max(Math.round(emissions), 1) : Math.round(emissions);
      let data = {
        emissions,
        amount, ...p
      };
      data.output = convertOutput[p.output];
      return data;
    }).filter((p) => p.amount > 0);
    let demand = Object.keys(state.gameState.output_demand).reduce((acc, k) => {
        acc[k] = Math.round(state.gameState.output_demand[k] * outputDemandUnits[k]);
        return acc;
      }, {});
    let produced = Object.keys(state.gameState.produced).reduce((acc, k) => {
        acc[k] = Math.round(state.gameState.produced[k] * outputDemandUnits[k]);
        return acc;
      }, {});
    let byp = state.gameState.byproducts;
    produced['emissions'] = Math.round((byp.co2 + byp.ch4 * 36 + byp.n2o * 298) * 1e-15); // Gt CO2eq;

    return {
      demand,
      produced,
      processes,
    }
  },
}
</script>

<style>
/* TODO temp */
.production {
  position: absolute;
  left: 1em;
  right: 1em;
  top: 1em;
  bottom: 1em;
  z-index: 100;
  background: #eee;
  border: 1px solid black;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
.production--demand {
  display: flex;
  justify-content: space-around;
  padding: 0.5em 1em;
  position: relative;
  z-index: 1;
  background: #111;
  color: #fff;
  border-radius: 0.2em;
}

.production--process {
  text-align: center;
  padding: 0.5em;
  background: #eee;
  border-radius: 0.5em;
  border: 1px solid;
  margin: 0.5em;
  width: 120px;
}

.production--surplus {
  text-align: center;
}
.production button {
  margin: 0 0 0.5em 0;
}
.production--surplus h2 {
  text-align: center;
}
.production--surplus > div {
  display: flex;
  justify-content: space-around;
}
.production--surplus img {
  width: 22px;
  vertical-align: middle;
}

.production--processes {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  overflow-y: scroll;
}
.production--process-name {
  font-size: 0.8em;
  border-bottom: 1px solid;
}
</style>
