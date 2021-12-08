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
    <div class="dashboard--item">
      <div class="minicard">
      {{`${state.gameState.emissions.toFixed(1)}Gt`}}
      </div>
      <img :src="icons.emissions" />
      <div class="dashboard--item-name">Emissions</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
      {{`${((state.gameState.resources_demand.land/104e12)*100).toFixed(0)}%`}}
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
    <div class="dashboard--item">
      <div class="minicard">
        TODO
      </div>
      <img :src="icons.sea_level_rise" />
      <div class="dashboard--item-name">Sea Level Rise</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
        <span :style="{color: extinction.color}">{{extinction.label}}</span>
      </div>
      <img :src="icons.extinction_rate" />
      <div class="dashboard--item-name">Extinction Rate</div>
    </div>
    <div class="dashboard--item">
      <div class="minicard">
        {{`${(state.gameState.population * 1e-9).toFixed(1)}b`}}
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
import display from 'lib/display';
import MiniCard from 'components/cards/MiniCard.vue';

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
      let intensity = display.scaleIntensity(state.gameState.world.extinction_rate, 'extinction');
      let label;
      if (intensity === 0) {
        label = 'Very Low';
      } else if (intensity === 1) {
        label = 'Low';
      } else if (intensity === 2) {
        label = 'Moderate';
      } else if (intensity === 3) {
        label = 'High';
      } else {
        label = 'Very High';
      }
      return {
        label,
        color: display.intensityColor(intensity, false)
      }
    },
    avgHabitability() {
      let totalHabitability = 0;
      state.gameState.world.regions.forEach((r) => {
        totalHabitability += game.regionHabitability(r);
      });
      let avgHabitability = Math.round(totalHabitability/state.gameState.world.regions.length);
      let intensity = display.scaleIntensity(avgHabitability, 'habitability');
      let label;
      if (intensity === 0) {
        label = 'Very Low';
      } else if (intensity === 1) {
        label = 'Low';
      } else if (intensity === 2) {
        label = 'Decent';
      } else if (intensity === 3) {
        label = 'High';
      } else {
        label = 'Very High';
      }
      return {
        label,
        color: display.intensityColor(intensity, true)
      }
    },
    avgIncomeLevel() {
      let totalIncome = 0;
      state.gameState.world.regions.forEach((r) => {
        let income = 0;
        switch (r.income) {
          case 'Low': income = 1; break;
          case 'LowerMiddle': income = 2; break;
          case 'UpperMiddle': income = 3; break;
          case 'High': income = 4; break;
        }
        income += r.development;
        totalIncome += income;
      });
      let avgIncomeLevel = Math.round(totalIncome/state.gameState.world.regions.length);

      let label;
      if (avgIncomeLevel === 1) {
        label = 'Very Low';
      } else if (avgIncomeLevel === 2) {
        label = 'Low';
      } else if (avgIncomeLevel === 3) {
        label = 'Decent';
      } else if (avgIncomeLevel === 4) {
        label = 'High';
      } else {
        label = 'Very High';
      }

      return {
        label,
        color: display.intensityColor(avgIncomeLevel, true)
      }
    },
    waterStress() {
      let label;
      let percentUse = state.gameState.resources_demand.water/45500000000000000.0;
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
        color: display.intensityColor(percentUse * 4, false)
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
