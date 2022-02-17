<template>
<div class="planning--page planning--page--dashboard">
  <div class="dashboard-breakdown-menu-overlay" v-if="showBreakdownMenu">
    <div class="dashboard-breakdown-menu">
      <div v-for="choice in breakdownChoices" @click="chooseBreakdown(choice)">
        <img :src="icons[choice]" />{{choice.replace('_', ' ')}}
      </div>
    </div>
  </div>
  <div class="planning--dashboard">
    <div class="dashboard--item">
      <div class="minicard">
        +{{state.gameState.world.temperature.toFixed(1)}}C
      </div>
      <img :src="icons.warming" />
      <div class="dashboard--item-name">Temp. Anomaly</div>
    </div>
    <div class="dashboard--item" v-tip="factors.tips.emissions('Current annual emissions, in gigatonnes of CO2 equivalent.')">
      <div class="minicard">
        {{`${state.gameState.world.emissions.toFixed(1)}Gt`}}
        <div class="dashboard--change" v-if="changes.emissions != 0" v-tip="{icon: 'emissions', text: 'The estimated value after production changes have finished.'}">
          <img :src="icons.down_arrow_small" />
          <div class="dashboard--change-value">
            {{`${((changes.emissions * 1e-15) + state.gameState.world.emissions).toFixed(1)}Gt`}}
          </div>
        </div>
      </div>
      <img :src="icons.emissions" />
      <div class="dashboard--item-name">Emissions</div>
    </div>
    <div class="dashboard--item" v-tip="factors.tips.land('Current land use.')">
      <div class="minicard">
        {{format.landUsePercent(state.gameState.resources_demand.land).toFixed(0)}}%
        <div class="dashboard--change" v-if="changes.land != 0" v-tip="{icon: 'land', text: 'The estimated value after production changes have finished.'}">
          <img :src="icons.down_arrow_small" />
          <div class="dashboard--change-value">
            {{format.landUsePercent(changes.land + state.gameState.resources_demand.land).toFixed(0)}}%
          </div>
        </div>
      </div>
      <img :src="icons.land" />
      <div class="dashboard--item-name">Land Use</div>
    </div>
    <div class="dashboard--item" v-tip="factors.tips.energy('Current energy use.')">
      <div class="minicard">
        {{`${format.twh(state.gameState.output_demand.electricity + state.gameState.output_demand.fuel).toLocaleString()}TWh`}}
        <div class="dashboard--change" v-if="changes.energy != 0" v-tip="{icon: 'energy', text: 'The estimated value after production changes have finished.'}">
          <img :src="icons.down_arrow_small" />
          <div class="dashboard--change-value">
            {{`${format.twh(changes.energy + state.gameState.output_demand.electricity + state.gameState.output_demand.fuel).toLocaleString()}TWh`}}
          </div>
        </div>
      </div>
      <img :src="icons.energy" />
      <div class="dashboard--item-name">Energy Use</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
        <span :style="{color: currentWaterStress.color}">{{currentWaterStress.label}}</span>
        <div class="dashboard--change" v-if="changes.water != 0" v-tip="{icon: 'water', text: 'The estimated value after production changes have finished.'}">
          <img :src="icons.down_arrow_small" />
          <div class="dashboard--change-value" :style="{color: afterWaterStress.color}">
            {{afterWaterStress.label}}
          </div>
        </div>
      </div>
      <img :src="icons.water" />
      <div class="dashboard--item-name">Water Stress</div>
    </div>
    <div class="dashboard--item" v-tip="{icon: 'sea_level_rise', text: `Average sea levels have risen by ${state.gameState.world.sea_level_rise.toFixed(2)}m and are rising at a rate of ${(state.gameState.world.sea_level_rise_rate * 1000).toFixed(1)}mm per year`}">
      <div class="minicard">
        {{state.gameState.world.sea_level_rise.toFixed(2)}}m
      </div>
      <img :src="icons.sea_level_rise" />
      <div class="dashboard--item-name">Sea Level Rise</div>
    </div>
    <div class="dashboard--item" v-tip="factors.tips.biodiversity('The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse.')">
      <div class="minicard">
        <span :style="{color: currentExtinction.color}">{{currentExtinction.label}}</span>
        <div class="dashboard--change" v-if="changes.extinction != 0" v-tip="{icon: 'extinction_rate', text: 'The estimated value after production changes have finished.'}">
          <img :src="icons.down_arrow_small" />
          <div class="dashboard--change-value" :style="{color: afterExtinction.color}">
            {{afterExtinction.label}}
          </div>
        </div>
      </div>
      <img :src="icons.extinction_rate" />
      <div class="dashboard--item-name">Extinction Rate</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
        {{format.formatNumber(state.gameState.world.population)}}
      </div>
      <img :src="icons.population" />
      <div class="dashboard--item-name">Population</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
        <span :style="{color: avgIncomeLevel.color}">{{avgIncomeLevel.label}}</span>
      </div>
      <img :src="icons.wealth" />
      <div class="dashboard--item-name">Avg. Living Standards</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
        <span :style="{color: avgHabitability.color}">{{avgHabitability.label}}</span>
      </div>
      <img :src="icons.habitability" />
      <div class="dashboard--item-name">Avg. Habitability</div>
    </div>
  </div>
  <div class="dashboard-breakdown">
      <h3>Breakdown</h3>
      <div class="dashboard-breakdown-select" @click="showBreakdownMenu = true">
        <img :src="icons[breakdownFactor]" />{{ breakdownFactor.replace('_', ' ') }} ▼
      </div>
      <PieChart :dataset="dataset" :colors="colors" />
      <FactorsList :factors="tableData" />
      <div class="dashboard-breakdown-note">Only direct impacts are shown.</div>
  </div>
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import format from '/src/display/format';
import factors from '/src/display/factors';
import display from '/src/display/display';
import intensity from '/src/display/intensity';
import MiniCard from 'components/cards/mini/MiniCard.vue';
import FactorsList from 'components/cards/FactorsList.vue';
import PieChart from '../PieChart.vue';

