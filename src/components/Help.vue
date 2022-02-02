<template>
<div class="help-tip" :style="style" @click="hide" v-if="!state.help[text] && !state.hideHelp">
  <img :src="icons.help" /> {{text}}
</div>
</template>

<script>
import state from '/src/state';

export default {
  props: ['text', 'x', 'y', 'center'],
  data() {
    return {
      state
    }
  },
  computed: {
    style() {
      let style = {
        left: this.x,
        top: this.y,
      };
      if (this.center) {
        style.transform = 'translate(-50%, -50%)';
      }
      return style;
    },
  },
  methods: {
    hide() {
      state.help[this.text] = true;
    }
  }
}
</script>

<style>
.help-tip {
  position: absolute;
  background: #FFFCE2;
  border: 1px solid #000000;
  font-family: 'W95FA', monospace;
  font-size: 0.7em;
  padding: 0.5em 1em;
  z-index: 10;
  box-shadow: 2px 2px 0 #000000;
  width: fit-content;
}
.help-tip img {
  top: 1px;
  position: relative;
}
</style>
