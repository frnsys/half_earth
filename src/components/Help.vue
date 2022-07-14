<template>
<transition name="tipfade">
<div class="help-tip--outer" :style="style"  @click="hide" v-if="!state.help[text] && !state.hideHelp">
  <div class="help-tip--inner" :class="{center : center}">
  <img :src="icons.help" /> {{t(text)}}
</div>
</div>

</transition>
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
      // if (this.center) {
      //   style.transform = 'translate(-50%, -50%)';
      // }
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
.help-tip--outer {
  position: absolute;
  z-index: 10;
  width: fit-content;
  cursor: pointer;
  color: #000;
}
.help-tip--inner {
  background: #FFFCE2;
  border: 1px solid #000000;
  font-family: 'W95FA', monospace;
  font-size: 0.8em;
  padding: 0.5em 1em;
  box-shadow: 2px 2px 0 #000000;
  width: fit-content;
}
.help-tip--inner.center {
  transform: translate(-50%, -50%);
}
.help-tip img {
  top: 1px;
  position: relative;
}
</style>