const COLORS = {
  'land': [0xF0FEC4, 0x306B33],
  'water': [0xDCF7FD, 0x4C6AA5],
  'energy': [0xFDCE4C, 0xE81224],
  'emissions': [0xF2F7E2, 0x9CB55C],
  'biodiversity': [0xe5bce6, 0xA442A6],
  'electricity': [0xD6F5A1, 0xFF8956],
  'fuel': [0xF7F6C7, 0xBF8462],
  'animal_calories': [0xF8AD72, 0xCA5704],
  'plant_calories': [0xB1EF8F, 0x06CA9B],
};

export default {
  components: {
    PieChart,
    MiniCard,
    FactorsList,
  },
  data() {
    return {
      state,
      showBreakdownMenu: false,
      breakdownFactor: 'land',
      breakdownChoices: [
        'land', 'water', 'energy', 'emissions',
        'biodiversity', 'electricity', 'fuel',
        'animal_calories', 'plant_calories',
      ]
    }
  },
  computed: {
    colors() {
      return COLORS[this.breakdownFactor];
    },
    tableData() {
      return factors.tips[this.breakdownFactor]().card.data;
    },
    dataset() {
      let data = {};
      state.factors[this.breakdownFactor].forEach((d) => {
        data[d.name] = d.amount;
      });
      return data;
    },
    changes() {
      let changes = {
        'land': 0,
        'emissions': 0,
        'water': 0,
        'energy': 0,
        'extinction': 0
      };
      state.gameState.processes.filter((p) => !p.locked).forEach((p) => {
        let mix_change = (state.processMixChanges[p.output][p.id] || 0) * 0.05;
        if (mix_change !== 0) {
          let multiplier = mix_change * state.gameState.output_demand[display.enumKey(p.output)];
          changes['land'] += p.resources.land * multiplier;
          changes['water'] += p.resources.water * multiplier;
          changes['energy'] += (p.resources.electricity + p.resources.fuel) * multiplier;
          changes['emissions'] += format.co2eq(p.byproducts) * multiplier;
          changes['extinction'] += p.byproducts.biodiversity * multiplier;
        }
      });

      Object.keys(changes).forEach((k) => {
        changes[k] = Math.round(changes[k]);
      });

      return changes;
    },
    currentExtinction() {
      return this.extinction(state.gameState.world.extinction_rate);
    },
    afterExtinction() {
      return this.extinction(this.changes.extinction + state.gameState.world.extinction_rate);
    },
    avgHabitability() {
      let total = state.gameState.world.regions.reduce((acc, r) => {
        return acc + r.habitability;
      }, 0);
      let avg = Math.round(total/state.gameState.world.regions.length);
      let int = intensity.scale(avg, 'habitability');
      return {
        label: intensity.describe(int),
        color: intensity.color(int, true)
      }
    },
    avgIncomeLevel() {
      let total = 0;
      state.gameState.world.regions.forEach((r) => {
        let income = r.income_level + 1;
        income += r.development;
        total += income;
      });
      let avg = Math.round(total/state.gameState.world.regions.length);

      return {
        label: intensity.describe(avg - 1),
        color: intensity.color(avg, true)
      }
    },
    currentWaterStress() {
      return this.waterStress(state.gameState.resources_demand.water);
    },
    afterWaterStress() {
      return this.waterStress(this.changes.water + state.gameState.resources_demand.water);
    },
  },
  methods: {
    chooseBreakdown(choice) {
      this.showBreakdownMenu = false;
      this.breakdownFactor = choice;
    },
    waterStress(demand) {
      let label;
      let percentUse = format.waterUsePercent(demand);
      if (percentUse <= 0.2) {
        label = 'Very Low';
      } else if (percentUse <= 0.4) {
        label = 'Low';
      } else if (percentUse <= 0.6) {
        label = 'High';
      } else {
        label = 'Very High';
      }
      return {
        label,
        color: intensity.color(percentUse * 4, false)
      }
    },
    extinction(extinction) {
      let int = intensity.scale(extinction, 'extinction');
      return {
        label: intensity.describe(int),
        color: intensity.color(int, false)
      }
    }
  }
}
</script>

