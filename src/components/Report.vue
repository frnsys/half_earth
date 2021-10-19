<template>
  <Hud />
  <h2>REPORT</h2>
  <h2>Temperature: {{state.cycleStartState.temperature.toFixed(1)}} -> {{state.gameState.world.temperature.toFixed(1)}} ({{politicalCapital.temperature}})</h2>
  <h2>Contentedness: {{state.cycleStartState.contentedness.toFixed(0)}} -> {{state.gameState.contentedness.toFixed(0)}} ({{politicalCapital.contentedness}})</h2>
  <h2>Extinction Rate: {{state.cycleStartState.extinctionRate}} -> {{state.gameState.world.extinction_rate}} ({{politicalCapital.extinctionRate}})</h2>
  <h2 v-if="lost">you lost</h2>
  <button @click="nextPhase">Next</button>
</template>

<script>
import state from '../state';
import Hud from './Hud.vue';

export default {
  components: {
    Hud
  },
  data() {
    return {
      state
    }
  },
  computed: {
    politicalCapital() {
      let temperatureChange = parseFloat(state.gameState.world.temperature.toFixed(1)) - parseFloat(state.cycleStartState.temperature.toFixed(1));
      let contentednessChange = parseFloat(state.gameState.contentedness.toFixed(0)) - parseFloat(state.cycleStartState.contentedness.toFixed(0));
      let extinctionRateChange = state.gameState.world.extinction_rate - state.cycleStartState.extinctionRate;
      return {
        temperature: Math.round(temperatureChange * -10),
        contentedness: Math.round(contentednessChange/2),
        extinctionRate: Math.round(extinctionRateChange * 5),
      }
    }
  },
  methods: {
    nextPhase() {
      state.politicalCapital +=
        this.politicalCapital.temperature +
        this.politicalCapital.contentedness +
        this.politicalCapital.exinctionRate;

      let lost = state.gameState.political_capital <= 0;
      if (lost) {
        state.phase = 'BREAK';
      } else {
        state.phase = 'PLANNING';
      }
    }
  }
}
</script>

<style scoped>
h2 {
  color: #fff;
}
</style>
