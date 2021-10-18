<template>
  <div class="hud">
    <div>{{state.gameState.world.year}}</div>
    <div>
      <img src="/assets/icons/extinction.png">{{state.gameState.world.extinction_rate.toFixed(0)}}
    </div>
    <div>
      <img src="/assets/icons/contentedness.png">{{contentedness.toFixed(0)}}
    </div>
    <div>
      <img src="/assets/icons/emissions.png">{{emissions.toFixed(1)}}
    </div>
    <div>
      <img src="/assets/icons/warming.png">+{{state.gameState.world.temperature.toFixed(1)}}°C
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
  computed: {
    contentedness() {
      return state.gameState.world.regions.reduce((acc, r) => {
        return acc + r.base_contentedness + (r.health + r.outlook)/2;
      }, 0)/state.gameState.world.regions.length;
    },
    emissions() {
      let world = state.gameState.world;
      return (world.co2_emissions + (world.n2o_emissions * 298.) + (world.ch4_emissions * 36.)) * 1e-15;
    }
  }
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
  width: 13px;
  vertical-align: middle;
  margin-right: 2px;
}
</style>
