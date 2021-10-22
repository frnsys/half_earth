<template>
  <div class="hud">
    <div>{{state.gameState.world.year}}</div>
    <div><img src="/assets/icons/pips/political_capital.png">{{state.gameState.political_capital}}</div>
    <div>
      <img src="/assets/icons/extinction.png">{{state.gameState.world.extinction_rate.toFixed(0)}}
    </div>
    <div>
      <img src="/assets/icons/contentedness.png">{{state.gameState.contentedness.toFixed(0)}}
    </div>
    <div>
      <img src="/assets/icons/emissions.png">{{state.gameState.emissions.toFixed(1)}}
    </div>
    <div>
      <img src="/assets/icons/warming.png">+{{state.gameState.world.temperature.toFixed(1)}}°C
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
  z-index: 1;
}
.hud img {
  height: 12px;
  width: auto;
  vertical-align: middle;
  margin-right: 2px;
}
</style>
