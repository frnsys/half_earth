<template>
<div class="intensity-icon">
  <img :src="assets.icons[resource]"/>
  <div class="intensity-pips">
    <div class="intensity-pip" :style="{background:color}" v-for="i in intensity"></div>
  </div>
</div>
</template>

<script>
export default {
  props: ['resource', 'intensity'],
  computed: {
    color() {
      if (this.intensity === 1) {
        return '#43CC70';
      } else if (this.intensity === 2) {
        return '#FBC011';
      } else {
        return '#EF3838';
      }
    }
  }
}
</script>

<style>
.intensity-icon {
  width: 30px;
}
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
  margin: 0 2px;
}
</style>
