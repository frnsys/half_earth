<template>
<div class="card-icon">
  <img :src="icons[resource]"/>
  <div class="intensity-pips">
    <div class="intensity-pip" :style="{background:color}" v-for="i in intensity"></div>
  </div>
</div>
</template>

<script>
export default {
  props: ['resource', 'intensity', 'invert'],
  computed: {
    color() {
      let intensity = this.invert ? 5 - this.intensity : this.intensity;
      if (intensity === 1) {
        return '#43CC70';
      } else if (intensity === 2) {
        return '#FBC011';
      } else if (intensity === 3) {
        return '#f28435';
      } else {
        return '#EF3838';
      }
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
