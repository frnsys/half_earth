<template>
  <Tip />
  <Planning v-if="state.phase == 'PLANNING'" />
  <Stream v-else-if="state.phase == 'EVENTS'" />
  <Report v-else-if="state.phase == 'REPORT'" />
  <Break v-else-if="state.phase == 'BREAK'" />
</template>

<script>
import state from '/src/state';
import Tip from './tip/Tip.vue';
import Break from './phases/Break.vue';
import Report from './phases/Report.vue';
import Stream from './phases/events/Events.vue';
import Planning from './phases/planning/Planning.vue';
import Playlist from 'lib/music';

const playlist = [
  '/assets/music/airtone_-_disOrder.mp3',
  '/assets/music/airtone_-_spacetime(whitecube).mp3',
];

export default {
  data() {
    return {
      state,
      playlist: new Playlist(playlist)
    };
  },
  mounted() {
    this.playlist.play();
  },
  components: {
    Tip,
    Break,
    Report,
    Stream,
    Planning,
  },
}
</script>
