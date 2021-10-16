<template>
  <Interstitial v-if="dialogue" :dialogue="dialogue" />
  <Planning v-if="state.phase == 'PLANNING'" />
  <Stream v-else-if="state.phase == 'EVENTS'" />
  <Report v-else-if="state.phase == 'REPORT'" />
</template>

<script>
import state from '../state';
import Stream from './events/Stream.vue';
import Planning from './planning/Planning.vue';
import Report from './Report.vue';
import Interstitial from './interstitials/Interstitial.vue';

export default {
  data() {
    return {
      state,
      dialogue: null
    };
  },
  components: {
    Stream,
    Planning,
    Report,
    Interstitial,
  },
  mounted() {
    fetch('/assets/content/events/79.json').then((resp) => {
      return resp.json();
    }).then(({dialogue}) => {
      this.dialogue = dialogue;
    });
  }
}
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=DM+Mono:ital,wght@0,400;0,500;1,400&display=swap');

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
  position: relative;

  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

button {
  padding: 0.5em 1em;
  border: 1px solid #000;
  cursor: pointer;
  background: #B9B9B9;
  border-top: 2px solid #F1F1F1;
  border-left: 2px solid #F1F1F1;
  border-right: 2px solid #5D5D5D;
  border-bottom: 2px solid #5D5D5D;
  border-radius: 3px;
}
button:hover {
  background: #D3D3D3;
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

img {
  max-width: 100%;
}
</style>
