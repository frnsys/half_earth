<template>
  <Menu v-if="showMenu" @close="showMenu = false" />
  <div class="hud" v-else>
    <div>{{state.gameState.world.year}}</div>
    <div class="hud-bars">
      <div v-tip="{icon: 'political_capital', text: 'How much political capital you have. Political capital is what you spend to implement your plans.'}">
        <img :src="icons.hud_political_capital">{{Math.max(state.gameState.political_capital, 0)}}
      </div>
      <div v-tip="factors.tips.biodiversity('The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse.')">
        <img :src="icons.hud_extinction_rate">
        <IntensityBar :intensity="extinction" :max="5" />
      </div>
      <div :class="{'bad': state.gameState.world.contentedness < 0}"
        v-tip="factors.tips.contentedness('How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions.')">
        <img :src="icons.hud_contentedness">
        <IntensityBar :intensity="contentedness" :max="5" :invert="true" />
      </div>
      <div v-tip="{icon: 'warming', text: `The current global temperature anomaly is +${state.gameState.world.temperature.toFixed(1)}°C. The higher this is, the more unpredictable the climate becomes.`}">
        <img :src="icons.hud_warming">
        <IntensityBar :intensity="warming" :max="5" />
      </div>
    </div>
    <div class="hud-settings" @click="showMenu = true">
      <img :src="icons.settings" />
      <span>Menu</span>
    </div>
  </div>
</template>

<script>
import state from '../state';
import Menu from 'components/Menu.vue';
import IntensityBar from './cards/IntensityBar.vue';
import intensity from '/src/display/intensity';

export default {
  components: {
    Menu,
    IntensityBar,
  },
  data() {
    return {
      state,
      showMenu: false,
    };
  },
  computed: {
    contentedness() {
      return intensity.scale(state.gameState.world.contentedness, 'world_outlook');
    },
    extinction() {
      return intensity.scale(state.gameState.world.extinction_rate, 'extinction');
    },
    warming() {
      return intensity.scale(state.gameState.world.temperature, 'warming');
    },
  }
};
</script>

<style>
.hud {
  display: flex;
  background: #202020;
  color: #fff;
  justify-content: space-between;
  font-size: 0.75em;
  z-index: 5;
  font-family: 'Inter', sans-serif;
  font-weight: 600;
  box-shadow: 0 1px 4px rgb(0 0 0 / 60%);
}
.hud > div {
  padding: 0.5em 0.25em;
}
.hud > div:first-child {
  padding-left: 0.5em;
}
.hud img {
  height: 12px;
  width: auto;
  vertical-align: middle;
  margin-right: 2px;
  margin-top: -2px;
}
.hud-settings img {
  margin-right: 0;
  margin-top: -3px;
}

.stat-pip {
  height: 8px;
}

.hud-settings {
  padding: 0.5em 0.5em !important;
	border-left: 1px solid rgba(255,255,255,0.25);
  box-shadow: -1px 0 0 #000;
  cursor: pointer;
}

.hud .intensity-pips {
  display: inline-flex;
  margin-left: 2px;
}

.hud-bars {
  display: flex;
  flex: 1;
  justify-content: space-around;
  max-width: 320px;
}
.hud-bars > div {
  margin: 0 0.25em;
}

.hud-settings span {
  margin-left: 5px;
  text-transform: uppercase;
  font-size: 0.8em;
  position: relative;
  top: -1px;
  vertical-align: middle;
}

@media only screen and (max-width: 520px) {
  .hud-settings span {
    display: none;
  }
}
</style>
