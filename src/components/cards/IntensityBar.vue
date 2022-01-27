<template>
<div class="intensity-pips">
  <div class="intensity-pip" :style="style(i)" v-for="i in max"></div>
</div>
</template>

<script>
import intensity from '/src/display/intensity';

export default {
  props: {
    intensity: Number,
    invert: Boolean,
    max: {
      type: Number,
      default: 4
    }
  },
  computed: {
    color() {
      return intensity.color(this.intensity, this.invert);
    }
  },
  methods: {
    style(i) {
      if (i <= this.intensity) {
        return {
          background: this.color,
          boxShadow: `0px 0px 5px ${this.color}`,
        }
      } else {
        return {}
      }
    }
  }
}
</script>

<style>
.intensity-pips {
  text-align: center;
  line-height: 1;
  display: flex;
}
.intensity-pip {
  width: 8px;
  height: 6px;
  display: inline-block;
  border-right: 1px solid #000;
  background: #453E3E;
}
.intensity-pip:first-of-type {
  border-radius: 10em 0 0 10em;
}
.intensity-pip:last-of-type {
  border-radius: 0 10em 10em 0;
  border-right: none;
}
</style>
