<template>
<div class="planning--page">
  <div class="planning--dashboard">
    <div class="dashboard--item">
      <div class="minicard">
        +{{state.gameState.world.temperature.toFixed(1)}}°C
      </div>
      <img :src="icons.warming" />
      <div class="dashboard--item-name">Temperature Anomaly</div>
    </div>
    <div class="dashboard--item" v-tip="factors.tips.emissions('Current annual emissions, in gigatonnes of CO2 equivalent.')">
      <div class="minicard">
      {{`${state.gameState.world.emissions.toFixed(1)}Gt`}}
      </div>
      <img :src="icons.emissions" />
      <div class="dashboard--item-name">Emissions</div>
    </div>
    <div class="dashboard--item" v-tip="factors.tips.land('Current land use.')">
      <div class="minicard">
      {{format.landUsePercent(state.gameState.resources_demand.land).toFixed(0)}}%
      </div>
      <img :src="icons.land" />
      <div class="dashboard--item-name">Land Use</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
        <span :style="{color: waterStress.color}">{{waterStress.label}}</span>
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
        <span :style="{color: extinction.color}">{{extinction.label}}</span>
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
</div>
</template>

<script>
import game from '/src/game';
import state from '/src/state';
import format from '/src/display/format';
import intensity from '/src/display/intensity';
import MiniCard from 'components/cards/mini/MiniCard.vue';

export default {
  components: {
    MiniCard
  },
  data() {
    return {
      state
    }
  },
  computed: {
    extinction() {
      let int = intensity.scale(state.gameState.world.extinction_rate, 'extinction');
      return {
        label: intensity.describe(int),
        color: intensity.color(int, false)
      }
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
    waterStress() {
      let label;
      let percentUse = format.waterUsePercent(state.gameState.resources_demand.water);
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
    }
  },
}
</script>

<style>
.planning--dashboard {
	display: flex;
	flex-wrap: wrap;
	justify-content: space-evenly;
}
.dashboard--item {
  margin: 1.5em 1em;
  position: relative;
  width: 120px;
  text-align: center;
}
.dashboard--item .minicard {
  color: #fff;
  font-size: 1.3em;
  width: 120px;
  height: 80px;
}
.dashboard--item img {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 32px;
}

.planning--dashboard .minicard {
  background: #222;
}
</style>
