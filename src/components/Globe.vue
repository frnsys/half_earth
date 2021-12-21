<template>
<div id="globe"></div>
</template>

<script>
// Dynamically import (code split) the Globe module.
// It's the only one that uses threejs, which adds like ~300-400KB
// to the bundle size.
const getGlobe = () => import('../earth/globe');

let globe = null;

export default {
  mounted() {
    getGlobe().then(({default: Globe}) => {
      if (globe == null) {
        globe = new Globe(this.$el);
        globe.render();
        globe.init();
        globe.onReady(() => {
          if (this.onReady) {
            this.onReady(globe);
          }
        });
      } else {
        globe.setEl(this.$el);
        if (this.onReady) {
          this.onReady(globe);
        }
      }
      this.globe = globe;
    });
  },
  methods: {
    onReady(fn) {
      this.onReady = fn;
    }
  }
}
</script>

<style>
#globe {
  flex: 1;
}
</style>
