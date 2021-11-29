<template>
<div class="card-icon">
  <img :src="icons[resource]"/>
  <div class="intensity-pips">
    <div class="intensity-pip" :style="{background:color}" v-for="i in intensity"></div>
  </div>
</div>
</template>

<script>
import display from 'lib/display';

export default {
  props: ['resource', 'intensity', 'invert'],
  computed: {
    color() {
      return display.intensityColor(this.intensity, this.invert);
    }
  }
}
</script>

<style>
.intensity-pips {
  text-align: center;
  line-height: 1;
  margin-top: -8px;
}
.intensity-pip {
  border-radius: 10em;
  background: #aaaaaa;
  width: 5px;
  height: 6px;
  display: inline-block;
  margin: 0 1px;
}
</style>
