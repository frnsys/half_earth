<template>
  <div class="hud">
    <div>{{state.gameState.world.year}}</div>
    <div v-tip="{icon: 'political_capital', text: 'How much political capital you have. Political capital is what you spend to implement your plans.'}">
      <img :src="icons.political_capital">{{Math.max(state.gameState.political_capital, 0)}}
    </div>
    <div v-tip="factors.tips.biodiversity('The current biodiversity pressure. High land use and other factors increase this, and with it, the risk of ecological collapse.')">
      <img :src="icons.extinction_rate">
      <div class="intensity-pip stat-pip" :style="{background:extinction.color}" v-for="i in extinction.intensity"></div>
    </div>
    <div :class="{'bad': state.gameState.contentedness < 0}"
      v-tip="factors.tips.contentedness('How people around the world feel about the state of things. This is a combination of regional contentedness, crises, and policy decisions.')">
      <img :src="icons.contentedness">
      <div class="intensity-pip stat-pip" :style="{background:contentedness.color}" v-for="i in contentedness.intensity"></div>
    </div>
    <div v-tip="factors.tips.emissions('Current annual emissions, in gigatonnes of CO2 equivalent.')">
      <img :src="icons.emissions">{{state.gameState.emissions.toFixed(1)}}
    </div>
    <div v-tip="{icon: 'warming', text: 'The current global temperature anomaly. The higher this is, the more unpredictable the climate becomes.'}">
      <img :src="icons.warming">+{{state.gameState.world.temperature.toFixed(1)}}°C
    </div>
    <img class="sound-toggle" :src="state.sound ? icons.sound : icons.no_sound" @click="toggleSound" />
  </div>
</template>

<script>
import state from '../state';
import intensity from '/src/display/intensity';

export default {
  data() {
    return {
      state,
    };
  },
  computed: {
    contentedness() {
      let val = intensity.scale(state.gameState.contentedness, 'world_outlook');
      return {
        intensity: val,
        color: intensity.color(val, true)
      }
    },
    extinction() {
      let val = intensity.scale(state.gameState.world.extinction_rate, 'extinction');
      return {
        intensity: val,
        color: intensity.color(val, false)
      }
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
</style>
