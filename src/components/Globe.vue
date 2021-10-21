<template>
<div id="globe"></div>
</template>

<script>
// Dynamically import (code split) the Globe module.
// It's the only one that uses threejs, which adds like ~300-400KB
// to the bundle size.
const getGlobe = () => import('../earth/globe');

export default {
  mounted() {
    getGlobe().then(({default: Globe}) => {
      this.globe = new Globe(this.$el);
      this.globe.render();
      this.globe.init();
      this.globe.onReady(() => {
        if (this.onReady) {
          this.onReady(this.globe);
        }
      })
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
