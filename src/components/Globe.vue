<template>
<div id="globe"></div>
</template>

<script>
import Globe from '../earth/globe';
export default {
  mounted() {
    this.globe = new Globe(this.$el);
    this.globe.render();
    this.globe.init();
  }
}
</script>

<style>
#globe {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  top: 0;
  z-index: -1;
}
</style>
