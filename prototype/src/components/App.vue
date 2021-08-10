<template>
  <Planning v-if="state.phase == 'PLANNING'" />
  <Stream v-else-if="state.phase == 'IMPLEMENTATION'" />
  <Report v-else-if="state.phase == 'REPORT'" />
</template>

<script>
import state from '../state';
import Planning from './Planning.vue'
import Stream from './Stream.vue'
import Report from './Report.vue'
export default {
  data() {
    return {
      state
    };
  },
  components: {
    Planning,
    Stream,
    Report
  }
}
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inconsolata:wght@400;700&display=swap');

body {
  font-size: 14px;
  font-family: 'Inconsolata', monospace;
}

* {
  box-sizing: border-box;
}

button {
  font-family: 'Inconsolata', monospace;
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
  margin: 0;
  padding: 0;
  list-style-type: none;
}
li {
  margin: 1em 0;
}

main {
  max-width: 640px;
  margin: 0 auto;
}

.cards {
  display: flex;
  align-items: stretch;
  flex-wrap: wrap;
  justify-content: space-around;
  margin-bottom: 1em;
}

.stats {
  display: flex;
  justify-content: space-between;
  border-bottom: 1px solid #000;
}

.icon {
  font-size: 0.6em;
  line-height: 1.8;
  vertical-align: top;
}

p.help {
  font-style: italic;
  color: #777;
}

.bar {
  display: flex;
  padding: 0.5em 0;
  justify-content: space-around;
}
.bar li {
  margin: 0 1em 0 0;
}
</style>
