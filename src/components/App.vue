<template>
  <template v-if="!started">
    <Start @started="start" />
  </template>
  <template v-else-if="!loaded">
    <Loading @loaded="loaded = true" />
  </template>
  <template v-else>
    <Tip />
    <Cutscene v-if="state.phase == 'INTRO'" />
    <Interstitial v-if="state.phase == 'INTERSTITIAL'" />
    <Planning v-if="state.phase == 'PLANNING'" />
    <Stream v-else-if="state.phase == 'EVENTS'" />
    <Report v-else-if="state.phase == 'REPORT'" />
    <GameOver v-else-if="state.phase == 'GAMEOVER'" />
    <GameWin v-else-if="state.phase == 'GAMEWIN'" />
  </template>
</template>

<script>
import debug from '/src/debug';
import state from '/src/state';
import Tip from './tip/Tip.vue';
import Start from './Start.vue';
import Loading from './Loading.vue';
import GameWin from './phases/GameWin.vue';
import GameOver from './phases/GameOver.vue';
import Cutscene from './phases/Cutscene.vue';
import Interstitial from './phases/Interstitial.vue';
import Report from './phases/Report.vue';
import Stream from './phases/events/Events.vue';
import Planning from './phases/planning/Planning.vue';
import AudioManager from '/src/audio/manager';

// Hacky
window.audioManager = new AudioManager();
if (!state.sound) {
  window.audioManager.muted = true;
}

export default {
  data() {
    return {
      state,
      started: false,
      loaded: false,
    };
  },
  methods: {
    start() {
      this.started = true;
    }
  },
  components: {
    Tip,
    Start,
    Report,
    Stream,
    Planning,
    GameOver,
    GameWin,
    Loading,
    Cutscene,
    Interstitial,
  },
}
</script>
