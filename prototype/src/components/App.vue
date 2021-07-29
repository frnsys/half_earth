<template>
  <Planning v-if="phase == 'PLANNING'" />
  <Implementation v-else-if="phase == 'IMPLEMENTATION'" />
  <Report v-else-if="phase == 'REPORT'" />
</template>

<script>
import state from '../state';
import Planning from './Planning.vue'
import Implementation from './Implementation.vue'
import Report from './Report.vue'
export default {
  data() {
    return state;
  },
  components: {
    Planning,
    Implementation,
    Report
  },
}
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inconsolata:wght@400;700&display=swap');

body {
  font-size: 14px;
  font-family: 'Inconsolata', monospace;
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
.actions button {
  margin: 1em auto 0 auto;
  display: block;
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
</style>
