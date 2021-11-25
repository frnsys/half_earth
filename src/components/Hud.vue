<template>
  <div class="hud">
    <div>{{state.gameState.world.year}}</div>
    <div v-tip="{icon: 'political_capital', text: 'How much political capital you have. Political capital is what you spend to implement your plans.'}">
      <img :src="icons.political_capital">{{state.gameState.political_capital}}
    </div>
    <div v-tip="{icon: 'extinction_rate', text: 'The current extinction rate. High land use and other factors increase this, and with it, the risk of ecological collapse.'}">
      <img :src="icons.extinction_rate">{{state.gameState.world.extinction_rate.toFixed(0)}}
    </div>
    <div :class="{'bad': state.gameState.contentedness < 0}" v-tip="{icon: 'contentedness', text: 'How people around the world feel about the state of things.'}">
      <img :src="icons.contentedness">{{state.gameState.contentedness.toFixed(0)}}
    </div>
    <div v-tip="{icon: 'emissions', text: 'Current annual emissions, in gigatonnes of CO2 equivalent.'}">
      <img :src="icons.emissions">{{state.gameState.emissions.toFixed(1)}}
    </div>
    <div v-tip="{icon: 'warming', text: 'The current global temperature anomaly. The higher this is, the more unpredictable the climate becomes.'}">
      <img :src="icons.warming">+{{state.gameState.world.temperature.toFixed(1)}}°C
    </div>
  </div>
</template>

<script>
import state from '../state';

export default {
  data() {
    return {
      state,
    };
  },
};
</script>

<style>
.hud {
  display: flex;
  background: #202020;
  color: #fff;
  justify-content: space-between;
  padding: 0 0.5em;
  font-size: 0.75em;
  z-index: 5;
  height: 16px;
}
.hud img {
  height: 12px;
  width: auto;
  vertical-align: middle;
  margin-right: 2px;
}
</style>
