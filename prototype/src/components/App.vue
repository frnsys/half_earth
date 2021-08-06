<template>
  <div id="select-prototype" v-if="proto == null">
    Select a prototype:
      <div>
        <span @click="() => setPrototype('spatial')">Spatial</span>
        <p class="help">Implementation phase has you playing projects onto a grid representing a region. This one is probably more meticulous/detailed (much more to keep track of, many more decisions to make) and harder to do on mobile.</p>
      </div>
      <div>
        <span @click="() => setPrototype('stream')">Stream</span>
        <p class="help">Implementation phase has you reacting to events. Projects are decided during the planning phase. This one is probably quicker, more Reigns-like, better for mobile. Space is not as strongly represented but we can still include it with events in different locations.</p>
      </div>
  </div>
  <div v-else>
    <StreamApp v-if="proto == 'stream'" />
    <SpatialApp v-else-if="proto == 'spatial'" />
  </div>
</template>

<script>
import state from '../state';
import StreamApp from './stream/App.vue'
import SpatialApp from './spatial/App.vue'
export default {
  data() {
    return {
      proto: null,
    };
  },
  components: {
    StreamApp,
    SpatialApp,
  },
  methods: {
    setPrototype(t) {
      this.proto = t;
    }
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

#select-prototype span {
  margin-top: 1em;
  cursor: pointer;
  display: block;
}
#select-prototype span:hover {
  text-decoration: underline;
}
#select-prototype p {
  margin-top: 0;
}
</style>
