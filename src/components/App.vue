<template>
  <!-- for dev purposes -->
  <Globe v-if="debug.globe" />
  <EventSwipe v-else-if="debug.swipe" />

  <Planning v-else-if="state.phase == 'PLANNING'" />
  <Stream v-else-if="state.phase == 'IMPLEMENTATION'" />
  <Report v-else-if="state.phase == 'REPORT'" />
</template>

<script>
import state from '../state';
import Stream from './stream/Stream.vue';
import Planning from './planning/Planning.vue';
import Report from './Report.vue';
import EventSwipe from './stream/EventSwipe.vue';
import Globe from './Globe.vue';
export default {
  data() {
    return {
      state
    };
  },
  components: {
    Stream,
    Planning,
    Report,
    Globe,
    EventSwipe
  },
}
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=DM+Mono:ital@0;1&display=swap');

* {
  box-sizing: border-box;
}

html, body {
  margin: 0;
  padding: 0;
  overscroll-behavior-y: contain;
  font-family: "DM Mono", monospace;
}

/* TODO temporary mobile constraint */
body {
  background: #000;
}
main {
  max-width: 480px;
  margin: 0 auto;
  overflow-x: hidden;

  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

button {
  padding: 0.5em 1em;
  border: 1px solid #000;
  cursor: pointer;
  background: #fff;
}
button:hover {
  background: #000;
  color: #fff;
}
button:disabled {
  opacity: 0.5;
  border: 1px solid #aaa;
  pointer-events: none;
}
.actions {
  text-align: center;
  margin: 1em 0;
}
.actions button {
  margin: 0 1em;
}

ul, li {
  list-style-type: none;
  margin: 0;
  padding: 0;
}
figure {
  padding: 0;
  margin: 0;
}

/* For prototyping/testing */
.help {
  font-size: 0.6em;
  font-style: italic;
  color: #888;
}
</style>
