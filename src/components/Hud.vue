<template>
  <div class="hud">
    <div>{{state.gameState.world.year}}</div>
    <div><img :src="icons.political_capital">{{state.gameState.political_capital}}</div>
    <div>
      <img :src="icons.extinction_rate">{{state.gameState.world.extinction_rate.toFixed(0)}}
    </div>
    <div :class="{'bad': state.gameState.contentedness < 0}">
      <img :src="icons.contentedness">{{state.gameState.contentedness.toFixed(0)}}
    </div>
    <div>
      <img :src="icons.emissions">{{state.gameState.emissions.toFixed(1)}}
    </div>
    <div>
      <img :src="icons.warming">+{{state.gameState.world.temperature.toFixed(1)}}°C
    </div>
  </div>
</template>

<script>
import state from '../state';

// TODO tips on tap of hud icons

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
}
.hud img {
  height: 12px;
  width: auto;
  vertical-align: middle;
  margin-right: 2px;
}
</style>