<style>
.planning--dashboard {
  display: flex;
  flex-wrap: wrap;
  max-width: 720px;
  margin: 0 auto;
  column-gap: 1em;
  justify-content: center;
}
.dashboard--item {
  margin: 1.5em 0.25em;
  position: relative;
  width: 120px;
  text-align: center;
  image-rendering: auto;
}
.dashboard--item .minicard {
  color: #fff;
  font-size: 1.2em;
  width: 120px;
  height: 80px;
  font-family: 'W95FA', monospace;
}
.dashboard--item > img {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 32px;
  box-shadow: 0px 1px 2px rgb(0 0 0 / 50%);
  border-radius: 2em;
}
.dashboard--item-name {
  text-transform: uppercase;
  font-size: 0.6em;
  margin-top: 0.5em;
  font-weight: bold;
  font-family: 'Inter', sans-serif;
}

.planning--dashboard .minicard {
  background: #222;
}

.planning--page--dashboard {
  background: url('/assets/backgrounds/dashboard.jpg');
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center center;
  image-rendering: pixelated;
}

.dashboard-breakdown {
  margin: 0 auto;
}
.dashboard-breakdown h3 {
  text-align: center;
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  font-size: 0.65em;
}
.dashboard-breakdown .factors--users {
  background: #EBDEC6;
  border-radius: 0.5em;
  max-height: 50vh;
  overflow-y: scroll;
  margin: 2em 0;
  font-size: 0.75em;
  padding: 0 0.5em;
  font-family: 'Inter', sans-serif;
  width: 280px;
  border-left: 1px solid #706041;
  border-top: 1px solid #706041;
  border-right: 1px solid #f3ecde;
  border-bottom: 1px solid #f3ecde;
  box-shadow: inset 1px 1px 4px rgb(0 0 0 / 30%);
}
.dashboard-breakdown .factors--total {
  background: #bbb4a7;
  border: 1px solid #6b6161;
  top: 0.5em;
  box-shadow: 0 0 2px rgba(0,0,0,0.5);
}
.dashboard-breakdown .factors--usage {
  font-size: 12px;
}

.dashboard-breakdown-menu-overlay {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  background: rgba(0,0,0,0.8);
  z-index: 10;
  display: flex;
  align-items: center;
}

.dashboard-breakdown-menu {
  background: #EBDEC6;
  max-width: 300px;
  border-radius: 0.5em;
  border-top: 1px solid #fff;
  border-left: 1px solid #fff;
  border-right: 1px solid #8d8068;
  border-bottom: 1px solid #8d8068;
  max-height: 80vh;
  margin: 1em auto 0;
}
.dashboard-breakdown-menu img {
  width: 26px;
  margin-right: 5px;
}
.dashboard-breakdown-menu > div {
  padding: 0.5em 2em;
  border-bottom: 1px solid #93856c;
  border-top: 1px solid #fff;
  display: flex;
  align-items: center;
  text-transform: capitalize;
}
.dashboard-breakdown-menu > div:hover {
  background: #d7c5a5;
}
.dashboard-breakdown-menu > div:first-child {
  border-top: none;
}
.dashboard-breakdown-menu > div:last-child {
  border-bottom: none;
  cursor: pointer;
}
.dashboard-breakdown-select {
  background: #fff;
  border-radius: 0.5em;
  box-shadow: 2px 2px 0 #747474;
  text-align: center;
  text-transform: capitalize;
  font-size: 1.5em;
  width: fit-content;
  margin: 0 auto 0.5em;
  padding: 0.5em 0.75em;
}
.dashboard-breakdown-select img {
  width: 24px;
  margin-right: 5px;
  position: relative;
  top: 3px;
}
.dashboard-breakdown-note {
  text-align: center;
  margin: 0 0 2em 0;
  font-size: 0.8em;
}

.dashboard--change {
  font-size: 0.75em;
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0.5em;
}
</style>
