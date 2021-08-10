<template>
  <Planning v-if="state.phase == 'PLANNING'" />
  <Stream v-else-if="state.phase == 'IMPLEMENTATION'" />
  <Report v-else-if="state.phase == 'REPORT'" />
</template>

<script>
import state from '../state';
import Stream from './Stream.vue';
import Planning from './Planning.vue';
import Report from './Report.vue';
export default {
  data() {
    return {
      state
    };
  },
  components: {
    Stream,
    Planning,
    Report
  },
}
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=Uchen&display=swap');
html, body {
  margin: 0;
  padding: 0;
  overscroll-behavior-y: contain;
  font-family: "Uchen", serif;
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
</style>
