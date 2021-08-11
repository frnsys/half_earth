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
  flex: 1;
}
</style>
