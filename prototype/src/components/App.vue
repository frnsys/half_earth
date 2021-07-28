<template>
  <Planning v-if="phase == 'PLANNING'" />
  <Implementation v-else-if="phase == 'IMPLEMENTATION'" />
</template>

<script>
import state from '../state';
import Planning from './Planning.vue'
import Implementation from './Implementation.vue'
export default {
  data() {
    return state;
  },
  components: {
    Planning,
    Implementation
  },
}
</script>

<style>
body {
  font-family: monospace;
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

ul, li {
  margin: 0;
  padding: 0;
  list-style-type: none;
}
ul {
  margin: 1em 0;
}

main {
  max-width: 640px;
  margin: 0 auto;
}
</style>
