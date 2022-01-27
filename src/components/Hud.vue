<template>
  <div class="hud">
    <div>{{state.gameState.world.year}}</div>
    <div v-tip="{icon: 'political_capital', text: 'How much political capital you have. Political capital is what you spend to implement your plans.'}">
      <img :src="icons.hud_political_capital">{{Math.max(state.gameState.political_capital, 0)}}
    </div>
    <div v-tip="factors.tips.biodiversity('The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse.')">
      <img :src="icons.extinction_rate">
      <IntensityBar :intensity="extinction" :max="5" />
    </div>
    <div :class="{'bad': state.gameState.world.contentedness < 0}"
      v-tip="factors.tips.contentedness('How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions.')">
      <img :src="icons.hud_contentedness">
      <IntensityBar :intensity="contentedness" :max="5" :invert="true" />
    </div>
    <div v-tip="{icon: 'warming', text: 'The current global temperature anomaly. The higher this is, the more unpredictable the climate becomes.'}">
      <img :src="icons.warming">+{{state.gameState.world.temperature.toFixed(1)}}°C
    </div>
    <div class="hud-settings">
      <img class="sound-toggle" :src="state.sound ? icons.sound : icons.no_sound" @click="toggleSound" />
    </div>
  </div>
</template>

<script>
import state from '../state';
import IntensityBar from './cards/IntensityBar.vue';
import intensity from '/src/display/intensity';

export default {
  components: {
    IntensityBar,
  },
  data() {
    return {
      state,
    };
  },
  computed: {
    contentedness() {
      return intensity.scale(state.gameState.world.contentedness, 'world_outlook');
    },
    extinction() {
      return intensity.scale(state.gameState.world.extinction_rate, 'extinction');
    },
  },
  methods: {
    toggleSound() {
      state.sound = !state.sound;
      if (state.sound && window.music.paused) {
        window.music.play();
      } else if (!state.sound && !window.music.paused) {
        window.music.pause();
      }
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
  padding: 0.1em 0.5em;
  font-size: 0.75em;
  z-index: 5;
}
.hud img {
  height: 12px;
  width: auto;
  vertical-align: middle;
  margin-right: 2px;
  margin-top: -2px;
}

.stat-pip {
  height: 8px;
}

.hud .sound-toggle {
  margin-top: 0.15em;
  margin-right: 0;
}

.hud-settings {
	padding-left: 0.5em;
	border-left: 1px solid rgba(255,255,255,0.25);
  margin-top: -2px;
  box-shadow: -1px 0 0 #000;
  cursor: pointer;
}

.hud .intensity-pips {
  display: inline-flex;
  margin-left: 2px;
}
</style>
