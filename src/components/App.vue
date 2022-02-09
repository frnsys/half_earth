<template>
  <template v-if="!loaded">
    <Loading  @loaded="loaded = true" />
  </template>
  <template v-else>
    <Tip />
    <Planning v-if="state.phase == 'PLANNING'" />
    <Stream v-else-if="state.phase == 'EVENTS'" />
    <Report v-else-if="state.phase == 'REPORT'" />
    <GameOver v-else-if="state.phase == 'BREAK'" />
    <GameWin v-else-if="state.phase == 'END'" />
  </template>
</template>

<script>
import debug from '/src/debug';
import state from '/src/state';
import Tip from './tip/Tip.vue';
import Loading from './Loading.vue';
import GameWin from './phases/GameWin.vue';
import GameOver from './phases/GameOver.vue';
import Report from './phases/Report.vue';
import Stream from './phases/events/Events.vue';
import Planning from './phases/planning/Planning.vue';
import Playlist from 'lib/playlist';

const playlist = [
  '/assets/music/airtone_-_disOrder.mp3',
  '/assets/music/airtone_-_spacetime(whitecube).mp3',
];

// Hacky
window.music = new Playlist(playlist);

export default {
  data() {
    return {
      state,
      loaded: false,
    };
  },
  mounted() {
    if (!debug.noSound && state.sound) {
      window.music.play();
    }
  },
  components: {
    Tip,
    Report,
    Stream,
    Planning,
    GameOver,
    GameWin,
    Loading,
  },
}
</script>
