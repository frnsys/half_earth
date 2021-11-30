<template>
  <Tip />
  <Planning v-if="state.phase == 'PLANNING'" />
  <Stream v-else-if="state.phase == 'EVENTS'" />
  <Report v-else-if="state.phase == 'REPORT'" />
  <Break v-else-if="state.phase == 'BREAK'" />
  <End v-else-if="state.phase == 'END'" />
</template>

<script>
import debug from '/src/debug';
import state from '/src/state';
import Tip from './tip/Tip.vue';
import End from './phases/End.vue';
import Break from './phases/Break.vue';
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
    };
  },
  mounted() {
    if (!debug.noSound) {
      window.music.play();
    }
  },
  components: {
    Tip,
    End,
    Break,
    Report,
    Stream,
    Planning,
  },
}
</script>
